"""ComfyUI custom node for posting a single IMAGE input as PNG."""

from __future__ import annotations

import struct
import zlib
from http import HTTPStatus
from typing import Iterable
from urllib.error import HTTPError, URLError
from urllib.parse import urlparse
from urllib.request import Request, urlopen


def _normalize_url(url: str) -> str:
    normalized = (url or "").strip()
    if not normalized:
        raise ValueError("Invalid URL: URL is empty")

    parsed = urlparse(normalized)
    if parsed.scheme not in {"http", "https"}:
        raise ValueError("Invalid URL: scheme must be http or https")
    if not parsed.netloc:
        raise ValueError("Invalid URL: host is missing")
    return normalized


def _status_phrase(code: int) -> str:
    try:
        return HTTPStatus(code).phrase
    except ValueError:
        return "Unknown Status"


def _excerpt(body: bytes, limit: int = 120) -> str:
    if not body:
        return ""
    text = body.decode("utf-8", errors="replace")
    compact = " ".join(text.split())
    if len(compact) <= limit:
        return compact
    return compact[: limit - 3] + "..."


def _byte_from_channel(value: object) -> int:
    try:
        numeric = float(value)  # type: ignore[arg-type]
    except (TypeError, ValueError) as exc:
        raise ValueError("Invalid image: non-numeric channel value") from exc

    if numeric <= 1.0:
        numeric *= 255.0
    return max(0, min(255, int(round(numeric))))


def _as_nested_list(images: object) -> list:
    current = images
    for method_name in ("detach", "cpu"):
        method = getattr(current, method_name, None)
        if callable(method):
            current = method()

    tolist = getattr(current, "tolist", None)
    if callable(tolist):
        current = tolist()
    else:
        numpy_like = getattr(current, "numpy", None)
        if callable(numpy_like):
            current = numpy_like()
            tolist = getattr(current, "tolist", None)
            if callable(tolist):
                current = tolist()

    if not isinstance(current, list):
        raise ValueError("Invalid image: unsupported IMAGE container")
    return current


def _encode_png(width: int, height: int, rows: Iterable[bytes], channels: int) -> bytes:
    color_type = 6 if channels == 4 else 2

    def chunk(tag: bytes, payload: bytes) -> bytes:
        length = struct.pack(">I", len(payload))
        crc = zlib.crc32(tag)
        crc = zlib.crc32(payload, crc) & 0xFFFFFFFF
        return length + tag + payload + struct.pack(">I", crc)

    ihdr = struct.pack(">IIBBBBB", width, height, 8, color_type, 0, 0, 0)
    raw = b"".join(b"\x00" + row for row in rows)
    compressed = zlib.compress(raw)

    return b"".join(
        [
            b"\x89PNG\r\n\x1a\n",
            chunk(b"IHDR", ihdr),
            chunk(b"IDAT", compressed),
            chunk(b"IEND", b""),
        ]
    )


def _image_to_png_bytes(images: object) -> bytes:
    batch = _as_nested_list(images)
    if len(batch) != 1:
        raise ValueError("Invalid image: only a single IMAGE input is supported")

    image = batch[0]
    if not isinstance(image, list) or not image:
        raise ValueError("Invalid image: image data is empty")

    first_row = image[0]
    if not isinstance(first_row, list) or not first_row:
        raise ValueError("Invalid image: row data is empty")

    first_pixel = first_row[0]
    if not isinstance(first_pixel, list) or len(first_pixel) not in {3, 4}:
        raise ValueError("Invalid image: pixel must have 3 or 4 channels")

    width = len(first_row)
    height = len(image)
    channels = len(first_pixel)
    rows: list[bytes] = []

    for row in image:
        if not isinstance(row, list) or len(row) != width:
            raise ValueError("Invalid image: row width mismatch")

        encoded = bytearray()
        for pixel in row:
            if not isinstance(pixel, list) or len(pixel) != channels:
                raise ValueError("Invalid image: channel layout mismatch")
            encoded.extend(_byte_from_channel(channel) for channel in pixel)
        rows.append(bytes(encoded))

    return _encode_png(width, height, rows, channels)


def _post_png(url: str, png_bytes: bytes, timeout: int = 10) -> str:
    request = Request(
        url=url,
        data=png_bytes,
        method="POST",
        headers={"Content-Type": "image/png"},
    )

    try:
        with urlopen(request, timeout=timeout) as response:
            status = getattr(response, "status", response.getcode())
            body = response.read()
    except HTTPError as exc:
        detail = _excerpt(exc.read())
        suffix = f" -> {detail}" if detail else ""
        raise RuntimeError(
            f"POST failed: {exc.code} {exc.reason}{suffix}"
        ) from exc
    except URLError as exc:
        raise RuntimeError(f"POST failed: network error -> {exc.reason}") from exc

    if status != 200:
        detail = _excerpt(body)
        suffix = f" -> {detail}" if detail else ""
        raise RuntimeError(f"POST failed: {status} {_status_phrase(status)}{suffix}")

    detail = _excerpt(body)
    suffix = f" -> {detail}" if detail else ""
    return f"POST success: {status} {_status_phrase(status)}{suffix}"


class PhotopainterPngPost:
    @classmethod
    def INPUT_TYPES(cls):
        return {
            "required": {
                "image": ("IMAGE",),
                "url": ("STRING", {"default": "http://127.0.0.1:8000/upload", "multiline": False}),
            }
        }

    RETURN_TYPES = ()
    FUNCTION = "post_image"
    OUTPUT_NODE = True
    CATEGORY = "photopainter/http"

    def post_image(self, image, url):
        normalized_url = _normalize_url(url)
        png_bytes = _image_to_png_bytes(image)
        message = _post_png(normalized_url, png_bytes)
        return {"ui": {"text": [f"{message} [{normalized_url}]"]}, "result": ()}


NODE_CLASS_MAPPINGS = {
    "PhotopainterPngPost": PhotopainterPngPost,
}

NODE_DISPLAY_NAME_MAPPINGS = {
    "PhotopainterPngPost": "PhotoPainter PNG POST",
}

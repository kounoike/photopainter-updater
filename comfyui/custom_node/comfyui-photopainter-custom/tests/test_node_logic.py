from __future__ import annotations

import importlib.util
import socket
import struct
import threading
import unittest
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path


MODULE_PATH = Path(__file__).resolve().parents[1] / "__init__.py"


def load_module():
    spec = importlib.util.spec_from_file_location("photopainter_custom_node", MODULE_PATH)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


class FakeTensor:
    def __init__(self, data):
        self._data = data

    def detach(self):
        return self

    def cpu(self):
        return self

    def tolist(self):
        return self._data


def single_rgb_tensor():
    return FakeTensor(
        [
            [
                [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
                [[0.0, 0.0, 1.0], [1.0, 1.0, 1.0]],
            ]
        ]
    )


def batch_rgb_tensor():
    return FakeTensor(
        [
            [[[1.0, 0.0, 0.0]]],
            [[[0.0, 1.0, 0.0]]],
        ]
    )


class EmptyHandler(BaseHTTPRequestHandler):
    request_count = 0

    def do_POST(self):  # noqa: N802
        type(self).request_count += 1
        self.send_response(200)
        self.end_headers()
        self.wfile.write(b"ok")

    def log_message(self, format, *args):  # noqa: A003
        return


class NodeLogicTests(unittest.TestCase):
    def setUp(self):
        self.module = load_module()
        EmptyHandler.request_count = 0

    def _server(self):
        server = HTTPServer(("127.0.0.1", 0), EmptyHandler)
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        return server, thread

    def test_invalid_url_rejected(self):
        with self.assertRaisesRegex(ValueError, "scheme must be http or https"):
            self.module._normalize_url("ftp://example.com/upload")

    def test_png_encoder_generates_expected_dimensions(self):
        png_bytes = self.module._image_to_png_bytes(single_rgb_tensor())
        self.assertEqual(png_bytes[:8], b"\x89PNG\r\n\x1a\n")
        width, height = struct.unpack(">II", png_bytes[16:24])
        self.assertEqual((width, height), (2, 2))

    def test_multiple_images_are_rejected(self):
        with self.assertRaisesRegex(ValueError, "only a single IMAGE input is supported"):
            self.module._image_to_png_bytes(batch_rgb_tensor())

    def test_network_errors_raise_runtime_error(self):
        probe = socket.socket()
        probe.bind(("127.0.0.1", 0))
        _, port = probe.getsockname()
        probe.close()

        node = self.module.PhotopainterPngPost()
        with self.assertRaisesRegex(RuntimeError, r"POST failed: network error"):
            node.post_image(single_rgb_tensor(), f"http://127.0.0.1:{port}/upload")

    def test_repeated_execution_uses_current_url_each_time(self):
        server1, thread1 = self._server()
        server2, thread2 = self._server()
        try:
            node = self.module.PhotopainterPngPost()
            node.post_image(single_rgb_tensor(), f"http://127.0.0.1:{server1.server_port}/upload")
            node.post_image(single_rgb_tensor(), f"http://127.0.0.1:{server2.server_port}/upload")
        finally:
            server1.shutdown()
            server2.shutdown()
            server1.server_close()
            server2.server_close()
            thread1.join(timeout=2)
            thread2.join(timeout=2)

        self.assertEqual(EmptyHandler.request_count, 2)


if __name__ == "__main__":
    unittest.main()

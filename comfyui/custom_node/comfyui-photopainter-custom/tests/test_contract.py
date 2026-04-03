from __future__ import annotations

import importlib.util
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


class RecordingHandler(BaseHTTPRequestHandler):
    expected_status = 200
    response_body = b"updated image.png from PNG input; stored to 480x800"
    recorded = {}

    def do_POST(self):  # noqa: N802
        length = int(self.headers.get("Content-Length", "0"))
        body = self.rfile.read(length)
        type(self).recorded = {
            "method": "POST",
            "path": self.path,
            "content_type": self.headers.get("Content-Type"),
            "body": body,
        }
        self.send_response(type(self).expected_status)
        self.end_headers()
        self.wfile.write(type(self).response_body)

    def log_message(self, format, *args):  # noqa: A003
        return


class ContractTests(unittest.TestCase):
    def setUp(self):
        self.module = load_module()
        RecordingHandler.recorded = {}

    def _server(self):
        server = HTTPServer(("127.0.0.1", 0), RecordingHandler)
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        return server, thread

    def test_node_metadata_contract(self):
        self.assertIn("PhotopainterPngPost", self.module.NODE_CLASS_MAPPINGS)
        node = self.module.PhotopainterPngPost
        self.assertEqual(node.RETURN_TYPES, ())
        self.assertTrue(node.OUTPUT_NODE)
        self.assertEqual(node.CATEGORY, "photopainter/http")
        self.assertEqual(self.module.NODE_DISPLAY_NAME_MAPPINGS["PhotopainterPngPost"], "PhotoPainter PNG POST")

    def test_success_posts_png_raw_body(self):
        server, thread = self._server()
        try:
            node = self.module.PhotopainterPngPost()
            result = node.post_image(single_rgb_tensor(), f"http://127.0.0.1:{server.server_port}/upload")
        finally:
            server.shutdown()
            server.server_close()
            thread.join(timeout=2)

        recorded = RecordingHandler.recorded
        self.assertEqual(recorded["method"], "POST")
        self.assertEqual(recorded["path"], "/upload")
        self.assertEqual(recorded["content_type"], "image/png")
        self.assertTrue(recorded["body"].startswith(b"\x89PNG\r\n\x1a\n"))
        self.assertEqual(result["result"], ())
        self.assertIn("POST success: 200 OK", result["ui"]["text"][0])
        self.assertIn("updated image.png", result["ui"]["text"][0])

    def test_non_200_response_is_failure(self):
        RecordingHandler.expected_status = 400
        RecordingHandler.response_body = b"invalid payload"
        server, thread = self._server()
        try:
            node = self.module.PhotopainterPngPost()
            with self.assertRaisesRegex(RuntimeError, r"POST failed: 400 Bad Request -> invalid payload"):
                node.post_image(single_rgb_tensor(), f"http://127.0.0.1:{server.server_port}/upload")
        finally:
            server.shutdown()
            server.server_close()
            thread.join(timeout=2)
            RecordingHandler.expected_status = 200
            RecordingHandler.response_body = b"updated image.png from PNG input; stored to 480x800"


if __name__ == "__main__":
    unittest.main()

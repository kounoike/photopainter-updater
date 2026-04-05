from __future__ import annotations

import importlib.util
import os
import sys
import threading
import unittest
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path


MODULE_PATH = Path(__file__).resolve().parents[1] / "__init__.py"


def load_module():
    spec = importlib.util.spec_from_file_location("photopainter_custom_node", MODULE_PATH)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    sys.modules[spec.name] = module
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

    def test_png_post_node_metadata_contract(self):
        self.assertIn("PhotopainterPngPost", self.module.NODE_CLASS_MAPPINGS)
        node = self.module.PhotopainterPngPost
        self.assertEqual(node.RETURN_TYPES, ())
        self.assertTrue(node.OUTPUT_NODE)
        self.assertEqual(node.CATEGORY, "photopainter/http")
        self.assertEqual(self.module.NODE_DISPLAY_NAME_MAPPINGS["PhotopainterPngPost"], "PhotoPainter PNG POST")

    def test_transformers_llm_node_metadata_contract(self):
        self.assertIn("PhotopainterTransformersLlmGenerate", self.module.NODE_CLASS_MAPPINGS)
        self.assertNotIn("PhotopainterLlmGenerate", self.module.NODE_CLASS_MAPPINGS)

        node = self.module.PhotopainterTransformersLlmGenerate
        self.assertEqual(node.RETURN_TYPES, ("STRING", "STRING", "STRING"))
        self.assertEqual(node.RETURN_NAMES, ("output_text", "debug_json", "raw_text"))
        self.assertFalse(node.OUTPUT_NODE)
        self.assertEqual(node.CATEGORY, "photopainter/llm")
        self.assertEqual(
            self.module.NODE_DISPLAY_NAME_MAPPINGS["PhotopainterTransformersLlmGenerate"],
            "PhotoPainter LLM Generate (Transformers)",
        )

        required = node.INPUT_TYPES()["required"]
        self.assertNotIn("model_file", required)
        self.assertEqual(required["quantization_mode"][0], list(self.module.SUPPORTED_QUANTIZATION_MODES))
        self.assertEqual(required["think_mode"][0], list(self.module.SUPPORTED_THINK_MODES))
        self.assertEqual(required["response_budget"][0], list(self.module.SUPPORTED_RESPONSE_BUDGETS))
        self.assertEqual(required["temperature"][1]["default"], 0.7)
        self.assertEqual(required["response_budget"][0][0], "auto")
        self.assertEqual(required["max_tokens"][1]["default"], 512)
        self.assertEqual(required["max_tokens"][1]["max"], 262144)

    def test_llama_cpp_llm_node_metadata_contract(self):
        self.assertIn("PhotopainterLlamaCppLlmGenerate", self.module.NODE_CLASS_MAPPINGS)

        node = self.module.PhotopainterLlamaCppLlmGenerate
        self.assertEqual(node.RETURN_TYPES, ("STRING", "STRING", "STRING"))
        self.assertEqual(node.RETURN_NAMES, ("output_text", "debug_json", "raw_text"))
        self.assertFalse(node.OUTPUT_NODE)
        self.assertEqual(node.CATEGORY, "photopainter/llm")
        self.assertEqual(
            self.module.NODE_DISPLAY_NAME_MAPPINGS["PhotopainterLlamaCppLlmGenerate"],
            "PhotoPainter LLM Generate (llama-cpp)",
        )

        required = node.INPUT_TYPES()["required"]
        self.assertIn("model_file", required)
        self.assertNotIn("quantization_mode", required)
        self.assertNotIn("think_mode", required)
        self.assertEqual(required["response_budget"][0], list(self.module.SUPPORTED_RESPONSE_BUDGETS))
        self.assertEqual(required["temperature"][1]["default"], 0.7)
        self.assertEqual(required["response_budget"][0][0], "auto")
        self.assertEqual(required["max_tokens"][1]["default"], 512)
        self.assertEqual(required["max_tokens"][1]["max"], 262144)

    def test_node_mappings_only_expose_split_llm_nodes(self):
        self.assertEqual(
            set(self.module.NODE_CLASS_MAPPINGS),
            {
                "PhotopainterPngPost",
                "PhotopainterTransformersLlmGenerate",
                "PhotopainterLlamaCppLlmGenerate",
            },
        )
        self.assertEqual(
            set(self.module.NODE_DISPLAY_NAME_MAPPINGS),
            {
                "PhotopainterPngPost",
                "PhotopainterTransformersLlmGenerate",
                "PhotopainterLlamaCppLlmGenerate",
            },
        )

    def test_transformers_debug_contract_exposes_think_off_guarantee_fields(self):
        debug_json = self.module._build_llm_debug_json(
            self.module.GenerationDebugInfo(
                backend="transformers",
                family="qwen",
                think_mode="off",
                quantization_mode="none",
                documented_control_available=True,
                control_kind="qwen_enable_thinking",
                requested_enable_thinking=False,
                fallback_to_generic_prompt=False,
                json_output=False,
                raw_had_think_block=False,
                sanitized_output=False,
                attempts=1,
                retry_count=0,
                retry_reason=None,
                context_window=4096,
                prompt_tokens=64,
                response_budget="manual",
                resolved_max_tokens=32,
                model_file=None,
                off_enforcement_supported=True,
                off_enforcement_guaranteed=True,
                off_failure_reason=None,
            )
        )
        self.assertIn('"off_enforcement_supported": true', debug_json)
        self.assertIn('"off_enforcement_guaranteed": true', debug_json)
        self.assertIn('"off_failure_reason": null', debug_json)

    def test_llm_cache_env_contract(self):
        temp_dir = MODULE_PATH.parent / "tmp-cache"
        try:
            os.environ[self.module.LLM_CACHE_DIR_ENV] = str(temp_dir)
            resolved = self.module._resolve_llm_cache_dir()
            self.assertEqual(resolved, str(temp_dir))
            self.assertTrue(temp_dir.is_dir())
        finally:
            os.environ.pop(self.module.LLM_CACHE_DIR_ENV, None)
            if temp_dir.exists():
                temp_dir.rmdir()

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

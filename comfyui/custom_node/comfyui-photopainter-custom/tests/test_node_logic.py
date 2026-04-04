from __future__ import annotations

import importlib.util
import json
import socket
import struct
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


class FakeJsonSchemaModule:
    class ValidationError(Exception):
        pass

    class validators:
        @staticmethod
        def validator_for(schema):
            class Validator:
                @staticmethod
                def check_schema(candidate):
                    if candidate.get("type") != "object":
                        raise FakeJsonSchemaModule.ValidationError("schema type must be object")

            return Validator

    @staticmethod
    def validate(instance, schema):
        required = schema.get("required", [])
        properties = schema.get("properties", {})
        for key in required:
            if key not in instance:
                raise FakeJsonSchemaModule.ValidationError(f"'{key}' is a required property")
        for key, definition in properties.items():
            if key in instance and definition.get("type") == "string" and not isinstance(instance[key], str):
                raise FakeJsonSchemaModule.ValidationError(f"'{key}' must be string")


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

    def test_invalid_model_id_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "config_error: model_id must be a Hugging Face repo"):
            self.module._build_llm_config(
                backend="transformers",
                model_id="bad-model-id",
                model_file="",
                system_prompt="system",
                user_prompt="user",
                think_mode="off",
                json_output=False,
                json_schema="",
                max_retries=1,
                temperature=0,
                max_tokens=32,
            )

    def test_llm_text_mode_returns_single_string_output(self):
        observed = {}

        def fake_generate(config, retry_feedback=None):
            observed["config"] = config
            observed["retry_feedback"] = retry_feedback
            return "hello from llm"

        self.module._generate_llm_output = lambda config: (fake_generate(config), 1)
        node = self.module.PhotopainterLlmGenerate()
        result = node.generate_text(
            "system",
            "user",
            "transformers",
            "Qwen/Qwen3-0.6B",
            "off",
            False,
            1,
            0.0,
            32,
        )

        self.assertEqual(result["result"], ("hello from llm",))
        self.assertEqual(observed["config"].model_id, "Qwen/Qwen3-0.6B")
        self.assertIn("attempts=1", result["ui"]["text"][0])

    def test_llm_json_mode_returns_json_string_output(self):
        self.module._load_jsonschema_module = lambda: FakeJsonSchemaModule
        self.module._generate_llm_output = lambda config: ('{"positive_prompt":"a","negative_prompt":"b"}', 1)
        node = self.module.PhotopainterLlmGenerate()
        result = node.generate_text(
            "system",
            "user",
            "transformers",
            "Qwen/Qwen3-0.6B",
            "generic",
            True,
            1,
            0.0,
            32,
            json_schema=json.dumps(
                {
                    "type": "object",
                    "required": ["positive_prompt", "negative_prompt"],
                    "properties": {
                        "positive_prompt": {"type": "string"},
                        "negative_prompt": {"type": "string"},
                    },
                }
            ),
        )

        self.assertEqual(json.loads(result["result"][0])["positive_prompt"], "a")

    def test_invalid_schema_is_rejected_before_generation(self):
        self.module._load_jsonschema_module = lambda: FakeJsonSchemaModule
        node = self.module.PhotopainterLlmGenerate()
        with self.assertRaisesRegex(ValueError, r"config_error: json_schema is not a valid schema"):
            node.generate_text(
                "system",
                "user",
                "transformers",
                "Qwen/Qwen3-0.6B",
                "generic",
                True,
                1,
                0.0,
                32,
                json_schema='{"type":"array"}',
            )

    def test_json_parse_retry_then_success(self):
        self.module._load_jsonschema_module = lambda: FakeJsonSchemaModule
        attempts = iter(['not-json', '{"positive_prompt":"ok"}'])

        def fake_attempt(config, retry_feedback=None):
            return next(attempts)

        self.module._run_generation_attempt = fake_attempt
        config = self.module._build_llm_config(
            backend="transformers",
            model_id="Qwen/Qwen3-0.6B",
            model_file="",
            system_prompt="system",
            user_prompt="user",
            think_mode="generic",
            json_output=True,
            json_schema='{"type":"object","required":["positive_prompt"],"properties":{"positive_prompt":{"type":"string"}}}',
            max_retries=1,
            temperature=0,
            max_tokens=32,
        )

        output, attempts_used = self.module._generate_llm_output(config)
        self.assertEqual(json.loads(output)["positive_prompt"], "ok")
        self.assertEqual(attempts_used, 2)

    def test_schema_retry_exhaustion_raises_schema_error(self):
        self.module._load_jsonschema_module = lambda: FakeJsonSchemaModule
        self.module._run_generation_attempt = lambda config, retry_feedback=None: "{}"
        config = self.module._build_llm_config(
            backend="transformers",
            model_id="Qwen/Qwen3-0.6B",
            model_file="",
            system_prompt="system",
            user_prompt="user",
            think_mode="qwen",
            json_output=True,
            json_schema='{"type":"object","required":["positive_prompt"],"properties":{"positive_prompt":{"type":"string"}}}',
            max_retries=1,
            temperature=0,
            max_tokens=32,
        )

        with self.assertRaisesRegex(RuntimeError, r"schema_error: 'positive_prompt' is a required property"):
            self.module._generate_llm_output(config)

    def test_backend_failure_is_not_retried(self):
        self.module._run_generation_attempt = lambda config, retry_feedback=None: (_ for _ in ()).throw(
            RuntimeError("backend_error: model load failed")
        )
        config = self.module._build_llm_config(
            backend="transformers",
            model_id="Qwen/Qwen3-0.6B",
            model_file="",
            system_prompt="system",
            user_prompt="user",
            think_mode="deepseek_r1",
            json_output=False,
            json_schema="",
            max_retries=3,
            temperature=0,
            max_tokens=32,
        )

        with self.assertRaisesRegex(RuntimeError, r"backend_error: model load failed"):
            self.module._generate_llm_output(config)


if __name__ == "__main__":
    unittest.main()

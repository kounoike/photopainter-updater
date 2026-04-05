from __future__ import annotations

import importlib.util
import json
import socket
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
        self._data = list(data)
        self.shape = (1, len(self._data))
        self.device = None

    def to(self, device):
        self.device = device
        return self

    def __getitem__(self, key):
        return self._data[key]

    def tolist(self):
        return list(self._data)


class FakeBatchTensor:
    def __init__(self, data):
        self._data = data

    def detach(self):
        return self

    def cpu(self):
        return self

    def tolist(self):
        return self._data


def single_rgb_tensor():
    return FakeBatchTensor(
        [
            [
                [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
                [[0.0, 0.0, 1.0], [1.0, 1.0, 1.0]],
            ]
        ]
    )


def batch_rgb_tensor():
    return FakeBatchTensor(
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


class FakeTorchModule:
    class cuda:
        @staticmethod
        def is_available():
            return True


class FakeTokenizer:
    def __init__(self):
        self.chat_template_calls = []
        self.model_max_length = 4096

    def apply_chat_template(self, messages, **kwargs):
        self.chat_template_calls.append(kwargs)
        return "PROMPT"

    def __call__(self, prompt_text, return_tensors=None):
        return {
            "input_ids": FakeTensor([10, 11, 12]),
            "attention_mask": FakeTensor([1, 1, 1]),
        }

    def decode(self, tokens, skip_special_tokens=True):
        mapping = {
            (90, 91): "hello world",
            (70, 71): '{"prompt":"ok"}',
        }
        return mapping[tuple(tokens)]


class FakeModel:
    def __init__(self):
        self.hf_device_map = None
        self.to_device = None
        self.generate_calls = []
        self.config = type("Config", (), {"max_position_embeddings": 4096})()

    def to(self, device):
        self.to_device = device
        return self

    def generate(self, **kwargs):
        self.generate_calls.append(kwargs)
        if kwargs.get("prefix_allowed_tokens_fn") is not None:
            return [[10, 11, 12, 70, 71]]
        return [[10, 11, 12, 90, 91]]


class FakeAutoTokenizer:
    last_instance = None

    @classmethod
    def from_pretrained(cls, *args, **kwargs):
        cls.last_instance = FakeTokenizer()
        return cls.last_instance


class FakeAutoModel:
    last_instance = None

    @classmethod
    def from_pretrained(cls, *args, **kwargs):
        cls.last_instance = FakeModel()
        return cls.last_instance


class FakeLlamaInstance:
    def __init__(self):
        self.calls = []
        self._n_ctx = 4096

    def n_ctx(self):
        return self._n_ctx

    def tokenize(self, data, add_bos=False):
        return [1, 2, 3]

    def create_chat_completion(self, **kwargs):
        self.calls.append(kwargs)
        return {"choices": [{"message": {"content": '{"prompt":"ok"}'}}]}


class FakeLlama:
    last_kwargs = None
    last_instance = None

    @classmethod
    def from_pretrained(cls, **kwargs):
        cls.last_kwargs = kwargs
        cls.last_instance = FakeLlamaInstance()
        return cls.last_instance


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
        width = int.from_bytes(png_bytes[16:20], "big")
        height = int.from_bytes(png_bytes[20:24], "big")
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
                temperature=1.0,
                max_tokens=32,
            )

    def test_invalid_schema_is_rejected_before_generation(self):
        self.module._load_jsonschema_module = lambda: FakeJsonSchemaModule
        node = self.module.PhotopainterLlmGenerate()
        with self.assertRaisesRegex(ValueError, r"config_error: json_schema is not a valid schema"):
            node.generate_text(
                "system",
                "user",
                "transformers",
                "Qwen/Qwen3.5-4B",
                "generic",
                True,
                1,
                1.0,
                32,
                json_schema='{"type":"array"}',
            )

    def test_llm_text_mode_returns_single_string_output(self):
        observed = {}

        def fake_generate(config):
            observed["config"] = config
            return "hello from llm", 1

        self.module._generate_llm_output = fake_generate
        node = self.module.PhotopainterLlmGenerate()
        result = node.generate_text(
            "system",
            "user",
            "transformers",
            "Qwen/Qwen3.5-4B",
            "off",
            False,
            1,
            1.0,
            32,
        )

        self.assertEqual(result["result"], ("hello from llm",))
        self.assertEqual(observed["config"].model_id, "Qwen/Qwen3.5-4B")
        self.assertIn("attempts=1", result["ui"]["text"][0])

    def test_llm_json_mode_returns_json_string_output(self):
        self.module._load_jsonschema_module = lambda: FakeJsonSchemaModule
        self.module._generate_llm_output = lambda config: ('{"positive_prompt":"a","negative_prompt":"b"}', 1)
        node = self.module.PhotopainterLlmGenerate()
        result = node.generate_text(
            "system",
            "user",
            "transformers",
            "Qwen/Qwen3.5-4B",
            "generic",
            True,
            1,
            1.0,
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

    def test_qwen_off_uses_documented_thinking_disable_control(self):
        config = self.module._build_llm_config(
            backend="transformers",
            model_id="Qwen/Qwen3.5-4B",
            model_file="",
            system_prompt="system",
            user_prompt="user",
            think_mode="off",
            json_output=False,
            json_schema="",
            max_retries=1,
            temperature=1.0,
            max_tokens=32,
        )
        plan = self.module._resolve_think_control(config)

        self.assertTrue(plan.documented_control_available)
        self.assertEqual(plan.control_kind, "qwen_enable_thinking")

    def test_family_specific_think_mode_requires_matching_family(self):
        config = self.module._build_llm_config(
            backend="transformers",
            model_id="google/gemma-4-E4B",
            model_file="",
            system_prompt="system",
            user_prompt="user",
            think_mode="qwen",
            json_output=False,
            json_schema="",
            max_retries=1,
            temperature=1.0,
            max_tokens=32,
        )
        with self.assertRaisesRegex(RuntimeError, r"think_mode_error: qwen think_mode requires a Qwen family model"):
            self.module._resolve_think_control(config)

    def test_gemma_think_mode_prefixes_system_prompt(self):
        config = self.module._build_llm_config(
            backend="transformers",
            model_id="google/gemma-4-E4B",
            model_file="",
            system_prompt="system",
            user_prompt="user",
            think_mode="gemma",
            json_output=False,
            json_schema="",
            max_retries=1,
            temperature=1.0,
            max_tokens=32,
        )
        plan = self.module._resolve_think_control(config)
        messages = self.module._build_messages(config, plan)
        self.assertTrue(messages[0]["content"].startswith("<|think|>\n"))

    def test_deepseek_r1_mode_uses_family_specific_fallback_plan(self):
        config = self.module._build_llm_config(
            backend="transformers",
            model_id="deepseek-ai/DeepSeek-R1",
            model_file="",
            system_prompt="system",
            user_prompt="user",
            think_mode="deepseek_r1",
            json_output=False,
            json_schema="",
            max_retries=1,
            temperature=1.0,
            max_tokens=32,
        )
        plan = self.module._resolve_think_control(config)
        self.assertEqual(plan.family, "deepseek_r1")
        self.assertFalse(plan.documented_control_available)
        self.assertTrue(plan.fallback_to_generic_prompt)

    def test_json_parse_retry_then_success(self):
        self.module._load_jsonschema_module = lambda: FakeJsonSchemaModule
        attempts = iter(['not-json', '{"positive_prompt":"ok"}'])

        def fake_attempt(config, retry_feedback=None):
            return next(attempts)

        self.module._run_generation_attempt = fake_attempt
        config = self.module._build_llm_config(
            backend="transformers",
            model_id="Qwen/Qwen3.5-4B",
            model_file="",
            system_prompt="system",
            user_prompt="user",
            think_mode="generic",
            json_output=True,
            json_schema='{"type":"object","required":["positive_prompt"],"properties":{"positive_prompt":{"type":"string"}}}',
            max_retries=1,
            temperature=1.0,
            max_tokens=32,
        )

        output, attempts_used = self.module._generate_llm_output(config)
        self.assertEqual(json.loads(output)["positive_prompt"], "ok")
        self.assertEqual(attempts_used, 2)

    def test_qwen_think_block_is_stripped_from_final_output(self):
        config = self.module._build_llm_config(
            backend="transformers",
            model_id="Qwen/Qwen3.5-4B",
            model_file="",
            system_prompt="system",
            user_prompt="user",
            think_mode="off",
            json_output=False,
            json_schema="",
            max_retries=0,
            temperature=1.0,
            max_tokens=32,
        )
        sanitized = self.module._validate_generation_output(
            config,
            "<think>internal reasoning</think>\nFinal answer.",
        )
        self.assertEqual(sanitized, "Final answer.")

    def test_incomplete_qwen_think_block_is_failure(self):
        config = self.module._build_llm_config(
            backend="transformers",
            model_id="Qwen/Qwen3.5-4B",
            model_file="",
            system_prompt="system",
            user_prompt="user",
            think_mode="off",
            json_output=False,
            json_schema="",
            max_retries=0,
            temperature=1.0,
            max_tokens=32,
        )
        with self.assertRaisesRegex(RuntimeError, r"backend_error: generation returned an incomplete qwen think block"):
            self.module._validate_generation_output(config, "<think>internal reasoning")

    def test_schema_retry_exhaustion_raises_schema_error(self):
        self.module._load_jsonschema_module = lambda: FakeJsonSchemaModule
        self.module._run_generation_attempt = lambda config, retry_feedback=None: "{}"
        config = self.module._build_llm_config(
            backend="transformers",
            model_id="Qwen/Qwen3.5-4B",
            model_file="",
            system_prompt="system",
            user_prompt="user",
            think_mode="generic",
            json_output=True,
            json_schema='{"type":"object","required":["positive_prompt"],"properties":{"positive_prompt":{"type":"string"}}}',
            max_retries=1,
            temperature=1.0,
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
            model_id="Qwen/Qwen3.5-4B",
            model_file="",
            system_prompt="system",
            user_prompt="user",
            think_mode="off",
            json_output=False,
            json_schema="",
            max_retries=3,
            temperature=1.0,
            max_tokens=32,
        )

        with self.assertRaisesRegex(RuntimeError, r"backend_error: model load failed"):
            self.module._generate_llm_output(config)

    def test_transformers_generation_passes_qwen_enable_thinking_flag(self):
        self.module._load_transformers_modules = lambda: (FakeTorchModule, FakeAutoModel, FakeAutoTokenizer)
        config = self.module._build_llm_config(
            backend="transformers",
            model_id="Qwen/Qwen3.5-4B",
            model_file="",
            system_prompt="system",
            user_prompt="user",
            think_mode="off",
            json_output=False,
            json_schema="",
            max_retries=0,
            temperature=1.0,
            max_tokens=16,
        )
        plan = self.module._resolve_think_control(config)
        messages = self.module._build_messages(config, plan)

        output = self.module._run_transformers_generation(
            config,
            plan,
            self.module._build_structured_output_plan(config),
            messages,
        )

        self.assertEqual(output, "hello world")
        kwargs = FakeAutoTokenizer.last_instance.chat_template_calls[-1]
        self.assertEqual(kwargs["chat_template_kwargs"]["enable_thinking"], False)

    def test_transformers_json_mode_configures_generation_time_constraint(self):
        self.module._load_transformers_modules = lambda: (FakeTorchModule, FakeAutoModel, FakeAutoTokenizer)
        self.module._load_lmfe_json_parser = lambda: (lambda schema: {"schema": schema})
        self.module._load_lmfe_transformers_builder = lambda: (lambda tokenizer, parser: "prefix-fn")
        self.module._load_jsonschema_module = lambda: FakeJsonSchemaModule
        config = self.module._build_llm_config(
            backend="transformers",
            model_id="Qwen/Qwen3.5-4B",
            model_file="",
            system_prompt="system",
            user_prompt="user",
            think_mode="off",
            json_output=True,
            json_schema='{"type":"object","properties":{"prompt":{"type":"string"}}}',
            max_retries=0,
            temperature=1.0,
            max_tokens=16,
        )
        plan = self.module._resolve_think_control(config)
        messages = self.module._build_messages(config, plan)

        output = self.module._run_transformers_generation(
            config,
            plan,
            self.module._build_structured_output_plan(config),
            messages,
        )

        self.assertEqual(output, '{"prompt":"ok"}')
        generate_kwargs = FakeAutoModel.last_instance.generate_calls[-1]
        self.assertEqual(generate_kwargs["prefix_allowed_tokens_fn"], "prefix-fn")

    def test_structured_output_constraint_failure_is_explicit(self):
        self.module._load_transformers_modules = lambda: (FakeTorchModule, FakeAutoModel, FakeAutoTokenizer)
        self.module._load_lmfe_json_parser = lambda: (lambda schema: {"schema": schema})

        def raise_constraint_failure():
            raise RuntimeError("backend_error: lm-format-enforcer transformers integration is unavailable")

        self.module._load_lmfe_transformers_builder = raise_constraint_failure
        self.module._load_jsonschema_module = lambda: FakeJsonSchemaModule
        config = self.module._build_llm_config(
            backend="transformers",
            model_id="Qwen/Qwen3.5-4B",
            model_file="",
            system_prompt="system",
            user_prompt="user",
            think_mode="off",
            json_output=True,
            json_schema='{"type":"object","properties":{"prompt":{"type":"string"}}}',
            max_retries=0,
            temperature=1.0,
            max_tokens=16,
        )
        plan = self.module._resolve_think_control(config)
        messages = self.module._build_messages(config, plan)

        with self.assertRaisesRegex(RuntimeError, r"backend_error: lm-format-enforcer transformers integration is unavailable"):
            self.module._run_transformers_generation(
                config,
                plan,
                self.module._build_structured_output_plan(config),
                messages,
            )

    def test_llama_cpp_json_mode_configures_logits_processor(self):
        self.module._load_llama_cpp_module = lambda: FakeLlama
        self.module._load_lmfe_json_parser = lambda: (lambda schema: {"schema": schema})
        self.module._load_lmfe_llama_cpp_builder = lambda: (lambda llm, parser: "logits-processor")
        self.module._load_jsonschema_module = lambda: FakeJsonSchemaModule
        config = self.module._build_llm_config(
            backend="llama-cpp",
            model_id="Qwen/Qwen3.5-4B-GGUF",
            model_file="model.gguf",
            system_prompt="system",
            user_prompt="user",
            think_mode="off",
            json_output=True,
            json_schema='{"type":"object","properties":{"prompt":{"type":"string"}}}',
            max_retries=0,
            temperature=1.0,
            max_tokens=16,
        )
        plan = self.module._resolve_think_control(config)
        messages = self.module._build_messages(config, plan)

        output = self.module._run_llama_cpp_generation(
            config,
            plan,
            self.module._build_structured_output_plan(config),
            messages,
        )

        self.assertEqual(output, '{"prompt":"ok"}')
        self.assertEqual(FakeLlama.last_kwargs["filename"], "model.gguf")
        self.assertEqual(FakeLlama.last_instance.calls[-1]["logits_processor"], ["logits-processor"])


if __name__ == "__main__":
    unittest.main()

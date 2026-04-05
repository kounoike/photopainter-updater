"""ComfyUI custom nodes for PhotoPainter workflow integrations."""

from __future__ import annotations

import json
import os
import re
import struct
import zlib
from dataclasses import dataclass
from http import HTTPStatus
from pathlib import Path
from typing import Any, Iterable
from urllib.error import HTTPError, URLError
from urllib.parse import urlparse
from urllib.request import Request, urlopen


MODULE_PATH = Path(__file__).resolve()
LLM_CACHE_DIR_ENV = "COMFYUI_LLM_MODEL_CACHE_DIR"
SUPPORTED_BACKENDS = ("transformers", "llama-cpp")
SUPPORTED_THINK_MODES = ("off", "generic", "qwen", "gemma", "deepseek_r1")
HF_REPO_ID_PATTERN = re.compile(r"^[A-Za-z0-9][\w.-]*/[\w.-]+$")

GENERIC_THINK_PROMPT = "Think carefully before answering, but return only the final answer."
DEEPSEEK_R1_THINK_PROMPT = "Use a DeepSeek-R1-style reasoning pass internally, then return only the final answer."
FINAL_ONLY_PROMPT = "Return only the final answer. Do not output any reasoning or thinking content."


@dataclass(frozen=True)
class LlmNodeConfig:
    backend: str
    model_id: str
    model_file: str | None
    system_prompt: str
    user_prompt: str
    think_mode: str
    json_output: bool
    json_schema_text: str | None
    json_schema: dict[str, Any] | None
    max_retries: int
    temperature: float
    max_tokens: int
    cache_dir: str | None


@dataclass(frozen=True)
class ThinkControlPlan:
    think_mode: str
    family: str | None
    documented_control_available: bool
    control_kind: str | None
    fallback_to_generic_prompt: bool
    prompt_suffix: str | None


@dataclass(frozen=True)
class StructuredOutputPlan:
    enabled: bool
    schema: dict[str, Any] | None


@dataclass(frozen=True)
class GenerationDebugInfo:
    family: str | None
    think_mode: str
    documented_control_available: bool
    control_kind: str | None
    fallback_to_generic_prompt: bool
    json_output: bool
    raw_had_think_block: bool
    sanitized_output: bool
    attempts: int


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
        raise RuntimeError(f"POST failed: {exc.code} {exc.reason}{suffix}") from exc
    except URLError as exc:
        raise RuntimeError(f"POST failed: network error -> {exc.reason}") from exc

    if status != 200:
        detail = _excerpt(body)
        suffix = f" -> {detail}" if detail else ""
        raise RuntimeError(f"POST failed: {status} {_status_phrase(status)}{suffix}")

    detail = _excerpt(body)
    suffix = f" -> {detail}" if detail else ""
    return f"POST success: {status} {_status_phrase(status)}{suffix}"


def _config_error(message: str) -> ValueError:
    return ValueError(f"config_error: {message}")


def _think_mode_error(message: str) -> RuntimeError:
    return RuntimeError(f"think_mode_error: {message}")


def _backend_error(message: str) -> RuntimeError:
    return RuntimeError(f"backend_error: {message}")


def _json_parse_error(message: str) -> RuntimeError:
    return RuntimeError(f"json_parse_error: {message}")


def _schema_error(message: str) -> RuntimeError:
    return RuntimeError(f"schema_error: {message}")


def _normalize_backend(value: object) -> str:
    backend = str(value or "").strip()
    if backend not in SUPPORTED_BACKENDS:
        raise _config_error(f"unsupported backend: {backend or '<empty>'}")
    return backend


def _normalize_hf_repo_id(value: object) -> str:
    model_id = str(value or "").strip()
    if not HF_REPO_ID_PATTERN.match(model_id):
        raise _config_error("model_id must be a Hugging Face repo in user/repo format")
    return model_id


def _normalize_model_file(value: object) -> str | None:
    normalized = str(value or "").strip()
    return normalized or None


def _normalize_prompt(value: object, field_name: str) -> str:
    normalized = str(value or "").strip()
    if not normalized:
        raise _config_error(f"{field_name} must not be empty")
    return normalized


def _normalize_think_mode(value: object) -> str:
    think_mode = str(value or "").strip()
    if think_mode not in SUPPORTED_THINK_MODES:
        raise _config_error(f"unsupported think_mode: {think_mode or '<empty>'}")
    return think_mode


def _normalize_json_schema_text(value: object) -> str | None:
    normalized = str(value or "").strip()
    return normalized or None


def _normalize_max_retries(value: object) -> int:
    try:
        retries = int(value)
    except (TypeError, ValueError) as exc:
        raise _config_error("max_retries must be an integer") from exc
    if retries < 0 or retries > 5:
        raise _config_error("max_retries must be between 0 and 5")
    return retries


def _normalize_temperature(value: object) -> float:
    try:
        temperature = float(value)
    except (TypeError, ValueError) as exc:
        raise _config_error("temperature must be numeric") from exc
    if temperature < 0:
        raise _config_error("temperature must be >= 0")
    return temperature


def _normalize_max_tokens(value: object) -> int:
    try:
        max_tokens = int(value)
    except (TypeError, ValueError) as exc:
        raise _config_error("max_tokens must be an integer") from exc
    if max_tokens <= 0:
        raise _config_error("max_tokens must be > 0")
    return max_tokens


def _resolve_llm_cache_dir() -> str | None:
    configured = os.environ.get(LLM_CACHE_DIR_ENV, "").strip()
    if not configured:
        return None

    path = Path(configured).expanduser()
    try:
        path.mkdir(parents=True, exist_ok=True)
    except OSError as exc:
        raise _config_error(f"{LLM_CACHE_DIR_ENV} is not writable: {path}") from exc
    if not path.is_dir():
        raise _config_error(f"{LLM_CACHE_DIR_ENV} is not a directory: {path}")
    if not os.access(path, os.W_OK):
        raise _config_error(f"{LLM_CACHE_DIR_ENV} is not writable: {path}")
    return str(path)


def _load_jsonschema_module():
    try:
        import jsonschema  # type: ignore
    except ImportError as exc:
        raise _backend_error("jsonschema is not installed") from exc
    return jsonschema


def _load_transformers_modules():
    try:
        import torch  # type: ignore
        from transformers import AutoModelForCausalLM, AutoTokenizer  # type: ignore
    except ImportError as exc:
        raise _backend_error("transformers backend dependencies are not installed") from exc
    return torch, AutoModelForCausalLM, AutoTokenizer


def _load_llama_cpp_module():
    try:
        from llama_cpp import Llama  # type: ignore
    except ImportError as exc:
        raise _backend_error("llama-cpp backend dependencies are not installed") from exc
    return Llama


def _load_lmfe_json_parser():
    try:
        from lmformatenforcer import JsonSchemaParser  # type: ignore
    except ImportError as exc:
        raise _backend_error("lm-format-enforcer is not installed") from exc
    return JsonSchemaParser


def _load_lmfe_transformers_builder():
    try:
        import transformers.tokenization_utils as tokenization_utils  # type: ignore
        from transformers.tokenization_utils_base import PreTrainedTokenizerBase  # type: ignore
    except ImportError as exc:
        raise _backend_error("transformers backend dependencies are not installed") from exc

    if not hasattr(tokenization_utils, "PreTrainedTokenizerBase"):
        tokenization_utils.PreTrainedTokenizerBase = PreTrainedTokenizerBase

    try:
        from lmformatenforcer.integrations.transformers import (  # type: ignore
            build_transformers_prefix_allowed_tokens_fn,
        )
    except ImportError as exc:
        raise _backend_error("lm-format-enforcer transformers integration is unavailable") from exc
    return build_transformers_prefix_allowed_tokens_fn


def _load_lmfe_llama_cpp_builder():
    try:
        from lmformatenforcer.integrations.llamacpp import (  # type: ignore
            build_llamacpp_logits_processor,
        )
    except ImportError as exc:
        raise _backend_error("lm-format-enforcer llama-cpp integration is unavailable") from exc
    return build_llamacpp_logits_processor


def _validate_schema_text(schema_text: str) -> dict[str, Any]:
    try:
        schema = json.loads(schema_text)
    except json.JSONDecodeError as exc:
        raise _config_error(f"json_schema is not valid JSON: {exc.msg}") from exc

    jsonschema = _load_jsonschema_module()
    try:
        validator_cls = jsonschema.validators.validator_for(schema)
        validator_cls.check_schema(schema)
    except Exception as exc:
        raise _config_error(f"json_schema is not a valid schema: {exc}") from exc
    return schema


def _validate_json_instance(instance: object, schema: dict[str, Any]) -> None:
    jsonschema = _load_jsonschema_module()
    try:
        jsonschema.validate(instance=instance, schema=schema)
    except Exception as exc:
        raise _schema_error(str(exc)) from exc


def _detect_model_family(model_id: str) -> str | None:
    lowered = model_id.lower()
    if "deepseek" in lowered and "r1" in lowered:
        return "deepseek_r1"
    if "gemma" in lowered:
        return "gemma"
    if "qwen" in lowered:
        return "qwen"
    return None


def _resolve_think_control(config: LlmNodeConfig) -> ThinkControlPlan:
    family = _detect_model_family(config.model_id)

    if config.think_mode == "off":
        if family == "qwen":
            return ThinkControlPlan("off", family, True, "qwen_enable_thinking", False, FINAL_ONLY_PROMPT)
        if family == "gemma":
            return ThinkControlPlan("off", family, True, "gemma_system_token", False, FINAL_ONLY_PROMPT)
        return ThinkControlPlan("off", family, False, None, False, FINAL_ONLY_PROMPT)

    if config.think_mode == "generic":
        return ThinkControlPlan("generic", family, False, None, True, GENERIC_THINK_PROMPT)

    if config.think_mode == "qwen":
        if family != "qwen":
            raise _think_mode_error("qwen think_mode requires a Qwen family model")
        return ThinkControlPlan("qwen", family, True, "qwen_enable_thinking", False, FINAL_ONLY_PROMPT)

    if config.think_mode == "gemma":
        if family != "gemma":
            raise _think_mode_error("gemma think_mode requires a Gemma family model")
        return ThinkControlPlan("gemma", family, True, "gemma_system_token", False, FINAL_ONLY_PROMPT)

    if config.think_mode == "deepseek_r1":
        if family != "deepseek_r1":
            raise _think_mode_error("deepseek_r1 think_mode requires a DeepSeek-R1 family model")
        return ThinkControlPlan("deepseek_r1", family, False, None, True, DEEPSEEK_R1_THINK_PROMPT)

    raise _config_error(f"unsupported think_mode: {config.think_mode}")


def _build_structured_output_plan(config: LlmNodeConfig) -> StructuredOutputPlan:
    if not config.json_output:
        return StructuredOutputPlan(enabled=False, schema=None)
    return StructuredOutputPlan(enabled=True, schema=config.json_schema)


def _build_llm_config(
    *,
    backend: object,
    model_id: object,
    model_file: object,
    system_prompt: object,
    user_prompt: object,
    think_mode: object,
    json_output: object,
    json_schema: object,
    max_retries: object,
    temperature: object,
    max_tokens: object,
) -> LlmNodeConfig:
    normalized_backend = _normalize_backend(backend)
    normalized_model_id = _normalize_hf_repo_id(model_id)
    normalized_model_file = _normalize_model_file(model_file)
    normalized_system_prompt = _normalize_prompt(system_prompt, "system_prompt")
    normalized_user_prompt = _normalize_prompt(user_prompt, "user_prompt")
    normalized_think_mode = _normalize_think_mode(think_mode)
    normalized_json_output = bool(json_output)
    normalized_json_schema_text = _normalize_json_schema_text(json_schema)
    normalized_retries = _normalize_max_retries(max_retries)
    normalized_temperature = _normalize_temperature(temperature)
    normalized_max_tokens = _normalize_max_tokens(max_tokens)
    cache_dir = _resolve_llm_cache_dir()

    parsed_schema = None
    if normalized_json_output and normalized_json_schema_text is not None:
        parsed_schema = _validate_schema_text(normalized_json_schema_text)

    return LlmNodeConfig(
        backend=normalized_backend,
        model_id=normalized_model_id,
        model_file=normalized_model_file,
        system_prompt=normalized_system_prompt,
        user_prompt=normalized_user_prompt,
        think_mode=normalized_think_mode,
        json_output=normalized_json_output,
        json_schema_text=normalized_json_schema_text,
        json_schema=parsed_schema,
        max_retries=normalized_retries,
        temperature=normalized_temperature,
        max_tokens=normalized_max_tokens,
        cache_dir=cache_dir,
    )


def _build_messages(
    config: LlmNodeConfig,
    think_plan: ThinkControlPlan,
    retry_feedback: str | None = None,
) -> list[dict[str, str]]:
    system_prompt = config.system_prompt
    user_parts = [config.user_prompt]

    if think_plan.control_kind == "gemma_system_token" and config.think_mode != "off":
        system_prompt = f"<|think|>\n{system_prompt}"

    if think_plan.prompt_suffix:
        system_prompt = f"{system_prompt}\n\n{think_plan.prompt_suffix}".strip()

    if config.json_output:
        user_parts.append("Return only valid JSON.")

    if retry_feedback:
        user_parts.append(retry_feedback)

    return [
        {"role": "system", "content": system_prompt},
        {"role": "user", "content": "\n\n".join(part for part in user_parts if part).strip()},
    ]


def _render_messages_as_text(messages: list[dict[str, str]]) -> str:
    rendered = []
    for message in messages:
        role = message.get("role", "user").upper()
        content = message.get("content", "").strip()
        rendered.append(f"{role}:\n{content}")
    rendered.append("ASSISTANT:")
    return "\n\n".join(rendered)


def _build_structured_output_parser(
    config: LlmNodeConfig,
    structured_output_plan: StructuredOutputPlan,
) -> Any | None:
    if not structured_output_plan.enabled:
        return None
    JsonSchemaParser = _load_lmfe_json_parser()
    try:
        return JsonSchemaParser(structured_output_plan.schema)
    except Exception as exc:
        raise _backend_error(f"failed to initialize structured output parser: {exc}") from exc


def _infer_transformers_context_window(model: Any, tokenizer: Any) -> int | None:
    config = getattr(model, "config", None)
    for attr in ("max_position_embeddings", "n_positions", "max_seq_len", "seq_length"):
        value = getattr(config, attr, None) if config is not None else None
        if isinstance(value, int) and value > 0:
            return value

    tokenizer_value = getattr(tokenizer, "model_max_length", None)
    if isinstance(tokenizer_value, int) and 0 < tokenizer_value < 10_000_000:
        return tokenizer_value
    return None


def _infer_llama_cpp_context_window(llm: Any) -> int | None:
    candidate = getattr(llm, "n_ctx", None)
    if callable(candidate):
        try:
            value = candidate()
        except Exception:
            value = None
    else:
        value = candidate
    if isinstance(value, int) and value > 0:
        return value
    return None


def _apply_transformers_chat_template(
    tokenizer: Any,
    messages: list[dict[str, str]],
    think_plan: ThinkControlPlan,
) -> str:
    apply_chat_template = getattr(tokenizer, "apply_chat_template", None)
    if not callable(apply_chat_template):
        return _render_messages_as_text(messages)

    kwargs: dict[str, Any] = {
        "tokenize": False,
        "add_generation_prompt": True,
    }
    if think_plan.control_kind == "qwen_enable_thinking":
        kwargs["chat_template_kwargs"] = {
            "enable_thinking": think_plan.think_mode != "off",
        }

    try:
        return apply_chat_template(messages, **kwargs)
    except TypeError:
        kwargs.pop("chat_template_kwargs", None)
        return apply_chat_template(messages, **kwargs)
    except Exception:
        return _render_messages_as_text(messages)


def _run_transformers_generation(
    config: LlmNodeConfig,
    think_plan: ThinkControlPlan,
    structured_output_plan: StructuredOutputPlan,
    messages: list[dict[str, str]],
) -> str:
    torch, AutoModelForCausalLM, AutoTokenizer = _load_transformers_modules()

    try:
        tokenizer = AutoTokenizer.from_pretrained(
            config.model_id,
            cache_dir=config.cache_dir,
            trust_remote_code=True,
        )
        model = AutoModelForCausalLM.from_pretrained(
            config.model_id,
            cache_dir=config.cache_dir,
            trust_remote_code=True,
            torch_dtype="auto",
        )
    except Exception as exc:
        raise _backend_error(f"failed to load transformers model: {exc}") from exc

    prompt_text = _apply_transformers_chat_template(tokenizer, messages, think_plan)
    inputs = tokenizer(prompt_text, return_tensors="pt")
    context_window = _infer_transformers_context_window(model, tokenizer)

    hf_device_map = getattr(model, "hf_device_map", None)
    if hf_device_map is None:
        device = "cuda" if torch.cuda.is_available() else "cpu"
        if hasattr(model, "to"):
            model.to(device)
        inputs = {name: tensor.to(device) for name, tensor in inputs.items()}

    generate_kwargs: dict[str, Any] = {
        "max_new_tokens": config.max_tokens,
        "do_sample": config.temperature > 0,
    }
    if config.temperature > 0:
        generate_kwargs["temperature"] = config.temperature
    if context_window is not None and (inputs["input_ids"].shape[-1] + config.max_tokens) > context_window:
        raise _backend_error(
            f"requested tokens ({inputs['input_ids'].shape[-1] + config.max_tokens}) exceed model context window of {context_window}"
        )

    parser = _build_structured_output_parser(config, structured_output_plan)
    if parser is not None:
        build_prefix_allowed_tokens_fn = _load_lmfe_transformers_builder()
        try:
            generate_kwargs["prefix_allowed_tokens_fn"] = build_prefix_allowed_tokens_fn(tokenizer, parser)
        except Exception as exc:
            raise _backend_error(f"failed to configure transformers structured output: {exc}") from exc

    try:
        generated = model.generate(**inputs, **generate_kwargs)
    except Exception as exc:
        raise _backend_error(f"transformers generation failed: {exc}") from exc

    prompt_length = inputs["input_ids"].shape[-1]
    output_tokens = generated[0][prompt_length:]
    output_text = tokenizer.decode(output_tokens, skip_special_tokens=True).strip()
    if not output_text:
        raise _backend_error("transformers generation returned empty output")
    return output_text


def _extract_llama_content(response: object) -> str:
    try:
        choice = response["choices"][0]
        message = choice.get("message", {})
        content = message.get("content", "")
    except Exception as exc:
        raise _backend_error(f"llama-cpp response parsing failed: {exc}") from exc
    normalized = str(content).strip()
    if not normalized:
        raise _backend_error("llama-cpp generation returned empty output")
    return normalized


def _sanitize_generation_output(config: LlmNodeConfig, output_text: str) -> tuple[str, bool, bool]:
    normalized = output_text.strip()
    family = _detect_model_family(config.model_id)
    raw_had_think_block = False
    sanitized_output = False

    if family == "qwen":
        if "</think>" in normalized:
            raw_had_think_block = True
            final = normalized.rsplit("</think>", 1)[-1].strip()
            if final:
                sanitized_output = final != normalized
                return final, raw_had_think_block, sanitized_output
            raise _backend_error("generation returned only a qwen think block without final content")
        if normalized.startswith("<think>"):
            raw_had_think_block = True
            raise _backend_error("generation returned an incomplete qwen think block without final content")

    return normalized, raw_had_think_block, sanitized_output


def _run_llama_cpp_generation(
    config: LlmNodeConfig,
    think_plan: ThinkControlPlan,
    structured_output_plan: StructuredOutputPlan,
    messages: list[dict[str, str]],
) -> str:
    Llama = _load_llama_cpp_module()

    kwargs: dict[str, Any] = {
        "repo_id": config.model_id,
        "n_ctx": 0,
        "n_gpu_layers": -1,
        "verbose": False,
    }
    if config.cache_dir is not None:
        kwargs["local_dir"] = config.cache_dir
    if config.model_file is not None:
        kwargs["filename"] = config.model_file

    try:
        llm = Llama.from_pretrained(**kwargs)
    except Exception as exc:
        raise _backend_error(f"failed to load llama-cpp model: {exc}") from exc
    context_window = _infer_llama_cpp_context_window(llm)

    create_kwargs: dict[str, Any] = {
        "messages": messages,
        "max_tokens": config.max_tokens,
        "temperature": config.temperature,
    }

    parser = _build_structured_output_parser(config, structured_output_plan)
    if parser is not None:
        build_llamacpp_logits_processor = _load_lmfe_llama_cpp_builder()
        try:
            create_kwargs["logits_processor"] = [build_llamacpp_logits_processor(llm, parser)]
        except Exception as exc:
            raise _backend_error(f"failed to configure llama-cpp structured output: {exc}") from exc

    try:
        if context_window is not None:
            prompt_text = _render_messages_as_text(messages)
            prompt_tokens = llm.tokenize(prompt_text.encode("utf-8"), add_bos=False)
            requested = len(prompt_tokens) + config.max_tokens
            if requested > context_window:
                raise _backend_error(
                    f"requested tokens ({requested}) exceed model context window of {context_window}"
                )
        response = llm.create_chat_completion(**create_kwargs)
    except Exception as exc:
        raise _backend_error(f"llama-cpp generation failed: {exc}") from exc
    return _extract_llama_content(response)


def _run_generation_attempt(config: LlmNodeConfig, retry_feedback: str | None = None) -> str:
    think_plan = _resolve_think_control(config)
    structured_output_plan = _build_structured_output_plan(config)
    messages = _build_messages(config, think_plan, retry_feedback)

    if config.backend == "transformers":
        return _run_transformers_generation(config, think_plan, structured_output_plan, messages)
    if config.backend == "llama-cpp":
        return _run_llama_cpp_generation(config, think_plan, structured_output_plan, messages)
    raise _config_error(f"unsupported backend: {config.backend}")


def _retry_feedback(attempt: int, error_kind: str, detail: str) -> str:
    return (
        f"Previous attempt {attempt} failed with {error_kind}. "
        f"Return only a corrected response. Detail: {detail}"
    )


def _validate_generation_output(config: LlmNodeConfig, output_text: str) -> tuple[str, dict[str, Any]]:
    normalized, raw_had_think_block, sanitized_output = _sanitize_generation_output(config, output_text)
    if not normalized:
        raise _backend_error("generation returned empty output")

    debug = {
        "raw_had_think_block": raw_had_think_block,
        "sanitized_output": sanitized_output,
    }

    if not config.json_output:
        return normalized, debug

    try:
        parsed = json.loads(normalized)
    except json.JSONDecodeError as exc:
        raise _json_parse_error(exc.msg) from exc

    if config.json_schema is not None:
        _validate_json_instance(parsed, config.json_schema)
    return normalized, debug


def _generate_llm_output(config: LlmNodeConfig) -> tuple[str, GenerationDebugInfo]:
    retry_feedback: str | None = None
    total_attempts = config.max_retries + 1

    for attempt in range(1, total_attempts + 1):
        think_plan = _resolve_think_control(config)
        output_text = _run_generation_attempt(config, retry_feedback)
        try:
            validated, validation_debug = _validate_generation_output(config, output_text)
            debug_info = GenerationDebugInfo(
                family=think_plan.family,
                think_mode=config.think_mode,
                documented_control_available=think_plan.documented_control_available,
                control_kind=think_plan.control_kind,
                fallback_to_generic_prompt=think_plan.fallback_to_generic_prompt,
                json_output=config.json_output,
                raw_had_think_block=validation_debug["raw_had_think_block"],
                sanitized_output=validation_debug["sanitized_output"],
                attempts=attempt,
            )
            return validated, debug_info
        except RuntimeError as exc:
            detail = str(exc)
            if not detail.startswith(("json_parse_error:", "schema_error:")):
                raise
            if attempt >= total_attempts:
                raise
            error_kind, _, message = detail.partition(": ")
            retry_feedback = _retry_feedback(attempt, error_kind, message)

    raise _backend_error("generation exhausted without producing output")


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


class PhotopainterLlmGenerate:
    @classmethod
    def INPUT_TYPES(cls):
        return {
            "required": {
                "system_prompt": ("STRING", {"default": "You are a helpful assistant.", "multiline": True}),
                "user_prompt": ("STRING", {"default": "", "multiline": True}),
                "backend": (list(SUPPORTED_BACKENDS),),
                "model_id": ("STRING", {"default": "Qwen/Qwen3.5-4B", "multiline": False}),
                "model_file": ("STRING", {"default": "", "multiline": False}),
                "think_mode": (list(SUPPORTED_THINK_MODES),),
                "json_output": ("BOOLEAN", {"default": False}),
                "json_schema": ("STRING", {"default": "", "multiline": True}),
                "max_retries": ("INT", {"default": 1, "min": 0, "max": 5}),
                "temperature": ("FLOAT", {"default": 1.0, "min": 0.0, "step": 0.1}),
                "max_tokens": ("INT", {"default": 2048, "min": 1, "max": 8192}),
            },
        }

    RETURN_TYPES = ("STRING", "STRING")
    RETURN_NAMES = ("output_text", "debug_json")
    FUNCTION = "generate_text"
    OUTPUT_NODE = False
    CATEGORY = "photopainter/llm"

    def generate_text(
        self,
        system_prompt,
        user_prompt,
        backend,
        model_id,
        think_mode,
        json_output,
        max_retries,
        temperature,
        max_tokens,
        model_file="",
        json_schema="",
    ):
        config = _build_llm_config(
            backend=backend,
            model_id=model_id,
            model_file=model_file,
            system_prompt=system_prompt,
            user_prompt=user_prompt,
            think_mode=think_mode,
            json_output=json_output,
            json_schema=json_schema,
            max_retries=max_retries,
            temperature=temperature,
            max_tokens=max_tokens,
        )
        output_text, debug_info = _generate_llm_output(config)
        summary = (
            f"LLM success: {config.backend} / {config.model_id} / "
            f"think_mode={config.think_mode} / attempts={debug_info.attempts}"
        )
        debug_json = json.dumps(
            {
                "family": debug_info.family,
                "think_mode": debug_info.think_mode,
                "documented_control_available": debug_info.documented_control_available,
                "control_kind": debug_info.control_kind,
                "fallback_to_generic_prompt": debug_info.fallback_to_generic_prompt,
                "json_output": debug_info.json_output,
                "raw_had_think_block": debug_info.raw_had_think_block,
                "sanitized_output": debug_info.sanitized_output,
                "attempts": debug_info.attempts,
            },
            ensure_ascii=True,
            sort_keys=True,
        )
        return {"ui": {"text": [summary]}, "result": (output_text, debug_json)}


NODE_CLASS_MAPPINGS = {
    "PhotopainterPngPost": PhotopainterPngPost,
    "PhotopainterLlmGenerate": PhotopainterLlmGenerate,
}

NODE_DISPLAY_NAME_MAPPINGS = {
    "PhotopainterPngPost": "PhotoPainter PNG POST",
    "PhotopainterLlmGenerate": "PhotoPainter LLM Generate",
}

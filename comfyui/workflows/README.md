# ComfyUI LLM Workflow Examples

このディレクトリには、backend 別の local LLM node を手動確認するための簡易 workflow 例を置きます。

## Files

- `llm-smoke-transformers.json`
- `llm-smoke-llama-cpp.json`

## Transformers smoke

`llm-smoke-transformers.json` は `PhotoPainter LLM Generate (Transformers)` を 1 つだけ置いた最小例です。

確認観点:

- `quantization_mode` が UI に存在する
- `think_mode` が UI に存在する
- `debug_json` で `quantization_mode`、`control_kind`、`requested_enable_thinking`、`retry_reason` を確認できる
- `raw_text` で sanitize 前の出力を確認できる

## llama-cpp smoke

`llm-smoke-llama-cpp.json` は `PhotoPainter LLM Generate (llama-cpp)` を 1 つだけ置いた最小例です。

確認観点:

- `model_file` が UI に存在する
- `think_mode` と `quantization_mode` が存在しない
- `debug_json` で `model_file`、`context_window`、`retry_reason` を確認できる
- `raw_text` で sanitize 前の出力を確認できる

## Retry の見方

- retry は `json_parse_error` または `schema_error` の場合だけ発生します
- `debug_json.retry_count` が 0 より大きいと retry が発生しています
- `debug_json.retry_reason` で最後に発生した retry 理由を確認できます

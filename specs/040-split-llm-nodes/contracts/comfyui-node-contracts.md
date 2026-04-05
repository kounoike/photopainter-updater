# contracts/comfyui-node-contracts.md

## Transformers Node Contract

### Node Name
- Internal: `PhotopainterTransformersLlmGenerate`
- Display: `PhotoPainter LLM Generate (Transformers)`

### Inputs
- `system_prompt`: STRING
- `user_prompt`: STRING
- `model_id`: STRING
- `quantization_mode`: `none | bnb_8bit | bnb_4bit`
- `think_mode`: `off | generic | qwen | gemma | deepseek_r1`
- `json_output`: BOOLEAN
- `json_schema`: STRING
- `max_retries`: INT
- `temperature`: FLOAT
- `max_tokens`: INT

### Outputs
- `output_text`: STRING
- `debug_json`: STRING
- `raw_text`: STRING

### Notes
- `model_file` は存在しない
- `debug_json` は `quantization_mode`, `requested_enable_thinking` を含む

## LlamaCpp Node Contract

### Node Name
- Internal: `PhotopainterLlamaCppLlmGenerate`
- Display: `PhotoPainter LLM Generate (llama-cpp)`

### Inputs
- `system_prompt`: STRING
- `user_prompt`: STRING
- `model_id`: STRING
- `model_file`: STRING
- `json_output`: BOOLEAN
- `json_schema`: STRING
- `max_retries`: INT
- `temperature`: FLOAT
- `max_tokens`: INT

### Outputs
- `output_text`: STRING
- `debug_json`: STRING
- `raw_text`: STRING

### Notes
- `think_mode` は存在しない
- `quantization_mode` は存在しない
- `debug_json` は GGUF / context window / validation 状態を含む

## Migration Contract
- 旧 `PhotopainterLlmGenerate` は削除する
- README と quickstart に、新 node 名と旧入力からの対応表を記載する

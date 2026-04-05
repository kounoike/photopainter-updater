# data-model.md

## Entities

### TransformersLlmNodeInput
- `system_prompt`: STRING, required
- `user_prompt`: STRING, required
- `model_id`: Hugging Face `user/repo`, required
- `quantization_mode`: enum(`none`, `bnb_8bit`, `bnb_4bit`), required
- `think_mode`: enum(`off`, `generic`, `qwen`, `gemma`, `deepseek_r1`), required
- `json_output`: BOOLEAN, required
- `json_schema`: STRING, optional but always present as empty-or-schema text
- `max_retries`: INT, required
- `temperature`: FLOAT, required
- `max_tokens`: INT, required

Validation rules:
- `model_id` は `user/repo` 形式
- `quantization_mode` は `transformers` 専用 enum のみ
- `think_mode` は family と整合する場合のみ family-specific mode を許可

### LlamaCppLlmNodeInput
- `system_prompt`: STRING, required
- `user_prompt`: STRING, required
- `model_id`: Hugging Face `user/repo`, required
- `model_file`: STRING, required
- `json_output`: BOOLEAN, required
- `json_schema`: STRING, optional but always present as empty-or-schema text
- `max_retries`: INT, required
- `temperature`: FLOAT, required
- `max_tokens`: INT, required

Validation rules:
- `model_id` は GGUF repo を指す `user/repo`
- `model_file` は空文字不可
- `think_mode` と `quantization_mode` は存在しない

### SharedGenerationResult
- `output_text`: STRING
- `debug_json`: STRING (JSON object)
- `raw_text`: STRING

Validation rules:
- `json_output=true` の場合 `output_text` は valid JSON string
- `debug_json` には backend 固有フィールドを含める

## Relationships
- `TransformersLlmNodeInput` は `SharedGenerationResult` を返す
- `LlamaCppLlmNodeInput` は `SharedGenerationResult` を返す
- backend ごとの差分は UI/input contract にあり、結果契約は共有する

## State / Lifecycle Notes
- 各 node 実行は単発推論として扱う
- 生成後は backend メモリ解放を行う
- 旧単一ノードは削除し、2 ノードへの移行を前提とする

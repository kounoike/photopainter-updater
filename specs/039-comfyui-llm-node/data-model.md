# データモデル: ComfyUI local LLM node

## 1. LocalLlmNodeConfig

| 項目 | 型 | 必須 | 説明 |
|------|----|------|------|
| `backend` | `enum` | 必須 | `transformers` または `llama-cpp` |
| `model_id` | `str` | 必須 | Hugging Face 上の model 識別子、または backend が解決可能な model 名 |
| `system_prompt` | `str` | 必須 | system message として渡す文字列 |
| `user_prompt` | `str` | 必須 | user message として渡す文字列 |
| `think_mode` | `enum` | 必須 | `off` / `qwen` / `gemma` / `deepseek_r1` |
| `json_output` | `bool` | 必須 | JSON mode を有効化するか |
| `json_schema` | `str` | 任意 | multiline widget で受ける JSON Schema 文字列 |
| `max_retries` | `int` | 必須 | parse/schema failure に対する retry 上限 |
| `temperature` | `float` | 任意 | backend へ渡す生成パラメータ |
| `max_tokens` | `int` | 任意 | backend へ渡す生成長の上限 |

### Validation Rules

- `backend` は 2 値のどちらかでなければならない
- `model_id` は空文字不可
- `system_prompt` と `user_prompt` は両方空にしない
- `think_mode` は 4 値のいずれかでなければならない
- `json_output=false` のとき `json_schema` は解釈しない
- `max_retries` は 0 以上の小さい整数に制限する

## 2. ModelPathPolicy

| 項目 | 型 | 必須 | 説明 |
|------|----|------|------|
| `env_var_name` | `str` | 必須 | `PHOTOPAINTER_LLM_MODEL_ROOT` |
| `configured_root` | `path \| None` | 条件付き | 環境変数が設定されている場合の値 |
| `resolved_root` | `path \| None` | 条件付き | backend に適用する最終保存先 |
| `uses_backend_default` | `bool` | 必須 | backend 既定保存先へ fallback したか |

### Validation Rules

- 環境変数が未設定なら `uses_backend_default=true`
- 環境変数が設定されている場合、存在しない・書き込み不能 path は設定不備として扱う
- backend は `resolved_root` を model 解決時に利用する

## 3. LlmGenerationAttempt

| 項目 | 型 | 必須 | 説明 |
|------|----|------|------|
| `attempt_index` | `int` | 必須 | 1 始まりの試行回数 |
| `prompt_payload` | `object` | 必須 | backend に渡す最終入力 |
| `raw_text` | `str \| None` | 条件付き | backend が返した生テキスト |
| `failure_kind` | `str \| None` | 条件付き | `json_parse_error` / `schema_error` / `backend_error` など |
| `retryable` | `bool` | 必須 | 再試行対象か |

### State Transitions

1. `prepared`
2. `generated`
3. `validated`
4. `succeeded` または `failed`

`failed` のうち `json_parse_error` と `schema_error` のみ次の `prepared` へ遷移できる。

## 4. JsonValidationContract

| 項目 | 型 | 必須 | 説明 |
|------|----|------|------|
| `json_output` | `bool` | 必須 | JSON mode 有効フラグ |
| `schema_source` | `enum` | 必須 | `none` または `inline_string` |
| `schema_text` | `str \| None` | 条件付き | `json_schema` の中身 |
| `parsed_json` | `object \| None` | 条件付き | `json.loads()` 後の結果 |
| `validation_passed` | `bool` | 必須 | parse + schema 検証の成否 |
| `validation_error` | `str \| None` | 条件付き | parse または schema mismatch の要約 |

### Validation Rules

- `json_output=false` のとき `schema_source=none`
- `json_output=true` かつ `json_schema` が空なら parse 成功のみで通す
- `json_output=true` かつ `json_schema` が非空なら parse 成功後に schema 検証する
- schema 自体が不正なら推論前に失敗する

## 5. LlmGenerationResult

| 項目 | 型 | 必須 | 説明 |
|------|----|------|------|
| `text` | `str` | 必須 | 最終的なプレーンテキスト出力 |
| `json_text` | `str` | 条件付き | JSON mode 成功時の JSON 文字列 |
| `ui_message` | `str` | 必須 | ComfyUI UI に表示する summary |
| `attempt_count` | `int` | 必須 | 実行に使った総試行回数 |
| `success` | `bool` | 必須 | 最終成功判定 |
| `error_kind` | `str \| None` | 条件付き | `config_error` / `backend_error` / `json_parse_error` / `schema_error` |

### Invariants

- `success=true` のとき `text` は空にしない
- `json_output=true` で成功した場合、`json_text` は parse 可能である
- `success=false` のとき node は例外送出により workflow を失敗扱いにする

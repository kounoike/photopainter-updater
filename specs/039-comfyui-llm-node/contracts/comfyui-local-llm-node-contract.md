# Contract: ComfyUI local LLM node

## 1. Node Metadata

| 項目 | 値 |
|------|----|
| Category | `photopainter/llm` |
| Display Name | `PhotoPainter LLM Generate` |
| Output Node | `false` |
| Return Types | `("STRING", "STRING")` |

## 2. Inputs

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `system_prompt` | `STRING` | Yes | system message として使う |
| `user_prompt` | `STRING` | Yes | user message として使う |
| `backend` | `STRING` or choice | Yes | `transformers` / `llama-cpp` |
| `model_id` | `STRING` | Yes | model 識別子 |
| `think_mode` | `STRING` or choice | Yes | `off` / `qwen` / `gemma` / `deepseek_r1` |
| `json_output` | `BOOLEAN` | Yes | JSON mode 有効化 |
| `json_schema` | `STRING` | No | multiline schema 文字列 |
| `max_retries` | `INT` | Yes | parse/schema failure の retry 上限 |
| `temperature` | `FLOAT` | No | 生成パラメータ |
| `max_tokens` | `INT` | No | 生成長上限 |

## 3. Environment Contract

| Name | Required | Meaning |
|------|----------|---------|
| `PHOTOPAINTER_LLM_MODEL_ROOT` | No | local model 保存先。未設定時は backend 既定保存先を使う |

## 4. Success Contract

### Preconditions

- `backend` が対応値である
- `model_id` が空でない
- `think_mode` が対応値である
- `json_schema` が与えられている場合は parse 可能な schema 文字列である

### Success Condition

- backend 推論が成功する
- `json_output=false` の場合は文字列生成が成功する
- `json_output=true` の場合は JSON parse が成功する
- `json_output=true` かつ `json_schema` 非空の場合は schema 検証にも成功する

### Success Result

node は `(text, json_text)` を返す。

- `text`: 常に最終文字列を返す
- `json_text`: JSON mode 成功時は JSON 文字列、非 JSON mode では空文字を許可

例:

```python
{
    "ui": {
        "text": ["LLM success: transformers / Qwen / attempts=1"]
    },
    "result": (
        "plain output text",
        "{\"positive_prompt\":\"...\",\"negative_prompt\":\"...\"}",
    ),
}
```

## 5. Failure Contract

次のケースでは node 実行を失敗扱いにし、例外を送出する。

| Failure Kind | Condition | Retry |
|--------------|-----------|-------|
| `config_error` | backend / `think_mode` 不正、schema 不正、環境変数 path 不正 | No |
| `backend_error` | import 失敗、model load 失敗、推論実行失敗 | No |
| `json_parse_error` | `json_output=true` で JSON parse 不能 | Yes |
| `schema_error` | schema 不一致 | Yes |

### Retry Contract

- retry 対象は `json_parse_error` と `schema_error` のみ
- retry 回数は `max_retries` を超えない
- 最終失敗時は最後の failure kind をメッセージに含める

例:

```text
LLM failed: schema_error after 3 attempts
```

## 6. `think_mode` Contract

| Value | Meaning |
|------|---------|
| `off` | 思考モード無効 |
| `qwen` | Qwen 系に対応する思考モード |
| `gemma` | Gemma 系に対応する思考モード |
| `deepseek_r1` | DeepSeek R1 系に対応する思考モード |

- 上記 4 値のみ初期対応とする
- backend がその family に対応しない場合は暗黙変換せず失敗する

## 7. Out of Scope

- 外部 HTTP endpoint 呼び出し
- prompt planner 固有テンプレートの注入
- 会話履歴永続化
- tool calling や multi-turn orchestration

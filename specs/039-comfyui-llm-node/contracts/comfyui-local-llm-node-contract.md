# Contract: ComfyUI local LLM node

## 1. Node Metadata

| 項目 | 値 |
|------|----|
| Category | `photopainter/llm` |
| Display Name | `PhotoPainter LLM Generate` |
| Output Node | `false` |
| Return Types | `("STRING",)` |

## 2. Inputs

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `system_prompt` | `STRING` | Yes | system message として使う |
| `user_prompt` | `STRING` | Yes | user message として使う |
| `backend` | `STRING` or choice | Yes | `transformers` / `llama-cpp` |
| `model_id` | `STRING` | Yes | Hugging Face Hub の `user/repo` |
| `model_file` | `STRING` | No | 主に `llama-cpp` で repo 内 GGUF を指定する |
| `think_mode` | `STRING` or choice | Yes | `off` / `generic` / `qwen` / `gemma` / `deepseek_r1` |
| `json_output` | `BOOLEAN` | Yes | JSON mode 有効化 |
| `json_schema` | `STRING` | No | multiline schema 文字列 |
| `max_retries` | `INT` | Yes | parse/schema failure の retry 上限 |
| `temperature` | `FLOAT` | No | 生成パラメータ |
| `max_tokens` | `INT` | No | 生成長上限 |

## 3. Environment Contract

| Name | Required | Meaning |
|------|----------|---------|
| `COMFYUI_LLM_MODEL_CACHE_DIR` | No | local model 保存先。未設定時は backend 既定保存先を使う |

## 4. Success Contract

### Preconditions

- `backend` が対応値である
- `model_id` が Hugging Face Hub の `user/repo` 形式で空でない
- `think_mode` が対応値である
- `json_schema` が与えられている場合は parse 可能な schema 文字列である
- `llama-cpp` で repo 内に複数 GGUF がある場合、`model_file` が必要になる

### Success Condition

- backend 推論が成功する
- documented think 制御が利用可能な family では、その制御が優先的に適用される
- `json_output=false` の場合は文字列生成が成功する
- `json_output=true` の場合は generation-time structured output と JSON parse が成功する
- `json_output=true` かつ `json_schema` 非空の場合は schema 検証にも成功する

### Success Result

node は単一 `STRING` を返す。

- `json_output=false`: plain text
- `json_output=true`: valid JSON string

例:

```python
{
    "ui": {
        "text": ["LLM success: transformers / Qwen / attempts=1"]
    },
    "result": ("{\"positive_prompt\":\"...\",\"negative_prompt\":\"...\"}",),
}
```

## 5. Failure Contract

次のケースでは node 実行を失敗扱いにし、例外を送出する。

| Failure Kind | Condition | Retry |
|--------------|-----------|-------|
| `config_error` | backend / `think_mode` 不正、schema 不正、環境変数 path 不正 | No |
| `think_mode_error` | selected family に documented control がなく未対応として扱う場合 | No |
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
| `off` | family ごとの documented control があれば思考モードを無効化する |
| `generic` | best-effort の汎用 mode |
| `qwen` | Qwen 系に対応する思考モード |
| `gemma` | Gemma 系に対応する思考モード |
| `deepseek_r1` | DeepSeek R1 系に対応する思考モード |

- 上記 5 値のみ初期対応とする
- `generic` は family 固有最適化を持たない best-effort mode であり、特定 model での thinking 挙動を保証しない
- family 固有 mode は documented control を優先し、単なる prompt formatting を主手段にしない

## 7. Structured Output Contract

- `json_output=true` の場合、node は generation-time structured output を優先する
- `json_schema` がある場合、node は schema 準拠の結果だけを成功扱いにする
- 自由文 cleanup だけで JSON mode の成功を作ってはならない
- 選択された backend または model 解決経路で constraint を適用できない場合、node は明示 failure を返さなければならない

## 8. Out of Scope

- 外部 HTTP endpoint 呼び出し
- prompt planner 固有テンプレートの注入
- 会話履歴永続化
- tool calling や multi-turn orchestration

# ComfyUI PhotoPainter Custom Nodes

このディレクトリには、PhotoPainter 用の ComfyUI custom node をまとめています。

## Node 一覧

- `PhotoPainter PNG POST`
- `PhotoPainter LLM Generate (Transformers)`
- `PhotoPainter LLM Generate (llama-cpp)`

旧 `PhotoPainter LLM Generate` は削除しました。backend ごとの責務差を UI で明確にするため、
LLM node は 2 つへ分離しています。

## `PhotoPainter PNG POST`

`PhotoPainter PNG POST` は、ComfyUI の `IMAGE` 入力を `Content-Type: image/png`
の raw body として任意 URL へ `POST` する終端ノードです。

### 入力

- `image`: ComfyUI の単一 `IMAGE`
- `url`: `http` または `https` の送信先 URL

### 挙動

- 画像は 1 回の node 実行につき 1 枚だけ送信します
- 送信成功条件は `200 OK` 固定です
- 成功時は UI summary に status と応答本文要約を表示します
- URL 不正、入力不足、接続失敗、`200` 以外の status は例外になり、workflow を失敗扱いにします

## `PhotoPainter LLM Generate (Transformers)`

`transformers` backend 専用の local LLM node です。Hugging Face の通常重みを前提にし、
`think_mode` と `quantization_mode` を扱います。

### 入力

- `system_prompt`
- `user_prompt`
- `model_id`: Hugging Face Hub の `user/repo`
- `quantization_mode`: `none` / `bnb_8bit` / `bnb_4bit`
- `think_mode`: `off` / `generic` / `qwen` / `gemma` / `deepseek_r1`
- `json_output`
- `json_schema`
- `max_retries`
- `temperature`
- `response_budget`: `auto` / `small` / `medium` / `large` / `manual`
- `max_tokens`

### backend 固有仕様

- `model_file` は存在しません
- `quantization_mode` は `bitsandbytes` と `accelerate` を使った load-time quantization です
- Qwen/Gemma 系では documented think 制御を優先します
- `think_mode=off` は best-effort ではありません。documented disable を保証できる経路だけ成功します
- 現行の厳格 `off` 成功経路は Qwen 系の `enable_thinking=False` を通せる tokenizer/chat template 経路です
- Gemma 系や documented disable 非対応 family は `think_mode=off` で明示 failure になります
- tokenizer/chat template が `enable_thinking` 引数を受け付けず fallback した場合も `think_mode=off` は failure になります
- `debug_json` には少なくとも `quantization_mode`、`requested_enable_thinking`、
  `control_kind`、`retry_reason`、`raw_had_think_block`、`sanitized_output`、
  `off_enforcement_supported`、`off_enforcement_guaranteed`、`off_failure_reason`、
  `continuation_supported`、`continuation_used`、`continuation_count`、`continuation_stop_reason` を含みます
- default は `temperature=0.7`、`response_budget=auto`、`max_tokens=512` です。`manual` を選ばない限り `max_tokens` は token 数と think 設定から内部で解決します
- text mode の長文回答は continuation により自動で続きを取りにいけます
- continuation は現状 `transformers` backend の text mode に限定し、`think_mode=off` と `json_output=true` では使いません

## `PhotoPainter LLM Generate (llama-cpp)`

`llama-cpp` backend 専用の local LLM node です。GGUF repo と `model_file` を前提にします。

### 入力

- `system_prompt`
- `user_prompt`
- `model_id`: Hugging Face Hub の `user/repo`
- `model_file`: repo 内 GGUF file 名
- `json_output`
- `json_schema`
- `max_retries`
- `temperature`
- `response_budget`: `auto` / `small` / `medium` / `large` / `manual`
- `max_tokens`

### backend 固有仕様

- `think_mode` は存在しません
- `quantization_mode` は存在しません
- `model_file` は必須です
- `debug_json` には少なくとも `model_file`、`context_window`、`retry_reason` を含みます
- default は `temperature=0.7`、`response_budget=auto`、`max_tokens=512` です。`manual` を選ばない限り `max_tokens` は token 数から内部で解決します

## 共通仕様

### JSON mode

- `json_output=true` のとき、node は generation-time structured output を優先します
- `json_schema` がある場合は `lm-format-enforcer` と `jsonschema` の両方で制約します
- structured output constraint を適用できない場合は自由文 fallback ではなく明示 failure にします

### Retry

- retry は `json_parse_error` または `schema_error` の場合に限ります
- `backend_error` や `think_mode_error` では retry しません
- `think_mode=off` の unsupported / trace violation も retry しません
- `debug_json` で `retry_count` と `retry_reason` を確認できます

### Response Budget

- `response_budget=manual` のときだけ `max_tokens` をそのまま使います
- `auto` は実 token 数から `small` / `medium` / `large` を自動選択します
- `think_mode != off` のときは同じ prompt 長でも少し大きめの budget を選びます
- `debug_json` で `prompt_tokens`、`response_budget`、`resolved_max_tokens` を確認できます

### Output

- 成功時: 3 つの `STRING`
- `output_text`: text mode では plain text、json mode では valid JSON string
- `debug_json`: backend 固有設定を含む JSON object 文字列
- `raw_text`: sanitize 前の生出力
- 失敗時: 例外
- `think_mode=off` では reasoning trace を sanitize して成功扱いにしません

### failure kind

- `config_error`: 入力不正、schema 不正、未解決の model 指定など
- `think_mode_error`: family と `think_mode` の組み合わせ不正
- `backend_error`: import 失敗、model load 失敗、推論実行失敗
- `json_parse_error`: `json_output=true` で JSON parse 不能
- `schema_error`: schema mismatch

### model cache

`COMFYUI_LLM_MODEL_CACHE_DIR` を `.env` に設定すると、ComfyUI container へその値が渡され、
custom node はプロセス環境変数として参照します。未設定時は backend 既定保存先を使います。

```text
COMFYUI_LLM_MODEL_CACHE_DIR=./runpod-volume/llm-models
```

## 旧単一ノードからの移行

### `backend=transformers` だった場合

1. 旧ノードを削除する
2. `PhotoPainter LLM Generate (Transformers)` を配置する
3. `system_prompt`、`user_prompt`、`model_id`、`json_output`、`json_schema` を移す
4. `think_mode=off` を使う場合は、まず `debug_json.off_enforcement_guaranteed=true` を確認する
5. 必要なら `quantization_mode=bnb_4bit` を使う

## `think_mode=off` の見方

- 成功条件:
  - `debug_json.off_enforcement_supported=true`
  - `debug_json.off_enforcement_guaranteed=true`
  - `debug_json.off_failure_reason=null`
  - `raw_text` に `<think>` 相当が無い
- 主な failure:
  - documented disable 非対応 family
  - tokenizer/chat template が `enable_thinking=False` を受け付けない
  - `raw_text` に reasoning trace が出た

## 長文回答 continuation

- 1 回目の generation が token 予算いっぱいで本文途中までしか返していないと判断した場合、node は continuation を試みます
- continuation 成功時は `output_text` と `raw_text` に連結済み本文を返します
- continuation は無限に続かず、既定では最大 12 回の追加 generation までで、回数上限または進展なしで停止します
- `debug_json.continuation_used`、`debug_json.continuation_count`、`debug_json.continuation_stop_reason` で状況を確認できます
- 現在は `transformers` backend の text mode が対象で、`think_mode=off` と `json_output=true` は continuation 対象外です

### `backend=llama-cpp` だった場合

1. 旧ノードを削除する
2. `PhotoPainter LLM Generate (llama-cpp)` を配置する
3. `system_prompt`、`user_prompt`、`model_id`、`model_file`、`json_output`、`json_schema` を移す
4. 旧 `think_mode` と `quantization_mode` は削除する

## 026 との接続例

`docker compose up -d server` で 026 の upload server を起動したあと、ComfyUI から見える URL を
`url` に指定します。

```text
http://192.168.1.10:8000/upload
```

## runtime への配置

repo 管理ソースは `comfyui/custom_node/comfyui-photopainter-custom/` にあります。
`comfyui/runpod/Dockerfile` がこのディレクトリを共通 ComfyUI image の
`/comfyui/custom_nodes/comfyui-photopainter-custom` に copy するため、
container 起動時の追加 mount や copy は不要です。`comfyui-ollama` は同じ Dockerfile
内で導入します。

```bash
docker compose build comfyui
docker compose up -d comfyui
```

repo 側ソースを更新したあとは ComfyUI image を再 build します。

```bash
docker compose up -d --build comfyui
```

## workflow 例

backend 別の簡易 workflow 例は `comfyui/workflows/` にあります。

- `llm-smoke-transformers.json`
- `llm-smoke-llama-cpp.json`

## テスト

host 側の Python 標準ライブラリだけで回る unit test を同梱しています。runtime 依存は
lazy import にしているため、heavy dependency が未導入でも contract / logic test を実行できます。

```bash
python -m unittest discover -s comfyui/custom_node/comfyui-photopainter-custom/tests -v
```

## devcontainer での GPU 検証

repo の `.devcontainer` は GPU を見える前提で調整しています。custom node 配下の
`.venv` を使うと、repo 全体の Python 環境を汚さずに local LLM の確認ができます。

```bash
cd comfyui/custom_node/comfyui-photopainter-custom
python -m venv .venv
source .venv/bin/activate
pip install --extra-index-url https://download.pytorch.org/whl/cu128 torch torchvision torchaudio
pip install transformers jsonschema lm-format-enforcer accelerate bitsandbytes
python - <<'PY'
import torch
print(torch.cuda.is_available())
print(torch.cuda.get_device_name(0) if torch.cuda.is_available() else "no-gpu")
PY
```

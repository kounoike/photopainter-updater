# ComfyUI PhotoPainter Custom Nodes

このディレクトリには、PhotoPainter 用の ComfyUI custom node をまとめています。

## Node 一覧

- `PhotoPainter PNG POST`
- `PhotoPainter LLM Generate`

## `PhotoPainter PNG POST`

`PhotoPainter PNG POST` は、ComfyUI の `IMAGE` 入力を `Content-Type: image/png`
の raw body として任意 URL へ `POST` する終端ノードです。

## 入力

- `image`: ComfyUI の単一 `IMAGE`
- `url`: `http` または `https` の送信先 URL

## 挙動

- 画像は 1 回の node 実行につき 1 枚だけ送信します
- 送信成功条件は `200 OK` 固定です
- 成功時は UI summary に status と応答本文要約を表示します
- URL 不正、入力不足、接続失敗、`200` 以外の status は例外になり、workflow を失敗扱いにします

## `PhotoPainter LLM Generate`

`PhotoPainter LLM Generate` は、ComfyUI workflow 内でローカル LLM 推論を行い、
成功時に単一 `STRING` 出力を返す通常ノードです。`json_output=true` のときは、
その `STRING` が valid JSON 文字列になります。失敗時は例外を投げ、workflow を
失敗扱いにします。

### 入力

- `system_prompt`: system message 文字列
- `user_prompt`: user message 文字列
- `backend`: `transformers` または `llama-cpp`
- `model_id`: Hugging Face Hub の `user/repo`
- `model_file`: 任意。主に `llama-cpp` で repo 内 GGUF を指定
- `think_mode`: `off` / `generic` / `qwen` / `gemma` / `deepseek_r1`
- `json_output`: JSON mode を有効化するか
- `json_schema`: 任意の JSON Schema 文字列
- `max_retries`: parse failure / schema mismatch の retry 上限
- `temperature`, `max_tokens`: 推論パラメータ

### `backend` と `think_mode` の違い

- `backend` は推論実行基盤の選択です
- `think_mode` は model family 向けの prompt formatting preset です

`think_mode` は backend 固有 API の on/off ではありません。node が prompt を整形してから
backend に渡します。

### `think_mode`

- `off`: thinking preset を追加しない
- `generic`: best-effort の汎用 preset。特定 model での thinking 挙動は保証しない
- `qwen`: Qwen 系向け preset
- `gemma`: Gemma 系向け preset
- `deepseek_r1`: DeepSeek R1 系向け preset

### model cache

`COMFYUI_LLM_MODEL_CACHE_DIR` を `.env` に設定すると、ComfyUI container へその値が渡され、
custom node はプロセス環境変数として参照します。未設定時は backend 既定保存先を使います。

```text
COMFYUI_LLM_MODEL_CACHE_DIR=./comfyui-data/llm-models
```

### 出力

- 成功時: 単一 `STRING`
  - text mode: plain text
  - json mode: valid JSON string
- 失敗時: 例外

### failure kind

- `config_error`: 入力不正、schema 不正、未解決の model 指定など
- `backend_error`: import 失敗、model load 失敗、推論実行失敗
- `json_parse_error`: `json_output=true` で JSON parse 不能
- `schema_error`: schema mismatch

## 026 との接続例

`docker compose up -d server` で 026 の upload server を起動したあと、ComfyUI から見える URL を
`url` に指定します。

```text
http://192.168.1.10:8000/upload
```

## runtime への配置

repo 管理ソースは `comfyui/custom_node/comfyui-photopainter-custom/` にあります。
`comfyui/Dockerfile` がこのディレクトリを ComfyUI image の
`/root/ComfyUI/custom_nodes/comfyui-photopainter-custom` に copy するため、
container 起動時の追加 mount や copy は不要です。third-party custom node の clone と依存導入は `comfyui/install-custom-nodes.sh` にまとめています。

```bash
docker compose build comfyui
docker compose up -d comfyui
```

node 読み込み確認:

```bash
docker compose logs --tail=200 comfyui
```

読み込み失敗がなければ ComfyUI の Add Node から `PhotoPainter PNG POST` を選べます。

repo 側ソースを更新したあとは ComfyUI image を再 build します。

```bash
docker compose build comfyui
docker compose up -d comfyui
```

container を作り直して確認したい場合:

```bash
docker compose down
docker compose up -d comfyui
```

## テスト

host 側の Python 標準ライブラリだけで回る unit test を同梱しています。runtime 依存は
lazy import にしているため、heavy dependency が未導入でも contract / logic test を実行できます。

```bash
python -m unittest discover -s comfyui/custom_node/comfyui-photopainter-custom/tests -v
```

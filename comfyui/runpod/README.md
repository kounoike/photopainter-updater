# 共通 RunPod ComfyUI + Ollama runtime

このディレクトリには、local と RunPod Serverless が共有する
`worker-comfyui` ベース runtime assets を置きます。

## 目的

- `worker-comfyui` の起動前に Ollama を自動起動する
- Ollama API を `127.0.0.1:11434` の localhost 限定で使う
- local Compose でも RunPod と同じ image を使う
- RunPod Network Volume がある場合は model を永続化する
- Network Volume がない場合も一時領域で起動継続する
- 事前 pull model を単一 env 値のカンマ区切りで指定できるようにする
- repo 管理 custom node と `comfyui-ollama` を同じ image に同梱する

現行の local Compose `comfyui` service も
[`compose.yml`](/workspaces/photopainter-updater/compose.yml) から
この Dockerfile を build します。

## ファイル

- [Dockerfile](/workspaces/photopainter-updater/comfyui/runpod/Dockerfile): upstream `worker-comfyui` base を継承して Ollama と必要最小限の custom node を追加する
- [start-ollama-worker.sh](/workspaces/photopainter-updater/comfyui/runpod/start-ollama-worker.sh): Ollama 起動、readiness wait、model pull、upstream `/start.sh` への委譲を担当する

repo 管理の `comfyui-photopainter-custom` は `COPY` で同梱し、third-party node は
`ComfyUI-Easy-Use`、`comfyui-ollama`、`ComfyUI-basic_data_handling` を
既定値として `comfy-node-install` で導入します。必要なら build arg
`COMFYUI_CUSTOM_NODES` でカンマ区切りの URL 一覧へ差し替えられます。

## 環境変数

| 変数 | 用途 | 既定値 |
|------|------|--------|
| `RUNPOD_WORKER_COMFYUI_IMAGE` | 継承元 `worker-comfyui` base image（必要時のみ build arg で上書き） | `runpod/worker-comfyui:5.8.5-base-cuda12.8.1` |
| `COMFYUI_CUSTOM_NODES` | `comfy-node-install` へ渡す custom node URL 一覧（build arg） | `ComfyUI-Easy-Use`、`comfyui-ollama`、`ComfyUI-basic_data_handling` |
| `OLLAMA_HOST` | Ollama bind 先 | `127.0.0.1:11434` |
| `RUNPOD_OLLAMA_MODELS_DIR` | 永続利用時の model 保存先 | `/runpod-volume/ollama/models` |
| `EPHEMERAL_OLLAMA_MODELS_DIR` | fallback 時の一時保存先 | `/tmp/ollama/models` |
| `OLLAMA_PULL_MODELS` | 事前取得 model 一覧 | 空 |
| `OLLAMA_HEALTHCHECK_URL` | readiness 判定 URL | `http://127.0.0.1:11434/api/version` |
| `OLLAMA_START_TIMEOUT_SECONDS` | readiness 待機秒数 | `60` |
| `RUNPOD_START_SCRIPT` | 最後に委譲する upstream script | `/start.sh` |

## 起動フロー

1. `start-ollama-worker.sh` が先に起動する
2. `ollama serve` を background 起動する
3. `api/version` が成功するまで待つ
4. `OLLAMA_PULL_MODELS` を trim / split して順番に `ollama pull` する
5. pull 失敗は warning として残し、worker 起動は継続する
6. 最後に upstream `/start.sh` へ `exec` で委譲する

## local Compose

local では `compose.yml` の `comfyui` service がこの Dockerfile を build し、
host 側 `RUNPOD_VOLUME_DIR` を `/runpod-volume` へ bind mount します。
このとき `LOCAL_COMFYUI_ONLY=true` を使い、RunPod handler ではなく ComfyUI 本体だけを起動します。

```bash
mkdir -p ./runpod-volume/{models,ollama/models,input,output,user,dot-cache,dot-local}
cp .env.example .env
docker compose build comfyui
docker compose up -d comfyui
docker compose exec comfyui curl -fsS http://127.0.0.1:11434/api/version
```

local の成功導線では `/runpod-volume` bind mount を省略しません。

## Network Volume

RunPod serverless では Docker Compose の `volumes:` を自前定義する前提ではありません。
Endpoint の Advanced 設定で Network Volume を接続すると、
container 内では `/runpod-volume` として見えます。

- `/runpod-volume` が存在し書き込み可能: `persistent`
- `/runpod-volume` が使えない: `ephemeral`

`ephemeral` では起動継続しますが、container 再作成後の model 再利用は保証しません。

## Model pull

`OLLAMA_PULL_MODELS` は単一 env 値のカンマ区切り一覧です。

例:

```text
OLLAMA_PULL_MODELS=qwen3.5:4b,llama3.2:3b
```

- 空要素は無視します
- 重複 model は 1 回だけ処理します
- 既に存在する model は `reused` として扱います
- pull 失敗は warning としてログへ残します

## ローカル擬似検証

### 永続領域あり

```bash
mkdir -p ./.tmp-runpod-volume

docker build \
  -t photopainter-runpod-comfyui-ollama \
  -f comfyui/runpod/Dockerfile \
  comfyui

docker run --rm --gpus all \
  -p 3000:3000 \
  -v "$PWD/.tmp-runpod-volume:/runpod-volume" \
  -e OLLAMA_PULL_MODELS="qwen3.5:4b" \
  photopainter-runpod-comfyui-ollama
```

### 永続領域なし

```bash
docker run --rm --gpus all \
  -p 3000:3000 \
  -e OLLAMA_PULL_MODELS="qwen3.5:4b" \
  photopainter-runpod-comfyui-ollama
```

### 確認ポイント

- `curl http://127.0.0.1:11434/api/version` が成功する
- ログに `runtime_mode=persistent` または `runtime_mode=ephemeral` が出る
- `model_result ... result=pulled|reused|failed` を追える
- repo 管理 custom node と `comfyui-ollama` が同じ image 前提で利用できる
- upstream `worker-comfyui` development docs に沿って payload を `http://localhost:3000/run` へ送れる

## スコープ境界

Allowed Scope:
- local / RunPod 共通 image customization
- Ollama sidecar の起動と model 保存先切り替え
- RunPod / ローカル検証手順の文書化

Forbidden Scope:
- Ollama API の外部公開
- `KEEP_ALIVE` の image 側固定
- `transformers` node から Ollama への全面移植

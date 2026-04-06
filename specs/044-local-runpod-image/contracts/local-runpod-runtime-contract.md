# Contract: Local / RunPod 共通 ComfyUI runtime

## Purpose

local と RunPod が同じ `worker-comfyui` ベース image を共有し、ComfyUI と Ollama を同居起動するための runtime 契約を定義する。

## Runtime Image Contract

- Dockerfile source: `comfyui/runpod/Dockerfile`
- Wrapper entrypoint: `comfyui/runpod/start-ollama-worker.sh`
- Upstream worker delegation: `/start.sh`
- Ollama API bind: `127.0.0.1:11434`
- Published ComfyUI port: container `8188`

### Guarantees

- `comfyui` コンテナ起動時に `ollama serve` が自動起動する
- `api/version` readiness 確認後に upstream worker が起動する
- `OLLAMA_PULL_MODELS` 指定時は起動時 pull を順次実行する
- model pull 失敗は warning ログで残し、worker 起動は継続する

## Local Compose Contract

- compose service name: `comfyui`
- build context: `./comfyui`
- dockerfile path: `runpod/Dockerfile`
- local required bind mount: host path -> `/runpod-volume`
- local verification command:

```bash
docker compose exec comfyui curl -fsS http://127.0.0.1:11434/api/version
```

### Local Requirements

- local で独立 `ollama` service を定義しない
- `/runpod-volume` bind mount を省略しない
- ComfyUI Web UI は host 側 `COMFYUI_PORT` から到達できる
- Ollama API は host へ公開しない

## Storage Layout Contract

| Path | Purpose |
|------|---------|
| `/runpod-volume/models` | ComfyUI model root |
| `/runpod-volume/ollama/models` | Ollama model storage |
| `/runpod-volume/input` | 必要に応じた local input 永続化 |
| `/runpod-volume/output` | 必要に応じた local output 永続化 |

## Migration Contract

- 旧 local 専用 `comfyui/Dockerfile` は現行 runtime として扱わない
- 旧 local `ollama` service は compose から除去する
- README / quickstart / `comfyui/runpod/README.md` は共通 runtime 前提へ更新する

## Verification Contract

次を満たしたら contract 準拠とみなす。

1. `docker compose config` で `comfyui` service が `runpod/Dockerfile` を使う
2. `docker compose up -d comfyui` 後に ComfyUI Web UI が到達可能
3. `docker compose exec comfyui curl -fsS http://127.0.0.1:11434/api/version` が成功
4. 起動ログに `ollama_api_ready` と `delegating to upstream start script` が出る
5. 文書から独立 `ollama` service 前提を読み取れない

# Quickstart: Local RunPod image 統一

## 1. 前提

- Docker Engine / Docker Compose v2 が使える
- NVIDIA Container Toolkit が使える
- local でも `/runpod-volume` 用の host directory を用意する

例:

```bash
mkdir -p ./runpod-volume/{models,ollama/models,input,output,user,dot-cache,dot-local}
cp .env.example .env
```

## 2. local で共通 image を build / 起動する

```bash
docker compose build comfyui
docker compose up -d comfyui
docker compose logs --tail=200 comfyui
```

期待すること:

- `comfyui` service が `comfyui/runpod/Dockerfile` ベースで build される
- 起動ログに `ollama_api_ready` と `starting local ComfyUI only mode` が出る
- host 側 `http://127.0.0.1:${COMFYUI_PORT:-18188}` で ComfyUI Web UI へ到達できる
- 独立 `ollama` service を起動しなくても `comfyui` コンテナだけで確認できる
- local Compose では RunPod handler を起動しない

## 3. local で Ollama 同居起動を確認する

コンテナ内 localhost で確認する。

```bash
docker compose exec comfyui curl -fsS http://127.0.0.1:11434/api/version
```

期待すること:

- JSON 応答が返る
- host 側へ 11434 を公開しなくても確認できる

## 4. model path を確認する

local でも RunPod と同じ path 前提を使う。

- ComfyUI model root: `/runpod-volume/models`
- Ollama model storage: `/runpod-volume/ollama/models`

必要なら container 内で確認する。

```bash
docker compose exec comfyui sh -lc 'ls -ld /runpod-volume /runpod-volume/models /runpod-volume/ollama/models'
```

必要なら local の ComfyUI 側 path も確認する。

```bash
docker compose exec comfyui sh -lc 'ls -ld /comfyui/models /comfyui/input /comfyui/output /comfyui/user'
```

## 5. 起動時 model pull を使う

`.env` で `OLLAMA_PULL_MODELS` を設定してから再起動する。

例:

```text
OLLAMA_PULL_MODELS=qwen3.5:4b
```

```bash
docker compose up -d --force-recreate comfyui
docker compose logs --tail=200 comfyui
```

期待すること:

- `model_result model=qwen3.5:4b result=pulled` または `result=reused` が出る
- `OLLAMA_PULL_MODELS` は `comfyui` service に渡され、独立 `ollama` service は不要

## 6. RunPod でも同じ image を使う

RunPod 用 build も同じ Dockerfile を使う。

```bash
docker build -t photopainter-runpod-comfyui-ollama -f comfyui/runpod/Dockerfile comfyui
```

RunPod Network Volume を endpoint 側で接続すると `/runpod-volume` に見える。local は bind mount、RunPod は Network Volume という違いだけで、runtime 自体は同じである。

## 7. トラブルシュート

- `/runpod-volume` を mount していない: model path 前提が崩れるので local 成功導線ではない
- `curl http://127.0.0.1:11434/api/version` が失敗する: `docker compose logs comfyui` で `ollama_api_ready` 前後のログを確認する
- 旧 `ollama` service を探している: 現行構成では廃止済みで、Ollama は `comfyui` コンテナ内にいる
- 旧 `comfyui/Dockerfile` や `comfyui-data` を探している: 現行導線は `comfyui/runpod/Dockerfile` と `./runpod-volume` 前提へ移行済み
- local で handler 由来の job error が出る: 現行導線では `LOCAL_COMFYUI_ONLY=true` により RunPod handler を起動しない

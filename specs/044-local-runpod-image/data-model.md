# Data Model: Local RunPod image 統一

## Entity: 共通 `comfyui` runtime

- Description: local と RunPod の両方で使う `worker-comfyui` ベース image と、その起動 contract。
- Fields:
  - `image_source`: `comfyui/runpod/Dockerfile`
  - `service_name`: `comfyui`
  - `entrypoint_wrapper`: `start-ollama-worker.sh`
  - `upstream_start_script`: `/start.sh`
  - `ollama_bind`: `127.0.0.1:11434`
- Validation Rules:
  - local / RunPod のどちらでも同じ Dockerfile を使うこと
  - local で独立 `ollama` service を追加しないこと
  - `comfyui` service 名を変更しないこと

## Entity: `/runpod-volume` bind mount

- Description: local で RunPod の永続領域レイアウトを再現する host mount。
- Fields:
  - `container_path`: `/runpod-volume`
  - `model_root`: `/runpod-volume/models`
  - `ollama_models_root`: `/runpod-volume/ollama/models`
  - `host_source`: compose から bind される local directory
  - `required_for_local`: `true`
- Validation Rules:
  - local quickstart では必須手順として扱うこと
  - ComfyUI model path と Ollama model path を混同しないこと
  - bind mount なしを成功導線として説明しないこと

## Entity: local compose contract

- Description: local 利用者が使う `compose.yml` 上の ComfyUI 起動定義。
- Fields:
  - `build_context`: `./comfyui`
  - `dockerfile`: `runpod/Dockerfile`
  - `published_port`: `${COMFYUI_PORT:-18188}:8188`
  - `volume_mounts`: `/runpod-volume`、必要に応じた output / user 永続化
  - `environment`: `OLLAMA_PULL_MODELS`、`CLI_ARGS`、GPU 関連 env など
- Validation Rules:
  - `docker compose up -d comfyui` だけで起動できること
  - `depends_on: ollama` のような独立 service 依存を残さないこと
  - runtime 契約に沿った `/runpod-volume` path を渡すこと

## Entity: 旧 local 導線

- Description: 共通 runtime 化により現行構成から外す旧 local 専用資産。
- Fields:
  - `legacy_dockerfile`: `comfyui/Dockerfile`
  - `legacy_entrypoint`: `comfyui/entrypoint.sh`
  - `legacy_installer`: `comfyui/install-custom-nodes.sh`
  - `legacy_service`: `ollama`
  - `legacy_paths`: `./comfyui-data`、`./ollama-data`
- Validation Rules:
  - README と quickstart で現行導線として案内しないこと
  - 必要なら削除または非利用化し、保守対象を明確に減らすこと

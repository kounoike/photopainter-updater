# photopainter-updater

Waveshare `ESP32-S3-PhotoPainter` 向けの更新用ファームウェアと関連ドキュメントをまとめたリポジトリです。

ファームウェア target は `esp32s3` 固定です。`esp32` など他 target は対象外です。

## Release Draft

GitHub Actions によって、`main` への `push` 後に次回リリース向け draft を自動更新します。
設定ファイルは `.github/workflows/release-drafter.yml` と `.github/release-drafter.yml` です。

- 確認場所: GitHub の Releases 画面
- 更新契機: `main` への `push` のみ
- 分類基準: pull request labels
- 未分類変更: draft から除外せず、そのまま changelog に残す

詳しい確認手順は [specs/033-release-drafter/quickstart.md](/workspaces/photopainter-updater/specs/033-release-drafter/quickstart.md) を参照してください。

## Release Images

GitHub の draft release を正式 publish すると、`server` image と `comfyui` image を GHCR へ自動公開します。
設定ファイルは `.github/workflows/release-image-publish.yml` と `.github/release-image-publish.yml` です。

- 確認契機: Releases 画面で draft release を publish したとき
- 現在の公開対象: `server` image、`comfyui` image
- 公開先: `ghcr.io/<repository_owner>/photopainter-server:<release-version>`
- 公開先: `ghcr.io/<repository_owner>/photopainter-comfyui:<release-version>`
- 確認場所: GitHub Actions の `Release Image Publish` と GHCR package 画面
- tag 規則: release version がそのまま image tag になる
- 責務分離: Release Drafter は draft 更新のみ、release image publish は正式 release 後の image 公開のみを担当する

`comfyui` image は `.github/release-image-publish.yml` の target 一覧で `server` と同じ形式で管理します。workflow 本体に ComfyUI 専用分岐は追加せず、同じ matrix publish 導線で `./comfyui` と `./comfyui/runpod/Dockerfile` を使って build / push します。

将来ほかの image を公開したい場合は `.github/release-image-publish.yml` の `targets` へ同じ形式で追加します。詳しい確認手順は [specs/034-ghcr-release-publish/quickstart.md](/workspaces/photopainter-updater/specs/034-ghcr-release-publish/quickstart.md) を参照してください。

## HTTP サーバ

PhotoPainter 向けの HTTP サーバは Docker Compose で起動します。

```bash
cp .env.example .env
docker compose up -d server
docker compose logs --tail=200 server
```

既定では `http://127.0.0.1:8000/` で待ち受けます。host 側公開ポートは `.env` の
`SERVER_EXPOSE_PORT`、配信元ディレクトリは `SERVER_CONTENT_DIR` で変更できます。
詳細は [server/README.md](/workspaces/photopainter-updater/server/README.md) と
[specs/029-compose-http-server/quickstart.md](/workspaces/photopainter-updater/specs/029-compose-http-server/quickstart.md) を参照してください。
この repository 作業環境では `docker` コマンドが使えない場合があるため、Compose 実行確認は
Docker 利用可能な環境で行ってください。

## ComfyUI（画像生成）

NVIDIA GPU 搭載の環境では、local でも RunPod と同じ `worker-comfyui` ベース image を
Docker Compose から起動します。`compose.yml` の `comfyui` service は
[`comfyui/runpod/Dockerfile`](/workspaces/photopainter-updater/comfyui/runpod/Dockerfile) を build し、
Ollama は同じ `comfyui` コンテナ内で自動起動します。local Compose では
RunPod handler は起動せず、ComfyUI Web UI と Ollama 確認に集中します。

最初に `/runpod-volume` 用の host directory を作成します。

```bash
mkdir -p ./runpod-volume/{models,ollama/models,input,output,user,dot-cache,dot-local}
cp .env.example .env
```

```bash
docker compose build comfyui
docker compose up -d comfyui
docker compose logs --tail=200 comfyui
```

ブラウザで `http://127.0.0.1:18188` にアクセスすると ComfyUI Web UI が開きます。
Ollama は host へ公開せず、コンテナ内 localhost で確認します。

```bash
docker compose exec comfyui curl -fsS http://127.0.0.1:11434/api/version
```

local の model path は RunPod と同じです。

- ComfyUI model root: `/runpod-volume/models`
- Ollama model storage: `/runpod-volume/ollama/models`

詳細な local 手順は
[specs/044-local-runpod-image/quickstart.md](/workspaces/photopainter-updater/specs/044-local-runpod-image/quickstart.md)
を参照してください。

repo 管理の custom node は [`comfyui/custom_node/`](./comfyui/custom_node/) 配下に置き、
`docker compose build comfyui` 時に共有 runtime image へ焼き込まれます。
third-party custom node は `ComfyUI-Easy-Use`、`comfyui-ollama`、
`ComfyUI-basic_data_handling` を既定値として、
`worker-comfyui` の `customization.md` に沿って `comfy-node-install` で導入します。
必要なら `.env` の `COMFYUI_CUSTOM_NODES` で build 時の一覧を差し替えられます。
repo 側 custom node を更新したあとは再 build が必要です。
PhotoPainter 用の PNG POST ノードの導入と HTTP サーバとの接続例は
[specs/027-comfyui-post-node/quickstart.md](specs/027-comfyui-post-node/quickstart.md) を参照してください。

## RunPod Serverless（ComfyUI + Ollama）

RunPod Serverless でも local と同じ
[`comfyui/runpod/`](/workspaces/photopainter-updater/comfyui/runpod/) の image を使います。
wrapper start script が `ollama serve` を前置起動し、localhost の `127.0.0.1:11434`
で疎通確認してから upstream `/start.sh` へ委譲します。

```bash
docker build -t photopainter-runpod-comfyui-ollama -f comfyui/runpod/Dockerfile comfyui
```

RunPod の Network Volume を endpoint 側で接続すると container 内では `/runpod-volume`
に見えます。local は bind mount、RunPod は Network Volume という違いだけで、
runtime 自体は同じです。事前 pull model は
`OLLAMA_PULL_MODELS=qwen3.5:4b,llama3.2:3b` のような単一 env 値のカンマ区切りで指定します。

詳細は [comfyui/runpod/README.md](/workspaces/photopainter-updater/comfyui/runpod/README.md) と
[specs/044-local-runpod-image/quickstart.md](/workspaces/photopainter-updater/specs/044-local-runpod-image/quickstart.md)
を参照してください。

## AI Toolkit 試用環境

AI Toolkit は [`ostris/ai-toolkit`](https://github.com/ostris/ai-toolkit) を、このリポジトリの
`compose.yml` から `ai-toolkit` サービスとして起動して試すための追加導線です。
既存の ComfyUI 単独導線や Ollama 単独導線はそのまま残し、AI Toolkit は追加サービスとして共存します。

```bash
cp .env.example .env
docker compose up -d ai-toolkit
```

ブラウザで `http://localhost:8675` を開き、AI Toolkit Web UI に到達できれば試用開始可と判断できます。
認証を有効にしたい場合は `.env` の `AI_TOOLKIT_AUTH` を変更してください。

詳細な前提条件、保存先、復帰方法、既存導線との境界は
[specs/025-ai-toolkit-env/quickstart.md](specs/025-ai-toolkit-env/quickstart.md) を参照してください。

## Firmware

ファームウェアの build、merged image 作成、書き込み手順は [docs/firmware.md](/workspaces/photopainter-updater/docs/firmware.md) を参照してください。

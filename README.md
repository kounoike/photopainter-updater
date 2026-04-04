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

NVIDIA GPU 搭載の環境で ComfyUI を Docker Compose から self-build して起動できます。
この構成は CUDA 対応 Python base image から ComfyUI を組み立て、依存は `uv` で導入します。PyTorch backend は Docker build 時に `cu128` へ固定し、`auto` 判定には依存しません。repo 管理 custom node と pinned third-party custom node は image に焼き込まれます。

```bash
cp .env.example .env   # 必要に応じてポート・データパス・ref を編集
docker compose build comfyui
docker compose up -d comfyui
docker compose logs --tail=200 comfyui
```

ブラウザで `http://localhost:18188` にアクセスすると ComfyUI Web UI が開きます。既定の起動フラグには `--listen 0.0.0.0` を含め、host 側公開ポートから到達できる前提にしています。
再起動確認は `docker compose restart comfyui`、再作成確認は `docker compose down && docker compose up -d comfyui` です。
ローカルでは `COMFYUI_DATA_DIR/models` をそのまま使い、RunPod Serverless を意識する場合は `.env` で `COMFYUI_MODEL_ROOT` を `/runpod-volume/models` のような永続領域へ向けられます。

詳細な手順（build、起動、再起動、再作成、troubleshooting、GPU 確認等）は
[specs/030-build-comfyui-image/quickstart.md](specs/030-build-comfyui-image/quickstart.md) を参照してください。

repo 管理の custom node は [`comfyui/custom_node/`](./comfyui/custom_node/) 配下に置き、
`docker compose build comfyui` 時に ComfyUI image へ焼き込まれます。third-party custom node の導入処理は [install-custom-nodes.sh](/home/kounoike/ghq/github.com/kounoike/photopainter-updater/comfyui/install-custom-nodes.sh) にまとめてあり、既定では `ComfyUI-Manager@4.1`、`ComfyUI-Easy-Use@v1.3.6`、`comfyui-ollama@6db7560576e5a59488708e6be13e07b5aba2432a`、`ComfyUI-Xz3r0-Nodes@v1.7.0` を同梱します。repo 側 custom node や pinned ref を更新した場合は再 build が必要です。追加 custom node を手元で試すことはできても、再作成後の維持は保証しません。
PhotoPainter 用の PNG POST ノードの導入と HTTP サーバとの接続例は
[specs/027-comfyui-post-node/quickstart.md](specs/027-comfyui-post-node/quickstart.md) を参照してください。

## Ollama（LLM 推論）

ComfyUI と同じ compose プロジェクト内で Ollama を起動できます。Ollama はホストへ公開せず、Compose 内ネットワーク（`http://ollama:11434`）からのみアクセス可能です。

```bash
cp .env.example .env   # 必要に応じて OLLAMA_DATA_DIR を編集
docker compose up -d ollama
docker compose exec ollama ollama list
```

詳細な手順（モデルの取得・永続化確認・ComfyUI との共存）は
[specs/023-add-ollama-compose/quickstart.md](specs/023-add-ollama-compose/quickstart.md) を参照してください。

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

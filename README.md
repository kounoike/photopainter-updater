# photopainter-updater

Waveshare `ESP32-S3-PhotoPainter` 向けの更新用ファームウェアと関連ドキュメントをまとめたリポジトリです。

ファームウェア target は `esp32s3` 固定です。`esp32` など他 target は対象外です。

## ComfyUI（画像生成）

NVIDIA GPU 搭載の環境で ComfyUI を Docker Compose で起動できます。

```bash
cp .env.example .env   # 必要に応じてポート・データパスを編集
docker compose up
```

ブラウザで `http://localhost:18188` にアクセスすると ComfyUI Web UI が開きます。

詳細な手順（モデルの追加・カスタムノードのインストール・GPU 確認等）は
[specs/022-add-comfyui-compose/quickstart.md](specs/022-add-comfyui-compose/quickstart.md) を参照してください。

## Ollama（LLM 推論）

ComfyUI と同じ compose プロジェクト内で Ollama を起動できます。Ollama はホストへ公開せず、Compose 内ネットワーク（`http://ollama:11434`）からのみアクセス可能です。

```bash
cp .env.example .env   # 必要に応じて OLLAMA_DATA_DIR を編集
docker compose up -d ollama
docker compose exec ollama ollama list
```

詳細な手順（モデルの取得・永続化確認・ComfyUI との共存）は
[specs/023-add-ollama-compose/quickstart.md](specs/023-add-ollama-compose/quickstart.md) を参照してください。

## Firmware

ファームウェアの build、merged image 作成、書き込み手順は [docs/firmware.md](/workspaces/photopainter-updater/docs/firmware.md) を参照してください。

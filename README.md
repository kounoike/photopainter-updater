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

## Firmware

ファームウェアの build、merged image 作成、書き込み手順は [docs/firmware.md](/workspaces/photopainter-updater/docs/firmware.md) を参照してください。

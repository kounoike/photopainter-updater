# Quickstart: HTTPサーバ Compose 統合

## 1. HTTP サーバを起動する

```bash
cp .env.example .env
docker compose up -d server
```

## 2. 起動確認

```bash
docker compose logs --tail=200 server
curl -I http://127.0.0.1:8000/
curl -I http://127.0.0.1:8000/image.bmp
curl -I http://127.0.0.1:8000/image.bin
```

## 3. upload 確認

```bash
curl -i -X POST -H 'Content-Type: image/png' \
  --data-binary @server/contents/image1.png \
  http://127.0.0.1:8000/upload
```

## 4. 他サービスと共存させる

```bash
docker compose up -d comfyui ollama server
```

## 5. 停止

```bash
docker compose stop server
```

## 6. 備考

- この repository 作業環境では `docker` コマンドが使えない場合がある。その場合、上記確認は Docker 利用可能な実行環境で実施する。
- `server/run.sh` はこの feature 完了後に廃止する前提。

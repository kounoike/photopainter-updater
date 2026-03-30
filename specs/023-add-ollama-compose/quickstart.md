# クイックスタート: Ollama Docker Compose

## 前提条件

- Docker Engine と Docker Compose v2（`docker compose`）が使えること
- 既存の `compose.yml` と `.env.example` がリポジトリルートにあること
- ComfyUI と同じ compose プロジェクト内で Ollama を管理すること

## 1. 環境設定ファイルの準備

```bash
cp .env.example .env
```

必要に応じて `.env` の `OLLAMA_DATA_DIR` を編集します。

## 2. 永続化ディレクトリの準備

Docker が自動作成できますが、明示的に作る場合:

```bash
DATA_DIR=$(grep '^OLLAMA_DATA_DIR=' .env | cut -d= -f2)
DATA_DIR="${DATA_DIR:-./ollama-data}"
mkdir -p "${DATA_DIR}"
```

## 3. Ollama の起動

```bash
docker compose up -d ollama
```

ComfyUI も同時に起動する場合:

```bash
docker compose up -d
```

## 4. 起動確認

### サービス単体確認

```bash
docker compose exec ollama ollama list
```

### Compose 内ネットワーク疎通確認

```bash
docker compose exec comfyui curl -fsS http://ollama:11434/api/version
```

`curl` の JSON 応答が返れば、内部サービス名 `ollama` で到達できています。

## 5. モデルの取得

例として軽量モデルを pull します。

```bash
docker compose exec ollama ollama pull gemma3:1b
docker compose exec ollama ollama list
```

## 6. 永続化の確認

```bash
docker compose down
docker compose up -d ollama
docker compose exec ollama ollama list
```

再起動後も pull 済みモデルが表示されれば永続化できています。

## 7. 停止とログ確認

```bash
docker compose logs -f ollama
docker compose stop ollama
docker compose down
```

## よくある確認ポイント

### `docker compose exec comfyui ...` が失敗する

- `comfyui` サービスが起動しているか確認する
- `docker compose ps` で `ollama` と `comfyui` の両方が同じ project 内で起動しているか確認する

### モデルが再作成後に消える

- `.env` の `OLLAMA_DATA_DIR` が期待どおりか確認する
- `ls "${OLLAMA_DATA_DIR:-./ollama-data}"` でホスト側にデータが残っているか確認する

### ホストから `localhost:11434` に繋がらない

- 仕様どおりです。この feature では Ollama をホストへ公開しません

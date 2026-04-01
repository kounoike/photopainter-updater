# クイックスタート: AI Toolkit 試用環境

## 前提条件

- Docker Engine と Docker Compose v2（`docker compose`）が使えること
- NVIDIA GPU を使う場合は、AI Toolkit と既存 Compose サービスの起動条件を満たしていること
- リポジトリルートに `compose.yml` と `.env.example` があること
- AI Toolkit は `ostris/ai-toolkit` を `ai-toolkit` サービスとして起動する前提であること

## 1. 環境設定ファイルの準備

```bash
cp .env.example .env
```

必要に応じて AI Toolkit 用のポート、認証、保存先を編集します。

## 2. 保存先の準備

AI Toolkit 用に使用する保存先を確認します。必要なら事前に作成します。

```bash
mkdir -p ./ai-toolkit-data/config ./ai-toolkit-data/datasets ./ai-toolkit-data/output ./ai-toolkit-data/cache
touch ./ai-toolkit-data/aitk_db.db
```

## 3. AI Toolkit の起動

```bash
docker compose up -d ai-toolkit
```

状態確認:

```bash
docker compose ps ai-toolkit
```

## 4. Web UI の確認

README で案内した URL をブラウザで開き、AI Toolkit UI に到達できることを確認します。

## 5. 停止と再開

```bash
docker compose stop ai-toolkit
docker compose up -d ai-toolkit
```

不要になったら:

```bash
docker compose rm -sf ai-toolkit
```

## 6. 失敗時の確認ポイント

### `compose-state`

- `docker compose ps ai-toolkit` で `ai-toolkit` が起動しているか確認する
- 必要に応じて `docker compose logs --tail=100 ai-toolkit` を確認する

### `env-config`

- `.env` の AI Toolkit 用ポート、認証、保存先設定を確認する
- `docker compose config` で解決後の設定が妥当か確認する

### `storage-path`

- AI Toolkit 用の config / datasets / output / DB / cache が期待するパスに存在するか確認する
- 再起動後も同じ保存先を参照しているか確認する

## 7. 既存導線との関係

- ComfyUI 単体を使いたい場合は既存 README の ComfyUI 節をそのまま参照する
- Ollama 単体を使いたい場合は既存 README の Ollama 節をそのまま参照する
- AI Toolkit は追加サービスであり、既存導線の置き換えではない

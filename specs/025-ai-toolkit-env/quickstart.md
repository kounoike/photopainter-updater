# クイックスタート: AI Toolkit 試用環境

## 前提条件

- Docker Engine と Docker Compose v2（`docker compose`）が使えること
- NVIDIA GPU を使う場合は、既存 ComfyUI / Ollama の起動条件を満たしていること
- リポジトリルートに `compose.yml` と `.env.example` があること
- AI Toolkit は既存 ComfyUI と Ollama を土台にした Compose 試用環境として扱うこと

## 1. 環境設定ファイルの準備

```bash
cp .env.example .env
```

必要に応じて `.env` の `COMFYUI_PORT`、`COMFYUI_DATA_DIR`、`OLLAMA_DATA_DIR` を編集します。

## 2. 永続化ディレクトリの確認

既定値のまま試す場合は Docker に自動作成させても構いません。明示的に確認する場合:

```bash
mkdir -p ./comfyui-data ./ollama-data
```

## 3. 主要サービスの起動

```bash
docker compose up -d
```

起動直後は次で状態を確認します。

```bash
docker compose ps
```

## 4. 代表操作の確認

ComfyUI から Ollama へ到達できることを、AI Toolkit 試用の代表操作として確認します。

```bash
docker compose exec comfyui curl -fsS http://ollama:11434/api/version
```

JSON 応答が返れば、主要サービス起動と Compose 内疎通の両方を満たしており、試用成功と判断できます。

## 5. 停止と再開

```bash
docker compose stop
docker compose up -d
```

不要になったら:

```bash
docker compose down
```

## 6. 失敗時の確認ポイント

### `compose-state`

- `docker compose ps` で期待するサービスが `Up` か確認する
- 必要に応じて `docker compose logs --tail=100` で直近の起動失敗を確認する
- `comfyui` または `ollama` のどちらかが起動していない場合、代表操作は成功しない

### `env-config`

- `.env` の `COMFYUI_PORT`、`COMFYUI_DATA_DIR`、`OLLAMA_DATA_DIR` を確認する
- `docker compose config` で解決後の設定が妥当か確認する

### `persistent-data`

- `./comfyui-data` と `./ollama-data`、または `.env` で指定したパスが存在するか確認する
- 永続データを利用する前提が崩れていないか確認する

## 7. 既存導線との関係

- ComfyUI 単体を使いたい場合は既存 README の ComfyUI 節をそのまま参照する
- Ollama 単体を使いたい場合は既存 README の Ollama 節をそのまま参照する
- AI Toolkit は両者をまとめて試すための追加導線であり、既存導線の置き換えではない

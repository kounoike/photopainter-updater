# 実行コントラクト: Ollama Compose ランタイム

**Branch**: `023-add-ollama-compose` | **Date**: 2026-03-30

## 目的

既存 `compose.yml` に Ollama を追加する際、Compose 上で維持すべきサービス定義と利用者向けの見え方を固定する。

## 維持する既存インターフェース

| 項目 | 契約 |
|------|------|
| `comfyui` service 名 | 変更しない |
| `COMFYUI_PORT` | 既存どおりホスト公開に使う |
| `photopainter` network | 既存どおり共有 bridge network として維持する |
| `README.md` の ComfyUI 導線 | 破壊しない |

## 追加するインターフェース

### `.env` 変数

| 変数 | 型 | デフォルト | 役割 |
|------|----|-----------|------|
| `OLLAMA_DATA_DIR` | path | `./ollama-data` | Ollama の永続化ディレクトリ親パス |

### `ollama` service

| 項目 | 契約 |
|------|------|
| service 名 | `ollama` |
| image | `ollama/ollama` |
| container 名 | `photopainter-ollama` |
| volumes | `${OLLAMA_DATA_DIR:-./ollama-data}:/root/.ollama` |
| networks | `photopainter` に参加する |
| ports | 定義しない |
| restart | `unless-stopped` |

## 利用契約

- 起動は `docker compose up -d ollama` または `docker compose up -d` で行えること
- 内部疎通確認は `http://ollama:11434` を用いること
- ホストや LAN からの直接アクセスは本 feature の契約に含めないこと
- モデル pull 後に `docker compose down` しても、`OLLAMA_DATA_DIR` が残る限りモデルを再利用できること

## 検証契約

最低限、以下の確認が再現できること。

1. `docker compose config` が成功する
2. `docker compose up -d ollama` でサービス起動できる
3. `docker compose exec ollama ollama list` が成功する
4. `docker compose exec comfyui curl -fsS http://ollama:11434/api/version` が成功する
5. モデル pull 後にコンテナを再作成しても `ollama list` の結果が残る

## 非目標

- OpenAI 互換 API プロキシの提供
- Ollama の認証、TLS、外部公開設定
- モデルの自動 pre-pull
- アプリケーションコードからの Ollama 利用実装

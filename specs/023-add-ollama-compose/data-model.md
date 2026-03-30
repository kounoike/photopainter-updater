# データモデル・設定仕様: Ollama Docker Compose 統合

**Phase 1 成果物** | Branch: `023-add-ollama-compose`

## 1. 環境変数定義（`.env` / `.env.example`）

| 変数名 | デフォルト値 | 必須 | 説明 |
|--------|------------|------|------|
| `OLLAMA_DATA_DIR` | `./ollama-data` | 推奨 | Ollama のモデル・メタデータを保持する親ディレクトリ |

### `OLLAMA_DATA_DIR`

- **型**: ファイルシステムパス（絶対パスまたは `compose.yml` 相対パス）
- **デフォルト**: `./ollama-data`
- **用途**: `${OLLAMA_DATA_DIR:-./ollama-data}:/root/.ollama` の bind mount 起点
- **検証**:
  - 未作成でも Docker が作成できること
  - コンテナ再作成後も pull 済みモデルが残ること

## 2. Compose サービス定義

### `ollama` サービス

| 項目 | 値 |
|------|----|
| service 名 | `ollama` |
| image | `ollama/ollama` |
| container_name | `photopainter-ollama` |
| restart | `unless-stopped` |
| network | `photopainter` |
| volumes | `${OLLAMA_DATA_DIR:-./ollama-data}:/root/.ollama` |
| ports | なし |

### バリデーションルール

- `ports` は定義しない
- `photopainter` ネットワークに参加する
- `/root/.ollama` への永続化を持つ
- 既存 `comfyui` サービスの定義は削除・改変しない

## 3. 永続化ディレクトリ構成

```text
${OLLAMA_DATA_DIR}/
└── ... Ollama が管理するモデル・manifest・blob 類
```

| ホスト側 | コンテナ内 | 役割 |
|----------|-----------|------|
| `${OLLAMA_DATA_DIR}` | `/root/.ollama` | pull 済みモデル、メタデータ、blob を永続化する |

## 4. ネットワーク可視性

| 通信元 | 通信先 | 可否 | 備考 |
|--------|--------|------|------|
| `comfyui` | `http://ollama:11434` | 可 | 同一 `photopainter` ネットワーク |
| ホスト OS | `http://localhost:11434` | 不可 | `ports` を公開しないため |
| 外部 LAN | Ollama API | 不可 | 同上 |

## 5. ライフサイクル

| 操作 | 期待状態 |
|------|----------|
| `docker compose up -d ollama` | `ollama` コンテナが起動し、内部 API を待受ける |
| `docker compose stop ollama` | コンテナ停止。永続データは保持される |
| `docker compose down` | コンテナ削除。bind mount データは保持される |
| `docker compose up -d ollama` 再実行 | 既存モデルを再利用できる |

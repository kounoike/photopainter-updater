# データモデル: HTTPサーバ Compose 統合

## 1. HTTP サーバサービス

| 項目 | 型 | 説明 |
|------|----|------|
| `service_name` | string | compose 上の HTTP サーバ service 名 |
| `image_build_context` | path | server image を build する context |
| `port_mapping` | string | host の `SERVER_EXPOSE_PORT` と container 内 8000 の HTTP port 対応 |
| `content_mount` | path mapping | `server/contents` を runtime へ渡す bind mount |
| `server_expose_port_env` | string | `.env.example` の `SERVER_EXPOSE_PORT` |
| `server_content_dir_env` | string | `.env.example` の `SERVER_CONTENT_DIR` |

### Invariants

- 既存 endpoint 契約は維持する
- server は単体起動可能である
- 他 compose サービスと同居してもよい
- container 内 port は 8000 固定である
- `SERVER_EXPOSE_PORT` と `SERVER_CONTENT_DIR` は利用者が上書き可能である

## 2. 配信コンテンツ

| 項目 | 型 | 説明 |
|------|----|------|
| `content_dir` | path | host 側 `${SERVER_CONTENT_DIR:-./server/contents}` |
| `input_image` | file | `image.png` |
| `derived_outputs` | files | `image.bmp`、`image.bin` |

### Validation Rules

- compose 化後も upload は `content_dir` の現在画像を更新する
- 既存ファイルを破壊的に移行しない

## 3. compose 運用導線

| 項目 | 型 | 説明 |
|------|----|------|
| `start_command` | command | HTTP サーバ起動手順 |
| `stop_command` | command | 停止手順 |
| `log_command` | command | ログ確認手順 |
| `health_check` | action | 起動後の疎通確認 |
| `env_template` | file | `.env.example` に記載する `SERVER_EXPOSE_PORT` と `SERVER_CONTENT_DIR` |

### State

`stopped` → `starting` → `running` → `stopped`

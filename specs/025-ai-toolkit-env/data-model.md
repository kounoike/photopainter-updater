# データモデル: AI Toolkit 試用環境

**Phase 1 成果物** | Branch: `025-ai-toolkit-env`

## 1. `AiToolkitService`

`compose.yml` に追加される AI Toolkit 実行サービス。

| 項目 | 型 | 説明 |
|------|----|------|
| `service_name` | string | `ai-toolkit` |
| `image` | string | `ostris/aitoolkit:latest` を想定 |
| `ui_port` | number | Web UI 公開ポート |
| `restart_policy` | string | 再起動方針 |
| `gpu_requirement` | string | GPU 利用前提の説明 |
| `env_config` | list of `AiToolkitEnvConfig` | 起動時に参照する環境変数 |
| `storage` | list of `AiToolkitStorage` | 永続化対象 |

### ルール

- `service_name` は `ai-toolkit` として固定する
- 既存 `comfyui` と `ollama` は別 service のまま維持する

## 2. `AiToolkitStorage`

AI Toolkit が継続利用する設定、データ、出力の保存先。

| 項目 | 型 | 説明 |
|------|----|------|
| `name` | enum | `config` / `datasets` / `output` / `db` / `hf-cache` |
| `host_path` | path | ホスト側の保存先 |
| `container_path` | path | コンテナ内の対応パス |
| `persistence_role` | string | 再起動後に保持したい内容 |

### ルール

- US2 を満たすため、再起動後も同じ `host_path` を参照できること
- `db` は単一ファイルでも保存対象として扱う

## 3. `AiToolkitEnvConfig`

AI Toolkit 起動前に利用者が調整しうる `.env` 設定。

| 項目 | 型 | 説明 |
|------|----|------|
| `name` | string | 環境変数名 |
| `default_value` | string | 既定値 |
| `required` | boolean | 必須か任意か |
| `purpose` | string | 利用目的 |

### ルール

- 少なくとも UI 公開と認証に関わる入口を含める
- `.env.example` で説明される名前と一致させる

## 4. `AiToolkitAccessPath`

利用者が AI Toolkit UI へ到達するための入口情報。

| 項目 | 型 | 説明 |
|------|----|------|
| `start_command` | string | AI Toolkit を起動するコマンド |
| `access_url` | string | 利用者がブラウザで開く URL |
| `success_signal` | string | UI 到達を確認する条件 |
| `failure_signal` | string | 起動失敗時に見える兆候 |

### ルール

- `start_command` は `docker compose up -d ai-toolkit` を想定する
- `success_signal` は Web UI 到達で定義する

## 5. `RecoveryHint`

利用者が起動失敗時に最初に確認する切り分け情報。

| 項目 | 型 | 説明 |
|------|----|------|
| `category` | enum | `compose-state` / `env-config` / `storage-path` |
| `symptom` | string | 利用者が見ている症状 |
| `check_point` | string | 最初に確認する対象 |
| `next_action` | string | 次に取る行動 |

### ルール

- 切り口は 3 系統に統一する
- quickstart と contract で同じ分類を使う

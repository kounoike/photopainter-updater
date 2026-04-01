# データモデル: AI Toolkit 試用環境

**Phase 1 成果物** | Branch: `025-ai-toolkit-env`

## 1. `AiToolkitEnvironment`

AI Toolkit を試す利用者が参照する論理的な試用環境全体。

| 項目 | 型 | 説明 |
|------|----|------|
| `name` | string | 利用者向け名称。`AI Toolkit 試用環境` を想定 |
| `entrypoint` | enum | 正式導線。`compose` 固定 |
| `base_services` | list of `BaseServiceSet` | 土台として使う既存サービス群 |
| `representative_operation` | `RepresentativeOperation` | 試用成功判定に使う代表操作 |
| `recovery_hints` | list of `RecoveryHint` | 失敗時の確認観点 |
| `docs` | list of path | 利用者が参照する README / quickstart / contract |

### ルール

- AI Toolkit 試用環境は新規常駐サービス群ではなく、既存 Compose 構成の利用者向け束ね直しとして扱う
- `entrypoint` は Docker Compose 中心で一意に扱う

## 2. `BaseServiceSet`

既存構成のうち、AI Toolkit 試用環境の土台として維持するサービス定義。

| 項目 | 型 | 説明 |
|------|----|------|
| `service_name` | enum | `comfyui` または `ollama` |
| `role` | string | AI Toolkit 内での役割 |
| `startup_mode` | string | 単独起動または全体起動の想定 |
| `persistent_path` | path | ホスト側永続ディレクトリ |
| `user_visible_entry` | path or URL | 利用者が最初に触る入口 |

### ルール

- `service_name` は既存 `compose.yml` の命名を維持する
- AI Toolkit 追加後も単独利用導線を残す

## 3. `TrialEntryPoint`

利用者が AI Toolkit を試し始めるときに辿る入口情報。

| 項目 | 型 | 説明 |
|------|----|------|
| `prerequisites` | list of string | Docker/GPU/設定ファイルなどの前提 |
| `prepare_steps` | ordered list | `.env` 準備や必要ディレクトリ確認 |
| `start_command` | string | 主な起動コマンド |
| `verification_steps` | ordered list | 起動後に状態確認する順序 |

### ルール

- 入口手順は README で概要、quickstart で詳細を持つ
- 追加の口頭説明なしに利用開始判断できること

## 4. `RepresentativeOperation`

試用成功判定に使う最小の利用者操作。

| 項目 | 型 | 説明 |
|------|----|------|
| `operation_name` | string | 利用者が識別できる代表操作名 |
| `command` | string | 代表操作として実行するコマンド |
| `depends_on` | list of service name | 実行に必要な主要サービス |
| `success_signal` | string | 成功とみなす観測結果 |
| `fallback_signal` | string | 失敗時に次の確認先へ進むための兆候 |

### ルール

- 代表操作は `docker compose exec comfyui curl -fsS http://ollama:11434/api/version` を想定する
- 代表操作は 1 件でよいが、主要サービスの起動確認に続く手順であること
- 技術者以外でも読み取れる成功シグナル表現を使う

## 5. `RecoveryHint`

試用失敗時に利用者が参照する復帰・切り分け情報。

| 項目 | 型 | 説明 |
|------|----|------|
| `category` | enum | `compose-state` / `env-config` / `persistent-data` |
| `symptom` | string | 利用者が遭遇する症状 |
| `check_point` | string | 最初に確認する対象 |
| `next_action` | string | 次に取るべき行動 |

### ルール

- 復帰の切り口は 3 系統に統一する
- 代表操作失敗時も、まず `category` に対応する確認へ誘導する

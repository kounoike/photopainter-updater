# データモデル: RunPod Ollama sidecar

## 1. Ollama runtime mode

| 項目 | 型 | 説明 |
|------|----|------|
| `ollama_bind_host` | string | `127.0.0.1:11434` 固定の bind 先 |
| `runtime_mode` | enum | `persistent` または `ephemeral` |
| `runtime_ready` | boolean | Ollama API readiness が完了したか |
| `startup_delegate` | path | readiness 後に委譲する upstream `/start.sh` |

### Invariants

- Ollama API は localhost 限定で利用する
- readiness が完了する前に upstream worker 本体へ成功制御を渡さない
- worker 本体の起動責務は upstream `/start.sh` に残す

## 2. Model storage resolution

| 項目 | 型 | 説明 |
|------|----|------|
| `network_volume_root` | path | RunPod serverless で見える `/runpod-volume` |
| `persistent_models_dir` | path | Network Volume 利用時に `OLLAMA_MODELS` へ渡す保存先 |
| `ephemeral_models_dir` | path | Network Volume 未接続時に使う一時保存先 |
| `selected_models_dir` | path | 起動時に選ばれた実保存先 |
| `volume_available` | boolean | `/runpod-volume` が存在し書き込み可能か |

### Validation Rules

- `volume_available=true` のときは `selected_models_dir=persistent_models_dir`
- `volume_available=false` のときは `selected_models_dir=ephemeral_models_dir`
- `ephemeral_models_dir` は再作成後の再利用を保証しない
- 起動ログから persistent / ephemeral のどちらが選ばれたか判別できる

## 3. Model pull configuration

| 項目 | 型 | 説明 |
|------|----|------|
| `pull_models_raw` | string | 単一 env 値で渡されるカンマ区切り文字列 |
| `pull_models` | list[string] | trim 後の model 名一覧 |
| `pull_enabled` | boolean | 一覧が 1 件以上あるか |
| `pull_result` | enum | `pulled`、`reused`、`failed`、`skipped` |

### Invariants

- `pull_models_raw` が空なら `pull_enabled=false`
- 空白だけの要素は破棄する
- 同一 model 名の重複は起動時に 1 回だけ扱う
- `failed` があっても worker 起動は継続する

## 4. Pull execution report

| 項目 | 型 | 説明 |
|------|----|------|
| `model_name` | string | `ollama pull` 対象の model 名 |
| `storage_mode` | enum | `persistent` または `ephemeral` |
| `result` | enum | `pulled`、`reused`、`failed` |
| `message` | string | warning / info ログへ残す短い結果 |

### State

`pending` → `pulled`  
`pending` → `reused`  
`pending` → `failed`

### Validation Rules

- すべての対象 model に対し 1 件の結果を残す
- `failed` は warning として出力する
- 失敗結果があっても runtime は `runtime_ready=true` になり得る

## 5. Local verification profile

| 項目 | 型 | 説明 |
|------|----|------|
| `local_volume_bind` | enum | `attached` または `detached` |
| `worker_image_tag` | string | ローカル build した RunPod 用 image tag |
| `local_api_endpoint` | string | ローカル worker API 送信先 |
| `test_payload` | path | ローカル worker へ投げる test input |

### Invariants

- `attached` は `/runpod-volume` 相当の bind mount ありを意味する
- `detached` は一時領域フォールバックの確認モードである
- ローカル検証では API readiness と model storage mode の両方を確認する

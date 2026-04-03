# データモデル: ComfyUI 自前イメージ構築

## 1. ComfyUI build 入力

| 項目 | 型 | 説明 |
|------|----|------|
| `build_context` | path | `docker compose build comfyui` が参照する repo 内 build context |
| `dockerfile_path` | path | repo 管理の ComfyUI Dockerfile |
| `base_runtime_ref` | string | CUDA 対応 Python base image の参照値 |
| `entrypoint_path` | path | ComfyUI 起動を固定する repo 管理 entrypoint |
| `repo_custom_node_path` | path | repo 管理 custom node ソース |
| `compose_service_name` | string | compose 上で継続利用する service 名 |

### Invariants

- build 入力は repo 内に存在し、git 履歴で追跡できる
- `compose_service_name` は `comfyui` を維持する
- base runtime は floating 参照ではなく固定参照を前提にする
- Python 依存は `uv` を通じて導入する
- NVIDIA/CUDA 前提の PyTorch 導入経路を明示する

## 2. ComfyUI 永続データ

| 項目 | 型 | 説明 |
|------|----|------|
| `data_root` | path | `${COMFYUI_DATA_DIR:-./comfyui-data}` |
| `models_dir` | path | モデル保存先 |
| `custom_nodes_dir` | path | 利用者追加 custom node 保存先 |
| `dot_local_dir` | path | 依存ライブラリなどの保存先 |
| `output_dir` | path | 生成画像保存先 |
| `user_dir` | path | 利用者設定保存先 |
| `input_dir` | path | 入力素材保存先 |
| `dot_cache_dir` | path | cache 保存先 |

### Validation Rules

- 自前 image へ切り替えても各ディレクトリの役割は維持する
- 再作成後も host 側の既存データを継続参照できる
- repo 管理 custom node の追加導線は保持しつつ、利用者 custom node 全体保存先も残す

## 3. Compose 運用導線

| 項目 | 型 | 説明 |
|------|----|------|
| `build_command` | command | ComfyUI image の build または再 build 手順 |
| `start_command` | command | ComfyUI 起動手順 |
| `restart_command` | command | ComfyUI 再起動手順 |
| `recreate_command` | command | ComfyUI 再作成手順 |
| `log_command` | command | ログ確認手順 |
| `health_check` | action | UI 到達や healthcheck を使った確認手順 |
| `troubleshooting_entry` | action | build 失敗や CUDA 不整合時の最初の確認先 |

### State

`not_built` → `built` → `running` → `restarted` または `recreated` → `running`

## 4. 利用者向け設定入口

| 項目 | 型 | 説明 |
|------|----|------|
| `COMFYUI_PORT` | env | Web UI 公開ポート |
| `COMFYUI_DATA_DIR` | env | 永続データ親ディレクトリ |
| `COMFYUI_CLI_ARGS` | env | ComfyUI 起動フラグ |

### Invariants

- 既存の `.env.example` 入口は残す
- 既存 URL と利用開始手順を大きく変えない
- build 導線が追加されても、利用者が参照すべき設定入口は増やしすぎない

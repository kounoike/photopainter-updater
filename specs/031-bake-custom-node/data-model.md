# データモデル: ComfyUI custom node 同梱コンテナ

## 1. Repo 管理 custom node

| 項目 | 型 | 説明 |
|------|----|------|
| `repo_custom_node_source` | path | `comfyui/custom_node/comfyui-photopainter-custom` |
| `image_custom_node_path` | path | image 内で repo 管理 node が配置される ComfyUI custom node path |
| `rebuild_required` | boolean | repo 側 source 更新時に rebuild が必要であることを示す |

### Invariants

- repo 管理 custom node は runtime bind mount ではなく image に含まれる
- repo 管理 custom node の反映条件は rebuild である
- `PhotoPainter PNG POST` は baked-in node として起動直後に見える

## 2. 選定済み third-party custom node

| 項目 | 型 | 説明 |
|------|----|------|
| `third_party_repo_id` | string | `comfyui-manager`、`ComfyUI-Easy-Use`、`comfyui-ollama`、`ComfyUI-Xz3r0-Nodes` |
| `source_ref_type` | enum | `tag` または `commit` |
| `source_ref` | string | build 時に clone する固定 ref |
| `python_requirements` | path? | `requirements.txt` がある場合の導入元 |
| `system_dependencies` | set[string] | `ffmpeg` など OS package 前提 |

### Invariants

- 選定済み third-party custom node は image build 時に取得される
- tag がある repo は tag 固定、tag がない repo は commit 固定を使う
- `comfyui-ollama` は Ollama service への接続を前提にするが、node 自体は起動時点で存在する
- `ComfyUI-Xz3r0-Nodes` の `ffmpeg` 前提は image 側で満たす

## 3. 追加 custom node

| 項目 | 型 | 説明 |
|------|----|------|
| `temporary_custom_node` | path | 利用者が一時的に container へ追加できる custom node |
| `persistence_guarantee` | enum | `not_guaranteed` 固定 |

### Validation Rules

- repo 管理 baked-in node の存在を前提にする
- 追加 custom node は再作成後に残ることを保証しない
- 文書はこの非永続性を明示する

## 4. Runtime custom node view

| 項目 | 型 | 説明 |
|------|----|------|
| `runtime_custom_nodes_dir` | path | ComfyUI が実際に探索する custom node directory |
| `baked_repo_nodes` | set[path] | image 由来で常に存在する repo 管理 node |
| `baked_third_party_nodes` | set[path] | image 由来で常に存在する選定済み third-party custom node |
| `temporary_nodes` | set[path] | その場で追加されても再作成後の維持を保証しない node |

### State

`built` → `running` → `restarted` または `recreated` → `running`

## 5. 利用者向け運用入口

| 項目 | 型 | 説明 |
|------|----|------|
| `build_command` | command | repo 管理 custom node 更新を取り込む build 手順 |
| `start_command` | command | baked-in node を含む ComfyUI 起動手順 |
| `restart_command` | command | baked-in node が維持されることを確認する再起動手順 |
| `recreate_command` | command | baked-in node を保ったまま作り直す手順 |
| `troubleshooting_entry` | action | build 失敗または node 未読込時の最初の確認先 |

### Invariants

- 利用開始導線は `docker compose build comfyui` と `docker compose up -d comfyui`
- repo 管理 node 更新は rebuild を基本とする
- 同梱 third-party node の更新も Dockerfile の pinned ref 更新と rebuild を基本とする
- 公開 URL と service 名は既存の ComfyUI 導線を維持する

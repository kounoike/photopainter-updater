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

## 2. 追加 custom node

| 項目 | 型 | 説明 |
|------|----|------|
| `temporary_custom_node` | path | 利用者が一時的に container へ追加できる custom node |
| `persistence_guarantee` | enum | `not_guaranteed` 固定 |

### Validation Rules

- repo 管理 baked-in node の存在を前提にする
- 追加 custom node は再作成後に残ることを保証しない
- 文書はこの非永続性を明示する

## 3. Runtime custom node view

| 項目 | 型 | 説明 |
|------|----|------|
| `runtime_custom_nodes_dir` | path | ComfyUI が実際に探索する custom node directory |
| `baked_repo_nodes` | set[path] | image 由来で常に存在する repo 管理 node |
| `temporary_nodes` | set[path] | その場で追加されても再作成後の維持を保証しない node |

### State

`built` → `running` → `restart_scanned` または `recreated` → `running`

## 4. 利用者向け運用入口

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
- 公開 URL と service 名は既存の ComfyUI 導線を維持する

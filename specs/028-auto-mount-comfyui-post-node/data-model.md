# データモデル: ComfyUI custom node 自動登録

## 1. repo 管理 custom node

| 項目 | 型 | 説明 |
|------|----|------|
| `source_path` | path | `comfyui/custom_node/comfyui-photopainter-custom` |
| `ownership` | repository-managed | Git 管理対象の node ソース |
| `mount_mode` | read-only | container へは `:ro` で bind mount する |

## 2. runtime custom_nodes

| 項目 | 型 | 説明 |
|------|----|------|
| `base_path` | path | `/root/ComfyUI/custom_nodes` |
| `existing_mount` | bind mount | `${COMFYUI_DATA_DIR:-./comfyui-data}/custom_nodes` |
| `child_mount` | bind mount | `/root/ComfyUI/custom_nodes/comfyui-photopainter-custom` |

### Invariants

- `base_path` の既存内容は保持する
- `child_mount` は PhotoPainter node のみを追加する
- runtime path は ComfyUI 起動時に探索可能である

## 3. compose mount 構成

| 項目 | 型 | 説明 |
|------|----|------|
| `existing_custom_nodes_mount` | volume string | `${COMFYUI_DATA_DIR:-./comfyui-data}/custom_nodes:/root/ComfyUI/custom_nodes` |
| `photopainter_node_mount` | volume string | `./comfyui/custom_node/comfyui-photopainter-custom:/root/ComfyUI/custom_nodes/comfyui-photopainter-custom:ro` |

### Validation Rules

- `docker compose config` で両方の mount が解決される
- `photopainter_node_mount` は host 側 path が存在すること
- 既存 custom node と PhotoPainter node が併存できること

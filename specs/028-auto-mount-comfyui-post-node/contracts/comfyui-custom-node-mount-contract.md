# Contract: ComfyUI custom node 自動登録

## 1. Compose Volume Contract

ComfyUI service は以下 2 系統の custom node mount を持つ。

| Purpose | Volume |
|---------|--------|
| 既存 custom_nodes 全体 | `${COMFYUI_DATA_DIR:-./comfyui-data}/custom_nodes:/root/ComfyUI/custom_nodes` |
| PhotoPainter node 自動登録 | `./comfyui/custom_node/comfyui-photopainter-custom:/root/ComfyUI/custom_nodes/comfyui-photopainter-custom:ro` |

## 2. Runtime Expectations

- `docker compose up -d comfyui` 後、manual copy なしで `PhotoPainter PNG POST` が見える
- 既存 `comfyui-data/custom_nodes` の内容は保持される
- host 側 node ソース更新後、ComfyUI 再起動で反映される

## 3. Documentation Contract

- root README は compose 自動登録導線を案内する
- 027 quickstart は manual copy を前提にしない
- node README は runtime copy ではなく compose mount 前提に更新される

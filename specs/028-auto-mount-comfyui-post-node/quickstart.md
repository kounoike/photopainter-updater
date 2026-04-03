# Quickstart: ComfyUI custom node 自動登録

## 1. ComfyUI を起動する

```bash
docker compose up -d comfyui
```

追加の copy 手順は不要。`compose.yml` が repo 内 `comfyui/custom_node/comfyui-photopainter-custom`
を自動で mount する。

## 2. node が見えることを確認する

1. `http://localhost:18188` を開く
2. Add Node から `PhotoPainter PNG POST` を探す

必要ならログも確認する。

```bash
docker compose logs --tail=200 comfyui
```

## 3. 既存 custom node も残っていることを確認する

- ComfyUI Manager や既存 custom node が引き続き見えることを確認する
- `comfyui-data/custom_nodes` 内の既存ファイルが消えていないことを確認する

## 4. 更新反映

repo 内 `comfyui/custom_node/comfyui-photopainter-custom` を更新したあと:

```bash
docker compose restart comfyui
```

再起動後に node の更新内容を確認する。

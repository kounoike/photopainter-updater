# ComfyUI PhotoPainter PNG POST Node

`PhotoPainter PNG POST` は、ComfyUI の `IMAGE` 入力を `Content-Type: image/png`
の raw body として任意 URL へ `POST` する終端ノードです。

## 入力

- `image`: ComfyUI の単一 `IMAGE`
- `url`: `http` または `https` の送信先 URL

## 挙動

- 画像は 1 回の node 実行につき 1 枚だけ送信します
- 送信成功条件は `200 OK` 固定です
- 成功時は UI summary に status と応答本文要約を表示します
- URL 不正、入力不足、接続失敗、`200` 以外の status は例外になり、workflow を失敗扱いにします

## 026 との接続例

`docker compose up -d server` で 026 の upload server を起動したあと、ComfyUI から見える URL を
`url` に指定します。

```text
http://192.168.1.10:8000/upload
```

## runtime への配置

repo 管理ソースは `comfyui/custom_node/comfyui-photopainter-custom/` にあります。
`comfyui/Dockerfile` がこのディレクトリを ComfyUI image の
`/root/ComfyUI/custom_nodes/comfyui-photopainter-custom` に copy するため、
container 起動時の追加 mount や copy は不要です。third-party custom node の clone と依存導入は `comfyui/install-custom-nodes.sh` にまとめています。

```bash
docker compose build comfyui
docker compose up -d comfyui
```

node 読み込み確認:

```bash
docker compose logs --tail=200 comfyui
```

読み込み失敗がなければ ComfyUI の Add Node から `PhotoPainter PNG POST` を選べます。

repo 側ソースを更新したあとは ComfyUI image を再 build します。

```bash
docker compose build comfyui
docker compose up -d comfyui
```

container を作り直して確認したい場合:

```bash
docker compose down
docker compose up -d comfyui
```

## テスト

host 側の Python 標準ライブラリだけで回る unit test を同梱しています。

```bash
python -m unittest discover -s comfyui/custom_node/comfyui-photopainter-custom/tests -v
```

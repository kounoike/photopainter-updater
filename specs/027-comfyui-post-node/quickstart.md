# Quickstart: ComfyUI PNG POSTノード

## 前提

- リポジトリルートで `docker compose up -d comfyui` により ComfyUI が起動できる
- 026 の upload server を使う場合は `server/run.sh` などで `POST /upload` が到達可能になっている
- ComfyUI が Docker 内、server がホスト上で動く場合、送信先 URL は `localhost` ではなく ComfyUI から見えるホスト IP を使う

## 1. ComfyUI を起動する

repo 管理ソース `comfyui/custom_node/comfyui-photopainter-custom/` は `compose.yml`
により ComfyUI container の `custom_nodes` 探索先へ自動 mount される。

```bash
docker compose up -d comfyui
```

## 2. node が読み込まれることを確認する

```bash
docker compose logs --tail=200 comfyui
```

ComfyUI 起動ログに custom node 読み込み失敗が出ていないこと、UI の Add Node から
`PhotoPainter PNG POST` が見えることを確認する。既存の ComfyUI Manager や他の
custom node が消えていないことも確認する。

## 3. 026 upload server を起動する

```bash
cd server
./run.sh
```

ComfyUI コンテナから見える URL を使う。例:

```text
http://192.168.1.10:8000/upload
```

## 4. Workflow で node を使う

1. ComfyUI の Add Node メニューから `PhotoPainter PNG POST` を追加する
2. 画像生成または加工ノードの `IMAGE` 出力を接続する
3. `url` に送信先を入力する
4. workflow を実行する

期待結果:

- 成功時: node が成功し、UI summary に `200 OK` が見える
- 失敗時: node がエラー終了し、workflow 全体が失敗扱いになる

## 5. 026 との疎通を確認する

成功後に server 側で次を確認する。

```bash
curl -I http://127.0.0.1:8000/image.bmp
python - <<'PY'
from pathlib import Path
import struct
data = Path("server/contents/image.png").read_bytes()
assert data[:8] == b"\x89PNG\r\n\x1a\n"
width, height = struct.unpack(">II", data[16:24])
print(width, height)
PY
```

`480 800` が出れば 026 側の正規化済み画像更新まで到達している。

## 6. 失敗確認

- `url` を空にして実行する
- `http://127.0.0.1:9/upload` のような到達不能 URL にする
- `400` を返すテスト先を使う

いずれも node が成功扱いにならず、実行エラーになることを確認する。

## 7. node 更新を反映する

repo 側の custom node ソースを編集したあとは ComfyUI を再起動する。

```bash
docker compose restart comfyui
```

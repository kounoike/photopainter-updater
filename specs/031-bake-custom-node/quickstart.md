# Quickstart: ComfyUI custom node 同梱コンテナ

## 1. 前提条件

- `030-build-comfyui-image` の self-build ComfyUI 構成が使えること
- Docker Engine、Docker Compose v2、NVIDIA GPU 実行環境があること
- repo 管理 custom node は image に焼き込まれ、追加 custom node の永続維持は今回対象外であること

## 2. 設定ファイルを用意する

```bash
cp .env.example .env
```

必要に応じて `COMFYUI_PORT`、`COMFYUI_REF`、`COMFYUI_DATA_DIR`、`COMFYUI_CLI_ARGS` を編集する。

## 3. repo 管理 custom node 入り image を build する

```bash
docker compose build comfyui
```

この build で repo 管理 custom node が ComfyUI image へ焼き込まれる。`comfyui/custom_node/` 配下の source を更新した場合は、この手順を再実行する。

## 4. ComfyUI を起動する

```bash
docker compose up -d comfyui
docker compose logs --tail=200 comfyui
```

ブラウザで `http://localhost:${COMFYUI_PORT:-18188}` を開き、`PhotoPainter PNG POST` が Add Node から選べることを確認する。

## 5. 追加 custom node について

repo 管理 custom node は image に含まれている。追加 custom node を手元で試すことはできても、再作成後に残ることは保証しない。この feature では追加 custom node の永続化導線は提供しない。

## 6. 再作成を確認する

```bash
docker compose down
docker compose up -d comfyui
docker compose logs --tail=200 comfyui
```

再作成後も `PhotoPainter PNG POST` が見えることを確認する。

## 7. 困ったときの確認先

- build 失敗時: `docker compose build comfyui`
- repo 管理 custom node が見えない時: `docker compose logs --tail=200 comfyui`
- compose 展開確認: `docker compose config`

## 8. 運用ルール

- repo 管理 custom node を更新した時: `docker compose build comfyui` 後に `docker compose up -d comfyui`
- 既存 model / output / input / user 設定は `${COMFYUI_DATA_DIR}` を継続利用する
- 追加 custom node は baked-in feature の対象外であり、再作成後の維持を保証しない

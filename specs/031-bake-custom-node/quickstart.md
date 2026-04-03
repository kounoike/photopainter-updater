# Quickstart: ComfyUI custom node 同梱コンテナ

## 1. 前提条件

- `030-build-comfyui-image` の self-build ComfyUI 構成が使えること
- Docker Engine、Docker Compose v2、NVIDIA GPU 実行環境があること
- repo 管理 custom node と選定済み third-party custom node は image に焼き込まれ、追加 custom node の永続維持は今回対象外であること

## 2. 設定ファイルを用意する

```bash
cp .env.example .env
```

必要に応じて `COMFYUI_PORT`、`COMFYUI_REF`、`COMFYUI_DATA_DIR`、`COMFYUI_CLI_ARGS` を編集する。

## 3. repo 管理 custom node 入り image を build する

```bash
docker compose build comfyui
```

この build で repo 管理 custom node と初期同梱 third-party custom node が ComfyUI image へ焼き込まれる。`comfyui/custom_node/` 配下の source を更新した場合や、Dockerfile の pinned ref を更新した場合は、この手順を再実行する。

## 4. ComfyUI を起動する

```bash
docker compose up -d comfyui
docker compose logs --tail=200 comfyui
```

ブラウザで `http://localhost:${COMFYUI_PORT:-18188}` を開き、`PhotoPainter PNG POST` と `EasyUse`、`Ollama`、`Xz3r0` 系 node が Add Node から選べることを確認する。

## 5. 最初から同梱する third-party custom node

- `ComfyUI-Manager`: stable tag `4.1`
- `ComfyUI-Easy-Use`: stable tag `v1.3.6`
- `comfyui-ollama`: commit `6db7560576e5a59488708e6be13e07b5aba2432a`
- `ComfyUI-Xz3r0-Nodes`: stable tag `v1.7.0`

tag がある repo は tag 固定、tag がない repo は commit 固定にしている。

## 6. 追加 custom node について

repo 管理 custom node は image に含まれている。追加 custom node を手元で試すことはできても、再作成後に残ることは保証しない。この feature では追加 custom node の永続化導線は提供しない。

## 7. 再作成を確認する

```bash
docker compose down
docker compose up -d comfyui
docker compose logs --tail=200 comfyui
```

再作成後も `PhotoPainter PNG POST` と同梱 third-party custom node が見えることを確認する。

## 8. 困ったときの確認先

- build 失敗時: `docker compose build comfyui`
- repo 管理 custom node が見えない時: `docker compose logs --tail=200 comfyui`
- third-party custom node の import に失敗する時: `docker compose logs --tail=200 comfyui` と `docker compose build comfyui --progress=plain`
- compose 展開確認: `docker compose config`

## 9. 運用ルール

- repo 管理 custom node を更新した時: `docker compose build comfyui` 後に `docker compose up -d comfyui`
- third-party custom node を更新したい時: `comfyui/Dockerfile` の pinned ref を更新して `docker compose build comfyui`
- 既存 model / output / input / user 設定は `${COMFYUI_DATA_DIR}` を継続利用する
- 追加 custom node は baked-in feature の対象外であり、再作成後の維持を保証しない
- `comfyui-ollama` は node 自体を同梱するだけなので、実利用には `ollama` service または到達可能な Ollama server が別途必要

# Quickstart: ComfyUI 自前イメージ構築

## 1. 前提条件

- Docker Engine と Docker Compose v2（`docker compose`）が使えること
- NVIDIA GPU、ドライバ、NVIDIA Container Toolkit が利用可能であること
- `.env.example` を起点に ComfyUI の既存設定を流用できること

## 2. 設定ファイルを用意する

```bash
cp .env.example .env
```

必要に応じて `COMFYUI_PORT`、`COMFYUI_DATA_DIR`、`COMFYUI_CLI_ARGS` を編集する。

## 3. ComfyUI image を build する

```bash
docker compose build comfyui
```

repo 管理 Dockerfile を使って ComfyUI image を生成する。base runtime や repo 管理構成を更新した場合も、この手順で再 build する。

## 4. ComfyUI を起動する

```bash
docker compose up -d comfyui
docker compose logs --tail=200 comfyui
```

ブラウザで `http://localhost:${COMFYUI_PORT:-18188}` を開き、ComfyUI Web UI 到達可否を確認する。

## 5. 再起動と再作成を確認する

```bash
docker compose restart comfyui
docker compose logs --tail=200 comfyui

docker compose down
docker compose up -d comfyui
docker compose logs --tail=200 comfyui
```

再起動後と再作成後の両方で、同じ URL から UI 到達可否を確認する。

## 6. repo 管理 custom node を確認する

`PhotoPainter PNG POST` が Add Node から選べること、また既存の `COMFYUI_DATA_DIR/custom_nodes` 配下の custom node が見えなくなっていないことを確認する。

## 7. 困ったときの確認先

- build 失敗時: `docker compose build comfyui`
- runtime 起動失敗時: `docker compose logs --tail=200 comfyui`
- GPU 疎通確認: `docker exec photopainter-comfyui nvidia-smi`
- compose 展開確認: `docker compose config`

## 8. 備考

- この repository 作業環境では `docker` コマンドが使えない場合がある。その場合、上記確認は Docker 利用可能な実行環境で実施する。
- 既存の `COMFYUI_DATA_DIR` を継続利用するため、モデルや output を移し替える前提にはしない。

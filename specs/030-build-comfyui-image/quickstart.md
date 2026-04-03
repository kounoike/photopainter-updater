# Quickstart: ComfyUI 自前イメージ構築

## 1. 前提条件

- Docker Engine と Docker Compose v2（`docker compose`）が使えること
- NVIDIA GPU、ドライバ、NVIDIA Container Toolkit が利用可能であること
- 今回の構成は NVIDIA / CUDA 前提であり、CPU / AMD / Intel 向けではないこと
- `.env.example` を起点に ComfyUI の既存設定を流用できること

## 2. 設定ファイルを用意する

```bash
cp .env.example .env
```

必要に応じて `COMFYUI_PORT`、`COMFYUI_REF`、`COMFYUI_DATA_DIR`、`COMFYUI_MODEL_ROOT`、`COMFYUI_CLI_ARGS` を編集する。

## 3. ComfyUI image を build する

```bash
docker compose build comfyui
```

repo 管理 Dockerfile を使って ComfyUI image を生成する。CUDA 対応 Python base image 上に、ComfyUI upstream manual install 手順を `uv` で固定した runtime を構築する。PyTorch backend は `cu128` 固定とし、Docker build 時の `auto` 判定には依存しない。base ref や repo 管理構成を更新した場合も、この手順で再 build する。third-party custom node の clone / 依存導入は `comfyui/install-custom-nodes.sh` が担当する。

## 4. ComfyUI を起動する

```bash
docker compose up -d comfyui
docker compose logs --tail=200 comfyui
```

ブラウザで `http://localhost:${COMFYUI_PORT:-18188}` を開き、ComfyUI Web UI 到達可否を確認する。既定では `--listen 0.0.0.0 --fast --enable-manager` を使い、host 側公開ポートからの到達と Manager UI を使った custom node 運用を継続できる。

## 5. 再起動と再作成を確認する

```bash
docker compose restart comfyui
docker compose logs --tail=200 comfyui

docker compose down
docker compose up -d comfyui
docker compose logs --tail=200 comfyui
```

再起動後と再作成後の両方で、同じ URL から UI 到達可否を確認する。

`COMFYUI_MODEL_ROOT` を未指定なら既存どおり `COMFYUI_DATA_DIR/models` を使う。RunPod Serverless を前提にする場合は `/runpod-volume/models` のような永続領域を指定し、entrypoint が `ComfyUI/models` 配下へ symlink して吸収する。

## 6. repo 管理 custom node を確認する

`PhotoPainter PNG POST` が Add Node から選べることを確認する。既定構成では `ComfyUI-Manager`、`ComfyUI-Easy-Use`、`comfyui-ollama`、`ComfyUI-Xz3r0-Nodes` も最初から image に含まれる。repo 側 source や pinned ref を更新した場合は `docker compose build comfyui` を再実行する。

## 7. 困ったときの確認先

- build 失敗時: `docker compose build comfyui`
- runtime 起動失敗時: `docker compose logs --tail=200 comfyui`
- GPU 疎通確認: `docker exec photopainter-comfyui nvidia-smi`
- compose 展開確認: `docker compose config`
- `Torch not compiled with CUDA enabled` が出る場合: image を rebuild し、Dockerfile の PyTorch CUDA wheel 導入ログを確認する
- NVIDIA driver / CUDA mismatch が出る場合: `cu128` 固定のまま `nvidia-smi` と container log を見て、host driver と NVIDIA Container Toolkit 側を確認する
- 新規 clone から 20 分以内に build と起動判断まで到達できることを、README とこの quickstart を使って確認する

## 8. 備考

- この repository 作業環境では `docker` コマンドが使えない場合がある。その場合、上記確認は Docker 利用可能な実行環境で実施する。
- 既存の `COMFYUI_DATA_DIR` を継続利用するため、モデルや output を移し替える前提にはしない。

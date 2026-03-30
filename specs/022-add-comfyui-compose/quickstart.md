# クイックスタート: ComfyUI Docker Compose

## 前提条件

- Docker および Docker Compose v2（`docker compose` コマンド）インストール済み
- NVIDIA GPU + ドライバインストール済み（`nvidia-smi` が動作すること）
- [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html) インストール済み

NVIDIA Container Toolkit のインストール（Ubuntu/Debian）:
```bash
curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey | \
  sudo gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg
curl -s -L https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list | \
  sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' | \
  sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list
sudo apt-get update && sudo apt-get install -y nvidia-container-toolkit
sudo nvidia-ctk runtime configure --runtime=docker
sudo systemctl restart docker
```

---

## 起動手順

### 1. 環境設定ファイルの準備

```bash
cp .env.example .env
# 必要に応じて .env を編集（ポート番号・データディレクトリ等）
```

### 2. データディレクトリの作成（任意・Docker が自動作成するが明示的に準備する場合）

```bash
DATA_DIR=$(grep COMFYUI_DATA_DIR .env | cut -d= -f2)
DATA_DIR="${DATA_DIR:-./comfyui-data}"
mkdir -p "${DATA_DIR}"/{models,custom_nodes,dot-local,output,user,input,dot-cache}
```

### 3. ComfyUI の起動

```bash
docker compose up
```

初回起動はイメージの pull に数分かかります（イメージサイズ約 4.6 GB）。

### 4. ブラウザでアクセス

```
http://localhost:18188
```

デフォルトポートを変更した場合は `.env` の `COMFYUI_PORT` の値を使用してください。

---

## バックグラウンド起動・停止

```bash
# バックグラウンドで起動
docker compose up -d

# ログ確認
docker compose logs -f comfyui

# 停止（コンテナ削除）
docker compose down

# 停止（コンテナ保持）
docker compose stop
```

---

## モデルの追加

モデルファイルを `${COMFYUI_DATA_DIR}/models/` 以下の適切なサブディレクトリに配置します：

```
comfyui-data/models/
├── checkpoints/     ← Stable Diffusion / FLUX チェックポイント (.safetensors)
├── loras/           ← LoRA ファイル
├── controlnet/      ← ControlNet モデル
├── upscale_models/  ← 超解像モデル
└── vae/             ← VAE ファイル
```

起動中のコンテナへの追加も反映されます（ComfyUI がモデルをスキャンするため）。

---

## カスタムノードのインストール

1. ブラウザで ComfyUI Web UI を開く（`http://localhost:18188`）
2. 右上メニューから **Manager** を開く（ComfyUI Manager は初期搭載）
3. **Install Custom Nodes** からノードを検索・インストール
4. インストール後は ComfyUI を再起動（Web UI からまたは `docker compose restart comfyui`）

カスタムノードは `comfyui-data/custom_nodes/` に、依存ライブラリは `comfyui-data/dot-local/` に永続保存されます。

---

## GPU 利用の確認

```bash
# コンテナ内で GPU が認識されているか確認
docker exec photopainter-comfyui nvidia-smi

# ComfyUI のシステム情報は Web UI の Settings → System Stats で確認
```

---

## よくある問題

### GPU が認識されない

```bash
# NVIDIA Container Toolkit の確認
nvidia-ctk runtime configure --runtime=docker
sudo systemctl restart docker

# テスト
docker run --rm --gpus all nvidia/cuda:12.0-base-ubuntu22.04 nvidia-smi
```

### ポートが使用中

`.env` の `COMFYUI_PORT` を別の番号（例: `28188`）に変更して再起動。

### カスタムノードが起動後に消える

`comfyui-data/custom_nodes/` と `comfyui-data/dot-local/` の bind mount が正しく機能しているか確認：

```bash
ls comfyui-data/custom_nodes/
ls comfyui-data/dot-local/lib/
```

---

## 設定リファレンス（`.env`）

| 変数 | デフォルト | 説明 |
|------|-----------|------|
| `COMFYUI_PORT` | `18188` | ホスト側公開ポート |
| `COMFYUI_DATA_DIR` | `./comfyui-data` | データディレクトリ親パス |
| `COMFYUI_CLI_ARGS` | `--fast` | ComfyUI 起動フラグ |

# データモデル・設定仕様: ComfyUI Docker Compose 統合

**Phase 1 成果物** | Branch: `022-add-comfyui-compose`

---

## 1. 環境変数定義（`.env` / `.env.example`）

| 変数名 | デフォルト値 | 必須 | 説明 |
|--------|------------|------|------|
| `COMFYUI_PORT` | `18188` | 推奨 | ホスト側公開ポート番号 |
| `COMFYUI_DATA_DIR` | `./comfyui-data` | 推奨 | データディレクトリの親パス（bind mount ルート） |
| `COMFYUI_CLI_ARGS` | `--fast` | 任意 | ComfyUI 起動フラグ（`--fast`, `--lowvram`, `--highvram` 等） |

### 変数詳細

#### `COMFYUI_PORT`
- **型**: 整数（1–65535）
- **デフォルト**: `18188`（通常の ComfyUI ポート `8188` と区別するため `18188` を使用）
- **用途**: `0.0.0.0:${COMFYUI_PORT}:8188` としてホスト全インターフェースに公開

#### `COMFYUI_DATA_DIR`
- **型**: ファイルシステムパス（相対パスは `compose.yml` ディレクトリ基準）
- **デフォルト**: `./comfyui-data`
- **例**: `/mnt/nas/comfyui`、`/home/user/ai-data`
- **用途**: モデル・カスタムノード・出力等の親ディレクトリ。各サブディレクトリへの bind mount のプレフィックスとして使用

#### `COMFYUI_CLI_ARGS`
- **型**: 文字列（ComfyUI CLI フラグ）
- **デフォルト**: `--fast`（Ada Lovelace 以降 GPU での float8 最適化を有効化）
- **参考値**:
  - `--fast`: GPU 最適化有効（RTX 30/40 シリーズ推奨）
  - `--lowvram`: VRAM 節約モード（速度低下と引き換え）
  - `--highvram`: VRAM 積極利用（高性能 GPU 向け）

---

## 2. ボリューム構成

### バインドマウント一覧

```
${COMFYUI_DATA_DIR}/
├── models/          → /root/ComfyUI/models        （必須）
├── custom_nodes/    → /root/ComfyUI/custom_nodes  （必須）
├── dot-local/       → /root/.local                （必須）
├── output/          → /root/ComfyUI/output        （必須）
├── user/            → /root/ComfyUI/user          （推奨）
├── input/           → /root/ComfyUI/input         （推奨）
└── dot-cache/       → /root/.cache                （推奨）
```

### 各ディレクトリの役割

| ホスト側（`${COMFYUI_DATA_DIR}` 配下） | コンテナ内パス | 内容 | 永続化優先度 |
|--------------------------------------|--------------|------|-------------|
| `models/` | `/root/ComfyUI/models` | Stable Diffusion / FLUX チェックポイント、LoRA、ControlNet 等（数十 GB 規模） | **必須** |
| `custom_nodes/` | `/root/ComfyUI/custom_nodes` | ComfyUI Manager 経由でインストールしたカスタムノード本体コード | **必須** |
| `dot-local/` | `/root/.local` | カスタムノードの `pip install --user` 依存ライブラリ（`lib/python3.13/site-packages` 等） | **必須** |
| `output/` | `/root/ComfyUI/output` | 生成画像（PNG/JPG）、一時ファイル | **必須** |
| `user/` | `/root/ComfyUI/user` | ComfyUI ユーザー設定、保存済みワークフロー | 推奨 |
| `input/` | `/root/ComfyUI/input` | 画像生成に使用する入力素材 | 推奨 |
| `dot-cache/` | `/root/.cache` | pip キャッシュ、HuggingFace Hub モデルキャッシュ、PyTorch キャッシュ | 推奨 |

### カスタムノード依存ライブラリの永続化詳細

ComfyUI Manager がカスタムノードをインストールする際：

1. **ノード本体** → `/root/ComfyUI/custom_nodes/<node-name>/` → `custom_nodes/` にマウント
2. **Python 依存** → `pip install --user` → `/root/.local/lib/python3.13/site-packages/` → `dot-local/` にマウント

これにより、イメージを `docker pull` で更新しても両方が保持される（"dockerful design"）。

---

## 3. ネットワーク定義

### 名前付き bridge ネットワーク `photopainter`

```yaml
networks:
  photopainter:
    driver: bridge
```

| 属性 | 値 |
|------|-----|
| 名前 | `photopainter` |
| ドライバ | `bridge`（ローカル開発標準） |
| スコープ | `compose.yml` が存在するホスト上 |

### 将来のサービス追加時の変更範囲

HTTP サーバ（Rust axum）を追加する場合に必要な変更：

```yaml
# 変更不要: comfyui サービス定義はそのまま
services:
  comfyui:                      # ← 変更なし
    networks:
      - photopainter

  # 追加のみ ↓
  http-server:
    image: photopainter-server
    networks:
      - photopainter            # 同じネットワークに追加するだけ
    ports:
      - "${SERVER_PORT:-8080}:8080"

networks:
  photopainter:
    driver: bridge              # ← 変更なし
```

サービス間通信は `http://comfyui:8188` のようにサービス名で解決可能。

---

## 4. GPU 設定

### Docker Compose v2 GPU 設定

```yaml
deploy:
  resources:
    reservations:
      devices:
        - driver: nvidia
          count: all
          capabilities: [gpu]
environment:
  - NVIDIA_VISIBLE_DEVICES=all
  - NVIDIA_DRIVER_CAPABILITIES=compute,utility
```

### 前提条件

| 条件 | 確認方法 |
|------|---------|
| NVIDIA ドライバインストール済み | `nvidia-smi` が動作すること |
| NVIDIA Container Toolkit インストール済み | `nvidia-container-toolkit` パッケージ |
| Docker デーモンに nvidia ランタイム設定済み | toolkit インストール時に自動設定 |

---

## 5. サービスライフサイクル

| 設定 | 値 | 理由 |
|------|-----|------|
| `restart` | `unless-stopped` | ホスト再起動後に自動復帰。`docker compose stop` で手動停止した場合は再起動しない |
| `healthcheck.test` | `curl -f http://localhost:8188/system_stats` | ComfyUI 標準 API エンドポイントを利用 |
| `healthcheck.start_period` | `60s` | GPU 初期化・モデルロードに要する起動時間を考慮 |
| `healthcheck.interval` | `30s` | 定常監視の間隔 |

---

## 6. `.gitignore` 追加項目

```gitignore
# ComfyUI Docker 設定・データ
.env
comfyui-data/
```

- `.env`: ポート番号・パス等の個人設定が含まれるためコミット不要
- `comfyui-data/`: 数十 GB のモデルファイルや生成画像を含む Git 管理外ディレクトリ

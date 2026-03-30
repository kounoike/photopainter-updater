# Research: ComfyUI Docker Compose 統合

**Phase 0 成果物** | Branch: `022-add-comfyui-compose`

---

## 1. yanwk/comfyui-boot ボリューム構成

### Decision
`yanwk/comfyui-boot:cu130-slim`（または `cu130-slim-v2`）を採用し、以下のボリューム構成を使用する。

### cu130-slim vs cu130-slim-v2 の違い

| タグ | 説明 |
|------|------|
| `cu130-slim` | 安定版。`cu130-slim-v2` と同一イメージを指すことが多い |
| `cu130-slim-v2` | "dockerful design" を明示採用。カスタムノードとその依存を bind mount で分離することが設計思想として明記されている |

**採用タグ**: `cu130-slim-v2`（設計思想が明確・ドキュメントが充実）

### コンテナ内ディレクトリとボリュームのマッピング

| コンテナ内パス | 用途 | 永続化優先度 |
|----------------|------|-------------|
| `/root/ComfyUI/models` | モデルファイル（SD, FLUX 等） | **必須** |
| `/root/ComfyUI/custom_nodes` | カスタムノード本体コード | **必須** |
| `/root/.local` | カスタムノードの pip 依存ライブラリ（`pip install --user`） | **必須** |
| `/root/ComfyUI/output` | 生成画像の出力先 | **必須** |
| `/root/ComfyUI/user` | ComfyUI ユーザープロファイル・ワークフロー設定 | 推奨 |
| `/root/ComfyUI/input` | 画像入力素材 | 推奨 |
| `/root/.cache` | pip キャッシュ・HuggingFace Hub キャッシュ・torch キャッシュ | 任意（再ダウンロード防止） |
| `/root/.config` | アプリケーション設定 | 任意 |
| `/root/user-scripts` | `pre-start.sh` などのユーザースクリプト | 任意 |

**注意**: `/root/.cache/huggingface/hub` と `/root/.cache/torch/hub` は `/root/.cache` の子ディレクトリ。`/root/.cache` をまとめてマウントすれば個別指定は不要だが、ネストマウントの問題が生じる場合は個別指定に切り替える。

### Rationale
- カスタムノードの依存ライブラリが `/root/.local` に書き込まれるのは `pip install --user` によるもの
- `cu130-slim-v2` の "dockerful design" は「イメージ更新（`docker pull`）でコアを更新、カスタムノードと依存はボリュームで維持」という分離思想を採用
- モデルファイルは数十 GB になるためホストの任意ディレクトリに配置できることが重要

---

## 2. Docker Compose v2 NVIDIA GPU 設定

### Decision
Docker Compose v2 の `deploy.resources.reservations.devices` を使用する。

```yaml
services:
  comfyui:
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: all
              capabilities: [gpu]
```

### 前提条件
1. ホスト OS に NVIDIA ドライバがインストール済み
2. ホスト OS に [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html) がインストール済み
3. Docker デーモンの設定で `nvidia` ランタイムが有効（通常は toolkit インストール時に自動設定）

### Rationale
- `docker run --runtime=nvidia` の旧方式は非推奨
- `deploy` ブロックは `docker compose up` でのみ有効（`docker run` には不適用）
- `count: all` で全 GPU をコンテナに渡す。特定 GPU のみ必要な場合は `device_ids: ["0"]` で指定可能
- `capabilities: [gpu]` は必須フィールド

### Alternatives considered
- `runtime: nvidia` を直接指定する方式 → Docker Compose v2 では非推奨
- `--gpus all` を `command` に埋め込む → 不可（CLI オプション）

---

## 3. Docker Compose .env 変数パターン

### Decision
`.env` ファイルで環境変数を定義し、`compose.yml` から `${VAR:-default}` 構文で参照する。

```dotenv
# .env
COMFYUI_PORT=18188
COMFYUI_DATA_DIR=./comfyui-data
```

```yaml
# compose.yml
services:
  comfyui:
    ports:
      - "${COMFYUI_PORT:-18188}:8188"
    volumes:
      - ${COMFYUI_DATA_DIR:-./comfyui-data}/models:/root/ComfyUI/models
      - ${COMFYUI_DATA_DIR:-./comfyui-data}/custom_nodes:/root/ComfyUI/custom_nodes
```

### .env.example の用意
`.env` 自体は `.gitignore` に追加し、`.env.example` をリポジトリにコミットする。

### Rationale
- Docker Compose は自動的にプロジェクトルートの `.env` を読み込む（明示的な `env_file` 不要）
- `${VAR:-default}` 形式でデフォルト値を定義すれば `.env` なしでも動作する
- ホスト側パスは絶対パスも相対パスも使用可能（相対パスは `compose.yml` のあるディレクトリ基準）

---

## 4. Docker Compose ネットワーク定義

### Decision
将来の HTTP サーバ追加を考慮し、名前付き bridge ネットワークを定義する。

```yaml
services:
  comfyui:
    networks:
      - photopainter

networks:
  photopainter:
    driver: bridge
```

### 将来の HTTP サーバ追加時
既存の `comfyui` サービス定義を変更せず、新サービスに同じネットワークを追加するだけでよい：

```yaml
services:
  comfyui:
    networks:
      - photopainter  # 変更不要

  http-server:        # 追加するだけ
    networks:
      - photopainter

networks:
  photopainter:
    driver: bridge
```

### Rationale
- 名前付きネットワークを使うことで、サービス間通信をホスト名（サービス名）で行える
- `default` ネットワークでも動作するが、将来の明示的なサービス分離に備えて名前付きを採用
- `driver: bridge` はローカル開発用の標準設定

---

## 5. compose.yml vs docker-compose.yml

### Decision
`compose.yml` を使用する（Docker Compose v2 の推奨ファイル名）。

### Rationale
- Docker Compose v2（`docker compose` コマンド）は `compose.yml` を優先的に読み込む
- `docker-compose.yml` は後方互換のために残されているが、新規プロジェクトでは `compose.yml` が推奨
- 本プロジェクトはすでに spec で `compose.yml` を明記している

---

## 6. セキュリティ・運用ベストプラクティス

### .gitignore 設定
```gitignore
.env
comfyui-data/
```
- `.env` にはポート番号やデータディレクトリパスが含まれるためコミット不要
- `comfyui-data/` は大容量モデルファイルを含むため Git 管理外

### restart ポリシー
```yaml
services:
  comfyui:
    restart: unless-stopped
```
開発・実験用途なのでホスト再起動時に自動起動。`no` にするとホスト再起動後に手動起動が必要。

### healthcheck
ComfyUI は HTTP エンドポイント `/system_stats` を提供するため healthcheck が設定可能：
```yaml
healthcheck:
  test: ["CMD", "curl", "-f", "http://localhost:8188/system_stats"]
  interval: 30s
  timeout: 10s
  retries: 3
  start_period: 60s
```

---

## 総括

| 決定事項 | 採用内容 |
|---------|---------|
| イメージ | `yanwk/comfyui-boot:cu130-slim-v2` |
| GPU 設定 | `deploy.resources.reservations.devices` |
| ボリューム方式 | bind mount、パスは `.env` で変更可 |
| 必須ボリューム | models, custom_nodes, .local, output |
| 推奨ボリューム | user, input, .cache |
| ネットワーク | 名前付き bridge: `photopainter` |
| ポート | `0.0.0.0:${COMFYUI_PORT:-18188}:8188` |
| ファイル名 | `compose.yml`（リポジトリルート） |
| restart | `unless-stopped` |

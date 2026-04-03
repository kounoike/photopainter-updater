# Contract: ComfyUI 自前イメージ runtime

## 1. Service Contract

`compose.yml` は引き続き `comfyui` service を提供する。

| 項目 | 契約 |
|------|------|
| service 名 | `comfyui` を維持する |
| 起動入口 | `docker compose up -d comfyui` を維持する |
| build 入口 | `docker compose build comfyui` で repo 管理 image を生成できる |
| Web UI | `${COMFYUI_PORT:-18188}` で到達できる |
| 依存関係 | 既存 `ollama` との compose 共存を壊さない |

## 2. Storage Contract

- `${COMFYUI_DATA_DIR:-./comfyui-data}` を引き続き永続データ親ディレクトリとして使う
- `models`、`custom_nodes`、`dot-local`、`output`、`user`、`input`、`dot-cache` の主要導線を壊さない
- container 再作成後も host 側の既存データを継続利用できる

## 3. Repo-managed Runtime Contract

- ComfyUI 実行環境の基準は repo 管理 Dockerfile から再生成できること
- runtime 成立に必要な repo 管理構成は手作業ではなく build 導線から再現できること
- repo 管理 custom node は新しい構成でも ComfyUI から見えること

## 4. Documentation Contract

- root README は ComfyUI の build、起動、再起動、再作成の導線を案内する
- feature quickstart は `docker compose build comfyui` を含む
- 既存の ComfyUI 利用者が、起動 URL と主要設定入口の継続を理解できること

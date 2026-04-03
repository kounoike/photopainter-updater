# Contract: ComfyUI baked custom node runtime

## 1. Service Contract

`compose.yml` は引き続き `comfyui` service を提供する。

| 項目 | 契約 |
|------|------|
| service 名 | `comfyui` を維持する |
| build 入口 | `docker compose build comfyui` で repo 管理 custom node 入り image を生成できる |
| 起動入口 | `docker compose up -d comfyui` を維持する |
| Web UI | `${COMFYUI_PORT:-18188}` で到達できる |
| repo 管理 node | image build 時に baked-in される |

## 2. Storage Contract

- `${COMFYUI_DATA_DIR:-./comfyui-data}` を引き続き永続データ親ディレクトリとして使う
- `models`、`output`、`input`、`user`、`dot-cache`、`dot-local` の主要導線を壊さない
- `${COMFYUI_DATA_DIR}/custom_nodes` の永続互換はこの feature の保証対象外とする

## 3. Runtime Contract

- repo 管理 custom node は container 作成時点で利用可能であること
- repo 管理 custom node の source 更新反映には rebuild が必要であること
- 再起動と再作成後も repo 管理 node の利用状態を維持すること
- 追加 custom node は再作成後の維持を保証しないこと

## 4. Documentation Contract

- root README は repo 管理 baked-in node と追加 custom node の違いを案内する
- feature quickstart は rebuild 条件と追加 custom node 非永続を説明する
- custom node README は runtime 配置説明を build 時同梱前提へ更新する

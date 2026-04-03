# 調査メモ: ComfyUI custom node 自動登録

## Decision 1: 自動登録は追加 bind mount で行う

**Decision**: 既存の `${COMFYUI_DATA_DIR}/custom_nodes` mount を残したまま、repo 内 node ディレクトリを `/root/ComfyUI/custom_nodes/comfyui-photopainter-custom` に追加 bind mount する。  
**Rationale**: custom node 本体はすでに repo 管理されており、別 installer や copy 手順を挟む必要がない。compose だけで完結する最小構成で、ローカル優先の方針にも合う。  
**Alternatives considered**:
- container 起動後に copy する: manual step が残る
- 専用 init service を追加する: 過剰

## Decision 2: 既存 custom_nodes ディレクトリ全体 mount は維持する

**Decision**: `${COMFYUI_DATA_DIR}/custom_nodes:/root/ComfyUI/custom_nodes` は残したまま、PhotoPainter node を子 path に追加する。  
**Rationale**: ComfyUI Manager や既存 custom node の保存先を壊さないことが優先。全体 mount を置き換えると、ユーザー既存資産を壊すリスクがある。  
**Alternatives considered**:
- 全体 mount を repo 側へ置き換える: 既存運用を壊す
- `${COMFYUI_DATA_DIR}` 側へ node ソースを複製する: repo と runtime の二重管理になる

## Decision 3: repo 側 mount は read-only とする

**Decision**: repo 管理 node の bind mount には `:ro` を付ける。  
**Rationale**: container 内から repo ソースを書き換えない方が安全で、Git 管理との責務分離が明確になる。  
**Alternatives considered**:
- read-write mount: 不要な書き戻し経路が増える

## Decision 4: 文書は manual copy 導線を自動 mount 導線へ置き換える

**Decision**: root README と 027 quickstart、node README を compose 自動登録前提に更新する。  
**Rationale**: 実装だけ自動化しても、手順書が copy 前提のままだと運用がぶれる。  
**Alternatives considered**:
- 旧手順を残す: 理解コストが増える

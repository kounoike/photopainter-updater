# 調査メモ: ComfyUI custom node 同梱コンテナ

## Decision 1: repo 管理 custom node は image build 時に copy する

**Decision**: `comfyui/custom_node/comfyui-photopainter-custom` は `comfyui/Dockerfile` から ComfyUI image へ copy し、repo mount には依存しない。  
**Rationale**: feature の中心要求は「container を作る時点で custom node 入りの状態にする」ことにある。runtime bind mount のままでは host 側の状態に依存し、container 作成時点の再現性が弱い。  
**Alternatives considered**:
- repo 管理 custom node を bind mount のまま残す: baked-in container にならない
- 起動時に host から copy する: runtime 時点の可変作業が残る

## Decision 2: `${COMFYUI_DATA_DIR}/custom_nodes` の互換は今回 scope 外とする

**Decision**: 利用者追加 custom node の永続維持は今回対象外とし、`${COMFYUI_DATA_DIR}/custom_nodes` の bind mount は削除する。  
**Rationale**: `ComfyUI/custom_nodes` 全体 mount は baked-in node を隠すため両立しない。今回 user 指示で「追加したら消えても問題ない」が明示されたため、repo 管理 node の baked-in 化に scope を絞る方が単純で誤解も少ない。  
**Alternatives considered**:
- 従来どおり `ComfyUI/custom_nodes` 全体 mount を続ける: baked-in node が見えなくなる
- staging path に mount して entrypoint で統合する: 今回の要求には不要な複雑化になる

## Decision 3: entrypoint は baked-in node 前提で最小化する

**Decision**: `comfyui/entrypoint.sh` に追加 custom node 統合作業は持ち込まず、baked-in node が既に `custom_nodes` 配下へ存在する前提で起動する。  
**Rationale**: repo 管理 node の baked-in 化だけで目的を満たせるなら、entrypoint を複雑化しない方が再現性と保守性が高い。  
**Alternatives considered**:
- entrypoint で追加 custom node を symlink する: 今回は不要な運用分岐が増える
- entrypoint で host から copy する: runtime 可変状態が増える

## Decision 4: repo 管理 node と追加 node の運用条件を分けて文書化する

**Decision**: repo 管理 node の変更は rebuild、追加 node は維持対象外であることを README / quickstart / custom node README に明記する。  
**Rationale**: baked-in node と一時的な追加 node では持続性が異なるため、ここを曖昧にすると利用者が再作成後の挙動を誤解する。  
**Alternatives considered**:
- 追加 node の扱いを文書化しない: 再作成後の期待がずれる
- 追加 node も永続化対象にする: scope が広がる

## Decision 5: repo 管理 node の責務はこのリポジトリ配下に限定する

**Decision**: image に焼き込む対象はこのリポジトリで管理している custom node のみに限定し、第三者 node の自動導入は scope 外とする。  
**Rationale**: feature の要求は repo 管理 node の baked-in 化であり、第三者 node まで扱うと download source、versioning、license、failure path が広がりすぎる。  
**Alternatives considered**:
- 任意の第三者 node も一緒に image へ含める: scope が過大
- ComfyUI Manager 連携で一括導入する: runtime 依存と外部依存が増える

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

**Decision**: third-party custom node は利用者指定の 4 件に限定し、`ComfyUI-Manager` は stable tag `4.1`、`ComfyUI-Easy-Use` は stable tag `v1.3.6`、`ComfyUI-Xz3r0-Nodes` は stable tag `v1.7.0`、`comfyui-ollama` は tag 不在のため commit `6db7560576e5a59488708e6be13e07b5aba2432a` で固定する。  
**Rationale**: tag がある repo は tag 固定の方が更新点と rollback を追いやすい。一方で `comfyui-ollama` は tag が見当たらないため commit 固定が最も再現性が高い。指定 4 件だけに絞れば、scope と failure path をまだ管理できる。  
**Alternatives considered**:
- すべて HEAD 追従にする: build の再現性が落ちる
- 4 件すべて commit 固定にする: tag がある repo でも人間にとって追跡しにくい
- 任意の第三者 node も一緒に image へ含める: scope が過大

## Decision 6: third-party custom node の依存は build 時に導入する

**Decision**: 選定済み third-party custom node は Dockerfile 内で clone し、各 `requirements.txt` を `uv pip install --system` で導入する。`ComfyUI-Xz3r0-Nodes` 向けに `ffmpeg` を apt で追加する。  
**Rationale**: `ComfyUI-Easy-Use`、`comfyui-ollama`、`ComfyUI-Manager`、`ComfyUI-Xz3r0-Nodes` は Python 依存を持つ。runtime に Manager で後入れさせると「最初から入っている」要求を満たさない。`Xz3r0` README は `ffmpeg` を system PATH 前提としているため、image 側へ含める必要がある。  
**Alternatives considered**:
- ComfyUI 起動後に Manager で入れる: runtime 可変になり要求を満たさない
- requirements は無視して clone のみ行う: import 失敗リスクが高い
- `ffmpeg` を host 依存にする: container 単体の再現性が落ちる

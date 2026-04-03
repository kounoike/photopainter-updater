# 機能仕様: ComfyUI custom node 同梱コンテナ

**Feature Branch**: `031-bake-custom-node`  
**Created**: 2026-04-04  
**Status**: Draft  
**Input**: ユーザー記述: "comfyuiにカスタムノードを入れた状態でコンテナを作りたい"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Clarifications

### Session 2026-04-04

- Q: repo 管理 custom node と同名の node が利用者側にもある場合、どちらを優先するか → A: repo 管理 custom node は volume mount せず image に焼き込み、このリポジトリで管理する custom node を同梱前提とする
- Q: `${COMFYUI_DATA_DIR}/custom_nodes` の利用者追加 custom node を維持する必要があるか → A: 追加 custom node は再作成で消えてよく、別 mount での維持は不要
- Q: 最初から入れておく third-party custom node は何か → A: `ComfyUI-Manager`、`ComfyUI-Easy-Use`、`comfyui-ollama`、`ComfyUI-Xz3r0-Nodes` を image build 時に同梱し、tag がある repo は tag 固定、tag がない repo は commit 固定にする

## User Scenarios & Testing *(mandatory)*

### User Story 1 - custom node 入りのコンテナをそのまま起動したい (Priority: P1)

ComfyUI の利用者は、repo 管理の custom node と選定済み third-party custom node が image build 時点で組み込まれた container を起動し、追加の mount や copy を意識せずに node を使いたい。

**Why this priority**: 今回の主目的は custom node の配置を runtime 依存から減らし、container 作成時点で利用可能な状態へ寄せることだから。

**Independent Test**: `docker compose build comfyui` 後に `docker compose up -d comfyui` を実行し、ComfyUI 上で `PhotoPainter PNG POST` と同梱対象 third-party custom node が直ちに選択可能であれば完了。

**Acceptance Scenarios**:

1. **Given** 利用者が repo の build 手順に従って ComfyUI image を生成している, **When** ComfyUI container を起動する, **Then** repo 管理 custom node と選定済み third-party custom node は追加作業なしで読み込まれる。
2. **Given** 新しい ComfyUI container を作成する状態, **When** 起動直後に node 一覧を確認する, **Then** `PhotoPainter PNG POST` と `comfyui-ollama`、`ComfyUI-Easy-Use`、`ComfyUI-Xz3r0-Nodes` 由来 node が利用可能になっている。

---

### User Story 2 - 再作成しても同じ custom node 構成を保ちたい (Priority: P2)

ComfyUI の利用者は、container の再起動や再作成のあとでも、repo 管理 custom node と選定済み third-party custom node が同じ状態で入ったまま復帰してほしい。

**Why this priority**: build 時に同梱しても再作成時に状態差分が出るなら、再現性改善の価値が薄れるため。

**Independent Test**: `docker compose restart comfyui` と `docker compose down && docker compose up -d comfyui` を実施し、その都度 `PhotoPainter PNG POST` と同梱対象 third-party custom node が見えることを確認できれば完了。

**Acceptance Scenarios**:

1. **Given** custom node 入りの ComfyUI container が起動している, **When** 利用者が `docker compose restart comfyui` を実行する, **Then** 再起動後も同じ custom node が利用できる。
2. **Given** 利用者が ComfyUI container を削除して作り直す状態, **When** `docker compose down && docker compose up -d comfyui` を実行する, **Then** custom node 導線を手で戻さなくても元の利用状態へ復帰できる。

---

### User Story 3 - baked-in node の運用条件を誤解したくない (Priority: P3)

ComfyUI の利用者は、repo 管理 custom node と一時的な追加 custom node の扱いの違いを理解し、何が再作成後に残り、何が残らないかを事前に把握したい。

**Why this priority**: repo 管理 node だけを image に焼き込む構成では、追加 custom node が再作成後に残ると誤解すると運用事故につながるため。

**Independent Test**: README と quickstart を見て、repo 管理 custom node は baked-in、追加 custom node は再作成で消えうることを判断できれば完了。

**Acceptance Scenarios**:

1. **Given** 利用者が custom node 運用手順を確認している, **When** repo 管理 node と追加 node の扱いを読む, **Then** repo 管理 node だけが image 同梱対象であると理解できる。
2. **Given** 利用者が追加 custom node を試す状態, **When** container を再作成する, **Then** 追加 custom node が維持対象外であることを文書から事前に判断できる。

---

### Edge Cases

- repo 管理 custom node または選定済み third-party custom node が build 時に取り込めない場合、build または起動ログから原因を追跡できること。
- repo 管理 custom node は volume mount せず image に焼き込むため、追加 custom node が維持対象外であることを文書で分かること。
- repo 側の custom node ソースを更新した場合、再 build が必要であることを利用者が判断できること。
- `ComfyUI-Xz3r0-Nodes` の `ffmpeg` 前提や `comfyui-ollama` の Ollama 接続前提が、image build と runtime の境界を壊さないこと。
- custom node 同梱に切り替えても、既存の model / output / input mount は維持されること。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST repo 管理の PhotoPainter custom node を ComfyUI image に焼き込まれた状態で提供しなければならない。
- **FR-002**: System MUST `ComfyUI-Manager`、`ComfyUI-Easy-Use`、`comfyui-ollama`、`ComfyUI-Xz3r0-Nodes` を ComfyUI image に焼き込まれた状態で提供しなければならない。
- **FR-003**: System MUST `docker compose build comfyui` と `docker compose up -d comfyui` の導線だけで repo 管理 custom node と選定済み third-party custom node が利用可能になるようにしなければならない。
- **FR-004**: System MUST container の再起動および再作成後も、同じ repo 管理 custom node と選定済み third-party custom node を追加作業なしで利用可能にしなければならない。
- **FR-005**: System MUST `${COMFYUI_DATA_DIR:-./comfyui-data}` 配下の既存 model、output、input、user 設定の導線を壊してはならない。
- **FR-006**: System MUST 利用者が repo 側 custom node 更新時に再 build が必要かどうかを README または quickstart から判断できるようにしなければならない。
- **FR-007**: System MUST このリポジトリで管理する custom node と選定済み third-party custom node は volume mount せず image へ焼き込み、追加 custom node は維持対象外である方針を明示しなければならない。
- **FR-008**: System MUST build 失敗または custom node 読み込み失敗時の最初の確認先を文書化しなければならない。
- **FR-009**: System MUST `PhotoPainter PNG POST` node が起動後に選択可能であることを検証できなければならない。
- **FR-010**: System MUST 選定済み third-party custom node の clone 元 ref を Dockerfile で固定し、tag がある repo は tag、tag がない repo は commit で pin しなければならない。
- **FR-011**: System MUST 選定済み third-party custom node の Python 依存と `ffmpeg` 前提を image build に含めなければならない。

### Key Entities *(include if feature involves data)*

- **repo 管理 custom node**: `comfyui/custom_node/comfyui-photopainter-custom` にある、ComfyUI image へ同梱したい node ソース。
- **選定済み third-party custom node**: `ComfyUI-Manager`、`ComfyUI-Easy-Use`、`comfyui-ollama`、`ComfyUI-Xz3r0-Nodes` の 4 件で、Dockerfile から clone / install して image に同梱する node 群。
- **ComfyUI runtime image**: repo 管理 Dockerfile から build され、ComfyUI 本体、repo 管理 custom node、選定済み third-party custom node を含む実行環境。
- **追加 custom node**: 利用者が一時的に container 内へ追加できるが、再作成後の維持対象ではない node。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `comfyui/Dockerfile`、`comfyui/entrypoint.sh`、`compose.yml` の custom node 同梱に必要な見直し
- README、quickstart、custom node README の導線更新
- custom node 同梱後の build / up / restart / recreate 検証
- 選定済み third-party custom node の pinned ref、Python 依存、`ffmpeg` 導入

### Forbidden Scope

- PhotoPainter custom node 自体の機能変更
- ComfyUI custom node manager の仕様変更
- server、firmware、AI Toolkit、Ollama の機能変更
- 指定 4 件以外の第三者 node 導入自動化

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は `docker compose build comfyui` と `docker compose up -d comfyui` のみで、追加の mount 調整や copy なしに `PhotoPainter PNG POST` と選定済み third-party custom node を ComfyUI 上で確認できる。
- **SC-002**: `docker compose restart comfyui` と `docker compose down && docker compose up -d comfyui` のあとも、同じ custom node 構成が継続して利用可能である。
- **SC-003**: 既存の `${COMFYUI_DATA_DIR:-./comfyui-data}` を保持した利用者は、model、output、input、user 設定の既存運用を変えずに移行できる。
- **SC-004**: README と quickstart を読んだ利用者が、repo 管理 custom node 更新時は再 build が必要であり、追加 custom node は再作成で維持されないことと、失敗時の最初の確認先を判断できる。
- **SC-005**: 選定済み third-party custom node の clone ref が tag または commit で固定され、再 build 時に同じ custom node 構成を再現できる。

## Assumptions

- repo 管理 custom node は `comfyui/custom_node/comfyui-photopainter-custom` とし、third-party custom node は `ComfyUI-Manager`、`ComfyUI-Easy-Use`、`comfyui-ollama`、`ComfyUI-Xz3r0-Nodes` の 4 件に限定する。
- 既存の ComfyUI self-build 導線、service 名、公開 URL、`COMFYUI_DATA_DIR` は維持する。
- custom node 同梱後も、repo 側ソース変更の反映には再 build が必要になる。
- このリポジトリで管理する custom node は image へ焼き込み、runtime の repo mount には依存しない。
- 追加 custom node は維持対象ではなく、再作成後に残らなくても問題ない。
- tag がある third-party repo は tag 固定を優先し、tag がない repo のみ commit 固定にする。

## Documentation Impact

- root README の ComfyUI 導線に、custom node 同梱と再 build 条件を追記する必要がある。
- root README の ComfyUI 導線に、初期同梱される third-party custom node 一覧と pinned ref 方針を追記する必要がある。
- `specs/030-build-comfyui-image/quickstart.md` の custom node 説明を、mount 前提から同梱前提へ見直す必要がある。
- `comfyui/custom_node/comfyui-photopainter-custom/README.md` の runtime 配置説明を、container 起動時 mount 前提から build 時同梱前提へ更新する必要がある。

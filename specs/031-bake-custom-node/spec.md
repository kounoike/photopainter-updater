# 機能仕様: ComfyUI custom node 同梱コンテナ

**Feature Branch**: `031-bake-custom-node`  
**Created**: 2026-04-04  
**Status**: Draft  
**Input**: ユーザー記述: "comfyuiにカスタムノードを入れた状態でコンテナを作りたい"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - custom node 入りのコンテナをそのまま起動したい (Priority: P1)

ComfyUI の利用者は、repo 管理の custom node が image build 時点で組み込まれた container を起動し、追加の mount や copy を意識せずに node を使いたい。

**Why this priority**: 今回の主目的は custom node の配置を runtime 依存から減らし、container 作成時点で利用可能な状態へ寄せることだから。

**Independent Test**: `docker compose build comfyui` 後に `docker compose up -d comfyui` を実行し、ComfyUI 上で `PhotoPainter PNG POST` が直ちに選択可能であれば完了。

**Acceptance Scenarios**:

1. **Given** 利用者が repo の build 手順に従って ComfyUI image を生成している, **When** ComfyUI container を起動する, **Then** repo 管理 custom node は追加作業なしで読み込まれる。
2. **Given** 新しい ComfyUI container を作成する状態, **When** 起動直後に node 一覧を確認する, **Then** `PhotoPainter PNG POST` が利用可能になっている。

---

### User Story 2 - 再作成しても同じ custom node 構成を保ちたい (Priority: P2)

ComfyUI の利用者は、container の再起動や再作成のあとでも、repo 管理 custom node が同じ状態で入ったまま復帰してほしい。

**Why this priority**: build 時に同梱しても再作成時に状態差分が出るなら、再現性改善の価値が薄れるため。

**Independent Test**: `docker compose restart comfyui` と `docker compose down && docker compose up -d comfyui` を実施し、その都度 `PhotoPainter PNG POST` が見えることを確認できれば完了。

**Acceptance Scenarios**:

1. **Given** custom node 入りの ComfyUI container が起動している, **When** 利用者が `docker compose restart comfyui` を実行する, **Then** 再起動後も同じ custom node が利用できる。
2. **Given** 利用者が ComfyUI container を削除して作り直す状態, **When** `docker compose down && docker compose up -d comfyui` を実行する, **Then** custom node 導線を手で戻さなくても元の利用状態へ復帰できる。

---

### User Story 3 - 既存の利用者データ導線を壊したくない (Priority: P3)

ComfyUI の利用者は、repo 管理 custom node を image へ同梱しても、既存の `COMFYUI_DATA_DIR` 配下のモデルや利用者追加 custom node 運用はそのまま残したい。

**Why this priority**: repo 管理 node の同梱が既存データや利用者の custom node を壊すと、単なる追加機能ではなく回帰になるため。

**Independent Test**: 既存の `${COMFYUI_DATA_DIR:-./comfyui-data}` を維持したまま ComfyUI を起動し、repo 管理 custom node と利用者追加 custom node の両方が見えることを確認できれば完了。

**Acceptance Scenarios**:

1. **Given** 利用者が `${COMFYUI_DATA_DIR:-./comfyui-data}/custom_nodes` を使っている, **When** custom node 同梱版 ComfyUI を起動する, **Then** 利用者追加 custom node は引き続き利用できる。
2. **Given** repo 管理 custom node が image に含まれている, **When** 利用者が既存の model や output 保存先を使う, **Then** 既存の保存先や URL 導線は変わらない。

---

### Edge Cases

- repo 管理 custom node が build 時に取り込めない場合、build または起動ログから原因を追跡できること。
- 同名の custom node が利用者側 `custom_nodes` 配下にも存在する場合、どちらを優先するかが文書で分かること。
- repo 側の custom node ソースを更新した場合、再 build が必要であることを利用者が判断できること。
- custom node 同梱に切り替えても、ComfyUI Manager や既存の model / output / input mount は維持されること。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST repo 管理の PhotoPainter custom node を ComfyUI image または image から確実に再現される runtime 構成へ含めなければならない。
- **FR-002**: System MUST `docker compose build comfyui` と `docker compose up -d comfyui` の導線だけで repo 管理 custom node が利用可能になるようにしなければならない。
- **FR-003**: System MUST container の再起動および再作成後も、同じ repo 管理 custom node を追加作業なしで利用可能にしなければならない。
- **FR-004**: System MUST `${COMFYUI_DATA_DIR:-./comfyui-data}` 配下の既存 model、output、input、user 設定、利用者追加 custom node の導線を壊してはならない。
- **FR-005**: System MUST 利用者が repo 側 custom node 更新時に再 build が必要かどうかを README または quickstart から判断できるようにしなければならない。
- **FR-006**: System MUST repo 管理 custom node と利用者追加 custom node の共存方針を明示しなければならない。
- **FR-007**: System MUST build 失敗または custom node 読み込み失敗時の最初の確認先を文書化しなければならない。
- **FR-008**: System MUST `PhotoPainter PNG POST` node が起動後に選択可能であることを検証できなければならない。

### Key Entities *(include if feature involves data)*

- **repo 管理 custom node**: `comfyui/custom_node/comfyui-photopainter-custom` にある、ComfyUI image へ同梱したい node ソース。
- **ComfyUI runtime image**: repo 管理 Dockerfile から build され、ComfyUI 本体と repo 管理 custom node を含む実行環境。
- **利用者 custom_nodes 導線**: `${COMFYUI_DATA_DIR:-./comfyui-data}/custom_nodes` を通じて追加される、利用者管理の custom node 保存先。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `comfyui/Dockerfile`、`comfyui/entrypoint.sh`、`compose.yml` の custom node 同梱に必要な見直し
- README、quickstart、custom node README の導線更新
- custom node 同梱後の build / up / restart / recreate 検証

### Forbidden Scope

- PhotoPainter custom node 自体の機能変更
- ComfyUI custom node manager の仕様変更
- server、firmware、AI Toolkit、Ollama の機能変更
- repo 管理 custom node 以外の第三者 node 導入自動化

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は `docker compose build comfyui` と `docker compose up -d comfyui` のみで、追加の mount 調整や copy なしに `PhotoPainter PNG POST` を ComfyUI 上で確認できる。
- **SC-002**: `docker compose restart comfyui` と `docker compose down && docker compose up -d comfyui` のあとも、同じ custom node が継続して利用可能である。
- **SC-003**: 既存の `${COMFYUI_DATA_DIR:-./comfyui-data}` を保持した利用者は、model、output、利用者追加 custom node の既存運用を変えずに移行できる。
- **SC-004**: README と quickstart を読んだ利用者が、repo 管理 custom node 更新時に再 build が必要であることと、失敗時の最初の確認先を判断できる。

## Assumptions

- 対象の repo 管理 custom node は `comfyui/custom_node/comfyui-photopainter-custom` に限定する。
- 既存の ComfyUI self-build 導線、service 名、公開 URL、`COMFYUI_DATA_DIR` は維持する。
- 利用者追加 custom node の保存先は引き続き host 側 bind mount を使う。
- custom node 同梱後も、repo 側ソース変更の反映には再 build が必要になる。

## Documentation Impact

- root README の ComfyUI 導線に、custom node 同梱と再 build 条件を追記する必要がある。
- `specs/030-build-comfyui-image/quickstart.md` の custom node 説明を、mount 前提から同梱前提へ見直す必要がある。
- `comfyui/custom_node/comfyui-photopainter-custom/README.md` の runtime 配置説明を、container 起動時 mount 前提から build 時同梱前提へ更新する必要がある。

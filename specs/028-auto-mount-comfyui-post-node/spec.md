# 機能仕様: ComfyUI custom node 自動登録

**Feature Branch**: `028-auto-mount-comfyui-post-node`  
**Created**: 2026-04-03  
**Status**: Draft  
**Input**: ユーザー記述: "今作ったカスタムノードをcompose.ymlでマウントするときに自動的に登録して"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 起動時に自動で使いたい (Priority: P1)

ComfyUI の利用者は、`docker compose up` 後に追加の copy や手動配置をせず、repo 内の PhotoPainter custom node をそのまま ComfyUI から使いたい。

**Why this priority**: 現在の手順では custom node 導入に追加作業が必要で、ComfyUI 利用導線として無駄が大きいため。

**Independent Test**: `docker compose up -d comfyui` 後に ComfyUI を開き、`PhotoPainter PNG POST` が Add Node 一覧に現れることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者がこの repo を clone している状態, **When** `docker compose up -d comfyui` を実行する, **Then** PhotoPainter custom node は追加作業なしで ComfyUI に読み込まれる。
2. **Given** custom node のソースが repo 内に存在する状態, **When** ComfyUI container が起動する, **Then** custom node は ComfyUI の `custom_nodes` 探索先に見える状態になる。

---

### User Story 2 - 既存 custom_nodes 運用を壊したくない (Priority: P2)

ComfyUI の利用者は、既存の `comfyui-data/custom_nodes` 運用や ComfyUI Manager の導線を壊さずに、repo 管理ノードだけを追加で自動登録したい。

**Why this priority**: 既存 custom node 運用を壊すと、追加機能ではなく回帰になるため。

**Independent Test**: 既存の `comfyui-data/custom_nodes` 内容を保持したまま起動し、ComfyUI Manager と PhotoPainter node の両方が見えることを確認する。

**Acceptance Scenarios**:

1. **Given** 既存の `comfyui-data/custom_nodes` に他ノードが入っている状態, **When** compose で ComfyUI を起動する, **Then** 既存ノードはそのまま使え、PhotoPainter node だけが追加で見える。
2. **Given** 利用者が ComfyUI Manager で他の custom node を管理している状態, **When** この feature を適用した compose を使う, **Then** 既存の custom node 保存先や manager の運用前提は維持される。

---

### User Story 3 - 導入手順を簡潔にしたい (Priority: P3)

ComfyUI の利用者は、README と quickstart を見たときに manual copy 手順ではなく、compose 起動だけで node が有効になる導線で理解したい。

**Why this priority**: 実装だけ自動化されても手順書が古いままだと運用で迷うため。

**Independent Test**: README と quickstart を見て、manual copy なしの compose 導線だけで node 利用開始まで到達できることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が README を読む状態, **When** custom node の導入手順を確認する, **Then** compose 起動時に自動登録される前提で理解できる。
2. **Given** 利用者が feature 配下の quickstart を使う状態, **When** 導入手順を実行する, **Then** manual copy を要求されずに node 利用へ進める。

---

### Edge Cases

- repo 内の custom node ディレクトリが存在しない場合、ComfyUI 起動時に原因を追跡できること。
- 既存の `comfyui-data/custom_nodes` 配下のノードを上書きまたは消去しないこと。
- compose の変更により ComfyUI Manager や既存 output / model / input mount を壊さないこと。
- host 側で custom node ソースを更新した場合、再起動後に更新内容を反映できること。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST `compose.yml` の ComfyUI サービス起動時に、repo 内の PhotoPainter custom node を自動的に ComfyUI の custom node 探索先へ見せなければならない。
- **FR-002**: System MUST 既存の `comfyui-data/custom_nodes` 全体 mount を維持しつつ、PhotoPainter custom node を追加登録しなければならない。
- **FR-003**: System MUST 利用者に manual copy や container 内での追加配置作業を要求してはならない。
- **FR-004**: System MUST `PhotoPainter PNG POST` node が ComfyUI 起動後に選択可能であることを検証できなければならない。
- **FR-005**: System MUST 既存の ComfyUI Manager と他 custom node の運用導線を壊してはならない。
- **FR-006**: System MUST README と quickstart を、自動登録前提の導線へ更新しなければならない。
- **FR-007**: System MUST repo 内 node ソース更新後、ComfyUI 再起動で更新を反映できる構成でなければならない。

### Key Entities *(include if feature involves data)*

- **repo 管理 custom node**: `comfyui/custom_node/comfyui-photopainter-custom` に存在する PhotoPainter 用 custom node ソース。
- **ComfyUI runtime custom_nodes**: ComfyUI container が起動時に読み取る custom node 探索先。
- **compose mount 構成**: repo 管理 custom node と既存 `comfyui-data/custom_nodes` を両立させるための volume 設定。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `compose.yml` の ComfyUI custom node mount 調整
- custom node 自動登録前提に合わせた README / quickstart 更新
- custom node 自動登録を確認するための最小限の検証・文書更新

### Forbidden Scope

- custom node 自体の機能仕様変更
- 既存 server / firmware の変更
- ComfyUI Manager の動作変更
- 新しい常駐サービスや配布基盤の導入

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は `docker compose up -d comfyui` 後、追加の copy 手順なしで `PhotoPainter PNG POST` node を ComfyUI 上で確認できる。
- **SC-002**: 既存の `comfyui-data/custom_nodes` 内容を維持したまま、PhotoPainter node だけが追加で読み込まれる。
- **SC-003**: README と quickstart は manual copy ではなく compose 自動登録導線を案内している。
- **SC-004**: repo 内の custom node ソースを更新して ComfyUI を再起動すると、更新内容を再反映できる。

## Assumptions

- 利用者は既存の `compose.yml` を使って ComfyUI を起動する。
- PhotoPainter custom node のソースは repo 内 `comfyui/custom_node/comfyui-photopainter-custom` に存在する。
- 既存の `comfyui-data/custom_nodes` は他 node や ComfyUI Manager の保存先として継続利用する。
- 自動登録は compose の mount 構成で実現し、追加サービスや installer は導入しない。

## Documentation Impact

- README の custom node 導線を自動登録前提に更新する必要がある。
- 027 の quickstart / node README にある manual copy 前提を見直す必要がある。

# タスク: ComfyUI custom node 自動登録

**Input**: `/specs/028-auto-mount-comfyui-post-node/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3`)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: compose 自動登録の前提確認

- [ ] T001 `compose.yml`、`README.md`、`specs/027-comfyui-post-node/quickstart.md` の現行 custom node 導線を確認する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 自動登録 mount の基盤を整える

- [ ] T002 `compose.yml` の `comfyui` service に repo 管理 custom node 用 bind mount を追加する
- [ ] T003 [P] `specs/028-auto-mount-comfyui-post-node/quickstart.md` に compose volume 契約と確認方法を反映する

---

## Phase 3: User Story 1 - 起動時に自動で使いたい (Priority: P1)

**Goal**: `docker compose up -d comfyui` だけで PhotoPainter node を読み込ませる

**Independent Test**: `docker compose config` と ComfyUI 起動後の node discovery で `PhotoPainter PNG POST` が見える

### Verification for User Story 1

- [ ] T004 [US1] `specs/028-auto-mount-comfyui-post-node/quickstart.md` に `docker compose up -d comfyui` から node discovery までの確認手順を確定する

### Implementation for User Story 1

- [ ] T005 [US1] `README.md` の custom node 導線を manual copy なしの compose 自動登録前提へ更新する
- [ ] T006 [US1] `comfyui/custom_node/comfyui-photopainter-custom/README.md` の導入手順を compose 自動 mount 前提へ更新する

---

## Phase 4: User Story 2 - 既存 custom_nodes 運用を壊したくない (Priority: P2)

**Goal**: 既存 `comfyui-data/custom_nodes` と ComfyUI Manager の運用を維持する

**Independent Test**: 既存 custom_nodes mount を保ったまま compose 設定が成立し、文書でも併存前提が明示される

### Verification for User Story 2

- [ ] T007 [US2] `specs/028-auto-mount-comfyui-post-node/contracts/comfyui-custom-node-mount-contract.md` と `compose.yml` の volume 定義が一致していることを確認する

### Implementation for User Story 2

- [ ] T008 [US2] `specs/027-comfyui-post-node/quickstart.md` から manual copy 前提を除去し、既存 custom_nodes と併存する compose 導線へ更新する
- [ ] T009 [US2] `README.md` と `specs/028-auto-mount-comfyui-post-node/quickstart.md` に既存 custom node / ComfyUI Manager を壊さない前提を明記する

---

## Phase 5: User Story 3 - 導入手順を簡潔にしたい (Priority: P3)

**Goal**: compose 起動だけで理解できる導線に統一する

**Independent Test**: README と quickstart を読めば manual copy なしで開始手順を追える

### Verification for User Story 3

- [ ] T010 [US3] `README.md`、`specs/027-comfyui-post-node/quickstart.md`、`specs/028-auto-mount-comfyui-post-node/quickstart.md` の導線が自動登録前提で整合していることを確認する

### Implementation for User Story 3

- [ ] T011 [US3] `specs/028-auto-mount-comfyui-post-node/quickstart.md` の更新反映手順を compose 再起動前提へ整理する
- [ ] T012 [US3] `specs/028-auto-mount-comfyui-post-node/tasks.md` と最終文書内容を整合させる

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 最終確認

- [ ] T013 [P] `docker compose config` を実行して `compose.yml` の volume 定義が妥当であることを確認する
- [ ] T014 `specs/028-auto-mount-comfyui-post-node/quickstart.md` に実施できた確認結果と未実施手動確認の境界を反映する

## Dependencies & Execution Order

- Setup → Foundational → US1 → US2 / US3 → Polish
- MVP は US1

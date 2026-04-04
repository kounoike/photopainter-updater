# タスク: Health Port Listener

**Input**: `/specs/036-health-port-listener/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: `PORT_HEALTH` の契約と実装境界を固定する

- [X] T001 `specs/036-health-port-listener/spec.md`、`plan.md`、`contracts/health-port-contract.md` を照合し、未指定 / 同一 / 別 port の 3 パターンを実装前提として確定する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: config・app・routes の責務分担を揃える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [X] T002 `server/src/config.rs` に `PORT_HEALTH` を追加する設計を反映し、config test の更新範囲を明確化する
- [X] T003 [P] `server/src/routes.rs` の main router と health-only router の分離方針を整理する
- [X] T004 [P] `server/src/app.rs` の listener 起動と startup message の分岐更新範囲を整理する

**Checkpoint**: config / routes / app の責務分担が確定していること

---

## Phase 3: User Story 1 - health check を別 port で受けたい (Priority: P1)

**Goal**: `PORT_HEALTH` が別値のとき `/ping` だけを返す dedicated listener を追加する

**Independent Test**: `PORT` と `PORT_HEALTH` を異なる値で起動した想定で config / route / app test が通り、health-only listener の計画が dedicated になることを確認する

### Verification for User Story 1

- [X] T005 [P] [US1] `PORT_HEALTH` が別値のとき dedicated health listener になる config / app test を `server/src/config.rs` と `server/src/app.rs` に追加する
- [X] T006 [US1] health-only router が `/ping` だけ成功し他 path は `404` になる route test を `server/src/routes.rs` に追加する

### Implementation for User Story 1

- [X] T007 [US1] `PORT_HEALTH` 設定を `server/src/config.rs` に実装する
- [X] T008 [US1] health-only router を `server/src/routes.rs` に実装する
- [X] T009 [US1] main listener と dedicated health listener を同時起動する処理を `server/src/app.rs` に実装する
- [X] T010 [US1] startup message に health port の listen 先を `server/src/app.rs` へ反映する

**Checkpoint**: 別 port 時に health-only listener を独立検証できること

---

## Phase 4: User Story 2 - 同じ port 指定でも壊したくない (Priority: P2)

**Goal**: `PORT_HEALTH == PORT` でも起動失敗せず main listener の `/ping` を使えるようにする

**Independent Test**: `PORT` と `PORT_HEALTH` を同じ値で解釈したとき、追加 bind を行わないことを config / app test で確認する

### Verification for User Story 2

- [X] T011 [P] [US2] `PORT_HEALTH == PORT` のとき shared mode になる test を `server/src/config.rs` と `server/src/app.rs` に追加する

### Implementation for User Story 2

- [X] T012 [US2] `PORT_HEALTH == PORT` 時の追加 bind 抑止と startup message 分岐を `server/src/app.rs` に実装する
- [X] T013 [US2] shared mode でも main router 上の `/ping` 契約が維持されることを `server/src/routes.rs` と `server/src/app.rs` で担保する

**Checkpoint**: 同一 port 指定で起動失敗しないこと

---

## Phase 5: User Story 3 - 既存導線と設定方法を維持したい (Priority: P3)

**Goal**: 既存 route 契約を維持しつつ `PORT_HEALTH` の使い方を文書化する

**Independent Test**: README と quickstart を見て `PORT_HEALTH` の未指定 / 同一 / 別 port の違いを理解でき、既存 route が維持されることを確認する

### Verification for User Story 3

- [X] T014 [US3] `specs/036-health-port-listener/quickstart.md` に未指定 / 同一 / 別 port の確認手順を整理する

### Implementation for User Story 3

- [X] T015 [US3] `server/README.md` に `PORT_HEALTH` の説明と `/ping` health-only listener の使い方を追記する
- [X] T016 [US3] `specs/036-health-port-listener/quickstart.md` に route 公開範囲と回帰確認観点を実装する
- [X] T017 [US3] 既存 `/hello`、`/`、`/image.bmp`、`/image.bin`、`/upload` 契約の回帰を `server/src/routes.rs` と `server/src/app.rs` で担保する

**Checkpoint**: README だけで `PORT_HEALTH` の運用を理解できること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 最終整合と回帰確認を行う

- [X] T018 [P] `cargo test` を `server/` で実行し config / app / route 回帰が成功することを確認する
- [X] T019 `specs/036-health-port-listener/plan.md`、`contracts/health-port-contract.md`、`tasks.md` の記述整合を確認する
- [X] T020 `server/README.md` と `specs/036-health-port-listener/quickstart.md` の手順差分がないことを確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能。MVP
- **User Story 2 (P2)**: User Story 1 の listener 基盤を前提に開始する
- **User Story 3 (P3)**: User Story 1 と 2 の挙動確定後に開始する

### Parallel Opportunities

- Phase 2 の T003 と T004 は並列実行可能
- US1 の config / app test と health-only route test は並列実行可能
- Final Phase の T018 と T019 は並列実行可能

## Notes

- 実装対象は `server/` と feature artifact に限定する
- health-only listener は `/ping` 以外を公開しない

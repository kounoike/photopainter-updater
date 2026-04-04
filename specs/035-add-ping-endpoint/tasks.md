# タスク: Ping 動作確認エンドポイント

**Input**: `/specs/035-add-ping-endpoint/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: `/ping` の契約と変更範囲を固定する

- [ ] T001 `specs/035-add-ping-endpoint/spec.md` と `specs/035-add-ping-endpoint/contracts/ping-endpoint-contract.md` を照合し、`/ping` が `200 OK` と空 body を返す契約を実装前提として確定する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: route 追加位置と文書更新範囲を揃える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T002 `server/src/routes.rs` の router 構成と既存 `/hello` test を確認し、`/ping` の追加位置と回帰確認対象を明確化する
- [ ] T003 `server/README.md` と `specs/035-add-ping-endpoint/quickstart.md` の起動確認導線を確認し、`/ping` を追記する更新範囲を整理する

**Checkpoint**: route 追加位置、文書更新範囲、回帰確認対象が確定していること

---

## Phase 3: User Story 1 - サーバの到達性だけ確認したい (Priority: P1)

**Goal**: `/ping` だけで server 到達性を確認できるようにする

**Independent Test**: `cargo test` で `/ping` が `200 OK` と空 body を返し、画像未配置でも成功することを確認する

### Verification for User Story 1

- [ ] T004 [P] [US1] `/ping` の `200 OK` と空 body、logging を検証する route test を `server/src/routes.rs` に追加する
- [ ] T005 [US1] 画像未配置状態でも `/ping` が成功し既存 image route と切り分けられる route test を `server/src/routes.rs` に追加する

### Implementation for User Story 1

- [ ] T006 [US1] `GET /ping` route と空 body の handler を `server/src/routes.rs` に実装する
- [ ] T007 [US1] `/ping` の成功応答を既存 logging 経路へ統合し `server/src/routes.rs` に反映する
- [ ] T008 [US1] `cargo test` を `server/` で実行し `/ping` 追加後も route test 一式が成功することを確認する

**Checkpoint**: `/ping` が独立して検証可能であり、画像状態に依存せず成功すること

---

## Phase 4: User Story 2 - 確認手順を増やしすぎたくない (Priority: P2)

**Goal**: 利用文書だけで `/ping` の用途を理解できるようにする

**Independent Test**: README と quickstart を見て `/ping` が軽量な到達性確認用 endpoint だと分かることを確認する

### Verification for User Story 2

- [ ] T009 [US2] `/ping` を使った手動確認手順と期待結果を `specs/035-add-ping-endpoint/quickstart.md` に反映する

### Implementation for User Story 2

- [ ] T010 [US2] 起動確認セクションへ `/ping` の使い方を追記する内容を `server/README.md` に実装する
- [ ] T011 [US2] `/ping` の後に `/hello` や画像 route を確認する切り分け手順を `specs/035-add-ping-endpoint/quickstart.md` に実装する

**Checkpoint**: 利用者が `/ping` を最小の疎通確認手順として辿れること

---

## Phase 5: User Story 3 - 既存 endpoint は維持したい (Priority: P3)

**Goal**: `/ping` 追加後も既存 endpoint と fallback 契約を維持する

**Independent Test**: `cargo test` と quickstart の回帰確認手順で `/hello`、画像 route、upload、fallback が維持されることを確認する

### Verification for User Story 3

- [ ] T012 [US3] 既存 route と fallback の回帰確認観点を `specs/035-add-ping-endpoint/quickstart.md` に整理する

### Implementation for User Story 3

- [ ] T013 [US3] `/ping` 追加後も unknown path の `404` 契約が変わらないことを `server/src/routes.rs` の既存 test 群で維持または必要最小限更新する
- [ ] T014 [US3] `/ping` 追加後も `/hello`、`/`、`/image.bmp`、`/image.bin`、`/upload` の既存回帰が通ることを `server/src/routes.rs` と `server/README.md` で担保する

**Checkpoint**: 既存 endpoint と fallback の期待挙動が維持されていること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 最終整合と静的検証を行う

- [ ] T015 [P] `specs/035-add-ping-endpoint/plan.md`、`specs/035-add-ping-endpoint/contracts/ping-endpoint-contract.md`、`specs/035-add-ping-endpoint/tasks.md` の記述整合を確認する
- [ ] T016 `specs/035-add-ping-endpoint/quickstart.md` と `server/README.md` の手順差分がないことを確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能。MVP
- **User Story 2 (P2)**: User Story 1 の `/ping` 契約が確定した後に開始する
- **User Story 3 (P3)**: User Story 1 実装後に回帰確認として開始する

### Within Each User Story

- route test を先に追加し、その後 handler / router 実装を行う
- 文書 story は quickstart の確認観点を先に明記し、その後 README / quickstart 本文を更新する
- 回帰 story は既存 route の観点整理後に最終整合を取る

### Parallel Opportunities

- Phase 2 の T002 と T003 は並列実行可能
- User Story 1 完了後、User Story 2 の文書更新と User Story 3 の回帰観点整理は並列進行可能
- Final Phase の T015 と T016 は並列実行可能

---

## Parallel Example: User Story 1

```bash
Task: "T004 `/ping` の 200 OK と空 body、logging を検証する route test を server/src/routes.rs に追加する"
Task: "T005 画像未配置状態でも /ping が成功し既存 image route と切り分けられる route test を server/src/routes.rs に追加する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 と Phase 2 を完了する
2. User Story 1 の route test と `/ping` 実装を完了する
3. `cargo test` で `/ping` の独立検証を通す

### Incremental Delivery

1. `/ping` の route と test を完成させる
2. 利用文書を更新する
3. 既存 endpoint と fallback の回帰確認を行う
4. 契約 / quickstart / tasks の整合を最終確認する

### Parallel Team Strategy

1. 1 人が `server/src/routes.rs` の route / test 実装を担当する
2. 別担当が `server/README.md` と `specs/035-add-ping-endpoint/quickstart.md` の文書更新を担当する
3. 最後に回帰観点と契約整合をまとめて確認する

---

## Notes

- すべての task は Allowed Scope 内の `server/` と feature artifact に限定している
- 自動テストは既存 `server/src/routes.rs` 内 route test に寄せる
- `/ping` は空 body の `200 OK` に固定し、それ以上の状態情報は返さない

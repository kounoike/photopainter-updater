# タスク: Hello 動作確認エンドポイント

**Input**: `/specs/032-add-hello-endpoint/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 現状の route / documentation / test 変更点を今回 feature 向けに固定する

- [ ] T001 `specs/032-add-hello-endpoint/spec.md`、`specs/032-add-hello-endpoint/plan.md`、`specs/032-add-hello-endpoint/contracts/hello-endpoint-contract.md` を照合し、`/hello` が `text/plain` で `hello` を返す契約を実装方針として確定する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に先行する共通の実装境界と検証導線を揃える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T002 `server/src/routes.rs` の router 構成、fallback、既存 route test の配置を確認し、`/hello` 追加位置と回帰確認対象を明確化する
- [ ] T003 `server/README.md` と `specs/032-add-hello-endpoint/quickstart.md` の起動確認導線を確認し、`/hello` を先頭確認手順に差し替える更新範囲を整理する

**Checkpoint**: route 追加位置、文書更新範囲、回帰確認対象が確定していること

---

## Phase 3: User Story 1 - サーバ疎通をすぐ確認したい (Priority: P1)

**Goal**: 利用者が画像状態に依存せず `/hello` だけで server 稼働確認を完了できるようにする

**Independent Test**: `cargo test` で `/hello` の成功応答テストを通し、画像未配置状態でも `GET /hello` が `200 OK` と `text/plain` の `hello` を返すことを確認する

### Verification for User Story 1

- [ ] T004 [US1] `/hello` の成功応答と logging を検証する route テストを `server/src/routes.rs` に追加する
- [ ] T005 [US1] 画像未配置状態でも `/hello` が成功し既存 image route と切り分けられることを検証する route テストを `server/src/routes.rs` に追加する

### Implementation for User Story 1

- [ ] T006 [US1] `GET /hello` を router に追加し固定本文 `hello` を返す handler を `server/src/routes.rs` に実装する
- [ ] T007 [US1] `/hello` の成功応答で既存 `record` と `text_response` を再利用し、access log と content type を既存方針に合わせて `server/src/routes.rs` に統合する
- [ ] T008 [US1] `cargo test` を `server/` で実行し、`/hello` の追加後も route テスト一式が成功することを確認する

**Checkpoint**: `/hello` が独立して検証可能であり、画像状態に依存せず成功すること

---

## Phase 4: User Story 2 - 動作確認手順を単純化したい (Priority: P2)

**Goal**: 利用者が README と quickstart だけで `/hello` ベースの疎通確認手順を理解できるようにする

**Independent Test**: 更新後の文書を見て、初見の利用者が `/hello` を使う起動確認手順を追加説明なしで再現できることを確認する

### Verification for User Story 2

- [ ] T009 [US2] `/hello` を使った手動確認手順と期待結果を `specs/032-add-hello-endpoint/quickstart.md` に反映し、body が `hello` である確認点を明記する

### Implementation for User Story 2

- [ ] T010 [US2] 起動確認セクションと troubleshooting 導線を `/hello` 優先に更新する内容を `server/README.md` に実装する
- [ ] T011 [US2] `/hello` と既存 `/image.bmp` `/image.bin` を段階的に確認する利用手順を `specs/032-add-hello-endpoint/quickstart.md` に実装する

**Checkpoint**: 利用者が `/hello` を最初の確認手順として迷わず辿れること

---

## Phase 5: User Story 3 - 既存 endpoint を壊したくない (Priority: P3)

**Goal**: `/hello` を追加しても既存の取得・更新・fallback 契約を維持する

**Independent Test**: `cargo test` と quickstart の回帰確認手順で、`/hello` 追加後も `/`、`/image.bmp`、`/image.bin`、`/upload`、未定義 path の期待挙動が維持されることを確認する

### Verification for User Story 3

- [ ] T012 [US3] 既存 route の回帰確認観点と fallback 維持条件を `specs/032-add-hello-endpoint/quickstart.md` に整理する

### Implementation for User Story 3

- [ ] T013 [US3] `/hello` 追加が unknown path の `404` 契約を壊さないことを検証する既存 not found test の期待値を `server/src/routes.rs` で維持または必要最小限更新する
- [ ] T014 [US3] `/hello` 追加後も `/image.bmp`、`/image.bin`、`/upload` の既存回帰が通ることを `server/src/routes.rs` の既存 test 群と `server/README.md` の確認手順で担保する

**Checkpoint**: 既存 endpoint と fallback の期待挙動が維持されていること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 複数 story にまたがる最終整合と検証

- [ ] T015 [P] `specs/032-add-hello-endpoint/contracts/hello-endpoint-contract.md`、`specs/032-add-hello-endpoint/plan.md`、`specs/032-add-hello-endpoint/tasks.md` の記述整合を確認する
- [ ] T016 `specs/032-add-hello-endpoint/quickstart.md` の手順どおりに実施した想定で `server/README.md` と実装結果の齟齬がないことを確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能。MVP
- **User Story 2 (P2)**: User Story 1 の `/hello` 契約が確定した後に開始する
- **User Story 3 (P3)**: User Story 1 実装後に回帰確認として開始する

### Within Each User Story

- route テストを先に追加し、その後に handler / router 実装を行う
- 文書 story は quickstart の検証観点を先に明記し、その後 README / quickstart 本文を更新する
- 回帰 story は fallback / 既存 route の確認観点を明確にしてから最終整合を取る

### Parallel Opportunities

- Phase 2 の T002 と T003 は別ファイル中心の確認なので並列実行可能
- User Story 1 完了後、User Story 2 の文書更新と User Story 3 の回帰確認観点整理は並列進行可能
- Final Phase の T015 と T016 は対象ファイルが主に異なるため並列実行可能

---

## Parallel Example: User Story 1

```bash
Task: "T004 `/hello` の成功応答と logging を検証する route テストを server/src/routes.rs に追加する"
Task: "T005 画像未配置状態でも /hello が成功し既存 image route と切り分けられることを検証する route テストを server/src/routes.rs に追加する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 と Phase 2 を完了する
2. User Story 1 の route テストと `/hello` 実装を完了する
3. `cargo test` で `/hello` の独立検証を通す
4. server 稼働確認の最小価値を先に得る

### Incremental Delivery

1. `/hello` の route とテストを先に完成させる
2. 利用文書を `/hello` 優先の導線へ更新する
3. 既存 endpoint と fallback の回帰確認を行う
4. 契約 / quickstart / tasks の整合を最終確認する

### Parallel Team Strategy

1. 1 人が `server/src/routes.rs` の route / test 実装を担当する
2. 別担当が `server/README.md` と `specs/032-add-hello-endpoint/quickstart.md` の文書更新を担当する
3. 最後に回帰観点と契約整合をまとめて確認する

---

## Notes

- すべての task は Allowed Scope 内の `server/` と feature artifact に限定している
- 自動テストは既存の `server/src/routes.rs` 内 route test に寄せ、不要な新規 test ファイルは作らない
- `/hello` の本文は `text/plain` の `hello` に固定し、それ以外の応答拡張はこの feature の対象外とする

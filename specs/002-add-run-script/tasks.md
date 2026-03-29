# タスク: server 配信スクリプト追加

**Input**: `/specs/002-add-run-script/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 最小構成の準備

- [ ] T001 `server/` と `server/contents/` ディレクトリを作成する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に先行する基盤整備

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T002 `server/run.sh` の基本構成と前提チェックを追加する

**Checkpoint**: 基盤完了後に user story 実装へ進む

---

## Phase 3: User Story 1 - ローカル配信開始 (Priority: P1)

**Goal**: `server/contents/` をHTTP配信できる

**Independent Test**: スクリプト起動後にブラウザで配信内容へアクセスできる

### Verification for User Story 1

- [ ] T003 [US1] 手動確認手順を `specs/002-add-run-script/quickstart.md` に追記する

### Implementation for User Story 1

- [ ] T004 [US1] `server/run.sh` で `server/contents/` の配信を開始する
- [ ] T005 [US1] 起動失敗時の分かりやすい出力を `server/run.sh` に追加する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - 停止と再起動 (Priority: P2)

**Goal**: 停止操作と再起動が分かる

**Independent Test**: 停止後に配信が終了し、再実行で再度配信できる

### Verification for User Story 2

- [ ] T006 [US2] 停止方法と再起動手順を `specs/002-add-run-script/quickstart.md` に追記する

### Implementation for User Story 2

- [ ] T007 [US2] 停止操作を案内する出力を `server/run.sh` に追加する

**Checkpoint**: User Story 2 が独立して検証可能であること

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: 横断的な確認

- [ ] T008 `specs/002-add-run-script/quickstart.md` の手順を実際に検証する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 完了後に開始可能

### Within Each User Story

- 検証タスクを省略しない
- 実装後に手動確認手順を更新する

### Parallel Opportunities

- `[P]` 付きタスクはなし（単一ファイル中心のため）

---

## Parallel Example: User Story 1

```bash
Task: "手動確認手順を specs/002-add-run-script/quickstart.md に追記する"
Task: "server/run.sh で server/contents/ の配信を開始する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. User Story 1 を独立検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して独立検証する
3. User Story 2 を追加して独立検証する
4. 前段 story を壊していないことを確認する

### Parallel Team Strategy

1. Setup + Foundational を完了する
2. Foundational 完了後に story ごとに担当を分ける
3. 各担当は story 単位で独立検証まで完了する

---

## Notes

- `[Story]` は traceability のために必須
- 曖昧なタスク、同一ファイル衝突、独立性を壊す cross-story 依存を避ける

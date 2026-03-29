# タスク: ACT LED アクティビティ表示

**Input**: `/specs/006-activity-led-indicator/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: LED 制御部品を `firmware/` から利用できる状態にする

- [X] T001 `firmware/CMakeLists.txt` に `xiaozhi-esp32/components/led_bsp` を参照 component として追加する
- [X] T002 `firmware/main/CMakeLists.txt` に `led_bsp` 依存を追加し、`firmware/` から LED 制御 API を利用可能にする

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story が共有する ACT LED 制御基盤を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [X] T003 `firmware/main/` 配下に更新ジョブ活動状態と ACT LED 表示状態の責務を反映した共通制御方針を `main.cc` と `update_job.cc` に整理する
- [X] T004 [P] `firmware/main/main.cc` に LED 初期化と待機時消灯の共通初期化を追加する
- [X] T005 [P] `firmware/main/update_job.cc` に活動表示 LED の既定点滅パターン選定と開始・停止の共通ヘルパーを追加する
- [X] T006 `firmware/main/update_job.cc` に更新ジョブ直列実行制御と矛盾しない単一オーナーの LED 状態遷移を統合する
- [X] T007 `specs/006-activity-led-indicator/research.md` または `specs/006-activity-led-indicator/plan.md` に実機 LED/GPIO 確認結果と `firmware/` 側の変更範囲確認結果を反映し、Forbidden Scope へ触れていないことを記録する

**Checkpoint**: 基盤完了後に user story 実装へ進む

---

## Phase 3: User Story 1 - 更新中を見分けたい (Priority: P1)

**Goal**: 起動時更新と BOOT ボタン更新で、進行中だけ ACT LED が点滅する

**Independent Test**: 正常な `config.txt` を置いて起動し、起動時更新中と BOOT ボタン再更新中だけ ACT LED が点滅し、各更新完了後に停止することを確認する

### Verification for User Story 1

- [X] T008 [US1] `specs/006-activity-led-indicator/quickstart.md` に起動時更新と BOOT ボタン更新での ACT LED 手動確認手順を具体化する

### Implementation for User Story 1

- [X] T009 [US1] `firmware/main/update_job.cc` に起動時更新の開始から正常完了まで ACT LED 点滅を維持する処理を実装する
- [X] T010 [US1] `firmware/main/update_job.cc` に BOOT ボタン更新でも同じ ACT LED 点滅ルールを適用する処理を実装する
- [X] T011 [US1] `firmware/main/main.cc` と `firmware/main/update_job.cc` を統合し、待機状態で ACT LED が活動中に見えないことを保証する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - 長い待ち時間を誤解したくない (Priority: P2)

**Goal**: SD 読込、Wi-Fi 接続、HTTP 取得、表示更新などの待機時間中も ACT LED 点滅を継続する

**Independent Test**: 応答や表示に時間がかかる更新条件で起動し、更新終了まで ACT LED の点滅が途切れないことを確認する

### Verification for User Story 2

- [X] T012 [US2] `specs/006-activity-led-indicator/quickstart.md` に長時間待機時の ACT LED 観察手順を追加する

### Implementation for User Story 2

- [X] T013 [US2] `firmware/main/update_job.cc` に更新ジョブ全体を単位として ACT LED 点滅を維持し、個別処理待機中に消灯しないよう統合する
- [X] T014 [US2] `firmware/main/display_update.cc` と `firmware/main/update_job.cc` の責務境界を調整し、HTTP 取得や表示更新の待ち時間を含めて ACT LED が継続点滅することを保証する

**Checkpoint**: User Story 1 と 2 が独立検証可能であること

---

## Phase 5: User Story 3 - 失敗時も進行終了を見分けたい (Priority: P3)

**Goal**: 設定不備、通信失敗、画像不正で終了する場合でも、失敗確定時に ACT LED 点滅を停止する

**Independent Test**: `config.txt` 欠落、Wi-Fi 失敗、HTTP 失敗、画像不正を発生させ、各ケースで失敗確定まで点滅し、その後停止することを確認する

### Verification for User Story 3

- [X] T015 [US3] `specs/006-activity-led-indicator/quickstart.md` に 4 種類の失敗系で ACT LED 停止を確認する手順を追加する

### Implementation for User Story 3

- [X] T016 [US3] `firmware/main/update_job.cc` に失敗終了時の ACT LED 停止を正常終了と同じ確実性で実行する処理を実装する
- [X] T017 [US3] `firmware/main/failure_state.cc` と `firmware/main/update_job.cc` の順序を見直し、失敗記録後に ACT LED が活動中に見える状態を残さないよう統合する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 文書整合と総合確認

- [X] T018 [P] `docs/firmware-http-epaper.md` に ACT LED 点滅の運用説明を追記する
- [X] T019 `specs/006-activity-led-indicator/plan.md`、`spec.md`、`tasks.md` と `firmware/` 実装の整合性を確認する
- [ ] T020 `specs/006-activity-led-indicator/quickstart.md` に従って正常系 2 本、失敗系 4 本、待機状態 1 本の ACT LED 手動確認を実施する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Phase 6)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の LED 点滅開始/停止が動作した後に開始する
- **User Story 3 (P3)**: User Story 1 と 2 の正常系点滅経路が揃った後に開始する

### Within Each User Story

- 手動確認手順を先に明示してから実装する
- 共通 LED 制御は個別フロー統合より先に実装する
- 終了時停止の保証は成功経路と失敗経路の両方で確認する

### Parallel Opportunities

- `[P]` 付き Setup / Foundational / Polish タスクは並列実行可能
- Foundational 完了後、文書更新タスクは実装タスクと一部並列化できる

---

## Parallel Example: User Story 1

```bash
Task: "`specs/006-activity-led-indicator/quickstart.md` に起動時更新と BOOT ボタン更新での ACT LED 手動確認手順を具体化する"
Task: "`firmware/main/update_job.cc` に起動時更新の開始から正常完了まで ACT LED 点滅を維持する処理を実装する"
Task: "`firmware/main/update_job.cc` に BOOT ボタン更新でも同じ ACT LED 点滅ルールを適用する処理を実装する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. 起動時更新と BOOT ボタン更新の点滅を独立検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して正常系の点滅開始/停止を確認する
3. User Story 2 を追加して長時間待機中の継続点滅を確認する
4. User Story 3 を追加して失敗系停止を確認する
5. 最後に文書整合と総合手動確認を行う

### Parallel Team Strategy

1. 1 人が `firmware/` の LED 基盤統合を進める
2. 別の 1 人が `quickstart.md` と利用者向け文書の更新を進める
3. 統合後に実機確認を共同で実施する

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために必須
- 今回は手動実機確認中心であり、自動テストタスクは要求されていないため生成していない

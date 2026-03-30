# タスク: ディザリング向け画像改善アイデア整理

**Input**: `/specs/019-dither-image-ideas/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 実験タスクを実行するための文書・fixture・起動入口をそろえる

- [X] T001 `specs/019-dither-image-ideas/plan.md` と `specs/019-dither-image-ideas/research.md` を見直し、5 件以上の改善案 catalog を維持したまま最初に試す 2-4 件へ絞る判断基準を `specs/019-dither-image-ideas/research.md` に追記する
- [X] T002 [P] 代表入力画像候補の用途と不足している画像カテゴリを洗い出し、必要な追加 fixture 方針を `server/testdata/image-dither-rotate/README.md` に追記する
- [X] T003 [P] 比較記録の記入先を `specs/019-dither-image-ideas/quickstart.md` に明記し、都度レビュー前提の実験手順を追記する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story で共有する profile・比較モード・記録基盤を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [X] T004 `server/src/config.rs` に `IMAGE_PROFILE`、`COMPARE_WITH_BASELINE`、`COMPARE_SPLIT` を読み込む設定と検証を追加する
- [X] T005 [P] `server/src/image_pipeline/mod.rs` に named profile を適用する入口と split view 比較モードの共通要求を追加する
- [X] T006 [P] `server/src/app.rs` と `server/README.md` に実験モード起動時の表示内容と追加環境変数の説明を追加する
- [X] T007 `specs/019-dither-image-ideas/contracts/experiment-config.md` と `specs/019-dither-image-ideas/data-model.md` を実装前提に合わせて更新し、profile・image set・experiment run の対応関係を確定する
- [X] T008 `server/src/config.rs` と `specs/019-dither-image-ideas/plan.md` で Allowed Scope / Forbidden Scope を再確認し、HTTP ルート・転送フォーマット・firmware 不変を記録する

**Checkpoint**: profile 切替、split view 比較モード指定、比較記録の共通前提がそろっていること

---

## Phase 3: User Story 1 - 改善案を一覧化する (Priority: P1)

**Goal**: 改善仮説を重複なく整理し、比較対象 profile を独立に説明できる状態にする

**Independent Test**: `specs/019-dither-image-ideas/research.md` と `specs/019-dither-image-ideas/quickstart.md` を読むだけで、少なくとも 5 件以上の改善案 catalog と、最初に試す 2-4 件の profile 候補について狙い、リスク、比較観点を説明できること

### Verification for User Story 1

- [X] T009 [US1] 改善 profile の説明、狙い、懸念、試す順序を `specs/019-dither-image-ideas/research.md` に整理し、候補が重複していないことを手動確認する
- [X] T010 [US1] 比較観点と議論時の確認項目を `specs/019-dither-image-ideas/quickstart.md` に整理し、都度レビューで使える状態にする

### Implementation for User Story 1

- [X] T011 [P] [US1] `server/src/config.rs` に baseline を含む profile 定義と profile 選択ロジックを追加する
- [X] T012 [P] [US1] `server/src/image_pipeline/dither.rs` に profile ごとの差し替え点を集約できる前処理・候補色選択の構造を追加する
- [X] T013 [US1] `server/src/app.rs` に現在の profile 名と比較モードを起動メッセージとして表示する
- [X] T014 [US1] `server/src/config.rs` の profile 解決と既定値の挙動を確認するテストを追加する

**Checkpoint**: 改善 profile 候補が一意に識別でき、baseline を含む最初の比較対象が起動時に選べること

---

## Phase 4: User Story 2 - 実験して比較する (Priority: P2)

**Goal**: 手動差し替え入力画像と split view を使って、候補 profile を実機比較できる状態にする

**Independent Test**: 2 件以上の profile について、同じ入力画像を手動で差し替えながら split view もしくは全画面表示で比較し、実機 ePaper 表示の所見を `specs/019-dither-image-ideas/` 配下に記録できること

### Verification for User Story 2

- [X] T015 [US2] split view 比較と全画面再確認の手順を `specs/019-dither-image-ideas/quickstart.md` に具体化し、手動差し替え画像と追加画像の使い分けを手動確認する
- [X] T016 [P] [US2] `server/src/image_pipeline/mod.rs` または `server/src/routes.rs` に追加する比較モードの回帰確認テストを追加する

### Implementation for User Story 2

- [X] T017 [P] [US2] `server/src/image_pipeline/mod.rs` に baseline と比較対象 profile を左右または上下で合成する split view 生成を実装する
- [X] T018 [P] [US2] `server/src/image_pipeline/dither.rs` に最初の比較対象 profile 群を実装する
- [X] T019 [US2] `server/testdata/image-dither-rotate/README.md` と必要な fixture を更新し、代表入力画像候補と手動差し替え手順を整備する
- [X] T020 [US2] `server/src/config.rs` と `server/run.sh` で baseline との split view 比較モードを実行可能にする
- [X] T021 [US2] 実機 ePaper 表示で baseline と少なくとも 2 件の profile を比較し、結果を `specs/019-dither-image-ideas/research.md` または関連記録へ追記する

**Checkpoint**: 実機主判定で 2 件以上の profile 比較結果が残り、split view と全画面再確認の両方が使えること

---

## Phase 5: User Story 3 - 次に試す案を決める (Priority: P3)

**Goal**: 実験結果から上位候補、保留候補、除外候補を区別し、次アクションへ繋げる

**Independent Test**: `specs/019-dither-image-ideas/` の成果物を読むことで、上位候補、保留候補、除外候補の理由と次に進める案を説明できること

### Verification for User Story 3

- [X] T022 [US3] 比較結果を見ながら、advance / hold / reject の判定基準を `specs/019-dither-image-ideas/research.md` に明記する
- [X] T023 [US3] 上位候補の次アクションと追加検証条件を `specs/019-dither-image-ideas/quickstart.md` に反映し、独立に読めることを手動確認する

### Implementation for User Story 3

- [X] T024 [P] [US3] `specs/019-dither-image-ideas/research.md` に profile ごとの比較結果、判断理由、残課題を整理する
- [ ] T025 [P] [US3] `specs/019-dither-image-ideas/data-model.md` と `specs/019-dither-image-ideas/contracts/experiment-config.md` を結果に合わせて更新し、実験記録モデルと運用ルールを確定する
- [X] T026 [US3] 実装を継続する上位候補と後回し候補を `specs/019-dither-image-ideas/plan.md` に反映し、次の具体化方針を記載する

**Checkpoint**: 次に具体化する候補が 1 件以上決まり、保留・除外の理由が文書で追跡できること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 複数 story にまたがる仕上げと回帰確認

- [X] T027 [P] `server/src/config.rs`、`server/src/image_pipeline/mod.rs`、`server/src/image_pipeline/dither.rs` のコード整理とコメント整備を行う
- [X] T028 `server/README.md` と `specs/019-dither-image-ideas/quickstart.md` の手順を突き合わせ、記載どおりに比較実験を再実行して差分を修正する
- [X] T029 `server` で `cargo test` を実行し、比較機能追加後も既存ルートと画像パイプラインの回帰がないことを確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Phase 6)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の profile 定義が入った後に開始する
- **User Story 3 (P3)**: User Story 2 の比較結果が出た後に開始する

### Within Each User Story

- 検証タスクと手動確認手順を先に整える
- 設定とモデルを処理ロジックより先に実装する
- 実験結果のレビュー後に次の profile 追加順を見直してよい
- split view で差分を見つけた案は全画面表示でも再確認する

### Parallel Opportunities

- `[P]` 付き Setup / Foundational タスクは並列実行可能
- US1 では profile 定義とパイプライン構造整理を並列化できる
- US2 では split view 実装、profile 実装、fixture 整備を並列化できる
- US3 では結果整理とモデル文書更新を並列化できる

---

## Parallel Example: User Story 2

```bash
Task: "server/src/image_pipeline/mod.rs に baseline と比較対象 profile を左右または上下で合成する split view 生成を実装する"
Task: "server/src/image_pipeline/dither.rs に最初の比較対象 profile 群を実装する"
Task: "server/testdata/image-dither-rotate/README.md と必要な fixture を更新し、代表入力画像候補と手動差し替え手順を整備する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. baseline と最初の比較対象 profile が選べることを確認する
5. 次に試す profile をレビューで確定する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 で profile 候補と起動設定を整える
3. User Story 2 で split view と比較実験を入れて都度レビューする
4. User Story 3 で比較結果を整理し、次の具体化候補を確定する
5. Polish で回帰確認と手順整備を行う

### Parallel Team Strategy

1. 1 人が設定・起動系（`config.rs`, `run.sh`, `app.rs`）を担当する
2. 1 人が画像パイプライン（`mod.rs`, `dither.rs`）を担当する
3. 1 人が fixture と文書成果物（`research.md`, `quickstart.md`, `README.md`）を担当する
4. 実機比較と判断は合流して実施する

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために必須
- 今回は都度レビューを前提とするため、US2 実施中に profile の優先順位を更新してよい
- 実機主判定を維持し、PC 上の見え方だけで採否を決めない

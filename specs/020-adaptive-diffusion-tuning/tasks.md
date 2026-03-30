# タスク: 写真調ディザリングの追加改善

**Input**: `/specs/020-adaptive-diffusion-tuning/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: `019` の結論を今回の feature へ引き継ぎ、比較対象と評価観点を明文化する

- [x] T001 `specs/020-adaptive-diffusion-tuning/research.md` に今回実装する追加改善案、既存比較基準、見送る案を整理し、`019` との差分を明記する
- [x] T002 [P] `specs/020-adaptive-diffusion-tuning/quickstart.md` に比較対象画像、観察ポイント、記録項目を具体化する
- [x] T003 [P] `specs/020-adaptive-diffusion-tuning/data-model.md` と `specs/020-adaptive-diffusion-tuning/contracts/adaptive-profile-config.md` を見直し、新 profile と評価画像の最小要件を整合させる

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に共通する比較基準・fixture・回帰確認の土台を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [x] T004 `server/src/config.rs` と `server/README.md` で現行 profile を維持しつつ比較基準を明確化する
- [x] T005 [P] `server/src/image_pipeline/dither.rs`、`server/src/image_pipeline/mod.rs`、`server/src/routes.rs` から試作コードを外し、現行 runtime に戻す
- [x] T006 [P] `server/testdata/dither-result-check/` に青系の広い面を含む代表画像を追加し、用途を `server/testdata/image-dither-rotate/README.md` に追記する
- [x] T007 `server/src/config.rs` と `server/src/image_pipeline/mod.rs` で新 profile が既存比較モードに接続されることを確認するテストを追加する
- [x] T008 `specs/020-adaptive-diffusion-tuning/plan.md` で Allowed Scope / Forbidden Scope を再確認し、HTTP ルート・転送フォーマット・firmware 不変を維持していることを記録する

**Checkpoint**: 比較基準、比較画像、基本回帰確認が揃っていること

---

## Phase 3: User Story 1 - 次の改善案を選べる (Priority: P1)

**Goal**: 今回実装する追加改善案の狙い、比較基準、後回し案を独立に説明できる状態にする

**Independent Test**: `specs/020-adaptive-diffusion-tuning/research.md` と `specs/020-adaptive-diffusion-tuning/quickstart.md` を読むだけで、追加改善案の狙い、比較対象、観察ポイント、後回し理由を説明できること

### Verification for User Story 1

- [x] T009 [US1] `specs/020-adaptive-diffusion-tuning/research.md` に追加改善案の判定軸と見送り案の理由を追記し、重複や曖昧さがないことを手動確認する
- [x] T010 [P] [US1] `specs/020-adaptive-diffusion-tuning/quickstart.md` に画像ごとの観察ポイントを追記し、青保持・低彩度面・肌の観点が揃っていることを手動確認する

### Implementation for User Story 1

- [x] T011 [P] [US1] `specs/020-adaptive-diffusion-tuning/data-model.md` に新 profile、評価画像、比較結果の関係を反映する
- [x] T012 [P] [US1] `specs/020-adaptive-diffusion-tuning/contracts/adaptive-profile-config.md` に新 profile と評価画像の契約を反映する
- [x] T013 [US1] `specs/020-adaptive-diffusion-tuning/plan.md` に今回の具体化対象と比較基準を最終反映する

**Checkpoint**: 追加改善案の意図と範囲が文書だけで説明できること

---

## Phase 4: User Story 2 - 追加改善を試せる (Priority: P2)

**Goal**: 写真調向け追加改善案を試作・評価し、現行上位候補との違いを判断できる状態にする

**Independent Test**: 既存上位候補を基準に手動比較を行い、試作案の有意差または差が見えないことを説明できること

### Verification for User Story 2

- [x] T014 [P] [US2] `specs/020-adaptive-diffusion-tuning/research.md` に試作アルゴリズムの再現手順を記録する
- [x] T015 [P] [US2] `specs/020-adaptive-diffusion-tuning/research.md` と `specs/020-adaptive-diffusion-tuning/quickstart.md` に比較観察の書式を追加する
- [x] T016 [US2] `specs/020-adaptive-diffusion-tuning/quickstart.md` に現行比較基準の起動手順を更新する

### Implementation for User Story 2

- [x] T017 [US2] `server/src/image_pipeline/dither.rs` で青系領域補正と局所誤差拡散制御の試作を行い、効果確認後に取り外す
- [x] T018 [US2] `server/src/config.rs` と `server/README.md` を現行 runtime に合わせて戻す
- [x] T019 [US2] `server/testdata/dither-result-check/` の代表画像を使って試作案を手動比較し、結果を `specs/020-adaptive-diffusion-tuning/research.md` に追記する

**Checkpoint**: 試作案の差分有無と撤回判断を説明できること

---

## Phase 5: User Story 3 - 採用判断へ進める (Priority: P3)

**Goal**: 比較結果から試作案を runtime 採用しない判断と次アクションを確定できる状態にする

**Independent Test**: `specs/020-adaptive-diffusion-tuning/` の成果物を読むことで、試作案を `reject` とし、判明した新問題点を説明できること

### Verification for User Story 3

- [x] T022 [US3] `specs/020-adaptive-diffusion-tuning/research.md` に `advance` / `hold` / `reject` の判定基準と今回の結論を記載する
- [x] T023 [P] [US3] `specs/020-adaptive-diffusion-tuning/quickstart.md` に比較結果の記録先と次アクション確認項目を追記する

### Implementation for User Story 3

- [x] T024 [P] [US3] `specs/020-adaptive-diffusion-tuning/research.md` に画像別の比較結果、良化点、悪化点、残課題を整理する
- [x] T025 [P] [US3] `specs/020-adaptive-diffusion-tuning/data-model.md` と `specs/020-adaptive-diffusion-tuning/contracts/adaptive-profile-config.md` を結果に合わせて更新する
- [x] T026 [US3] `specs/020-adaptive-diffusion-tuning/plan.md` に次の採用判断または follow-up 候補を反映する

**Checkpoint**: 新 profile の採否と次アクションが文書で追跡できること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 横断的な仕上げと回帰確認

- [x] T027 [P] `server/src/config.rs`、`server/src/image_pipeline/dither.rs`、`server/src/image_pipeline/mod.rs` のコード整理とコメント整備を行う
- [x] T028 `server/README.md` と `specs/020-adaptive-diffusion-tuning/quickstart.md` の手順を突き合わせ、記載どおりに比較を再実行して差分を修正する
- [x] T029 `server` で `cargo test` を実行し、既存機能と新 profile の回帰がないことを確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Phase 6)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の比較基準整理後に開始する
- **User Story 3 (P3)**: User Story 2 の比較結果が出た後に開始する

### Within Each User Story

- 検証タスクと手動確認手順を先に整える
- 設定と比較契約をロジック実装より先に固める
- 新 profile のロジック実装後に手動比較結果を記録する
- 結果整理後に次アクションと follow-up を確定する

### Parallel Opportunities

- `[P]` 付き Setup / Foundational タスクは並列実行可能
- US1 では data model と contract 更新を並列化できる
- US2 ではテスト追加、README 更新、fixture 整備の一部を並列化できる
- US3 では結果整理と成果物更新を並列化できる

---

## Parallel Example: User Story 2

```bash
Task: "server/src/image_pipeline/dither.rs に色域別の期待差分を確認するユニットテストを追加する"
Task: "server/src/image_pipeline/mod.rs または server/src/routes.rs に新 profile 選択時の応答回帰テストを追加する"
Task: "specs/020-adaptive-diffusion-tuning/quickstart.md にローカル確認手順と比較起動コマンドを更新する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. 追加改善案の狙いと比較基準を独立検証する
5. 次に実装する新 profile の境界を確定する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 で改善案と観察軸を確定する
3. User Story 2 で新 profile 実装と比較結果を追加する
4. User Story 3 で採否と次アクションを整理する
5. Polish で回帰確認と手順整備を行う

### Parallel Team Strategy

1. 1 人が設定・起動系（`config.rs`, `app.rs`, `README.md`）を担当する
2. 1 人が画像パイプライン（`dither.rs`, `mod.rs`）を担当する
3. 1 人が fixture と文書成果物（`research.md`, `quickstart.md`, `contracts/`）を担当する
4. 比較結果と採否判断は合流して実施する

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために必須
- 実機主判定は維持するが、今回の feature では runtime へ残さない判断と文書記録も完了条件に含める

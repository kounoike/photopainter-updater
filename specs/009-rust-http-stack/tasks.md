# タスク: Rust HTTPスタック再評価

**Input**: `/workspaces/photopainter-updater/specs/009-rust-http-stack/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3`)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 調査 feature の前提と比較対象を固定する

- [X] T001 `specs/009-rust-http-stack/spec.md` と `specs/009-rust-http-stack/plan.md` を照合し、比較対象とスコープ境界を確認する
- [X] T002 `specs/009-rust-http-stack/research.md` の冒頭に比較条件、評価軸、008 継承前提を整理する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story で共有する比較枠組みを固める

**CRITICAL**: この phase 完了まで user story 作業を開始しない

- [X] T003 `specs/009-rust-http-stack/data-model.md` に Rust HTTP Candidate、Evaluation Axis、Rust HTTP Selection Result の対応関係を整備する
- [X] T004 [P] `specs/009-rust-http-stack/contracts/rust-http-selection-contract.md` に最終候補、対抗候補、参考候補、再評価条件の必須項目を固定する
- [X] T005 [P] `specs/009-rust-http-stack/quickstart.md` に候補比較、画像前処理、telemetry、最終判断の確認手順を整理する
- [X] T006 `specs/009-rust-http-stack/plan.md` と `specs/009-rust-http-stack/tasks.md` の scope 記載が整合していることを確認する

**Checkpoint**: 比較枠組み完了後に user story 作業へ進む

---

## Phase 3: User Story 1 - Rust 内の候補比較をやり直したい (Priority: P1)

**Goal**: `axum` を含む Rust HTTP framework 候補を同一観点で比較し、後続 feature が再比較なしで参照できる状態にする

**Independent Test**: `research.md` に `axum`、`actix-web`、`warp` の比較と採否が同一構造で記録され、`quickstart.md` のレビュー手順で確認できること

### Verification for User Story 1

- [X] T007 [US1] `specs/009-rust-http-stack/quickstart.md` に基づき、候補比較と採否を文書レビューで確認する手順を具体化する

### Implementation for User Story 1

- [X] T008 [US1] `specs/009-rust-http-stack/research.md` に `axum` の比較結果と採用根拠を記録する
- [X] T009 [US1] `specs/009-rust-http-stack/research.md` に `actix-web` の比較結果と見送り理由を記録する
- [X] T010 [US1] `specs/009-rust-http-stack/research.md` に `warp` の比較結果と参考候補としての位置づけを記録する
- [X] T011 [US1] `specs/009-rust-http-stack/contracts/rust-http-selection-contract.md` に最終候補、第一対抗候補、参考候補を反映する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - 画像前処理と telemetry に対する適合性を見たい (Priority: P2)

**Goal**: 各 Rust 候補が画像前処理 workload と telemetry API の両方にどう適合するかを説明できる状態にする

**Independent Test**: `research.md` に `ref/convert.py` を根拠にした画像前処理評価と、telemetry POST/監視連携の評価が各候補ごとに含まれていることを確認する

### Verification for User Story 2

- [X] T012 [US2] `specs/009-rust-http-stack/quickstart.md` に基づき、画像前処理観点と telemetry 観点を文書レビューで確認する

### Implementation for User Story 2

- [X] T013 [US2] `specs/009-rust-http-stack/research.md` に `ref/convert.py` を踏まえた回転、スケーリング、ディザリング、6 色インデックス化の比較観点を明記する
- [X] T014 [US2] `specs/009-rust-http-stack/research.md` に telemetry POST、状態注入、監視連携の比較結果を候補ごとに追記する
- [X] T015 [US2] `specs/009-rust-http-stack/data-model.md` に image processing fit と telemetry fit の評価項目を最終比較軸として整合させる

**Checkpoint**: User Story 2 が独立して検証可能であること

---

## Phase 5: User Story 3 - `axum` を維持する場合も理由を残したい (Priority: P3)

**Goal**: `axum` 維持の理由、他候補の見送り理由、再評価条件、008 との差分有無を後続 feature が参照できる形で固定する

**Independent Test**: `research.md` と contract に `axum` 維持理由、`actix-web` / `warp` の見送り理由、再評価条件、008 との整合が揃っていることを確認する

### Verification for User Story 3

- [X] T016 [US3] `specs/009-rust-http-stack/quickstart.md` に基づき、最終候補、見送り理由、再評価条件、008 との整合を文書レビューで確認する

### Implementation for User Story 3

- [X] T017 [US3] `specs/009-rust-http-stack/research.md` に `axum` 維持理由と 008 の結論継続理由を明記する
- [X] T018 [US3] `specs/009-rust-http-stack/research.md` に `actix-web` と `warp` の見送り理由、および再評価条件を明記する
- [X] T019 [US3] `specs/009-rust-http-stack/contracts/rust-http-selection-contract.md` に adoption reason、rejection reasons、re_evaluation_triggers を最終形で反映する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 横断整合と完了確認

- [X] T020 `specs/009-rust-http-stack/plan.md`、`specs/009-rust-http-stack/research.md`、`specs/009-rust-http-stack/data-model.md`、`specs/009-rust-http-stack/contracts/rust-http-selection-contract.md` の用語と順位表現を統一する
- [X] T021 `specs/009-rust-http-stack/quickstart.md` に従って文書レビューを実施し、後続 feature が再比較なしで参照できることを確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Phase 6)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: Foundational 後に開始可能。比較対象は US1 と同じ候補を使う
- **User Story 3 (P3)**: US1 の候補比較結果を前提に開始する

### Within Each User Story

- 手動レビュー手順を先に固める
- `research.md` を主更新対象として比較結果を記録する
- `data-model.md` と contract は `research.md` の確定内容に合わせて更新する
- 検証タスクを省略しない

### Parallel Opportunities

- `[P]` 付き Foundational タスクは並列実行可能
- 調査観点の洗い出し自体は並列化できるが、同一ファイルへの反映は逐次統合する

---

## Parallel Example: User Story 1

```bash
Task: "`specs/009-rust-http-stack/research.md` に `axum` の比較結果と採用根拠を記録する"
Task: "`specs/009-rust-http-stack/research.md` に `actix-web` の比較結果と見送り理由を記録する"
Task: "`specs/009-rust-http-stack/research.md` に `warp` の比較結果と参考候補としての位置づけを記録する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. 候補比較が独立レビューできることを確認する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して Rust 内候補比較を固定する
3. User Story 2 を追加して画像前処理と telemetry 観点を補強する
4. User Story 3 を追加して最終選定理由と再評価条件を固定する
5. Polish で文書横断整合を確認する

### Parallel Team Strategy

1. 1 人が Setup + Foundational を進める
2. Foundational 完了後、候補ごとの比較記録を分担する
3. 画像前処理観点と telemetry 観点の整理を分担する
4. 最後に 1 人が contract と quickstart の整合を締める

---

## Notes

- `[P]` は別ファイルで編集衝突しにくい独立タスクを示す
- `[US1]`、`[US2]`、`[US3]` は traceability のために必須
- 今回は文書調査 feature なので、新規サーバ実装や PoC 作成は含めない

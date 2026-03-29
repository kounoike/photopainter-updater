# タスク: HTTPサーバ技術選定調査

**Input**: `/specs/008-http-server-stack/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 技術選定調査の前提資料と比較枠を整える

- [X] T001 `specs/008-http-server-stack/spec.md` と `specs/008-http-server-stack/plan.md` の比較対象、評価観点、スコープ境界を確認し、固定事項を `specs/008-http-server-stack/research.md` の比較条件へ反映する
- [X] T002 [P] `ref/convert.py` を確認し、画像前処理要件として扱う回転、スケーリング、ディザリング、6 色インデックス化の処理要素を `specs/008-http-server-stack/research.md` へ反映する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story で共通に使う比較基盤を固定する

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [X] T003 `specs/008-http-server-stack/data-model.md` に基づき、技術候補、評価観点、画像前処理要件、デバイス telemetry 要件の対応関係を `specs/008-http-server-stack/research.md` に明記する
- [X] T004 [P] `specs/008-http-server-stack/contracts/stack-selection-contract.md` に Required Comparison Axes と Revisit Conditions が `specs/008-http-server-stack/spec.md` の FR-003、FR-007、FR-008、FR-009 と一致していることを反映する
- [X] T005 `specs/008-http-server-stack/quickstart.md` に、候補比較、画像前処理、telemetry、再評価条件を文書レビューで確認できる検証手順を整理する

**Checkpoint**: 基盤完了後に user story 実行へ進む

---

## Phase 3: User Story 1 - 実装前に有力候補を比較したい (Priority: P1)

**Goal**: Rust+axum、Python+FastAPI、その他候補を同一観点で比較し、後続 feature が参照できる第一候補を固定する

**Independent Test**: `specs/008-http-server-stack/research.md` と `specs/008-http-server-stack/contracts/stack-selection-contract.md` を見て、3 候補以上の比較と暫定第一候補を説明できることを確認する

### Verification for User Story 1

- [X] T006 [US1] `specs/008-http-server-stack/quickstart.md` に 3 候補比較と第一候補確認のレビュー手順を明文化する

### Implementation for User Story 1

- [X] T007 [US1] `specs/008-http-server-stack/research.md` に Rust+axum、Python+FastAPI、Go+net/http の比較結果を同一構造で整理し、第三候補を残さない場合はその理由も記録する
- [X] T008 [US1] `specs/008-http-server-stack/contracts/stack-selection-contract.md` に暫定第一候補、第一対抗候補、参考候補を確定記載する
- [X] T009 [US1] `specs/008-http-server-stack/plan.md` に後続 feature が参照する前提技術スタックと比較理由を整合反映する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - ローカル運用に合う選択をしたい (Priority: P2)

**Goal**: ローカル運用、保守性、配布容易性の観点を比較結果へ反映する

**Independent Test**: 比較結果にローカル運用、依存の重さ、保守性、配布容易性の評価が含まれ、採用理由に反映されていることを確認する

### Verification for User Story 2

- [X] T010 [US2] `specs/008-http-server-stack/quickstart.md` にローカル運用観点の確認手順を追加する

### Implementation for User Story 2

- [X] T011 [US2] `specs/008-http-server-stack/research.md` にローカル運用、依存の重さ、開発体験、Docker Compose や軽量コンテナ配布を見据えた比較理由を整理する
- [X] T012 [US2] `specs/008-http-server-stack/data-model.md` の評価観点と `specs/008-http-server-stack/contracts/stack-selection-contract.md` の Required Comparison Axes を一致させる

**Checkpoint**: User Story 1 と 2 が独立して検証可能であること

---

## Phase 5: User Story 3 - 画像処理と telemetry まで含めて比較したい (Priority: P3)

**Goal**: `ref/convert.py` による画像前処理要件とデバイス telemetry 要件を、各候補で実現可能か比較する

**Independent Test**: `specs/008-http-server-stack/research.md` に各候補の画像前処理適合性と telemetry API 適合性が明示されていることを確認する

### Verification for User Story 3

- [X] T013 [US3] `specs/008-http-server-stack/quickstart.md` に画像前処理要件と telemetry 要件のレビュー手順を追加する

### Implementation for User Story 3

- [X] T014 [US3] `specs/008-http-server-stack/research.md` に `ref/convert.py` を参照した回転、スケーリング、ディザリング、6 色インデックス化の実現性比較を追記する
- [X] T015 [US3] `specs/008-http-server-stack/research.md` にバッテリー残量 POST、Grafana 監視、閾値通知の実現性比較を追記する
- [X] T016 [US3] `specs/008-http-server-stack/data-model.md` と `specs/008-http-server-stack/contracts/stack-selection-contract.md` に画像前処理要件と telemetry 要件の評価結果を整合反映する

**Checkpoint**: User Story 1 から 3 が独立して検証可能であること

---

## Phase 6: User Story 4 - 見送る候補の理由も残したい (Priority: P4)

**Goal**: 見送る候補と再評価条件を明確に残し、後続 feature の判断ぶれを防ぐ

**Independent Test**: 暫定第一候補以外の候補について見送り理由または位置づけが明記され、再比較条件も説明できることを確認する

### Verification for User Story 4

- [X] T017 [US4] `specs/008-http-server-stack/quickstart.md` に見送り候補と再評価条件の確認手順を追加する

### Implementation for User Story 4

- [X] T018 [US4] `specs/008-http-server-stack/research.md` に各候補の見送り理由または採用見送り条件を簡潔に整理する
- [X] T019 [US4] `specs/008-http-server-stack/contracts/stack-selection-contract.md` に Revisit Conditions を後続 feature で使える形へ整える
- [X] T020 [US4] `specs/008-http-server-stack/plan.md` に後続実装 feature の開始前提と比較やり直し条件を反映する

**Checkpoint**: すべての user story が独立して検証可能であること

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: 文書整合と総合確認

- [X] T021 [P] `specs/008-http-server-stack/spec.md`、`plan.md`、`research.md`、`data-model.md`、`contracts/stack-selection-contract.md` の用語と採用判断を相互整合確認する
- [X] T022 `specs/008-http-server-stack/quickstart.md` に従って文書レビューを実施し、暫定第一候補、対抗候補、画像前処理要件、telemetry 要件、再評価条件、およびローカル優先・最小構成との整合を確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Phase 7)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の候補比較結果が揃った後に開始する
- **User Story 3 (P3)**: User Story 1 と 2 の比較軸が固定された後に開始する
- **User Story 4 (P4)**: User Story 1 から 3 の比較結果が揃った後に開始する

### Within Each User Story

- 手動確認手順を先に明示してから比較結果を固める
- `research.md` の主判断を先に更新し、その後 `plan.md`、`data-model.md`、`contracts/` へ反映する
- 見送り理由と再評価条件は最後に全候補を見渡して確定する

### Parallel Opportunities

- `[P]` 付き Setup / Foundational / Polish タスクは並列実行可能
- User Story 1 完了後、User Story 2 の運用観点整理と User Story 3 の画像・telemetry 観点整理は一部並列化できる

---

## Parallel Example: User Story 3

```bash
Task: "`specs/008-http-server-stack/quickstart.md` に画像前処理要件と telemetry 要件のレビュー手順を追加する"
Task: "`specs/008-http-server-stack/research.md` に `ref/convert.py` を参照した回転、スケーリング、ディザリング、6 色インデックス化の実現性比較を追記する"
Task: "`specs/008-http-server-stack/research.md` にバッテリー残量 POST、Grafana 監視、閾値通知の実現性比較を追記する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. 暫定第一候補と比較対象の固定を独立検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 で比較対象と第一候補を固定する
3. User Story 2 でローカル運用・配布観点を確定する
4. User Story 3 で画像前処理と telemetry 観点を確定する
5. User Story 4 で見送り理由と再評価条件を固定する
6. 最後に文書整合レビューを行う

### Parallel Team Strategy

1. 1 人が `research.md` の候補比較本体を整理する
2. 別の 1 人が `quickstart.md`、`contracts/stack-selection-contract.md`、`data-model.md` の整合整理を進める
3. 最後に `plan.md` へ採用判断を集約して確定する

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために必須
- 今回は技術選定調査 feature のため、自動テストタスクは生成していない

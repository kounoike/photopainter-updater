# タスク: Release Drafter 導入

**Input**: `/specs/033-release-drafter/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: release-drafter 導入の対象ファイルと更新契機を固定する

- [X] T001 `specs/033-release-drafter/spec.md`、`specs/033-release-drafter/plan.md`、`specs/033-release-drafter/contracts/release-drafter-contract.md` を照合し、更新契機を `main` への `push` のみに固定する前提を確定する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に先行する repository 構成と運用前提を揃える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [X] T002 `.github/` 配下の既存構成を確認し、新規追加対象を `.github/workflows/release-drafter.yml` と `.github/release-drafter.yml` に限定する
- [X] T003 `README.md` と `specs/033-release-drafter/quickstart.md` の運用導線を確認し、release draft 設定場所と確認手順の追記範囲を整理する

**Checkpoint**: 対象ファイル、更新契機、文書更新範囲が確定していること

---

## Phase 3: User Story 1 - 次回リリースの下書きを自動更新したい (Priority: P1)

**Goal**: `main` への `push` 後に次回リリース向け draft を自動生成または更新できるようにする

**Independent Test**: 設定ファイルを確認し、`main` への `push` のみを契機に draft を作成または更新する契約になっていることを確認する

### Verification for User Story 1

- [X] T004 [US1] `main` への `push` のみを更新契機とする確認手順を `specs/033-release-drafter/quickstart.md` に記載する
- [X] T005 [US1] `main` へ反映後に GitHub の Releases 画面で draft が作成または更新される確認手順を `specs/033-release-drafter/quickstart.md` に記載する

### Implementation for User Story 1

- [X] T006 [US1] release draft 更新 workflow を `.github/workflows/release-drafter.yml` に実装する
- [X] T007 [US1] draft 名称、初回作成、既存 draft 更新、既定カテゴリの基本契約を `.github/release-drafter.yml` に実装する
- [X] T008 [US1] `main` 以外や merge 前 PR 更新で draft を更新しないことを `.github/workflows/release-drafter.yml` と `specs/033-release-drafter/quickstart.md` で担保する

**Checkpoint**: `main` への `push` 後だけ draft が更新される契約が独立して確認できること

---

## Phase 4: User Story 2 - 変更種別ごとに整理された下書きを見たい (Priority: P2)

**Goal**: pull request labels に応じて分類済みの release draft を確認できるようにする

**Independent Test**: 設定ファイルを参照し、label 付き pull request がカテゴリへ入り、未分類変更も既定カテゴリへ掲載されることを確認する

### Verification for User Story 2

- [X] T009 [US2] 分類ルールと未分類変更の既定扱い確認手順を `specs/033-release-drafter/quickstart.md` に記載する
- [X] T010 [US2] GitHub の Releases 画面で分類済み一覧と既定カテゴリ表示を確認する手順を `specs/033-release-drafter/quickstart.md` に記載する

### Implementation for User Story 2

- [X] T011 [US2] pull request labels に基づくカテゴリ定義を `.github/release-drafter.yml` に実装する
- [X] T012 [US2] 分類対象外の変更を欠落させない既定カテゴリを `.github/release-drafter.yml` に実装する

**Checkpoint**: 分類済み表示と既定カテゴリへのフォールバックが独立して確認できること

---

## Phase 5: User Story 3 - 導入後の運用方法を把握したい (Priority: P3)

**Goal**: 管理者が repository 内文書から release drafter の設定場所と確認方法を追えるようにする

**Independent Test**: README と quickstart を読むだけで、設定場所、更新契機、確認方法、対象外イベントを理解できることを確認する

### Verification for User Story 3

- [X] T013 [US3] 設定場所、更新契機、Releases 画面での確認手順を `specs/033-release-drafter/quickstart.md` に整理する

### Implementation for User Story 3

- [X] T014 [US3] release drafter の設定場所と運用概要を `README.md` に追記する
- [X] T015 [US3] troubleshooting を含む確認導線を `specs/033-release-drafter/quickstart.md` に実装する

**Checkpoint**: 運用者が repository 内文書だけで導入後の確認導線を辿れること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 複数 story にまたがる最終整合

- [X] T016 [P] `specs/033-release-drafter/contracts/release-drafter-contract.md`、`specs/033-release-drafter/plan.md`、`specs/033-release-drafter/tasks.md` の記述整合を確認する
- [X] T017 [P] `.github/workflows/release-drafter.yml`、`.github/release-drafter.yml`、`README.md`、`specs/033-release-drafter/quickstart.md` の導線が一致していることを確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能。MVP
- **User Story 2 (P2)**: User Story 1 の draft 更新導線が入った後に開始する
- **User Story 3 (P3)**: User Story 1 と 2 の設定場所が固まった後に開始する

### Within Each User Story

- quickstart の確認観点を先に書き、その後に設定ファイルや README を更新する
- workflow と release drafter 設定は相互参照するため同一 story 内で順番に進める
- 文書 story は quickstart の整理後に README を更新する

### Parallel Opportunities

- Phase 2 の T002 と T003 は別領域の確認なので並列実行可能
- Final Phase の T014 と T015 は別ファイル中心の整合確認なので並列実行可能

---

## Parallel Example: User Story 2

```bash
Task: "T008 分類ルールと未分類変更の既定扱い確認手順を specs/033-release-drafter/quickstart.md に記載する"
Task: "T009 pull request labels に基づくカテゴリ定義を .github/release-drafter.yml に実装する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 と Phase 2 を完了する
2. `main` への `push` だけで draft 更新する workflow を実装する
3. quickstart で確認手順を固める
4. release draft 自動更新の最小価値を先に得る

### Incremental Delivery

1. draft 更新 workflow と基本設定を入れる
2. 分類ルールと既定カテゴリを追加する
3. README と quickstart に運用導線を追記する
4. 契約と文書の整合を最終確認する

### Parallel Team Strategy

1. 1 人が `.github/workflows/release-drafter.yml` を担当する
2. 別担当が `.github/release-drafter.yml` と quickstart の分類確認を担当する
3. 最後に README と全体整合をまとめて確認する

---

## Notes

- すべての task は Allowed Scope 内の `.github/`、`README.md`、feature artifact に限定している
- release publish 自動化や versioning policy 変更は task に含めない
- 更新契機は `main` への `push` のみで、PR 作成時点の更新は対象外とする

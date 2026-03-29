---

description: "機能実装のためのタスクリストテンプレート"
---

# タスク: [FEATURE NAME]

**Input**: `/specs/[###-feature-name]/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Path Conventions

- **Single project**: `src/`, `tests/`
- **Web app**: `backend/src/`, `frontend/src/`
- **Mobile**: `api/src/`, `ios/src/` または `android/src/`
- 実パスは `plan.md` の構成決定に合わせて置き換える

<!--
  IMPORTANT:
  以下はサンプルであり、/speckit.tasks は実際の設計成果物に基づいて完全に
  置き換えなければならない。
-->

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: プロジェクト初期化と基本構成

- [ ] T001 実装計画に沿ってプロジェクト構成を作成する
- [ ] T002 [language] と [framework] の依存関係を初期化する
- [ ] T003 [P] lint / format / static check の設定を追加する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に先行する基盤整備

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T004 共有設定または永続化基盤を整備する
- [ ] T005 [P] 通信、認証、またはアクセス制御の共通基盤を実装する
- [ ] T006 [P] エラーハンドリングとログ基盤を整備する
- [ ] T007 全 story が依存する共通モデルまたはプロトコルを定義する
- [ ] T008 Allowed Scope / Forbidden Scope の実装境界を確認し記録する

**Checkpoint**: 基盤完了後に user story 実装へ進む

---

## Phase 3: User Story 1 - [Title] (Priority: P1)

**Goal**: [この story が提供する価値]

**Independent Test**: [単独検証方法]

### Verification for User Story 1

> **NOTE**: 自動テストがある場合は実装前に追加し、失敗を確認してから進める

- [ ] T009 [P] [US1] 契約検証を `tests/contract/test_[name].py` に追加する
- [ ] T010 [P] [US1] 統合検証を `tests/integration/test_[name].py` に追加する
- [ ] T011 [US1] 手動確認手順を `specs/[###-feature-name]/quickstart.md` または関連文書へ記載する

### Implementation for User Story 1

- [ ] T012 [P] [US1] [Entity1] を `src/models/[entity1].py` に実装する
- [ ] T013 [P] [US1] [Entity2] を `src/models/[entity2].py` に実装する
- [ ] T014 [US1] [Service] を `src/services/[service].py` に実装する
- [ ] T015 [US1] [endpoint/feature] を `src/[location]/[file].py` に実装する
- [ ] T016 [US1] 入力検証とエラー時挙動を追加する
- [ ] T017 [US1] 運用ログまたは観測情報を追加する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - [Title] (Priority: P2)

**Goal**: [この story が提供する価値]

**Independent Test**: [単独検証方法]

### Verification for User Story 2

- [ ] T018 [P] [US2] 契約検証を `tests/contract/test_[name].py` に追加する
- [ ] T019 [P] [US2] 統合検証を `tests/integration/test_[name].py` に追加する
- [ ] T020 [US2] 手動確認手順を関連文書へ記載する

### Implementation for User Story 2

- [ ] T021 [P] [US2] [Entity] を `src/models/[entity].py` に実装する
- [ ] T022 [US2] [Service] を `src/services/[service].py` に実装する
- [ ] T023 [US2] [endpoint/feature] を `src/[location]/[file].py` に実装する
- [ ] T024 [US2] 必要に応じて User Story 1 と統合する

**Checkpoint**: User Story 1 と 2 が独立検証可能であること

---

## Phase 5: User Story 3 - [Title] (Priority: P3)

**Goal**: [この story が提供する価値]

**Independent Test**: [単独検証方法]

### Verification for User Story 3

- [ ] T025 [P] [US3] 契約検証を `tests/contract/test_[name].py` に追加する
- [ ] T026 [P] [US3] 統合検証を `tests/integration/test_[name].py` に追加する
- [ ] T027 [US3] 手動確認手順を関連文書へ記載する

### Implementation for User Story 3

- [ ] T028 [P] [US3] [Entity] を `src/models/[entity].py` に実装する
- [ ] T029 [US3] [Service] を `src/services/[service].py` に実装する
- [ ] T030 [US3] [endpoint/feature] を `src/[location]/[file].py` に実装する

**Checkpoint**: すべての user story が独立検証可能であること

---

[必要に応じて同様の phase を追加する]

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: 複数 story にまたがる改善

- [ ] TXXX [P] 関連文書を更新する
- [ ] TXXX コード整理またはリファクタリングを行う
- [ ] TXXX 横断的な性能改善を行う
- [ ] TXXX [P] 追加ユニットテストを `tests/unit/` に追加する
- [ ] TXXX セキュリティまたは安全性の確認を行う
- [ ] TXXX `quickstart.md` の手順を検証する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: Foundational 後に開始可能
- **User Story 3 (P3)**: Foundational 後に開始可能

### Within Each User Story

- 自動テストがある場合は実装前に追加して失敗を確認する
- モデルをサービスより先に実装する
- サービスをエンドポイントまたは UI 統合より先に実装する
- 検証タスクを省略しない

### Parallel Opportunities

- `[P]` 付き Setup / Foundational タスクは並列実行可能
- Foundational 完了後は story ごとに並列進行可能
- 同一 story 内でも別ファイルのモデルと検証は並列化可能

---

## Parallel Example: User Story 1

```bash
Task: "契約検証を tests/contract/test_[name].py に追加する"
Task: "統合検証を tests/integration/test_[name].py に追加する"
Task: "[Entity1] を src/models/[entity1].py に実装する"
Task: "[Entity2] を src/models/[entity2].py に実装する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. User Story 1 を独立検証する
5. 必要なら内部デモまたは確認を行う

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して独立検証する
3. User Story 2 を追加して独立検証する
4. User Story 3 を追加して独立検証する
5. 前段 story を壊していないことを確認する

### Parallel Team Strategy

1. チームで Setup + Foundational を完了する
2. Foundational 完了後に story ごとに担当を分ける
3. 各担当は story 単位で独立検証まで完了する

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために必須
- 曖昧なタスク、同一ファイル衝突、独立性を壊す cross-story 依存を避ける

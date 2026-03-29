# タスク: xiaozhi-esp32 構造解析ドキュメント

**Input**: `/specs/004-document-xiaozhi-arch/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Path Conventions

- 技術文書: `docs/`
- feature 設計成果物: `specs/004-document-xiaozhi-arch/`
- 解析対象コード: `xiaozhi-esp32/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 文書作成の前提を揃え、調査対象と成果物配置を固定する

- [X] T001 `docs/` の配置方針と成果物名を確認し、作業対象を `docs/xiaozhi-esp32-architecture.md` に固定する
- [X] T002 `xiaozhi-esp32/main/main.cc` と `xiaozhi-esp32/main/application.cc` を読み、起動系の調査メモを `specs/004-document-xiaozhi-arch/quickstart.md` に反映する
- [X] T003 [P] `xiaozhi-esp32/main/protocols/`、`xiaozhi-esp32/main/audio/README.md`、`xiaozhi-esp32/main/settings.*`、`xiaozhi-esp32/main/ota.*` の調査対象一覧を `specs/004-document-xiaozhi-arch/research.md` と照合する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に共通する文書構成と検証基準を固める

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [X] T004 `specs/004-document-xiaozhi-arch/contracts/documentation-contract.md` を基に、`docs/xiaozhi-esp32-architecture.md` の必須見出し構成を定義する
- [X] T005 [P] `specs/004-document-xiaozhi-arch/data-model.md` を基に、関心領域と主要フローの対応表を `specs/004-document-xiaozhi-arch/quickstart.md` に追記する
- [X] T006 [P] `specs/004-document-xiaozhi-arch/quickstart.md` に手動検証観点を補強し、起動・通信・音声・設定・OTA・ボード差分の確認項目を明示する
- [X] T007 Allowed Scope / Forbidden Scope を確認し、コード変更禁止と網羅的ボード一覧化禁止を `docs/xiaozhi-esp32-architecture.md` の執筆方針として守る

**Checkpoint**: 基盤完了後に user story 実装へ進む

---

## Phase 3: User Story 1 - 全体構造を短時間で把握できる (Priority: P1)

**Goal**: 初見の開発者が `xiaozhi-esp32` の全体構造と代表的主要フローを短時間で把握できる文書を作る

**Independent Test**: `docs/xiaozhi-esp32-architecture.md` だけを読んで、主要ディレクトリの役割と起動・通信・音声の代表フローを説明できることを確認する

### Verification for User Story 1

- [X] T008 [US1] User Story 1 の手動確認手順を `specs/004-document-xiaozhi-arch/quickstart.md` に具体化する

### Implementation for User Story 1

- [X] T009 [P] [US1] `xiaozhi-esp32/CMakeLists.txt`、`xiaozhi-esp32/main/main.cc`、`xiaozhi-esp32/main/application.cc` を基に全体概要を `docs/xiaozhi-esp32-architecture.md` に記述する
- [X] T010 [P] [US1] `xiaozhi-esp32/main/main.cc` と `xiaozhi-esp32/main/application.cc` を基に起動フローを `docs/xiaozhi-esp32-architecture.md` に記述する
- [X] T011 [US1] `xiaozhi-esp32/main/protocols/` と `xiaozhi-esp32/main/audio/README.md` を基に通信・音声の代表的主要フローを `docs/xiaozhi-esp32-architecture.md` に記述する
- [X] T012 [US1] `docs/xiaozhi-esp32-architecture.md` に確認済み事項、推定事項、対象外事項の記法を追加する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - 関心領域ごとの実装位置を特定できる (Priority: P2)

**Goal**: 開発者が主要関心領域ごとの調査起点を文書から特定できるようにする

**Independent Test**: 文書から起動、音声、表示、通信、設定、OTA、ボード差分の実装位置を一意にたどれることを確認する

### Verification for User Story 2

- [X] T013 [US2] User Story 2 の手動確認手順を `specs/004-document-xiaozhi-arch/quickstart.md` に追記する

### Implementation for User Story 2

- [X] T014 [P] [US2] `xiaozhi-esp32/main/display/`、`xiaozhi-esp32/main/settings.*`、`xiaozhi-esp32/main/ota.*` を基に表示・設定・OTA の実装位置を `docs/xiaozhi-esp32-architecture.md` に記述する
- [X] T015 [P] [US2] `xiaozhi-esp32/main/protocols/`、`xiaozhi-esp32/main/audio/`、`xiaozhi-esp32/main/boards/README.md` を基に通信・音声・ボード差分の実装位置を `docs/xiaozhi-esp32-architecture.md` に記述する
- [X] T016 [US2] `docs/xiaozhi-esp32-architecture.md` に共通実装とボード固有実装の境界説明を追加する
- [X] T017 [US2] `docs/xiaozhi-esp32-architecture.md` に読者が次に読むべき代表ファイルまたはディレクトリ参照を追加する

**Checkpoint**: User Story 1 と 2 が独立検証可能であること

---

## Phase 5: User Story 3 - 調査結果を継続的に再利用できる (Priority: P3)

**Goal**: 解析結果を見つけやすく更新しやすい恒久文書として残す

**Independent Test**: 新規メンバーがリポジトリ内で文書を見つけ、補足説明なしに参照できることを確認する

### Verification for User Story 3

- [X] T018 [US3] User Story 3 の手動確認手順を `specs/004-document-xiaozhi-arch/quickstart.md` に追記する

### Implementation for User Story 3

- [X] T019 [US3] `docs/xiaozhi-esp32-architecture.md` に対象範囲、対象外、未確認事項、更新しやすい見出し構造を整備する
- [X] T020 [P] [US3] `xiaozhi-esp32/README.md` から `docs/xiaozhi-esp32-architecture.md` への導線追加要否を確認し、必要なら `xiaozhi-esp32/README.md` を更新する
- [X] T021 [US3] `docs/xiaozhi-esp32-architecture.md` と `specs/004-document-xiaozhi-arch/quickstart.md` を見直し、文書の発見性と再利用性を最終確認する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 複数 story にまたがる最終確認

- [X] T022 [P] `specs/004-document-xiaozhi-arch/spec.md`、`plan.md`、`tasks.md` と `docs/xiaozhi-esp32-architecture.md` の整合性を確認する
- [X] T023 `specs/004-document-xiaozhi-arch/quickstart.md` の検証手順に従って手動確認を実施し、結果を反映する
- [X] T024 `docs/xiaozhi-esp32-architecture.md` と `xiaozhi-esp32/README.md` の最終文言を見直し、日本語表現と用語統一を確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Phase 6)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の文書骨格完成後に開始すると効率が良い
- **User Story 3 (P3)**: User Story 1 と 2 の成果物が揃った後に開始する

### Within Each User Story

- 手動確認手順を先に明示してから実装する
- 全体構造を先にまとめ、詳細な関心領域説明を後から追加する
- 導線追加は主文書の内容が固まってから行う
- 検証タスクを省略しない

### Parallel Opportunities

- `[P]` 付き Setup / Foundational タスクは並列実行可能
- User Story 1 では全体概要整理と起動フロー整理を並列化できる
- User Story 2 では関心領域ごとの追跡を別ファイル群ごとに並列化できる
- Polish では整合性確認と用語確認を並列化できる

---

## Parallel Example: User Story 1

```bash
Task: "xiaozhi-esp32/CMakeLists.txt、xiaozhi-esp32/main/main.cc、xiaozhi-esp32/main/application.cc を基に全体概要を docs/xiaozhi-esp32-architecture.md に記述する"
Task: "xiaozhi-esp32/main/main.cc と xiaozhi-esp32/main/application.cc を基に起動フローを docs/xiaozhi-esp32-architecture.md に記述する"
```

## Parallel Example: User Story 2

```bash
Task: "xiaozhi-esp32/main/display/、xiaozhi-esp32/main/settings.*、xiaozhi-esp32/main/ota.* を基に表示・設定・OTA の実装位置を docs/xiaozhi-esp32-architecture.md に記述する"
Task: "xiaozhi-esp32/main/protocols/、xiaozhi-esp32/main/audio/、xiaozhi-esp32/main/boards/README.md を基に通信・音声・ボード差分の実装位置を docs/xiaozhi-esp32-architecture.md に記述する"
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
4. User Story 3 を追加して独立検証する
5. 最後に Polish で整合性と導線を確認する

### Parallel Team Strategy

1. 1 人が文書骨格と手動検証手順を担当する
2. 別担当が関心領域ごとの調査を分担する
3. 主文書の内容確定後に導線追加と最終レビューを行う

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために必須
- 今回の feature では自動テスト追加ではなく、手動確認手順を必須とする
- コード挙動変更、全ボード詳細一覧化、実装外の一般向け資料化は対象外

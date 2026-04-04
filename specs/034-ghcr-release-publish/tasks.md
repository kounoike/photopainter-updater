# タスク: Release 時の GHCR image publish

**Input**: `/specs/034-ghcr-release-publish/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: release publish feature の作業対象と前提を揃える

- [ ] T001 既存 release workflow と Docker build 入力の参照点を確認し `specs/034-ghcr-release-publish/plan.md` に実装境界メモを追記する
- [ ] T002 `.github/workflows/` と `.github/` 配下の命名方針を確認し release image publish 用ファイル名を `specs/034-ghcr-release-publish/contracts/release-image-publish-contract.md` に固定する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に共通する release publish 基盤を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T003 `.github/release-image-publish.yml` に publish target schema の初期定義を追加する
- [ ] T004 [P] `.github/workflows/release-image-publish.yml` に `release.published` trigger と job permissions の骨格を追加する
- [ ] T005 [P] `.github/workflows/release-image-publish.yml` で `.github/release-image-publish.yml` を参照する target 読み出し方針を実装する
- [ ] T006 `.github/workflows/release-image-publish.yml` と `.github/release-image-publish.yml` の責務分離を `specs/034-ghcr-release-publish/quickstart.md` に反映する
- [ ] T007 Allowed Scope / Forbidden Scope の実装境界と `server` のみ初期 enabled とする前提を `specs/034-ghcr-release-publish/tasks.md` に記録する

**Checkpoint**: 基盤完了後に user story 実装へ進む

---

## Phase 3: User Story 1 - release 時に server image を自動公開したい (Priority: P1)

**Goal**: draft release publish を契機に `server` image を build し GHCR へ publish できるようにする

**Independent Test**: release event を想定した workflow 実行で、`server` image が release version tag 付きで publish 対象として処理されることを確認する

### Verification for User Story 1

- [ ] T008 [P] [US1] `specs/034-ghcr-release-publish/quickstart.md` に `release.published` と `server` publish 成否の手動確認手順を追加する
- [ ] T009 [US1] `.github/workflows/release-image-publish.yml` と `.github/release-image-publish.yml` の YAML 構文検証手順を `specs/034-ghcr-release-publish/quickstart.md` に記載する

### Implementation for User Story 1

- [ ] T010 [P] [US1] `.github/release-image-publish.yml` に `server` target の build context、Dockerfile、image repository、enabled 状態を定義する
- [ ] T011 [US1] `.github/workflows/release-image-publish.yml` に GHCR login と `server` target の build/push step を実装する
- [ ] T012 [US1] `.github/workflows/release-image-publish.yml` に release version ベースの tag/label 生成を実装する
- [ ] T013 [US1] `.github/workflows/release-image-publish.yml` に release version 不正時の fail-fast と未対象 event の非実行条件を追加する
- [ ] T014 [US1] `README.md` に release publish 時の `server` image 公開先と確認場所を追記する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - 追加 image を同じ仕組みで載せたい (Priority: P2)

**Goal**: `server` 以外の image を同じ target schema に追加できる拡張点を残す

**Independent Test**: publish target 定義を見れば `server` が初期 target として存在し、同じ形式で追加 target を表現できることを確認する

### Verification for User Story 2

- [ ] T015 [P] [US2] `specs/034-ghcr-release-publish/contracts/release-image-publish-contract.md` に target 追加時の必須項目と非対象条件の確認観点を追記する
- [ ] T016 [US2] `specs/034-ghcr-release-publish/quickstart.md` に将来 `comfyui` などを追加する際の確認手順を追記する

### Implementation for User Story 2

- [ ] T017 [P] [US2] `.github/release-image-publish.yml` を複数 target を表現できる構造に整理し、初期値として `server` のみを enabled に保つ
- [ ] T018 [US2] `.github/workflows/release-image-publish.yml` で未定義または disabled target を publish しない制御を実装する
- [ ] T019 [US2] `README.md` に publish target 定義の拡張方針と今回の対象外 image の扱いを追記する

**Checkpoint**: User Story 2 が独立して検証可能であること

---

## Phase 5: User Story 3 - release image の運用方法を追いたい (Priority: P3)

**Goal**: release publish の契機、確認場所、失敗時の追跡方法を repository 内文書から理解できるようにする

**Independent Test**: README と quickstart だけで trigger、Actions、GHCR の確認導線を再現できることを確認する

### Verification for User Story 3

- [ ] T020 [P] [US3] `specs/034-ghcr-release-publish/quickstart.md` に Releases、Actions、GHCR の 3 点確認フローを記載する
- [ ] T021 [US3] `README.md` に release image publish の trigger、対象 image、確認場所が揃っていることを見直す

### Implementation for User Story 3

- [ ] T022 [US3] `README.md` に release drafter と release image publish の責務差分を明記する
- [ ] T023 [US3] `specs/034-ghcr-release-publish/quickstart.md` に失敗時の確認順序と対象ファイルを整理する
- [ ] T024 [US3] `specs/034-ghcr-release-publish/contracts/release-image-publish-contract.md` に visibility contract と scope guard の運用説明を補強する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 横断確認と最終整合を取る

- [ ] T025 [P] `.github/workflows/release-image-publish.yml` と `.github/release-image-publish.yml` の YAML 構文を確認する
- [ ] T026 `README.md`、`specs/034-ghcr-release-publish/quickstart.md`、`specs/034-ghcr-release-publish/contracts/release-image-publish-contract.md` の記述整合を確認する
- [ ] T027 `git diff --check` 相当で workflow / 文書差分の体裁を確認する
- [ ] T028 `specs/034-ghcr-release-publish/quickstart.md` の手順で live 確認に必要な GitHub 側作業だけが残ることを確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の target 定義と workflow 骨格を前提に開始する
- **User Story 3 (P3)**: User Story 1 と 2 の文書反映後に開始する

### Within Each User Story

- 文書で定義する検証手順を先に固め、その後 workflow / 設定を実装する
- publish target 定義を workflow の build/push 実装より先に整える
- README 反映は各 story の実装結果を受けて更新する
- 検証タスクを省略しない

### Parallel Opportunities

- `[P]` 付き Foundational タスクは並列実行可能
- US1 の verification と target 定義は並列実行可能
- US2 の contract 更新と target schema 整理は並列実行可能
- Polish の YAML 構文確認と文書整合確認は並列実行可能

---

## Parallel Example: User Story 1

```bash
Task: "specs/034-ghcr-release-publish/quickstart.md に release.published と server publish 成否の手動確認手順を追加する"
Task: ".github/release-image-publish.yml に server target の build context、Dockerfile、image repository、enabled 状態を定義する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. YAML 構文確認と文書上の独立検証を行う
5. draft release publish 後の live 確認を残リスクとして扱う

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して `server` publish を独立検証する
3. User Story 2 を追加して target 拡張性を独立検証する
4. User Story 3 を追加して運用導線を独立検証する
5. Polish で YAML と文書の整合を確認する

### Parallel Team Strategy

1. 1 人が Foundational で workflow 骨格を整える
2. もう 1 人が quickstart / contract の検証記述を先行して整える
3. US1 完了後に target schema 側と README 側へ担当を分ける

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために必須
- live の GitHub Release publish と GHCR 確認は実装後の手動検証対象として残る

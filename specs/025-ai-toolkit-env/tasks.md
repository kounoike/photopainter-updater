# タスク: AI Toolkit 試用環境

**Input**: `/specs/025-ai-toolkit-env/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Path Conventions

- Compose 定義: `compose.yml`
- 環境変数テンプレート: `.env.example`
- 利用者向け入口: `README.md`
- feature 成果物: `specs/025-ai-toolkit-env/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: upstream `ostris/ai-toolkit` の compose 前提と本リポジトリの既存 Compose 構成との差分を固定する

- [ ] T001 `compose.yml`、`.env.example`、`README.md` と `specs/025-ai-toolkit-env/plan.md` / `specs/025-ai-toolkit-env/research.md` の差分を確認する
- [ ] T002 `specs/025-ai-toolkit-env/contracts/ai-toolkit-compose-contract.md` と `specs/025-ai-toolkit-env/quickstart.md` を実装前提に合わせて見直し、AI Toolkit の service 名、保存先、UI 到達条件を確定する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に先行する AI Toolkit 追加方針を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T003 `compose.yml` に追加する `ai-toolkit` service の image、ports、volumes、environment、restart 方針を設計どおりに整理する
- [ ] T004 [P] `.env.example` に追加する AI Toolkit 用ポート、認証、保存先変数の候補を整理する
- [ ] T005 [P] `README.md` と `specs/025-ai-toolkit-env/quickstart.md` の役割分担を整理し、入口と詳細手順の境界を確定する
- [ ] T006 `specs/025-ai-toolkit-env/quickstart.md` に `compose-state` / `env-config` / `storage-path` の 3 系統復帰方針を最終反映する

**Checkpoint**: 基盤完了後に user story 実装へ進む

---

## Phase 3: User Story 1 - AI Toolkit を起動して触り始める (Priority: P1)

**Goal**: `docker compose up -d ai-toolkit` で AI Toolkit を起動し、Web UI 到達まで案内できるようにする

**Independent Test**: 利用者が `README.md` と `specs/025-ai-toolkit-env/quickstart.md` だけを参照し、`docker compose up -d ai-toolkit` 実行後に Web UI 到達可否を判断できれば完了

### Verification for User Story 1

- [ ] T007 [US1] User Story 1 の手動確認手順を `specs/025-ai-toolkit-env/quickstart.md` に記載する

### Implementation for User Story 1

- [ ] T008 [US1] `ai-toolkit` service を `compose.yml` に追加する
- [ ] T009 [US1] AI Toolkit 用の入口説明と起動コマンドを `README.md` に追加する
- [ ] T010 [US1] `docker compose up -d ai-toolkit` と Web UI 到達確認手順を `specs/025-ai-toolkit-env/quickstart.md` に具体化する
- [ ] T011 [US1] AI Toolkit 起動に必要な主要環境変数を `.env.example` に追加する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - 試用データと設定を保持して再開する (Priority: P2)

**Goal**: AI Toolkit の保存先を維持したまま停止・再起動・再開できるようにする

**Independent Test**: 利用者が AI Toolkit を起動・停止・再起動した後も、同じ保存先設定で再開できると手順書から判断できれば完了

### Verification for User Story 2

- [ ] T012 [US2] User Story 2 の手動確認手順を `specs/025-ai-toolkit-env/quickstart.md` に記載する

### Implementation for User Story 2

- [ ] T013 [US2] AI Toolkit 用の config / datasets / output / DB / cache の保存先を `compose.yml` に反映する
- [ ] T014 [US2] 保存先と再起動前提を `.env.example` に反映する
- [ ] T015 [US2] 保存先準備、停止、再起動、再開の手順を `specs/025-ai-toolkit-env/quickstart.md` に追加する
- [ ] T016 [US2] 保存先維持の前提と対象範囲を `specs/025-ai-toolkit-env/contracts/ai-toolkit-compose-contract.md` に同期する

**Checkpoint**: User Story 1 と 2 が独立検証可能であること

---

## Phase 5: User Story 3 - 既存 Compose 導線を壊さずに共存させる (Priority: P3)

**Goal**: AI Toolkit の追加が既存 ComfyUI / Ollama 導線を壊さないようにする

**Independent Test**: 利用者が `README.md` を確認したとき、ComfyUI / Ollama の既存導線が残り、AI Toolkit は追加サービスとして理解できれば完了

### Verification for User Story 3

- [ ] T017 [US3] User Story 3 の手動確認手順を `specs/025-ai-toolkit-env/quickstart.md` または `README.md` に記載する

### Implementation for User Story 3

- [ ] T018 [US3] 既存 ComfyUI / Ollama 導線を維持した説明へ `README.md` を調整する
- [ ] T019 [US3] AI Toolkit が追加サービスであることと非目標を `specs/025-ai-toolkit-env/contracts/ai-toolkit-compose-contract.md` に反映する
- [ ] T020 [US3] 既存サービスと独立して AI Toolkit を起動できる説明を `specs/025-ai-toolkit-env/quickstart.md` に追加する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 複数 story にまたがる最終確認

- [ ] T021 [P] `specs/025-ai-toolkit-env/spec.md`、`specs/025-ai-toolkit-env/plan.md`、`specs/025-ai-toolkit-env/tasks.md` の整合を最終確認する
- [ ] T022 `docker compose config` と `docker compose ps ai-toolkit` の確認結果に基づき `README.md`、`.env.example`、`specs/025-ai-toolkit-env/quickstart.md` の手順差分を修正する
- [ ] T023 `specs/025-ai-toolkit-env/quickstart.md` の手順どおりに AI Toolkit Web UI 到達まで通し確認し、残リスクを `specs/025-ai-toolkit-env/quickstart.md` に反映する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の service 追加後に進める
- **User Story 3 (P3)**: User Story 1 と 2 の反映後に境界整理として実施する

### Within Each User Story

- 手動確認手順を先に固め、その後に実装対象ファイルへ反映する
- `README.md` と `specs/025-ai-toolkit-env/quickstart.md` の説明は同じ起動条件を使う
- 既存導線非破壊の確認を省略しない

### Parallel Opportunities

- `[P]` 付き Foundational タスクは並列実行可能
- User Story 1 完了後、User Story 2 の contract 同期と `.env.example` 更新は並列化可能
- Polish では整合確認と手順差分修正を分けて進められる

---

## Parallel Example: User Story 2

```bash
Task: "AI Toolkit 用の config / datasets / output / DB / cache の保存先を compose.yml に反映する"
Task: "保存先と再起動前提を .env.example に反映する"
Task: "保存先維持の前提と対象範囲を specs/025-ai-toolkit-env/contracts/ai-toolkit-compose-contract.md に同期する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. User Story 1 を独立検証する
5. `ai-toolkit` 起動と Web UI 到達の導線が成立することを確認する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して独立検証する
3. User Story 2 を追加して保存先維持を独立検証する
4. User Story 3 を追加して既存導線非破壊を独立検証する
5. 最後に通し確認で前段 story を壊していないことを確認する

### Parallel Team Strategy

1. チームで Setup + Foundational を完了する
2. User Story 1 完了後、User Story 2 の保存先同期と文書更新を分担する
3. User Story 3 は既存導線レビュー担当が独立検証まで完了する

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために必須
- 曖昧なタスク、同一ファイル衝突、独立性を壊す cross-story 依存を避ける

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

**Purpose**: 実装対象と既存導線の現状を固定する

- [ ] T001 `compose.yml`、`.env.example`、`README.md` の現状と `specs/025-ai-toolkit-env/plan.md` の方針差分を確認する
- [ ] T002 `specs/025-ai-toolkit-env/contracts/ai-toolkit-compose-contract.md` と `specs/025-ai-toolkit-env/quickstart.md` を実装前提に合わせて見直し、実装対象のチェックポイントを確定する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に先行する共通導線と境界を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T003 `README.md` に追加する AI Toolkit 入口の見出し、説明範囲、既存 ComfyUI/Ollama 導線との関係を設計どおりに整理する
- [ ] T004 [P] `.env.example` に AI Toolkit 試用時の前提として参照する変数説明の不足を洗い出す
- [ ] T005 [P] `compose.yml` で AI Toolkit 試用導線に必要な補助説明や命名整理が必要か確認し、Allowed Scope 内の変更候補を確定する
- [ ] T006 `specs/025-ai-toolkit-env/quickstart.md` に代表操作と `compose-state` / `env-config` / `persistent-data` の 3 系統復帰方針を最終反映する

**Checkpoint**: 基盤完了後に user story 実装へ進む

---

## Phase 3: User Story 1 - すぐ試せる作業開始導線 (Priority: P1)

**Goal**: 新規利用者が AI Toolkit 試用環境を起動し、代表操作の成功可否まで迷わず到達できるようにする

**Independent Test**: 新しい開発者が `README.md` と `specs/025-ai-toolkit-env/quickstart.md` だけを参照し、`docker compose up -d` と `docker compose exec comfyui curl -fsS http://ollama:11434/api/version` まで到達できれば完了

### Verification for User Story 1

- [ ] T007 [US1] User Story 1 の手動確認手順を `specs/025-ai-toolkit-env/quickstart.md` に記載する

### Implementation for User Story 1

- [ ] T008 [US1] AI Toolkit 試用環境の入口説明を `README.md` に追加する
- [ ] T009 [US1] AI Toolkit 試用開始に必要な前提条件と `.env` 準備手順を `README.md` に反映する
- [ ] T010 [US1] `docker compose up -d` と代表操作 `docker compose exec comfyui curl -fsS http://ollama:11434/api/version` の実行手順を `specs/025-ai-toolkit-env/quickstart.md` に具体化する
- [ ] T011 [US1] AI Toolkit 試用導線に必要な説明補強を `.env.example` に反映する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - 再現可能な共有環境 (Priority: P2)

**Goal**: 複数の開発者が同じ Compose 前提から同じ試用開始判定へ到達できるようにする

**Independent Test**: 別の開発者が `.env.example`、`README.md`、`specs/025-ai-toolkit-env/quickstart.md` を参照し、同一の前提条件、起動手順、代表操作成功条件を解釈差異なく辿れれば完了

### Verification for User Story 2

- [ ] T012 [US2] User Story 2 の手動確認手順を `specs/025-ai-toolkit-env/quickstart.md` に記載する

### Implementation for User Story 2

- [ ] T013 [US2] 共通の前提条件、永続化ディレクトリ、起動順序を `specs/025-ai-toolkit-env/quickstart.md` に統一表現で整理する
- [ ] T014 [US2] 既存 ComfyUI/Ollama を土台にする再現可能な説明へ `specs/025-ai-toolkit-env/contracts/ai-toolkit-compose-contract.md` を同期する
- [ ] T015 [US2] 共通の利用開始判定と成功シグナルを `README.md` と `specs/025-ai-toolkit-env/quickstart.md` の両方で一致させる
- [ ] T016 [US2] 必要な補助説明やコメントを `compose.yml` に反映し、AI Toolkit 試用時の読み取りやすさを揃える

**Checkpoint**: User Story 1 と 2 が独立検証可能であること

---

## Phase 5: User Story 3 - 既存作業への影響を抑える (Priority: P3)

**Goal**: AI Toolkit の追加導線が既存 ComfyUI / Ollama 利用者の主要導線を壊さないようにする

**Independent Test**: 既存利用者が `README.md` を読んだとき、ComfyUI と Ollama の単独利用導線をそのまま辿れ、AI Toolkit が追加導線であると理解できれば完了

### Verification for User Story 3

- [ ] T017 [US3] User Story 3 の手動確認手順を `specs/025-ai-toolkit-env/quickstart.md` または `README.md` に記載する

### Implementation for User Story 3

- [ ] T018 [US3] 既存 ComfyUI と Ollama の個別導線を壊さない説明へ `README.md` を調整する
- [ ] T019 [US3] AI Toolkit の対象 / 非対象と非目標を `specs/025-ai-toolkit-env/contracts/ai-toolkit-compose-contract.md` に反映する
- [ ] T020 [US3] 既存導線を置き換えない境界説明と復帰方針を `specs/025-ai-toolkit-env/quickstart.md` に追加する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 複数 story にまたがる最終確認

- [ ] T021 [P] `specs/025-ai-toolkit-env/spec.md`、`specs/025-ai-toolkit-env/plan.md`、`specs/025-ai-toolkit-env/tasks.md` の整合を最終確認する
- [ ] T022 `docker compose config` と `docker compose ps` の確認結果に基づき `README.md`、`.env.example`、`specs/025-ai-toolkit-env/quickstart.md` の手順差分を修正する
- [ ] T023 `specs/025-ai-toolkit-env/quickstart.md` の手順どおりに代表操作まで通し確認し、残リスクを `specs/025-ai-toolkit-env/quickstart.md` に反映する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の導線表現を土台に進めると安全
- **User Story 3 (P3)**: User Story 1 と 2 の文書反映後に最終境界整理として実施する

### Within Each User Story

- 手動確認手順を先に固め、その後に実装対象ファイルへ反映する
- `README.md` と `specs/025-ai-toolkit-env/quickstart.md` の説明は同じ成功条件を使う
- 既存導線を壊さない確認を省略しない

### Parallel Opportunities

- `[P]` 付き Foundational タスクは並列実行可能
- User Story 1 完了後、User Story 2 の contract 同期と `compose.yml` 補助説明は並列化可能
- Polish では整合確認と手順差分修正を分けて進められる

---

## Parallel Example: User Story 2

```bash
Task: "共通の前提条件、永続化ディレクトリ、起動順序を specs/025-ai-toolkit-env/quickstart.md に統一表現で整理する"
Task: "既存 ComfyUI/Ollama を土台にする再現可能な説明へ specs/025-ai-toolkit-env/contracts/ai-toolkit-compose-contract.md を同期する"
Task: "必要な補助説明やコメントを compose.yml に反映し、AI Toolkit 試用時の読み取りやすさを揃える"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. User Story 1 を独立検証する
5. AI Toolkit 入口から代表操作までの導線が成立することを確認する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して独立検証する
3. User Story 2 を追加して再現性を独立検証する
4. User Story 3 を追加して既存導線非破壊を独立検証する
5. 最後に通し確認で前段 story を壊していないことを確認する

### Parallel Team Strategy

1. チームで Setup + Foundational を完了する
2. User Story 1 完了後、User Story 2 の文書同期と `compose.yml` 調整を分担する
3. User Story 3 は既存導線レビュー担当が独立検証まで完了する

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために必須
- 曖昧なタスク、同一ファイル衝突、独立性を壊す cross-story 依存を避ける

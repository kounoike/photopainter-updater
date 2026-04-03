# タスク: HTTPサーバ Compose 統合

**Input**: `/specs/029-compose-http-server/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3`)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: server compose 統合の前提確認

- [ ] T001 `compose.yml`、`server/run.sh`、`server/README.md`、`README.md` の現行導線と server 起動条件を確認する
- [ ] T002 [P] `server/Dockerfile` の設計前提に合わせて `server/.dockerignore` の要否と既存 `.dockerignore`、`.env.example` の不足項目を確認する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: server container 化の基盤を整える

- [ ] T003 `server/Dockerfile` に HTTP サーバ build / run の基本構成を追加する
- [ ] T004 [P] `compose.yml` に追加する `server` service の environment / volume / port / network 方針を `specs/029-compose-http-server/quickstart.md` と `contracts/compose-http-server-runtime-contract.md` に反映する
- [ ] T005 [P] `.env.example` と `server/README.md` に compose 運用で引き継ぐ `SERVER_PORT`、`SERVER_CONTENT_DIR`、ログ導線の前提を整理する

---

## Phase 3: User Story 1 - compose だけで HTTP サーバを使いたい (Priority: P1)

**Goal**: `docker compose` だけで HTTP サーバを起動し、既存 endpoint を利用できるようにする

**Independent Test**: `docker compose up -d server` 相当で server が起動し、`/`、`/image.bmp`、`/image.bin`、`/upload` が利用できる

### Verification for User Story 1

- [ ] T006 [US1] `specs/029-compose-http-server/quickstart.md` に server 単体起動、ログ確認、基本 endpoint 確認手順を確定する
- [ ] T007 [P] [US1] 手動確認: `docker compose config` と `docker compose up -d server`、`curl` による `/`、`/image.bmp`、`/image.bin`、`/upload` の疎通を確認する

### Implementation for User Story 1

- [ ] T008 [US1] `compose.yml` に HTTP サーバ `server` service を追加する
- [ ] T009 [US1] `server/Dockerfile`、`.env.example` と必要な ignore 設定で server image build / env 注入を実装する
- [ ] T010 [US1] `server/run.sh` の責務を compose / container 起動へ置き換えられるよう `server/README.md` を更新する

---

## Phase 4: User Story 2 - 既存 compose サービスと共存させたい (Priority: P2)

**Goal**: ComfyUI、Ollama、AI Toolkit と同じ compose 内で server を共存させる

**Independent Test**: compose 設定上、既存サービスが残ったまま server service が追加され、必要なサービス単位で起動できる

### Verification for User Story 2

- [ ] T011 [US2] `specs/029-compose-http-server/contracts/compose-http-server-runtime-contract.md` と `compose.yml` の service / network / volume 定義が一致していることを確認する
- [ ] T012 [P] [US2] 手動確認: 既存 `comfyui` / `ollama` / `ai-toolkit` 定義を壊さず `docker compose config` が通ることを確認する

### Implementation for User Story 2

- [ ] T013 [US2] `compose.yml` の既存サービス定義を壊さない形で `server` service を統合する
- [ ] T014 [US2] `README.md` に server 単体起動と既存サービス併用起動の compose 導線を追記する

---

## Phase 5: User Story 3 - 手順書を一本化したい (Priority: P3)

**Goal**: HTTP サーバ起動方法を compose 前提へ統一し、`server/run.sh` を廃止する

**Independent Test**: README、server README、feature quickstart を読めば `server/run.sh` なしで server の起動・停止・ログ確認方法が分かる

### Verification for User Story 3

- [ ] T015 [US3] `README.md`、`server/README.md`、`specs/029-compose-http-server/quickstart.md` の導線が compose 前提で整合していることを確認する

### Implementation for User Story 3

- [ ] T016 [US3] `server/run.sh` を削除し、必要な説明を `server/README.md` と `specs/029-compose-http-server/quickstart.md` に移す
- [ ] T017 [US3] `README.md` の HTTP サーバ導線を compose 起動前提に更新する
- [ ] T018 [US3] `specs/029-compose-http-server/quickstart.md` に起動、停止、ログ確認、upload 確認の最終手順を反映する

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 最終整合と残作業の明確化

- [ ] T019 [P] `specs/029-compose-http-server/tasks.md` を完了状態へ更新し、文書と実装のズレがないことを確認する
- [ ] T020 `README.md`、`server/README.md`、`specs/029-compose-http-server/quickstart.md` に Docker 未実施環境での未確認事項があれば明記する

## Dependencies & Execution Order

- Setup → Foundational → US1 → US2 → US3 → Polish
- MVP は US1

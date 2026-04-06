---

description: "Local RunPod image 統一の実装タスクリスト"
---

# タスク: Local RunPod image 統一

**Input**: `/specs/044-local-runpod-image/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/local-runpod-runtime-contract.md`、`quickstart.md`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3`)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 共通 runtime へ切り替える前提と作業境界を固定する

- [ ] T001 現行 runtime 契約と作業対象を `specs/044-local-runpod-image/plan.md`、`specs/044-local-runpod-image/spec.md`、`specs/044-local-runpod-image/contracts/local-runpod-runtime-contract.md` で再確認する
- [ ] T002 [P] local / RunPod 共通 runtime で更新対象になる実ファイルを `compose.yml`、`.env.example`、`README.md`、`comfyui/runpod/README.md`、`specs/044-local-runpod-image/quickstart.md` で棚卸しする

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に先行する compose / runtime の共通前提を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T003 `compose.yml` の `comfyui` service を `comfyui/runpod/Dockerfile` ベースへ切り替えるための build / env / volume 差分を `compose.yml` に整理する
- [ ] T004 `.env.example` の local 永続化設定を `/runpod-volume` 前提へ寄せるため、旧 `COMFYUI_DATA_DIR` / `OLLAMA_DATA_DIR` 導線の扱いを `.env.example` に整理する
- [ ] T005 [P] 旧 local 専用 runtime 資産の廃止方針を `comfyui/Dockerfile`、`comfyui/entrypoint.sh`、`comfyui/install-custom-nodes.sh`、`README.md` で明示できる状態に整理する
- [ ] T006 Allowed Scope / Forbidden Scope を逸脱していないことを `specs/044-local-runpod-image/spec.md` と実装対象ファイルで確認する

**Checkpoint**: compose 定義、env 前提、旧 runtime 資産の扱いが user story 実装の前提として固まっている

---

## Phase 3: User Story 1 - ローカルでも RunPod と同じ ComfyUI image を使う (Priority: P1)

**Goal**: local の `comfyui` 起動入口を維持したまま、RunPod と同じ `worker-comfyui` ベース image へ統一する

**Independent Test**: `docker compose config` で `comfyui` service が `comfyui/runpod/Dockerfile` を使い、`docker compose up -d comfyui` 後に Web UI 到達と `docker compose exec comfyui curl -fsS http://127.0.0.1:11434/api/version` の確認手順を追えること

### Verification for User Story 1

- [ ] T007 [US1] local の共通 image 起動確認手順と期待ログを `specs/044-local-runpod-image/quickstart.md` に明記する

### Implementation for User Story 1

- [ ] T008 [US1] `compose.yml` の `comfyui` service を `comfyui/runpod/Dockerfile` と `comfyui/runpod/start-ollama-worker.sh` 前提の定義へ更新する
- [ ] T009 [US1] local の Ollama 同居起動と `curl http://127.0.0.1:11434/api/version` 確認を `README.md` に反映する
- [ ] T010 [US1] local / RunPod 共通 runtime の起動 contract と upstream 委譲方式を `comfyui/runpod/README.md` に反映する

**Checkpoint**: `comfyui` service 名を維持したまま、local / RunPod が同じ image 前提で説明・起動できる

---

## Phase 4: User Story 2 - ローカルでも `/runpod-volume` 前提で model を扱う (Priority: P2)

**Goal**: local でも RunPod と同じ `/runpod-volume` 配下の model path 前提へ揃える

**Independent Test**: local 手順に `/runpod-volume` bind mount 準備が必須として明記され、ComfyUI と Ollama の model path を `docker compose exec comfyui` で確認する流れを辿れること

### Verification for User Story 2

- [ ] T011 [US2] `/runpod-volume` bind mount 必須、ComfyUI model root、Ollama model storage の確認手順を `specs/044-local-runpod-image/quickstart.md` に明記する

### Implementation for User Story 2

- [ ] T012 [US2] local bind mount と model path 契約を `compose.yml` に反映し、独立 `ollama` service 依存なしで `/runpod-volume` を渡す
- [ ] T013 [US2] `/runpod-volume` 前提の host directory 準備と `.env` 設定例を `.env.example` に反映する
- [ ] T014 [US2] local / RunPod で共通の storage layout と model path の役割を `README.md` と `comfyui/runpod/README.md` に反映する

**Checkpoint**: local でも `/runpod-volume/models` と `/runpod-volume/ollama/models` を混同せず運用できる

---

## Phase 5: User Story 3 - 旧 local 専用構成を廃止する (Priority: P3)

**Goal**: 旧 local 専用 runtime と独立 `ollama` service を現行導線から外し、保守対象を 1 系統へ畳む

**Independent Test**: repo の compose / README / quickstart / runtime README を読むだけで、現行導線が共通 image ベース 1 つに整理され、旧 local 専用構成が保守対象外だと判断できること

### Verification for User Story 3

- [ ] T015 [US3] 旧 local 専用導線の廃止、移行先、非推奨事項を `specs/044-local-runpod-image/quickstart.md` に明記する

### Implementation for User Story 3

- [ ] T016 [US3] 独立 `ollama` service 廃止と新しい local 導線への移行説明を `README.md` に反映する
- [ ] T017 [US3] local 専用 runtime 資産の扱いを `comfyui/Dockerfile`、`comfyui/entrypoint.sh`、`comfyui/install-custom-nodes.sh` の削除または非推奨化で整理する
- [ ] T018 [US3] 共通 runtime の唯一の一次情報としての役割を `comfyui/runpod/README.md` に反映する

**Checkpoint**: 現行導線の一次情報が `compose.yml`、`README.md`、`comfyui/runpod/README.md` に集約されている

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 全 story を横断して整合性と手順の実行可能性を仕上げる

- [ ] T019 [P] `specs/044-local-runpod-image/contracts/local-runpod-runtime-contract.md` を実装後の compose / docs と照合し、必要な文言差分を解消する
- [ ] T020 [P] `compose.yml`、`comfyui/runpod/Dockerfile`、`comfyui/runpod/README.md` を照合し、既存 custom node と `comfyui-ollama` 利用前提が維持されていることを確認する
- [ ] T021 `specs/044-local-runpod-image/quickstart.md` の手順に沿って `docker compose config`、`docker compose up -d comfyui`、`docker compose exec comfyui curl -fsS http://127.0.0.1:11434/api/version`、`OLLAMA_PULL_MODELS` 指定時の `model_result` ログ確認結果を記録する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Story 1 (Phase 3)**: Foundational 完了後に開始する
- **User Story 2 (Phase 4)**: User Story 1 の compose runtime 切替完了後に開始する
- **User Story 3 (Phase 5)**: User Story 1 と User Story 2 の導線整理後に開始する
- **Polish (Phase 6)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: 依存なし。MVP
- **User Story 2 (P2)**: `comfyui` service が共通 image ベースへ切り替わっていることに依存する
- **User Story 3 (P3)**: 新しい共通導線が README / quickstart / runtime README で確定していることに依存する

### Within Each User Story

- 検証タスクを先に更新し、受け入れ条件が文書化されてから実装を進める
- `compose.yml` の変更を基準に `.env.example` と各 README を追従させる
- 旧導線の削除や非推奨化は、新導線の説明が揃ってから行う

### Parallel Opportunities

- T002 と T005 は並列実行可能
- User Story 1 完了後、T013 と T014 は並列実行可能
- User Story 3 では T016 と T018 は並列実行可能
- Polish では契約照合の T019 と手順検証の T020 を並列で進められる
- Polish では契約照合の T019 と依存維持確認の T020 を並列で進められる

---

## Parallel Example: User Story 2

```bash
Task: "T013 [US2] `/runpod-volume` 前提の host directory 準備と `.env` 設定例を `.env.example` に反映する"
Task: "T014 [US2] local / RunPod で共通の storage layout と model path の役割を `README.md` と `comfyui/runpod/README.md` に反映する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 で対象ファイルと作業境界を固定する
2. Phase 2 で `compose.yml` と `.env.example` の共通前提を整える
3. Phase 3 で `comfyui` service を共通 image ベースへ切り替える
4. local の ComfyUI Web UI 到達と Ollama localhost 疎通を確認する

### Incremental Delivery

1. User Story 1 で runtime image と起動方式を統一する
2. User Story 2 で `/runpod-volume` 前提の storage layout を統一する
3. User Story 3 で旧 local 導線を廃止し、一次情報を整理する
4. Phase 6 で contract と quickstart の整合性を確認する
5. `OLLAMA_PULL_MODELS` と custom node 前提が維持されていることを確認する

### Parallel Team Strategy

1. 1 人が `compose.yml` と `.env.example` を担当する
2. 1 人が `README.md` と `comfyui/runpod/README.md` を担当する
3. 1 人が `specs/044-local-runpod-image/quickstart.md` と contract 整合確認を担当する

---

## Notes

- `[P]` は別ファイル中心で衝突しにくいタスクにのみ付与している
- 各 user story は `compose.yml`、`.env.example`、README 群、quickstart のどこを更新するかを明示している
- 自動テスト追加は spec で要求されていないため、各 story は手動検証手順ベースで独立検証する

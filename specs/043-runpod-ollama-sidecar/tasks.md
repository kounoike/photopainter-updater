# タスク: RunPod Ollama sidecar

**Input**: `/specs/043-runpod-ollama-sidecar/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3`, `US4`)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: RunPod 用 assets の配置先と文書の入口を整える

- [X] T001 `comfyui/runpod/` 配下の RunPod 用構成方針を [plan.md](/workspaces/photopainter-updater/specs/043-runpod-ollama-sidecar/plan.md) に合わせて確認する
- [X] T002 `comfyui/runpod/README.md` の初期骨子を作成し RunPod 用 assets の責務を記述する
- [X] T003 [P] `README.md` の ComfyUI / RunPod 記述を確認し既存ローカル Compose 導線を壊さない更新ポイントを整理する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に先行する RunPod worker 共通基盤を用意する

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [X] T004 `comfyui/runpod/Dockerfile` を作成し upstream `worker-comfyui` base 継承と Ollama 導入の骨組みを追加する
- [X] T005 [P] `comfyui/runpod/start-ollama-worker.sh` を作成し wrapper start script の基本フローを実装する
- [X] T006 [P] `comfyui/runpod/README.md` に必要な env 変数、localhost 制約、Network Volume 前提の共通定義を追加する
- [X] T007 `specs/043-runpod-ollama-sidecar/contracts/runpod-ollama-runtime-contract.md` と整合するよう `comfyui/runpod/README.md` の用語を統一する
- [X] T008 Allowed Scope / Forbidden Scope の実装境界を `comfyui/runpod/README.md` に明記し既存 `comfyui/Dockerfile` と `compose.yml` を不変更前提に固定する

**Checkpoint**: RunPod 用 assets の共通基盤完了後に user story 実装へ進む

---

## Phase 3: User Story 1 - ComfyUI コンテナ内で Ollama を常駐利用する (Priority: P1)

**Goal**: RunPod worker 起動時に Ollama を localhost sidecar として起動し、upstream worker に委譲する

**Independent Test**: RunPod 向け image を起動し、container 内 `curl http://127.0.0.1:11434/api/version` と ComfyUI worker 起動の両方が追加の手動起動なしで成功すること

### Verification for User Story 1

- [X] T009 [US1] [quickstart.md](/workspaces/photopainter-updater/specs/043-runpod-ollama-sidecar/quickstart.md) に localhost readiness 確認手順を具体化する
- [X] T010 [US1] `comfyui/runpod/README.md` に upstream `/start.sh` へ委譲する確認手順を記述する

### Implementation for User Story 1

- [X] T011 [US1] `comfyui/runpod/Dockerfile` に wrapper script 配置と起動入口を追加する
- [X] T012 [US1] `comfyui/runpod/start-ollama-worker.sh` に `ollama serve` の background 起動と readiness wait を実装する
- [X] T013 [US1] `comfyui/runpod/start-ollama-worker.sh` に localhost bind 前提と upstream `/start.sh` への `exec` 委譲を実装する
- [X] T014 [US1] `comfyui/runpod/README.md` に localhost 限定利用と外部公開しない前提を追記する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - 永続領域のモデルを再利用する (Priority: P2)

**Goal**: `/runpod-volume` 利用時は永続保存、未接続時は一時領域フォールバックで起動継続する

**Independent Test**: `/runpod-volume` 相当の bind mount あり・なし両方で起動し、保存先モードと再利用可否をログと手順から判別できること

### Verification for User Story 2

- [X] T015 [US2] [quickstart.md](/workspaces/photopainter-updater/specs/043-runpod-ollama-sidecar/quickstart.md) に `/runpod-volume` あり・なし両ケースの確認手順を記述する
- [X] T016 [US2] `comfyui/runpod/README.md` に RunPod Endpoint 側で Network Volume を接続する前提と fallback 動作を追記する

### Implementation for User Story 2

- [X] T017 [US2] `comfyui/runpod/start-ollama-worker.sh` に `/runpod-volume` 可用性判定と `OLLAMA_MODELS` 解決処理を実装する
- [X] T018 [US2] `comfyui/runpod/start-ollama-worker.sh` に persistent / ephemeral モードをログへ出力する処理を実装する
- [X] T019 [US2] `comfyui/runpod/README.md` に永続領域再利用と一時領域フォールバックの運用差分を記載する

**Checkpoint**: User Story 2 が独立して検証可能であること

---

## Phase 5: User Story 3 - pull 対象モデルを運用設定で切り替える (Priority: P3)

**Goal**: 単一 env 値のカンマ区切り一覧で事前 pull モデルを制御し、pull 失敗は warning で継続する

**Independent Test**: model 一覧 env を変更して再起動し、対象 model の選別、空一覧、pull failure warning を判別できること

### Verification for User Story 3

- [X] T020 [US3] [quickstart.md](/workspaces/photopainter-updater/specs/043-runpod-ollama-sidecar/quickstart.md) に `OLLAMA_PULL_MODELS` 設定例と failure 確認手順を追加する
- [X] T021 [US3] `comfyui/runpod/README.md` にカンマ区切り一覧、空一覧、warning 継続の契約を記述する

### Implementation for User Story 3

- [X] T022 [US3] `comfyui/runpod/start-ollama-worker.sh` に model 一覧の trim / split / 重複除去処理を実装する
- [X] T023 [US3] `comfyui/runpod/start-ollama-worker.sh` に `ollama pull` 実行と reused / pulled / failed のログ出力を実装する
- [X] T024 [US3] `comfyui/runpod/start-ollama-worker.sh` に pull failure 時も worker 起動継続する制御を実装する

**Checkpoint**: User Story 3 が独立して検証可能であること

---

## Phase 6: User Story 4 - ローカルで擬似検証する (Priority: P3)

**Goal**: RunPod 本番前にローカル Docker で worker image の挙動を擬似検証できるようにする

**Independent Test**: ローカル Docker で image build、`/runpod-volume` bind mount あり・なし起動、worker API への payload 送信の 3 系統が手順どおり再現できること

### Verification for User Story 4

- [X] T025 [US4] [quickstart.md](/workspaces/photopainter-updater/specs/043-runpod-ollama-sidecar/quickstart.md) にローカル build / run / payload 送信手順を具体化する
- [X] T026 [US4] `comfyui/runpod/README.md` に upstream `worker-comfyui` development docs ベースのローカル確認フローを記述する

### Implementation for User Story 4

- [X] T027 [US4] `comfyui/runpod/README.md` に `/runpod-volume` bind mount あり・なしの `docker run` 例を追加する
- [X] T028 [US4] `README.md` に RunPod 用 image build とローカル擬似検証への導線を追加する
- [X] T029 [US4] `specs/043-runpod-ollama-sidecar/quickstart.md` を最終実装に合わせて更新し本番確認とローカル確認の境界を明確化する

**Checkpoint**: User Story 4 が独立して検証可能であること

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: story 横断の整合確認と仕上げ

- [X] T030 [P] `specs/043-runpod-ollama-sidecar/quickstart.md` と `comfyui/runpod/README.md` と `README.md` の用語・env 名・手順順序を同期する
- [X] T031 `comfyui/runpod/start-ollama-worker.sh` と `comfyui/runpod/Dockerfile` の shellcheck / 実行性観点を見直す
- [X] T032 `specs/043-runpod-ollama-sidecar/quickstart.md` の手順を使った手動検証結果を feature 文書へ反映する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Phase 7)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 完了後に開始する
- **User Story 3 (P3)**: User Story 1 と User Story 2 完了後に開始する
- **User Story 4 (P3)**: User Story 1 から 3 の主要手順確定後に開始する

### Within Each User Story

- 手順文書タスクを先に整備し、実装後に最終内容へ更新する
- `comfyui/runpod/start-ollama-worker.sh` の挙動追加は US1 → US2 → US3 の順に積み上げる
- RunPod 用 README と root README は各 story の実装内容に合わせて更新する
- 検証タスクを省略しない

### Parallel Opportunities

- `[P]` 付き Setup / Foundational / Polish タスクは並列実行可能
- User Story 1 の文書確認タスク T009 と T010 は並列実行可能
- User Story 4 の文書整備タスク T025 と T026 は並列実行可能

---

## Parallel Example: User Story 1

```bash
Task: "localhost readiness 確認手順を specs/043-runpod-ollama-sidecar/quickstart.md に具体化する"
Task: "upstream /start.sh へ委譲する確認手順を comfyui/runpod/README.md に記述する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. User Story 1 を独立検証する
5. RunPod 用 worker が Ollama sidecar を前置起動できることを確認する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して localhost sidecar 起動を独立検証する
3. User Story 2 を追加して永続領域 / 一時領域の切替を独立検証する
4. User Story 3 を追加して model pull 設定と warning 継続を独立検証する
5. User Story 4 を追加してローカル擬似検証手順を固める

### Parallel Team Strategy

1. チームで Setup + Foundational を完了する
2. US1 実装後に、1 人は runtime script 拡張、別の 1 人は RunPod README / quickstart 更新を担当する
3. 最後にローカル擬似検証手順をまとめて story 単位で独立確認する

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために必須
- 実装ファイルは `comfyui/runpod/` 配下へ閉じ込め、既存ローカル Compose 導線への副作用を避ける

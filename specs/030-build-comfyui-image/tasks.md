# タスク: ComfyUI 自前イメージ構築

**Input**: `/specs/030-build-comfyui-image/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: ComfyUI 自前 build へ向けた作業入口を揃える

- [ ] T001 既存 `compose.yml`、`.env.example`、`README.md` の ComfyUI 現行導線を確認し、変更境界を `specs/030-build-comfyui-image/plan.md` と照合する
- [ ] T002 `comfyui/` 配下に build 資産を置く前提で `comfyui/Dockerfile` と必要な補助ファイルの配置方針を確定する
- [ ] T003 [P] `specs/030-build-comfyui-image/quickstart.md` の build・restart・recreate 検証導線を実装後確認に使える形へ更新準備する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story が依存する ComfyUI build 基盤を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T004 `comfyui/Dockerfile` を新規作成し、pinned upstream runtime を土台にした repo 管理 ComfyUI image build を定義する
- [ ] T005 [P] `compose.yml` の `comfyui` service を `image:` 直指定から `build:` 利用へ切り替え、既存 GPU・network・healthcheck・depends_on 条件を維持する
- [ ] T006 [P] `compose.yml` の `comfyui` volumes と環境変数を見直し、`${COMFYUI_DATA_DIR:-./comfyui-data}` と repo 管理 custom node 導線の互換条件を維持する
- [ ] T007 `.env.example` の ComfyUI 設定コメントを self-build 運用前提に更新し、既存 `COMFYUI_PORT`、`COMFYUI_DATA_DIR`、`COMFYUI_CLI_ARGS` の入口を維持する
- [ ] T008 Allowed Scope / Forbidden Scope に沿って変更対象を `compose.yml`、`.env.example`、`README.md`、`comfyui/` 配下、関連 feature 文書に限定することを確認する

**Checkpoint**: repo 管理 ComfyUI image を build できる基盤が揃っていること

---

## Phase 3: User Story 1 - 同じ ComfyUI 環境を再起動後も維持したい (Priority: P1)

**Goal**: 再起動と再作成のあとでも、同じ ComfyUI 利用状態へ戻せるようにする

**Independent Test**: `docker compose build comfyui`、`docker compose up -d comfyui`、`docker compose restart comfyui`、`docker compose down && docker compose up -d comfyui` を順に実施し、同じ UI 到達確認と node 利用確認を再現できれば完了。

### Verification for User Story 1

- [ ] T009 [US1] `specs/030-build-comfyui-image/quickstart.md` に build、起動、restart、recreate の確認手順を完成形で記載する
- [ ] T010 [US1] `specs/030-build-comfyui-image/contracts/comfyui-self-build-runtime-contract.md` の build 入口・起動入口・Web UI 契約に沿って手動検証観点を整合確認する

### Implementation for User Story 1

- [ ] T011 [US1] `comfyui/Dockerfile` に runtime 成立に必要な repo 管理初期構成を反映し、コンテナ再作成後も同じ起点へ戻れるようにする
- [ ] T012 [US1] `compose.yml` の `comfyui` service 起動定義を調整し、self-build image を使った `up`、`restart`、`down && up` で同じ service 名と到達 URL を維持する
- [ ] T013 [US1] `README.md` の ComfyUI セクションを pull 前提から build・起動・再起動・再作成前提へ更新する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - repo 管理の構成として再現したい (Priority: P2)

**Goal**: 別環境でも repo 内の構成だけで ComfyUI 環境を再現できるようにする

**Independent Test**: 新しい clone 済み環境で `.env.example` と `README.md` / `quickstart.md` の案内だけを使い、事前修正済みコンテナなしで ComfyUI image build と起動判断ができれば完了。

### Verification for User Story 2

- [ ] T014 [US2] `specs/030-build-comfyui-image/quickstart.md` に新規 clone 環境からの build 導線と困ったときの確認先を明記する
- [ ] T015 [US2] `specs/030-build-comfyui-image/data-model.md` の build 入力と運用導線が、repo 管理 Dockerfile と compose build 実装に一致していることを確認する

### Implementation for User Story 2

- [ ] T016 [US2] `comfyui/Dockerfile` と必要な補助ファイルを整え、repo 内 build context だけで ComfyUI image を再生成できるようにする
- [ ] T017 [US2] `compose.yml` と `.env.example` を更新し、`docker compose build comfyui` と `docker compose up -d comfyui` の入口を repo 管理構成へ揃える
- [ ] T018 [US2] `README.md` と `specs/030-build-comfyui-image/quickstart.md` を更新し、事前構築済み container に依存しない再現手順へ統一する

**Checkpoint**: User Story 2 が独立して検証可能であること

---

## Phase 5: User Story 3 - 既存のデータと導線を壊したくない (Priority: P3)

**Goal**: 既存の永続データ、custom node、関連 compose 導線を壊さずに self-build へ移行する

**Independent Test**: 既存の `${COMFYUI_DATA_DIR:-./comfyui-data}` を保持したまま self-build 構成へ切り替え、モデル、output、repo 管理 custom node、利用者 custom node、Ollama 共存前提が維持されることを確認できれば完了。

### Verification for User Story 3

- [ ] T019 [US3] `specs/030-build-comfyui-image/contracts/comfyui-self-build-runtime-contract.md` の storage 契約と documentation 契約に沿って保持対象を確認する
- [ ] T020 [US3] `specs/030-build-comfyui-image/quickstart.md` に既存データ移行不要と custom node 維持確認の手順を反映する

### Implementation for User Story 3

- [ ] T021 [US3] `compose.yml` の `comfyui` volumes と repo 管理 custom node mount を調整し、`${COMFYUI_DATA_DIR:-./comfyui-data}` 配下の既存データと node 導線を維持する
- [ ] T022 [US3] `README.md` と `comfyui/custom_node/comfyui-photopainter-custom/README.md` を更新し、self-build 後も repo 管理 custom node と既存データ保存先が継続利用できることを明示する
- [ ] T023 [US3] `compose.yml` の `comfyui` service が既存 `ollama`、`server`、`ai-toolkit` と共存する前提を壊していないか確認し、必要な文言を `README.md` に反映する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 実装結果を横断的に仕上げる

- [ ] T024 [P] `specs/030-build-comfyui-image/plan.md`、`research.md`、`data-model.md`、`quickstart.md` の最終整合を確認し、差分があれば文書を更新する
- [ ] T025 `docker compose config`、`docker compose build comfyui`、`docker compose up -d comfyui` の実施結果を確認し、失敗時は関連ファイルを修正する
- [ ] T026 `README.md`、`comfyui/custom_node/comfyui-photopainter-custom/README.md`、`specs/030-build-comfyui-image/quickstart.md` の手順文を最終確認し、重複や矛盾を解消する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の self-build 導線が成立した後に進めると安全
- **User Story 3 (P3)**: User Story 1 と 2 の起動導線が固まった後に既存データ互換確認へ進む

### Within Each User Story

- 手動確認手順を先に固め、その後に compose / Dockerfile / 文書を更新する
- `compose.yml` と `comfyui/Dockerfile` の self-build 基盤を先に整え、README 更新はその後に行う
- 既存データや custom node の互換性確認は最後にまとめて実施する

### Parallel Opportunities

- `[P]` 付き Setup / Foundational タスクは並列実行可能
- Phase 2 では `compose.yml` と `.env.example` の準備を並列化しやすい
- Polish では feature 文書整合確認と最終手順文確認を並列化できる

---

## Parallel Example: User Story 1

```bash
Task: "specs/030-build-comfyui-image/quickstart.md に build・restart・recreate 手順を記載する"
Task: "specs/030-build-comfyui-image/contracts/comfyui-self-build-runtime-contract.md の検証観点を確認する"
```

---

## Parallel Example: User Story 3

```bash
Task: "README.md と comfyui/custom_node/comfyui-photopainter-custom/README.md を self-build 前提へ更新する"
Task: "compose.yml の volumes と custom node mount を既存データ互換前提で見直す"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. build / up / restart / recreate の導線を独立検証する

### Incremental Delivery

1. self-build 基盤を整える
2. User Story 1 で再起動・再作成耐性を成立させる
3. User Story 2 で repo 管理構成としての再現性を固める
4. User Story 3 で既存データ・導線互換を仕上げる
5. Polish で文書と検証結果を揃える

### Parallel Team Strategy

1. 1 人が `comfyui/Dockerfile` と `compose.yml` を担当する
2. 1 人が `README.md`、custom node README、feature quickstart を担当する
3. 最後に合流して build / restart / recreate / 互換確認を実施する

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために必須
- 各 task は `compose.yml`、`comfyui/Dockerfile`、README 群、feature 成果物のどれを触るかを明記している

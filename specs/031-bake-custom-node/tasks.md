# タスク: ComfyUI custom node 同梱コンテナ

**Input**: `/specs/031-bake-custom-node/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: custom node 同梱へ向けた変更境界と前提を揃える

- [ ] T001 `specs/031-bake-custom-node/spec.md`、`plan.md`、`research.md` を確認し、repo 管理 custom node を image に焼き込み、追加 custom node は維持対象外とする前提を実装境界として確定する
- [ ] T002 `compose.yml`、`comfyui/Dockerfile`、`comfyui/entrypoint.sh`、`README.md`、`specs/030-build-comfyui-image/quickstart.md`、`comfyui/custom_node/comfyui-photopainter-custom/README.md` の現行 custom node 導線を確認し、変更対象を整理する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story が依存する baked-in custom node 基盤を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T003 `comfyui/Dockerfile` に repo 管理 custom node を image へ copy する build 手順を追加し、ComfyUI image に baked-in される起点を定義する
- [ ] T004 [P] `compose.yml` の `comfyui` service から repo 管理 custom node bind mount と `${COMFYUI_DATA_DIR}/custom_nodes` bind mount を外し、既存 model / output / input / user 導線を維持する
- [ ] T005 [P] `comfyui/entrypoint.sh` を見直し、repo 管理 custom node を baked-in 前提で利用する最小起動処理へ揃える
- [ ] T006 [P] `.env.example` の ComfyUI コメントを更新し、custom node は image 同梱・追加 custom node は維持対象外であることを明示する
- [ ] T007 [P] `specs/031-bake-custom-node/contracts/comfyui-baked-custom-node-contract.md` と `specs/031-bake-custom-node/data-model.md` の runtime / storage 契約に沿って compose・Dockerfile・entrypoint の責務を整合確認する
- [ ] T008 Allowed Scope / Forbidden Scope に沿って変更対象を `compose.yml`、`comfyui/`、`.env.example`、README 群、feature 文書に限定することを確認する

**Checkpoint**: repo 管理 custom node を image に焼き込む基盤が揃っていること

---

## Phase 3: User Story 1 - custom node 入りのコンテナをそのまま起動したい (Priority: P1)

**Goal**: `docker compose build comfyui` と `docker compose up -d comfyui` だけで baked-in custom node を利用可能にする

**Independent Test**: `docker compose build comfyui` 後に `docker compose up -d comfyui` を実行し、`PhotoPainter PNG POST` が起動直後から選択可能であれば完了。

### Verification for User Story 1

- [ ] T009 [US1] `specs/031-bake-custom-node/quickstart.md` に build・起動・node 確認・失敗時の最初の確認先を完成形で記載する
- [ ] T010 [US1] `specs/031-bake-custom-node/contracts/comfyui-baked-custom-node-contract.md` の build / start 契約と、`PhotoPainter PNG POST` の確認観点を手動検証手順として整合確認する

### Implementation for User Story 1

- [ ] T011 [US1] `comfyui/Dockerfile` に repo 管理 custom node copy と起動時 discovery に必要な配置を実装し、build 後の image に `PhotoPainter PNG POST` が含まれるようにする
- [ ] T012 [US1] `compose.yml` の `comfyui` service を更新し、repo mount なしで baked-in custom node を使う起動定義へ切り替える
- [ ] T013 [US1] `README.md` の ComfyUI セクションを更新し、repo 管理 custom node は image 同梱であることと build 導線を案内する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - 再作成しても同じ custom node 構成を保ちたい (Priority: P2)

**Goal**: restart / recreate 後も同じ baked-in custom node 構成へ復帰できるようにする

**Independent Test**: `docker compose restart comfyui` と `docker compose down && docker compose up -d comfyui` を実施し、その都度 `PhotoPainter PNG POST` が見えれば完了。

### Verification for User Story 2

- [ ] T014 [US2] `specs/031-bake-custom-node/quickstart.md` に restart / recreate の確認手順と rebuild が必要な条件を記載する
- [ ] T015 [US2] `specs/031-bake-custom-node/data-model.md` の runtime state と運用入口が restart / recreate 手順に一致していることを確認する

### Implementation for User Story 2

- [ ] T016 [US2] `comfyui/entrypoint.sh` を調整し、restart / recreate 後も baked-in custom node を同じ探索先で読み込めるようにする
- [ ] T017 [US2] `README.md` と `specs/030-build-comfyui-image/quickstart.md` を更新し、baked-in custom node 前提の restart / recreate 導線へ統一する
- [ ] T018 [US2] `comfyui/custom_node/comfyui-photopainter-custom/README.md` を更新し、runtime 配置説明を build 時同梱と rebuild 前提へ切り替える

**Checkpoint**: User Story 2 が独立して検証可能であること

---

## Phase 5: User Story 3 - baked-in node の運用条件を誤解したくない (Priority: P3)

**Goal**: 利用者が repo 管理 node の rebuild 条件と追加 custom node 非永続を文書から判断できるようにする

**Independent Test**: README と quickstart を読み、repo 管理 custom node は baked-in、追加 custom node は再作成で維持されないことを判断できれば完了。

### Verification for User Story 3

- [ ] T019 [US3] `specs/031-bake-custom-node/contracts/comfyui-baked-custom-node-contract.md` の documentation contract に沿って、運用条件の説明観点を確認する
- [ ] T020 [US3] `specs/031-bake-custom-node/quickstart.md` に追加 custom node 非永続と troubleshooting を明記する

### Implementation for User Story 3

- [ ] T021 [US3] `.env.example` と `README.md` を更新し、追加 custom node は維持対象外であることを明示する
- [ ] T022 [US3] `specs/030-build-comfyui-image/quickstart.md` と `comfyui/custom_node/comfyui-photopainter-custom/README.md` を更新し、旧 mount 前提の説明を除去する
- [ ] T023 [US3] `compose.yml` の custom node 関連 volume を最終確認し、repo 管理 node baked-in と既存 model / output / input / user mount の境界を維持する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 実装結果を横断的に仕上げる

- [ ] T024 [P] `specs/031-bake-custom-node/plan.md`、`research.md`、`data-model.md`、`quickstart.md`、`contracts/comfyui-baked-custom-node-contract.md` の最終整合を確認し、差分があれば文書を更新する
- [ ] T025 `docker compose config`、`docker compose build comfyui`、`docker compose up -d comfyui`、`docker compose restart comfyui`、`docker compose down && docker compose up -d comfyui`、troubleshooting 導線の実施結果を確認し、失敗時は関連ファイルを修正する
- [ ] T026 `README.md`、`specs/030-build-comfyui-image/quickstart.md`、`comfyui/custom_node/comfyui-photopainter-custom/README.md`、`specs/031-bake-custom-node/quickstart.md` の手順文を最終確認し、焼き込み前提と非永続前提の矛盾を解消する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の baked-in 起動導線成立後に進めると安全
- **User Story 3 (P3)**: User Story 1 と 2 の導線が固まった後に文書運用条件を仕上げる

### Within Each User Story

- 手動確認手順を先に固め、その後に Dockerfile / compose / entrypoint / 文書を更新する
- `comfyui/Dockerfile` と `compose.yml` の baked-in 基盤を先に整え、README 更新はその後に行う
- restart / recreate の再現確認と運用手順の最終整合は最後にまとめて実施する

### Parallel Opportunities

- `[P]` 付き Foundational タスクは並列実行可能
- Phase 2 では `compose.yml`、`.env.example`、契約整合確認を並列化しやすい
- Polish では feature 文書整合確認と最終手順文確認を並列化できる

---

## Parallel Example: User Story 1

```bash
Task: "specs/031-bake-custom-node/quickstart.md に build・起動・node確認手順を記載する"
Task: "specs/031-bake-custom-node/contracts/comfyui-baked-custom-node-contract.md の build/start 契約を確認する"
```

---

## Parallel Example: User Story 3

```bash
Task: ".env.example と README.md に追加 custom node 非永続を反映する"
Task: "specs/030-build-comfyui-image/quickstart.md と custom node README の旧 mount 前提説明を除去する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. baked-in custom node 起動導線を独立検証する

### Incremental Delivery

1. baked-in custom node 基盤を整える
2. User Story 1 で build / up だけで node が見える状態を成立させる
3. User Story 2 で restart / recreate の再現性を固める
4. User Story 3 で利用者向け運用条件の誤解を防ぐ
5. Polish で文書と検証結果を揃える

### Parallel Team Strategy

1. 1 人が `comfyui/Dockerfile`、`compose.yml`、`comfyui/entrypoint.sh` を担当する
2. 1 人が README、quickstart、custom node README、feature 文書を担当する
3. 最後に合流して build / restart / recreate / 文書整合確認を実施する

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために必須
- 各 task は `compose.yml`、`comfyui/Dockerfile`、`comfyui/entrypoint.sh`、README 群、feature 成果物のどれを触るかを明記している

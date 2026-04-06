# タスク: ComfyUI GHCR 公開

**Input**: `/specs/046-comfyui-ghcr-publish/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3`)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 既存 release publish 導線と文書の更新対象を確定する

- [ ] T001 既存 release publish target schema を `.github/workflows/release-image-publish.yml` と `.github/release-image-publish.yml` で確認する
- [ ] T002 [P] root README の `Release Images` 更新箇所を `README.md` で確認する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に先行する publish target 前提を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T003 `.github/release-image-publish.yml` の target schema を `specs/046-comfyui-ghcr-publish/contracts/release-image-publish-contract.md` と整合する形で確認する
- [ ] T004 [P] `specs/046-comfyui-ghcr-publish/quickstart.md` の手動検証手順を `.github/workflows/release-image-publish.yml` と `.github/release-image-publish.yml` の現行導線に合わせて補正する

**Checkpoint**: publish target schema と検証導線が固まり、story 実装へ進める

---

## Phase 3: User Story 1 - release publish で ComfyUI image を公開する (Priority: P1)

**Goal**: release publish で ComfyUI image も GHCR へ build / push されるようにする

**Independent Test**: `.github/release-image-publish.yml` で `comfyui` target が enabled になり、`./comfyui` と `./comfyui/runpod/Dockerfile` を使う GHCR 公開設定を確認できる

### Verification for User Story 1

- [ ] T005 [US1] `comfyui` target の build context / Dockerfile / image repository を `specs/046-comfyui-ghcr-publish/quickstart.md` に明記する

### Implementation for User Story 1

- [ ] T006 [US1] ComfyUI publish target を `.github/release-image-publish.yml` に追加する
- [ ] T007 [US1] `comfyui` target の image title label を `.github/release-image-publish.yml` に追加して `photopainter-comfyui` に揃える
- [ ] T008 [US1] `release` published event で matrix publish が継続することを `.github/workflows/release-image-publish.yml` で確認し、必要な最小調整のみを反映する

**Checkpoint**: ComfyUI image が release publish の公開対象として独立確認できる

---

## Phase 4: User Story 2 - server と同じ設定方式で target を管理する (Priority: P2)

**Goal**: ComfyUI の公開設定を既存 server と同じ target 一覧方式で管理する

**Independent Test**: `.github/release-image-publish.yml` だけで `server` と `comfyui` の両 target の有効化、build context、Dockerfile、image repository 名を判断できる

### Verification for User Story 2

- [ ] T009 [US2] target 一覧方式の確認観点を `specs/046-comfyui-ghcr-publish/quickstart.md` に追加する

### Implementation for User Story 2

- [ ] T010 [US2] `server` と `comfyui` が同一 schema で並ぶよう `.github/release-image-publish.yml` の target 定義順と項目構造を整理する
- [ ] T011 [US2] ComfyUI 専用分岐を追加していないことを `.github/workflows/release-image-publish.yml` で確認し、必要なら target 汎用性を保つ最小修正を反映する

**Checkpoint**: workflow 本体を大きく変えずに target 一覧方式の保守性を維持できる

---

## Phase 5: User Story 3 - 公開手順を文書から判断できる (Priority: P3)

**Goal**: README と quickstart から ComfyUI image の公開契機と公開先を判断できるようにする

**Independent Test**: `README.md` を読んだ保守者が `photopainter-comfyui` の公開契機、公開先、tag 規則、確認場所を判断できる

### Verification for User Story 3

- [ ] T012 [US3] ComfyUI image の公開契機と確認手順を `specs/046-comfyui-ghcr-publish/quickstart.md` に整理する

### Implementation for User Story 3

- [ ] T013 [US3] `Release Images` 節へ ComfyUI image の公開先と tag 規則を `README.md` に追加する
- [ ] T014 [US3] release publish 後に確認する GHCR package 名と Actions 導線を `README.md` に追記する

**Checkpoint**: 文書だけで ComfyUI image の公開導線を追える

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 横断確認と最終整合

- [ ] T015 [P] `.github/workflows/release-image-publish.yml` と `.github/release-image-publish.yml` の YAML 体裁を確認する
- [ ] T016 `git diff --check` と `specs/046-comfyui-ghcr-publish/quickstart.md` の手順見直しで最終差分を確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Phase 6)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能。最小の公開価値を提供する
- **User Story 2 (P2)**: User Story 1 の target 追加後に確認しやすいが、実装は同じ設定ファイル上で連続実施する
- **User Story 3 (P3)**: User Story 1 と 2 の確定後に文書化する

### Within Each User Story

- 検証タスクを先に更新し、その確認観点に沿って実装を進める
- `.github/release-image-publish.yml` の target 定義更新を中心に進める
- `.github/workflows/release-image-publish.yml` は target 汎用性維持の範囲でのみ触る
- README 更新は publish target と確認導線の確定後に行う

### Parallel Opportunities

- T002 と T003 は並列実行可能
- T004 と T005 は別文書更新なので並列実行可能
- T013 と T014 は同じ `README.md` だが内容整理を先に決めてからまとめて反映するのが安全
- T015 は実装完了後に他のレビュー作業と並列で実施可能

---

## Parallel Example: User Story 1

```bash
Task: "T005 [US1] `comfyui` target の build context / Dockerfile / image repository を specs/046-comfyui-ghcr-publish/quickstart.md に明記する"
Task: "T006 [US1] ComfyUI publish target を .github/release-image-publish.yml に追加する"
```

---

## Parallel Example: User Story 2

```bash
Task: "T009 [US2] target 一覧方式の確認観点を specs/046-comfyui-ghcr-publish/quickstart.md に追加する"
Task: "T011 [US2] ComfyUI 専用分岐を追加していないことを .github/workflows/release-image-publish.yml で確認し、必要なら target 汎用性を保つ最小修正を反映する"
```

---

## Parallel Example: User Story 3

```bash
Task: "T012 [US3] ComfyUI image の公開契機と確認手順を specs/046-comfyui-ghcr-publish/quickstart.md に整理する"
Task: "T013 [US3] `Release Images` 節へ ComfyUI image の公開先と tag 規則を README.md に追加する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. `comfyui` target の有効化と build 入力を独立確認する

### Incremental Delivery

1. Setup + Foundational で schema と検証観点を固める
2. User Story 1 で ComfyUI publish target を追加する
3. User Story 2 で target 一覧方式と workflow 汎用性を確認する
4. User Story 3 で README と quickstart の利用導線を仕上げる
5. Polish で YAML と差分体裁を確認する

### Parallel Team Strategy

1. 1 人が `.github/release-image-publish.yml` の target 追加を担当する
2. もう 1 人が `README.md` と `specs/046-comfyui-ghcr-publish/quickstart.md` の文書導線を担当する
3. 最後に workflow 汎用性確認と静的確認をまとめて行う

---

## Notes

- `[P]` は別ファイルまたは独立確認観点に基づく並列タスクのみ付与する
- User Story 2 と 3 は User Story 1 の公開 target 確定後に実施すると手戻りが少ない
- GHCR 上の live publish 確認は implement phase の最終検証として扱う

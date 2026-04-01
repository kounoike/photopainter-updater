# タスク: Z-Image キャラクター LoRA 試作基盤

**Input**: `/specs/024-zimage-character-lora/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: trial 学習用の作業場所と共通文書の雛形を整える

- [ ] T001 `docs/zimage-lora-trial/README.md` を作成し、trial 学習資産の配置方針と feature 成果物への導線を定義する
- [ ] T002 `scripts/zimage-lora/` ディレクトリを作成し、trial 学習用 script / config / dataset template の格納方針を整理する
- [ ] T003 [P] `docs/zimage-lora-trial/.gitignore` と `scripts/zimage-lora/.gitignore` を追加し、キャッシュ・出力物・ローカル secrets を除外する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story が依存する共通設定と trial contract をコード化する

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T004 `scripts/zimage-lora/configs/trial-12gb.env.example` に 12GB trial 向け共通環境変数雛形を定義する
- [ ] T005 [P] `scripts/zimage-lora/configs/trial-12gb.json` に `model_family=z-image` / `model_flavour=turbo` / 量子化 / offload を含む trial 学習設定雛形を定義する
- [ ] T006 [P] `scripts/zimage-lora/configs/multidatabackend.trial.json` に 少数画像 dataset と validation 用 cache の dataloader 雛形を定義する
- [ ] T007 `scripts/zimage-lora/validate-trial-layout.sh` を作成し、Reference Image Set と Trial Training Profile の最小前提を検証できるようにする
- [ ] T008 `docs/zimage-lora-trial/contract.md` に [zimage-trial-lora-contract.md](/home/kounoike/ghq/github.com/kounoike/photopainter-updater/specs/024-zimage-character-lora/contracts/zimage-trial-lora-contract.md) の実装境界と運用上の注意を同期する

**Checkpoint**: 基盤完了後に user story 実装へ進む

---

## Phase 3: User Story 1 - 少数画像で LoRA 試作を実行する (Priority: P1)

**Goal**: 12GB 前提のローカル環境で trial 学習を開始し、LoRA 成果物を出力できるようにする

**Independent Test**: 最小参照画像セットと trial 設定を使って学習を開始し、LoRA 成果物と validation 出力ディレクトリが作成されることを確認する

### Verification for User Story 1

- [ ] T009 [US1] `specs/024-zimage-character-lora/quickstart.md` に 12GB trial 学習の手動確認手順と完了判定を実装結果に合わせて追記する
- [ ] T010 [US1] `docs/zimage-lora-trial/manual-checklist.md` に 学習開始、OOM 時の縮退、成果物確認の手動チェック項目を記載する

### Implementation for User Story 1

- [ ] T011 [P] [US1] `scripts/zimage-lora/train-trial.sh` を実装し、trial-12gb 設定と dataloader 設定を使って SimpleTuner 学習を起動できるようにする
- [ ] T012 [P] [US1] `scripts/zimage-lora/lib/common.sh` を実装し、環境変数読込、出力先解決、依存確認の共通処理をまとめる
- [ ] T013 [US1] `scripts/zimage-lora/train-trial.sh` に `--quantize_via=cpu`、`gradient_checkpointing`、group offload の縮退分岐を追加する
- [ ] T014 [US1] `docs/zimage-lora-trial/README.md` に 学習開始コマンド、必要ディレクトリ、生成物の保存先を記載する
- [ ] T015 [US1] `scripts/zimage-lora/train-trial.sh` と `docs/zimage-lora-trial/manual-checklist.md` に 失敗時ログ採取と再実行時の確認ポイントを追加する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - 少数画像向けのデータ準備方針を決める (Priority: P2)

**Goal**: 少数画像 trial で必要なデータ条件と caption 条件を、次のキャラクターでも再利用できる形で定義する

**Independent Test**: データ準備ガイドに従って画像セットと caption を用意し、layout validation を通過できることを確認する

### Verification for User Story 2

- [ ] T016 [US2] `docs/zimage-lora-trial/data-prep-checklist.md` に 画像選定、除外基準、軽い前処理、caption 方針の手動確認項目を記載する
- [ ] T017 [US2] `specs/024-zimage-character-lora/quickstart.md` に dataset 準備と `repeats` 調整の確認手順を同期する

### Implementation for User Story 2

- [ ] T018 [P] [US2] `docs/zimage-lora-trial/data-prep.md` を作成し、高品質少数画像、恒常特徴優先、背景/ポーズ抑制の基準を文書化する
- [ ] T019 [P] [US2] `scripts/zimage-lora/templates/character-caption.txt` に 恒常特徴中心の caption template を追加する
- [ ] T020 [P] [US2] `scripts/zimage-lora/templates/dataset-layout.md` に 参照画像ディレクトリ構成、caption 配置、validation prompt 配置の雛形を追加する
- [ ] T021 [US2] `scripts/zimage-lora/validate-trial-layout.sh` に caption と画像枚数の妥当性確認を追加する
- [ ] T022 [US2] `docs/zimage-lora-trial/data-prep.md` と `scripts/zimage-lora/templates/dataset-layout.md` に 少数画像不足時の `repeats` 調整指針を追記する

**Checkpoint**: User Story 2 が独立して検証可能であること

---

## Phase 5: User Story 3 - 将来の自動生成フローへ接続可能にする (Priority: P3)

**Goal**: 学習成果物の最小再利用確認を行い、将来の自然言語ベース生成フローへ渡す責務分界を整理する

**Independent Test**: 学習済み LoRA を最小限の推論手順で再利用し、validation 画像と統合前提メモを残せることを確認する

### Verification for User Story 3

- [ ] T023 [US3] `docs/zimage-lora-trial/reuse-validation-checklist.md` に LoRA 再利用確認、validation 画像確認、継続可否判断の手順を記載する
- [ ] T024 [US3] `specs/024-zimage-character-lora/quickstart.md` に 最小再利用確認と将来統合前提の確認手順を同期する

### Implementation for User Story 3

- [ ] T025 [P] [US3] `scripts/zimage-lora/validate-reuse.sh` を実装し、LoRA 成果物、validation prompt、出力先を受けて再利用確認を実行できるようにする
- [ ] T026 [P] [US3] `scripts/zimage-lora/templates/validation-prompts.txt` に 同一キャラクター識別可否を判断する最小 prompt 集合を追加する
- [ ] T027 [US3] `docs/zimage-lora-trial/integration-boundary.md` を作成し、character 恒常要素と scene 可変要素の責務分界、および `keep / adapt / replace` の将来拡張点を整理する
- [ ] T028 [US3] `docs/zimage-lora-trial/README.md` に 学習成果物の保存先、最小再利用確認、ComfyUI 本格統合を今回 scope 外とする注意を追記する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 文書同期、artifact 導線、横断確認を整える

- [ ] T029 [P] `specs/024-zimage-character-lora/research.md` と `docs/zimage-lora-trial/README.md` を同期し、Qwen Image Edit 検討 artifact から LoRA 方針へ切り替えた理由を整理する
- [ ] T030 `specs/024-zimage-character-lora/artifacts/comfyui/README.md` を追加し、workflow JSON / run.png の意味と参照方法を記載する
- [ ] T031 `specs/024-zimage-character-lora/quickstart.md` の手順を実際の script 名とファイル配置に合わせて最終確認する
- [ ] T032 `git status` と関連文書を確認し、Allowed Scope / Forbidden Scope 逸脱がないことを `specs/024-zimage-character-lora/plan.md` と照合する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Phase 6)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: Foundational 後に開始可能
- **User Story 3 (P3)**: User Story 1 の学習成果物を利用するため、US1 完了後に開始する

### Within Each User Story

- 手動確認手順を先に整備し、実装後にその手順で検証する
- 共通 script / template を先に整え、その後に story 固有の確認処理を追加する
- dataset 準備方針は学習実行より前に検証できるようにする
- 最小再利用確認は学習成果物が生成された後にのみ実施する

### Parallel Opportunities

- `[P]` 付き Setup / Foundational タスクは並列実行可能
- US1 の `train-trial.sh` と `common.sh`、US2 の文書と template、US3 の validation script と prompt template は別ファイルなので並列化可能
- US1 完了後、US2 の文書整備と US3 の統合前提文書は一部並列進行可能

---

## Parallel Example: User Story 1

```bash
Task: "scripts/zimage-lora/train-trial.sh を実装する"
Task: "scripts/zimage-lora/lib/common.sh を実装する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. 12GB 前提の trial 学習を開始し、成果物出力を確認する
5. 学習成立性を先に判断する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して trial 学習開始まで確認する
3. User Story 2 を追加してデータ準備の再利用性を高める
4. User Story 3 を追加して最小再利用確認と将来統合前提を整理する
5. Polish で文書・artifact 導線を整える

### Parallel Team Strategy

1. 1 人が Setup + Foundational を完了する
2. US1 実装中に別担当が US2 の文書・template を進める
3. US1 完了後に US3 の validation script と統合前提文書を進める

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために必須
- trial 学習の最小成立性確認を優先し、本格統合は後続 feature に分離する

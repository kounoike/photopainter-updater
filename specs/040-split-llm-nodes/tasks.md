# タスク: ComfyUI LLM ノード分離

**Input**: `/specs/040-split-llm-nodes/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: backend 分離の前提となる契約差分と検証観点を具体化する

- [ ] T001 `specs/040-split-llm-nodes/contracts/comfyui-node-contracts.md` と `specs/040-split-llm-nodes/quickstart.md` を見直し、旧単一ノード削除、新ノード名、3 出力契約、retry が JSON parse/schema failure 限定であることを反映する
- [ ] T002 `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に追加する node metadata 検証項目を洗い出し、旧単一ノード削除、2 ノード追加、backend 別入力差分の観点を task 実装順へ反映する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 2 ノードで共有する helper / contract / debug 基盤を整理する

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T003 `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` の共通 helper を整理し、backend 非依存の config / debug / JSON/schema / memory release ロジックを node class から切り離す
- [ ] T004 `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に共通 helper 再編後も structured output、debug 出力、memory release、retry が JSON parse/schema failure 限定であることを確認する単体検証を追加する
- [ ] T005 `comfyui/custom_node/comfyui-photopainter-custom/README.md` に backend 分離の前提、旧単一ノード削除、3 出力契約、retry 条件、debug で retry 理由を確認できることを追記する

**Checkpoint**: backend 分離後も再利用する共通 helper と debug 契約が固定されていること

---

## Phase 3: User Story 1 - Transformers ノードへ責務を集約したい (Priority: P1)

**Goal**: `transformers` 専用ノードへ `quantization_mode` と `think_mode` を集中させ、GGUF 前提入力を排除する

**Independent Test**: `transformers` 専用ノードを workflow に配置し、`model_id`、`think_mode`、`quantization_mode`、`json_output` を指定して debug 出力と最終出力を確認できること

### Verification for User Story 1

- [ ] T006 [P] [US1] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に `PhotopainterTransformersLlmGenerate` の metadata 契約を追加し、`model_file` が存在しないことと `quantization_mode` が存在することを検証する
- [ ] T007 [P] [US1] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に `transformers` 専用ノードの生成経路、`quantization_mode`、`requested_enable_thinking`、Gemma documented control、retry 理由、3 出力契約を検証するテストを追加する
- [ ] T008 [US1] `specs/040-split-llm-nodes/quickstart.md` の Transformers 移行手順を実装内容に合わせて更新し、`bnb_4bit` の手動確認例と retry/debug 確認手順を追記する

### Implementation for User Story 1

- [ ] T009 [US1] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `PhotopainterTransformersLlmGenerate` を追加し、`model_id`、`quantization_mode`、`think_mode`、`json_output`、`json_schema`、3 出力だけを持つ UI 契約を実装する
- [ ] T010 [US1] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` の debug 出力に Transformers 専用情報（`quantization_mode`、`requested_enable_thinking`、Gemma documented control、retry 理由）を維持し、旧単一ノードに依存しないよう接続する
- [ ] T011 [US1] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に `PhotoPainter LLM Generate (Transformers)` の入力と移行手順を追記する
- [ ] T012 [US1] `comfyui/Dockerfile` の依存説明と `transformers` 量子化前提を `comfyui/custom_node/comfyui-photopainter-custom/README.md` と整合させる

**Checkpoint**: `transformers` 専用ノードが単独で検証可能であること

---

## Phase 4: User Story 2 - llama-cpp ノードを GGUF 専用として使いたい (Priority: P2)

**Goal**: `llama-cpp` 専用ノードへ GGUF / `model_file` を集中させ、`think_mode` と `quantization_mode` を排除する

**Independent Test**: `llama-cpp` 専用ノードを workflow に配置し、GGUF repo と `model_file` を与えて実行できること、`think_mode` と `quantization_mode` が UI に存在しないこと

### Verification for User Story 2

- [ ] T013 [P] [US2] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に `PhotopainterLlamaCppLlmGenerate` の metadata 契約を追加し、`model_file` が必須で `think_mode` と `quantization_mode` が存在しないことを検証する
- [ ] T014 [P] [US2] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に `llama-cpp` 専用ノードの生成経路、GGUF validation、retry 理由、3 出力契約を検証するテストを追加する
- [ ] T015 [US2] `specs/040-split-llm-nodes/quickstart.md` の llama-cpp 移行手順を実装内容に合わせて更新し、retry/debug 確認手順を追記する

### Implementation for User Story 2

- [ ] T016 [US2] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `PhotopainterLlamaCppLlmGenerate` を追加し、`model_id`、`model_file`、`json_output`、`json_schema`、3 出力だけを持つ UI 契約を実装する
- [ ] T017 [US2] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` で `llama-cpp` 専用ノードから `think_mode` と `quantization_mode` を完全に切り離し、GGUF validation、JSON failure 限定 retry、debug 出力に責務を限定する
- [ ] T018 [US2] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に `PhotoPainter LLM Generate (llama-cpp)` の入力と移行手順を追記する

**Checkpoint**: `llama-cpp` 専用ノードが単独で検証可能であること

---

## Phase 5: User Story 3 - backend 差を workflow レベルで明確にしたい (Priority: P3)

**Goal**: 2 ノードを同時配置したときに、backend 差が node 名・入力欄・debug 出力で明確に見えるようにする

**Independent Test**: ComfyUI 上で両専用ノードを同時に配置し、ノード名、入力項目、debug 出力が backend 固有であることを確認する

### Verification for User Story 3

- [ ] T019 [P] [US3] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に `NODE_CLASS_MAPPINGS` / `NODE_DISPLAY_NAME_MAPPINGS` の 2 ノード共存契約を追加する
- [ ] T020 [P] [US3] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に両専用ノードの debug 出力差分、retry 理由表示差分、旧単一ノード不在を確認するテストを追加する
- [ ] T021 [US3] `comfyui/workflows/README.md` と関連 workflow JSON に新ノード名を反映し、backend 比較手順を更新する

### Implementation for User Story 3

- [ ] T022 [US3] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` から旧 `PhotopainterLlmGenerate` と単一 display name を削除し、2 ノードの `NODE_CLASS_MAPPINGS` / `NODE_DISPLAY_NAME_MAPPINGS` に置き換える
- [ ] T023 [US3] `comfyui/workflows/llm-*.json` と `comfyui/workflows/README.md` を新ノード名と backend 別入力に合わせて更新する
- [ ] T024 [US3] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に旧単一ノード削除と新ノードへの対応表を追記する

**Checkpoint**: backend ごとの違いが ComfyUI UI と文書で明確に識別できること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: backend 分離後の整合と横断確認

- [ ] T025 [P] `python -m py_compile comfyui/custom_node/comfyui-photopainter-custom/__init__.py` と `python -m unittest discover -s comfyui/custom_node/comfyui-photopainter-custom/tests -v` を再実行し、結果を確認する
- [ ] T026 `comfyui/custom_node/comfyui-photopainter-custom/README.md`、`comfyui/workflows/README.md`、`specs/040-split-llm-nodes/quickstart.md` の用語と node 名を整合させる
- [ ] T027 `comfyui/Dockerfile` と backend 分離後の依存説明が `transformers` / `llama-cpp` の責務分離と一致しているか確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: Foundational 後に開始可能
- **User Story 3 (P3)**: US1 と US2 のノード名・契約が揃った後に開始する

### Within Each User Story

- 契約検証と単体検証を先に整える
- node class 追加・削除を README 更新より先に行う
- 旧単一ノード削除は新 2 ノード契約が揃ってから行う
- 検証タスクを省略しない

### Parallel Opportunities

- US1 と US2 の contract / logic test 追加は別ファイル単位で並列化可能
- US3 の contract test と workflow 文書更新は並列化可能

---

## Parallel Example: User Story 1

```bash
Task: "`comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に `PhotopainterTransformersLlmGenerate` の metadata 契約を追加する"
Task: "`comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に `transformers` 専用ノードの生成経路と quantization を検証するテストを追加する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. `transformers` 専用ノードを独立検証する
5. 必要なら ComfyUI で手動確認する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して独立検証する
3. User Story 2 を追加して独立検証する
4. User Story 3 を追加して node 名・文書・workflow を整える
5. 最後に横断テストと README 整合を確認する

### Parallel Team Strategy

1. チームで Setup + Foundational を完了する
2. Foundational 完了後に US1 と US2 を別担当で進める
3. US3 は 2 ノードの契約が固まった後にまとめて行う

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[US1]`〜`[US3]` は traceability のために必須
- backend 固有 UI を cross-story で混ぜないこと

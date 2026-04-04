# タスク: ComfyUI local LLM node

**Input**: `/specs/039-comfyui-llm-node/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: local LLM node 追加の設計成果物と導入前提を固定する

- [ ] T001 `specs/039-comfyui-llm-node/spec.md`、`plan.md`、`research.md`、`data-model.md`、`contracts/comfyui-local-llm-node-contract.md` を照合し、node の責務と `think_mode` 初期対応範囲を実装開始条件として確定する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story が依存する backend adapter、設定解決、共通 failure 契約を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T002 `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に local LLM node 用の共通 helper 骨格と node metadata を追加し、既存 `PhotopainterPngPost` と共存できる構成に整える
- [ ] T003 `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `backend`、`model_id`、任意の `model_file`、`think_mode`、`json_output`、`json_schema`、`max_retries` の入力正規化と validation を実装する
- [ ] T004 `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `COMFYUI_LLM_MODEL_CACHE_DIR` と backend 既定保存先の解決ポリシーを実装する
- [ ] T005 [P] `comfyui/Dockerfile` に `transformers`、`llama-cpp-python`、`jsonschema` を導入する build 手順を追加する
- [ ] T006 `compose.yml` に `COMFYUI_LLM_MODEL_CACHE_DIR` を ComfyUI container へ渡す環境変数設定を追加し、`.env` から runtime 注入できるようにする
- [ ] T007 `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に node metadata、入力 contract、`think_mode` 初期対応値、`model_id` / `model_file`、環境変数契約の検証を追加する

**Checkpoint**: shared helper、依存関係、環境変数契約が整い、US1 以降で再利用できること

---

## Phase 3: User Story 1 - Workflow 内でローカル LLM 推論を使いたい (Priority: P1)

**Goal**: ComfyUI workflow から text mode の local LLM 推論を行い、結果文字列を後続ノードへ渡せるようにする

**Independent Test**: `transformers` または `llama-cpp` を選んだ workflow で `json_output=false` の node 実行を行い、`text` 出力に生成結果が入り、後続ノードへ接続できることを確認する

### Verification for User Story 1

- [ ] T008 [US1] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に text mode の backend 選択、`model_id` / `model_file` 解決、単一 `STRING` 出力返却を検証する unit test を追加する
- [ ] T009 [US1] `specs/039-comfyui-llm-node/quickstart.md` に text mode の最小 workflow 手順が実装内容と一致することを反映する

### Implementation for User Story 1

- [ ] T010 [US1] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `transformers` backend adapter を実装し、text mode の単発推論を返せるようにする
- [ ] T011 [US1] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `llama-cpp` backend adapter を実装し、`model_id` + 任意 `model_file` で text mode の単発推論を返せるようにする
- [ ] T012 [US1] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `PhotoPainter LLM Generate` node 本体を実装し、単一 `STRING` 出力の戻り値契約を成立させる
- [ ] T013 [US1] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に backend 選択、`model_id` / `model_file`、`COMFYUI_LLM_MODEL_CACHE_DIR` の基本利用手順を追記する

**Checkpoint**: text mode の local LLM 推論が独立して検証可能であること

---

## Phase 4: User Story 2 - JSON 出力を固定し schema に合わせたい (Priority: P2)

**Goal**: JSON mode と inline schema 検証を使い、構造化出力だけを成功扱いにする

**Independent Test**: `json_output=true` で node を実行し、schema なしでは JSON parse 成功で通り、schema ありでは一致する JSON のみ成功することを確認する

### Verification for User Story 2

- [ ] T014 [US2] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に `json_schema` 文字列入力と単一 `STRING` 出力契約の検証を追加する
- [ ] T015 [US2] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に JSON parse 成功、schema 成功、schema 自体の不正を検証する unit test を追加する
- [ ] T016 [US2] `specs/039-comfyui-llm-node/quickstart.md` に JSON mode と schema 入力手順を実装に合わせて更新する

### Implementation for User Story 2

- [ ] T017 [US2] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `json_output=true` の parse 処理と単一 `STRING` 出力への JSON 文字列格納を実装する
- [ ] T018 [US2] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `json_schema` 文字列の parse と `jsonschema` による validation を実装する
- [ ] T019 [US2] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に JSON mode、schema 指定、成功条件を追記する

**Checkpoint**: JSON mode と schema 検証が独立して検証可能であること

---

## Phase 5: User Story 3 - schema 不一致時に限定リトライしたい (Priority: P3)

**Goal**: parse 失敗と schema 不一致だけを限定 retry し、最終 failure kind を判別できるようにする

**Independent Test**: parse 失敗または schema 不一致を起こす条件で node を実行し、retry 対象だけが上限回数まで再試行され、backend/model failure は即失敗することを確認する

### Verification for User Story 3

- [ ] T020 [US3] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に parse 失敗 retry、schema 不一致 retry、retry 上限後 failure、backend failure 非 retry、failure kind 表示を検証する unit test を追加する
- [ ] T021 [US3] `specs/039-comfyui-llm-node/quickstart.md` に retry と failure 切り分けの確認手順を実装に合わせて更新する

### Implementation for User Story 3

- [ ] T022 [US3] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `generic` / `qwen` / `gemma` / `deepseek_r1` の prompt formatting preset と retry ループ、再試行対象判定を実装する
- [ ] T023 [US3] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `config_error` / `backend_error` / `json_parse_error` / `schema_error` の failure 分類と UI message 生成を実装する
- [ ] T024 [US3] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に `think_mode` 一覧、`generic` の best-effort 性質、retry 条件、非 retry 条件、failure kind を追記する

**Checkpoint**: retry と failure 分類が独立して検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: ビルド・文書・回帰確認を行う

- [ ] T025 [P] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` と `test_node_logic.py` を含めて `python -m unittest discover -s comfyui/custom_node/comfyui-photopainter-custom/tests -v` を実行し、追加 node の回帰を確認する
- [ ] T026 [P] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` を対象に `python -m py_compile` 相当の構文確認を行う
- [ ] T027 `.env.example` に `COMFYUI_LLM_MODEL_CACHE_DIR` の説明を追加し、`compose.yml` と `specs/039-comfyui-llm-node/quickstart.md` と整合させる
- [ ] T028 `specs/039-comfyui-llm-node/plan.md`、`research.md`、`data-model.md`、`contracts/comfyui-local-llm-node-contract.md`、`tasks.md` の記述整合を確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能。MVP
- **User Story 2 (P2)**: US1 の node 本体と出力契約を前提に開始する
- **User Story 3 (P3)**: US2 の JSON/schema 検証完了後に開始する

### Parallel Opportunities

- Foundational の T005 と T006 は並列実行可能
- US1 の T010 と T011 は並列実行可能
- US2 の T017 と T018 は並列実行可能
- Polish の T025 と T026 は並列実行可能

## Parallel Example: User Story 1

```bash
Task: "`comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に text mode の backend 選択、model 指定、出力文字列返却を検証する unit test を追加する"
Task: "`compose.yml` に `COMFYUI_LLM_MODEL_CACHE_DIR` を ComfyUI container へ渡す環境変数設定を追加し、`.env` から runtime 注入できるようにする"
Task: "`comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `transformers` backend adapter を実装し、text mode の単発推論を返せるようにする"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. text mode の local LLM 推論を独立検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して text mode 推論を成立させる
3. User Story 2 を追加して JSON/schema 契約を成立させる
4. User Story 3 を追加して retry と failure 分類を固める
5. 最後に build / test / 文書整合を確認する

### Parallel Team Strategy

1. 1 人が Foundational を完了する
2. Foundational 完了後、US1 の backend adapter と contract test を分担できる
3. US2 以降は JSON/schema と retry を別担当で分けつつ、`__init__.py` 競合に注意して直列寄りに統合する

---

## Notes

- `__init__.py` に変更が集中するため、同一 phase でも統合順序を意識する
- `think_mode` の初期対応は `off`、`generic`、`qwen`、`gemma`、`deepseek_r1` のみとする
- retry は parse 失敗または schema 不一致に限定し、backend/model failure は即失敗のままにする

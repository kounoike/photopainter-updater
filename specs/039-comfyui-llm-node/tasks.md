# タスク: ComfyUI local LLM node

**Input**: `/specs/039-comfyui-llm-node/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 再設計後の成果物と実装境界を固定する

- [X] T001 `specs/039-comfyui-llm-node/spec.md`、`plan.md`、`research.md`、`data-model.md`、`contracts/comfyui-local-llm-node-contract.md` を照合し、documented think control と generation-time structured output を実装開始条件として確定する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story が依存する依存関係、設定解決、共通契約を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [X] T002 `comfyui/Dockerfile` に `lm-format-enforcer` を追加し、既存 `transformers`、`llama-cpp-python`、`jsonschema` と同居できる build 手順へ更新する
- [X] T003 [P] `compose.yml` と `.env.example` に `COMFYUI_LLM_MODEL_CACHE_DIR` の runtime 注入契約を整理し、ComfyUI container へ安定して渡せるようにする
- [X] T004 `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に node 入力正規化、`model_id` / `model_file` validation、`think_mode` enum、共通 error kind の基盤を実装する
- [X] T005 `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `COMFYUI_LLM_MODEL_CACHE_DIR` と backend 既定保存先の解決 helper を実装する
- [X] T006 `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に node metadata、入力 widget、`model_id` / `model_file`、`think_mode`、環境変数契約の検証を追加する

**Checkpoint**: 依存関係、環境変数契約、入力契約が整い、US1 以降の実装を開始できる

---

## Phase 3: User Story 1 - Workflow 内でローカル LLM 推論を使いたい (Priority: P1)

**Goal**: ComfyUI workflow から text mode の local LLM 推論を行い、単一 `STRING` 出力を後続ノードへ渡せるようにする

**Independent Test**: `transformers + Qwen/Qwen3.5-4B + think_mode=off + json_output=false` の smoke workflow を実行し、短い text 出力が単一 `STRING` として返ることを確認する

### Verification for User Story 1

- [X] T007 [US1] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に text mode の `transformers` backend、`llama-cpp` backend、`model_id` / `model_file` 解決、単一 `STRING` 出力を検証する unit test を追加する
- [X] T008 [US1] `specs/039-comfyui-llm-node/quickstart.md` に `transformers + Qwen3.5 + think_mode=off` の smoke 手順を反映し、手動確認基準を固定する

### Implementation for User Story 1

- [X] T009 [US1] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `transformers` backend adapter を実装し、text mode の単発推論と documented think off 制御を返せるようにする
- [X] T010 [US1] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `llama-cpp` backend adapter を実装し、`model_id` + `model_file` で text mode の単発推論を返せるようにする
- [X] T011 [US1] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `PhotoPainter LLM Generate` node 本体を実装し、backend 切替と単一 `STRING` 出力契約を成立させる
- [X] T012 [US1] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に backend 選択、`model_id` / `model_file`、`COMFYUI_LLM_MODEL_CACHE_DIR`、smoke 実行の基本利用手順を追記する

**Checkpoint**: text mode の local LLM 推論が独立して検証可能であること

---

## Phase 4: User Story 2 - JSON 出力を構造化して安定させたい (Priority: P2)

**Goal**: generation-time structured output と schema 検証を使い、構造化された JSON だけを成功扱いにする

**Independent Test**: `json_output=true` で node を実行し、schema なしでは valid JSON のみが通り、schema ありでは `lm-format-enforcer` による制約と `jsonschema` 検証を通った結果だけが成功することを確認する

### Verification for User Story 2

- [X] T013 [US2] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に `json_output`、`json_schema`、単一 `STRING` 出力、structured output 契約の検証を追加する
- [X] T014 [US2] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に constrained JSON 成功、schema 成功、schema 不正、constraint 非対応 backend/path の明示 failure、constraint なし text mode 非適用を検証する unit test を追加する
- [X] T015 [US2] `specs/039-comfyui-llm-node/quickstart.md` に JSON mode、schema 指定、generation-time constraint の確認手順を反映する

### Implementation for User Story 2

- [X] T016 [US2] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `lm-format-enforcer` を使った generation-time structured output helper を実装し、対応 backend/path ではそれを使い、非対応経路では明示 failure にする
- [X] T017 [US2] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `json_output=true` 時の JSON parse、`json_schema` parse、`jsonschema` validation、成功時の単一 `STRING` 返却を実装する
- [X] T018 [US2] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に JSON mode、schema 指定、structured output 制約、成功条件を追記する

**Checkpoint**: JSON mode と schema 検証が heuristic cleanup に依存せず独立検証可能であること

---

## Phase 5: User Story 3 - family ごとの think 制御を明示したい (Priority: P3)

**Goal**: documented think control を family ごとに切り替え、未対応や失敗を明示しながら retry/failure 分類を確立する

**Independent Test**: 同じ family で `think_mode=off` と family 特化 mode を切り替えて node を実行し、documented control が優先されること、未対応 mode を黙って通さないこと、parse/schema failure だけが retry されることを確認する

### Verification for User Story 3

- [X] T019 [US3] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に Qwen / Gemma / DeepSeek R1 の think control 計画、未対応 mode、retry 対象判定、failure kind 表示を検証する unit test を追加する
- [X] T020 [US3] `specs/039-comfyui-llm-node/quickstart.md` に family ごとの think 切替、retry、failure 切り分けの確認手順を反映する

### Implementation for User Story 3

- [X] T021 [US3] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に documented think control の解決ロジックを実装し、`off`、`generic`、`qwen`、`gemma`、`deepseek_r1` を family ごとに適用または明示失敗できるようにする
- [X] T022 [US3] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に parse 失敗と schema 不一致だけを対象にした retry ループと `config_error` / `think_mode_error` / `backend_error` / `json_parse_error` / `schema_error` の分類を実装する
- [X] T023 [US3] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に `think_mode` 一覧、documented control 優先方針、`generic` の best-effort 性質、retry 条件、failure kind を追記する

**Checkpoint**: think 制御、retry、failure 分類が独立して検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 文書、手動検証、回帰確認を行う

- [X] T024 [P] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` と `test_node_logic.py` を含めて `python -m unittest discover -s comfyui/custom_node/comfyui-photopainter-custom/tests -v` を実行し、回帰を確認する
- [X] T025 [P] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` を対象に `python -m py_compile` を実行し、構文エラーがないことを確認する
- [X] T026 `specs/039-comfyui-llm-node/plan.md`、`research.md`、`data-model.md`、`contracts/comfyui-local-llm-node-contract.md`、`quickstart.md`、`tasks.md` の記述整合を確認する
- [X] T027 `comfyui/custom_node/comfyui-photopainter-custom/.venv` を使った devcontainer 内 GPU 検証手順を `comfyui/custom_node/comfyui-photopainter-custom/README.md` または `specs/039-comfyui-llm-node/quickstart.md` に反映する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能。MVP
- **User Story 2 (P2)**: US1 の node 本体と backend adapter を前提に開始する
- **User Story 3 (P3)**: US2 の structured output 契約が成立した後に開始する

### Parallel Opportunities

- Foundational の T002 と T003 は並列実行可能
- Polish の T024 と T025 は並列実行可能

## Parallel Example: User Story 1

```bash
Task: "`comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に text mode の backend 選択、model 指定、出力文字列返却を検証する unit test を追加する"
Task: "`specs/039-comfyui-llm-node/quickstart.md` に `transformers + Qwen3.5 + think_mode=off` の smoke 手順を反映し、手動確認基準を固定する"
Task: "`comfyui/custom_node/comfyui-photopainter-custom/README.md` に backend 選択、model 指定、cache 環境変数の基本利用手順を追記する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. `transformers + Qwen3.5 + think_mode=off + text mode` の smoke を独立検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して text mode の local LLM 推論を成立させる
3. User Story 2 を追加して generation-time structured output と schema 契約を成立させる
4. User Story 3 を追加して documented think control と retry / failure 分類を固める
5. 最後に build / test / 文書整合を確認する

### Parallel Team Strategy

1. 1 人が Foundational を完了する
2. Foundational 完了後、1 人が text mode backend と node 本体、別の 1 人が smoke 文書と contract test を進められる
3. US2 以降は `__init__.py` 競合が大きいため、設計差分を先に固めてから直列寄りに統合する

---

## Notes

- `__init__.py` に変更が集中するため、同一 story 内の実装は統合順序を意識する
- `think_mode` の初期対応は `off`、`generic`、`qwen`、`gemma`、`deepseek_r1` のみとする
- structured output は generation-time constraint を優先し、自由文 cleanup を主経路にしない
- retry は parse 失敗または schema 不一致に限定し、backend/model failure と think 制御未対応は即失敗のままにする

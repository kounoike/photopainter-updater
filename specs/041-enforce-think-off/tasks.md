# タスク: ComfyUI think off 強制

**Input**: `/specs/041-enforce-think-off/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: `think_mode=off` 厳格化の契約と検証観点を実装前に固定する

- [X] T001 `specs/041-enforce-think-off/contracts/think-off-contract.md` と `specs/041-enforce-think-off/quickstart.md` を見直し、unsupported failure、trace violation、debug で見る項目を実装観点へ固定する
- [X] T002 `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` と `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に追加する `think_mode=off` 厳格化観点を洗い出し、task 実行順へ反映する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: `off` capability 判定と共通 debug 契約を先に整備する

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [X] T003 `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `think_mode=off` 専用の capability 判定と failure reason 表現を追加し、best-effort prompt 成功へ落ちない共通基盤を整備する
- [X] T004 `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` の debug 生成 helper を更新し、`off_enforcement_supported`、`off_enforcement_guaranteed`、`off_failure_reason` を返せるようにする
- [X] T005 `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に Foundational helper の期待値を固定する単体検証を追加し、retry 対象外の think control failure を確認する

**Checkpoint**: `think_mode=off` の成功条件と failure 理由が helper / debug 契約で固定されていること

---

## Phase 3: User Story 1 - `off` を実効化したい (Priority: P1)

**Goal**: documented disable を適用できる経路だけ `think_mode=off` を成功させる

**Independent Test**: `think_mode=off` を指定して Qwen 系モデルと documented control 未対応モデルを模擬し、前者だけ成功し後者は unsupported failure になること

### Verification for User Story 1

- [X] T006 [P] [US1] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に Qwen 系 `off` 成功、unsupported family failure、chat template fallback failure を検証するテストを追加する
- [X] T007 [P] [US1] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に Transformers ノードの debug 契約項目として `off` guarantee 状態が露出されることを検証する期待値を追加する
- [X] T008 [US1] `specs/041-enforce-think-off/quickstart.md` の成功/失敗確認手順を実装内容に合わせて更新する

### Implementation for User Story 1

- [X] T009 [US1] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` の `_resolve_think_control` と chat template 適用経路を更新し、`think_mode=off` で documented disable を保証できない場合は failure にする
- [X] T010 [US1] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` の generation 実行経路を更新し、`chat_template_kwargs` を捨てる silent fallback を `think_mode=off` では unsupported failure に変える
- [X] T011 [US1] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に `think_mode=off` の成功条件、主な unsupported ケース、debug の確認方法を追記する

**Checkpoint**: `think_mode=off` が documented disable を適用できる経路だけで独立して成功すること

---

## Phase 4: User Story 2 - `off` の成否を debug で確認したい (Priority: P2)

**Goal**: `debug_json` だけで `off` の保証成否と failure 理由を判別できるようにする

**Independent Test**: `think_mode=off` の成功ケースと failure ケースを模擬し、`debug_json` から guarantee 状態、control kind、failure 理由を読み取れること

### Verification for User Story 2

- [X] T012 [P] [US2] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に `debug_json` の `off_enforcement_supported`、`off_enforcement_guaranteed`、`off_failure_reason` を検証するテストを追加する
- [X] T013 [P] [US2] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に `debug_json` 必須キーの期待値更新を追加する
- [X] T014 [US2] `specs/041-enforce-think-off/contracts/think-off-contract.md` と `specs/041-enforce-think-off/quickstart.md` を実装済み debug 項目へ揃える

### Implementation for User Story 2

- [X] T015 [US2] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` の debug 情報組み立てを更新し、`think_mode=off` 成功/失敗の判定結果を `debug_json` へ反映する
- [X] T016 [US2] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に `debug_json` の新フィールドと読み方を追記する

**Checkpoint**: 成功と failure の両方で `debug_json` から `off` の状態を判別できること

---

## Phase 5: User Story 3 - sanitize 依存の成功を避けたい (Priority: P3)

**Goal**: `think_mode=off` で reasoning trace が出た場合に sanitize 成功へ逃がさない

**Independent Test**: `think_mode=off` で `<think>...</think>` を含む生出力を返すケースを模擬し、failure になることを確認する

### Verification for User Story 3

- [X] T017 [P] [US3] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に `think_mode=off` で reasoning trace を返した場合の failure と、`off` 以外での既存挙動維持を検証するテストを追加する
- [X] T018 [US3] `specs/041-enforce-think-off/quickstart.md` に trace violation の手動/模擬確認手順を実装内容へ合わせて更新する

### Implementation for User Story 3

- [X] T019 [US3] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` の output sanitize / validation を更新し、`think_mode=off` で trace が観測された場合は failure にする
- [X] T020 [US3] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に `off` では sanitize 救済しないことを明記する

**Checkpoint**: `think_mode=off` に sanitize 依存の成功経路が残っていないこと

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 文書・テスト・実装の横断整合を確認する

- [X] T021 [P] `python -m py_compile comfyui/custom_node/comfyui-photopainter-custom/__init__.py` と `python -m unittest discover -s comfyui/custom_node/comfyui-photopainter-custom/tests -v` を実行し、`think_mode=off` 厳格化の回帰を確認する
- [X] T022 `comfyui/custom_node/comfyui-photopainter-custom/README.md`、`specs/041-enforce-think-off/quickstart.md`、`specs/041-enforce-think-off/contracts/think-off-contract.md` の用語を整合させる

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の capability 判定と debug 基盤が入った後に開始する
- **User Story 3 (P3)**: User Story 1 の generation 経路が固まった後に開始する

### Within Each User Story

- 検証タスクを先に追加する
- `__init__.py` の capability 判定を README 更新より先に実装する
- `off` 失敗は JSON retry と混ぜない
- `llama-cpp` ノードは変更対象へ含めない

### Parallel Opportunities

- contract test と logic test の追加は同一 phase 内で並列化可能
- US2 の debug 契約更新と quickstart/contract 文書更新は並列化可能

---

## Parallel Example: User Story 1

```bash
Task: "`comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に Qwen 系 `off` 成功と unsupported failure の検証を追加する"
Task: "`comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に `off` guarantee 状態の debug 契約期待値を追加する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. `think_mode=off` の成功/unsupported を独立検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して `off` 成功条件を固定する
3. User Story 2 を追加して debug 読み取り性を仕上げる
4. User Story 3 を追加して sanitize 依存成功を排除する
5. 最後に横断テストと文書整合を確認する

### Parallel Team Strategy

1. Foundational 完了後、US1 実装担当と文書/contract 整理担当に分けられる
2. US1 固定後、US2 debug 契約と US3 trace validation を並行で進められる

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[US1]`〜`[US3]` は traceability のために必須
- `off` 以外の think mode の大規模仕様変更は今回のスコープ外

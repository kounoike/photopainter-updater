# タスク: ComfyUI 長文回答 continuation

**Input**: `/specs/042-continue-long-answers/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: continuation の対象条件と非対象条件を実装前に固定する

- [X] T001 `specs/042-continue-long-answers/contracts/continuation-contract.md` と `specs/042-continue-long-answers/quickstart.md` を見直し、長文回答 continuation、非対象ケース、停止理由を実装観点へ固定する
- [X] T002 `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` と `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に追加する continuation 観点を洗い出し、task 実行順へ反映する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: continuation 判定、停止条件、debug 契約の基盤を整備する

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [X] T003 `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に continuation 可否判定、上限回数、進展検知、停止理由を表す helper を追加する
- [X] T004 `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` の debug 生成 helper を更新し、`continuation_used`、`continuation_count`、`continuation_stop_reason` を返せるようにする
- [X] T005 `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に Foundational helper の単体検証を追加し、非対象ケースと進展なし停止を確認する

**Checkpoint**: continuation の対象条件、停止条件、debug 契約が helper レベルで固定されていること

---

## Phase 3: User Story 1 - 長い最終回答を最後まで受け取りたい (Priority: P1)

**Goal**: text mode の長文回答が途中終了しても continuation で完結させる

**Independent Test**: 1 回目で前半、2 回目で後半を返す模擬 backend を使い、連結済み `output_text` を返せること

### Verification for User Story 1

- [X] T006 [P] [US1] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に long answer continuation 成功、continuation 不要、上限到達のテストを追加する
- [X] T007 [US1] `specs/042-continue-long-answers/quickstart.md` の continuation 成功確認手順を実装内容に合わせて更新する

### Implementation for User Story 1

- [X] T008 [US1] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` の generation 実行経路を更新し、text mode の長文回答に continuation を適用する
- [X] T009 [US1] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` で generation 断片の連結と完結判定を実装し、完結した `output_text` / `raw_text` を返す
- [X] T010 [US1] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に長文回答 continuation の条件と上限を追記する

**Checkpoint**: text mode の長文回答が独立して最後まで返せること

---

## Phase 4: User Story 2 - continuation の発生有無を debug で確認したい (Priority: P2)

**Goal**: `debug_json` で continuation の回数と停止理由を判別できるようにする

**Independent Test**: continuation あり/なしのケースを比較し、debug 情報だけで差を判別できること

### Verification for User Story 2

- [X] T011 [P] [US2] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に `continuation_used`、`continuation_count`、`continuation_stop_reason` の期待値テストを追加する
- [X] T012 [P] [US2] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に continuation debug 項目の contract test を追加する
- [X] T013 [US2] `specs/042-continue-long-answers/contracts/continuation-contract.md` と `specs/042-continue-long-answers/quickstart.md` を実装済み debug 項目へ揃える

### Implementation for User Story 2

- [X] T014 [US2] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` の `GenerationDebugInfo` と `debug_json` 組み立てを更新し、continuation 情報を反映する
- [X] T015 [US2] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に continuation debug の見方を追記する

**Checkpoint**: continuation の有無と停止理由を `debug_json` だけで判別できること

---

## Phase 5: User Story 3 - continuation の上限や不適切なケースを制御したい (Priority: P3)

**Goal**: `think_mode=off` と JSON mode を壊さず、進展のない continuation を止める

**Independent Test**: `think_mode=off`、`json_output=true`、進展なし断片のケースを模擬し、continuation が不適切に走らないことを確認する

### Verification for User Story 3

- [X] T016 [P] [US3] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に `think_mode=off` 非 continuation、JSON mode 非 continuation、進展なし停止のテストを追加する
- [X] T017 [US3] `specs/042-continue-long-answers/quickstart.md` の非対象ケース確認手順を実装内容へ合わせて更新する

### Implementation for User Story 3

- [X] T018 [US3] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `think_mode=off` / JSON mode / 未対応 backend の continuation 禁止条件を実装する
- [X] T019 [US3] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に進展なし停止と上限停止を実装する
- [X] T020 [US3] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に非対象ケースと停止条件を明記する

**Checkpoint**: continuation が既存厳格契約を壊さず、無限継続しないこと

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 文書・テスト・実装の横断整合を確認する

- [X] T021 [P] `python -m py_compile comfyui/custom_node/comfyui-photopainter-custom/__init__.py` と `python -m unittest discover -s comfyui/custom_node/comfyui-photopainter-custom/tests -v` を実行し、continuation 回帰を確認する
- [X] T022 `comfyui/custom_node/comfyui-photopainter-custom/README.md`、`specs/042-continue-long-answers/quickstart.md`、`specs/042-continue-long-answers/contracts/continuation-contract.md` の用語を整合させる

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の continuation 実装が入った後に開始する
- **User Story 3 (P3)**: User Story 1 の継続経路が固まった後に開始する

### Within Each User Story

- 検証タスクを先に追加する
- continuation 判定 helper を README 更新より先に実装する
- `think_mode=off` と JSON mode の非対象条件を壊さない
- 無限継続防止の停止条件を省略しない

### Parallel Opportunities

- contract test と logic test の追加は同一 phase 内で並列化可能
- quickstart / contract 文書更新はテスト実装と並列化可能

---

## Parallel Example: User Story 2

```bash
Task: "`comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に continuation debug の期待値を追加する"
Task: "`comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に continuation debug 項目の contract test を追加する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. 長文回答 continuation を独立検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して長文回答 continuation を成立させる
3. User Story 2 を追加して debug 読み取り性を仕上げる
4. User Story 3 を追加して非対象条件と停止条件を固める
5. 最後に横断テストと文書整合を確認する

### Parallel Team Strategy

1. Foundational 完了後、US1 実装担当と文書 / contract 整理担当に分けられる
2. US1 固定後、US2 debug 契約と US3 停止条件を並行で進められる

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[US1]`〜`[US3]` は traceability のために必須
- 041 の `think_mode=off` 契約は緩めない

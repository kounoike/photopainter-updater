# タスク: ComfyUI PNG POSTノード

**Input**: `/specs/027-comfyui-post-node/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3`)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: カスタムノード実装の土台となる repo 配下の構成を作る

- [ ] T001 `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` と `comfyui/custom_node/comfyui-photopainter-custom/README.md` の初期ファイル構成を作成する
- [ ] T002 [P] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` と `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` のテスト配置を作成する
- [ ] T003 [P] `comfyui/custom_node/comfyui-photopainter-custom/tests/fixtures/` に PNG fixture 生成または保持方針を用意し、参照方法を `comfyui/custom_node/comfyui-photopainter-custom/README.md` に記載する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story で共有する node 骨格、変換処理、HTTP 共通処理を整備する

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T004 `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `PhotopainterPngPost` の node metadata、`INPUT_TYPES`、`RETURN_TYPES`、`OUTPUT_NODE`、`NODE_CLASS_MAPPINGS` の骨格を定義する
- [ ] T005 [P] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に URL 検証ヘルパーと request 生成ヘルパーを追加し、`Content-Type: image/png` raw body 契約を共通化する
- [ ] T006 [P] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `IMAGE` tensor を単一 PNG bytes へ変換する共通ヘルパーを追加する
- [ ] T007 [P] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に URL 検証、単一画像制約、PNG bytes 変換の基礎テストを追加する
- [ ] T008 `comfyui/custom_node/comfyui-photopainter-custom/README.md` に repo 配下ソースと `comfyui-data/custom_nodes/` runtime 配置の境界を記載する

**Checkpoint**: node 骨格と共通 helper が揃い、story 実装へ進める

---

## Phase 3: User Story 1 - Workflow から画像を送信したい (Priority: P1)

**Goal**: ComfyUI の画像を任意 URL へ PNG raw body で送る終端ノードを提供する

**Independent Test**: ComfyUI で画像を生成し、026 の `POST /upload` URL を指定して実行したとき、`200 OK` で送信成功し、server 側の画像更新まで確認できる

### Verification for User Story 1

- [ ] T009 [P] [US1] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に `POST` / `Content-Type: image/png` / raw body / `200 OK` 成功判定の契約テストを追加する
- [ ] T010 [US1] `specs/027-comfyui-post-node/quickstart.md` に 026 の `POST /upload` を使った成功確認手順を最終実装に合わせて更新する

### Implementation for User Story 1

- [ ] T011 [US1] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に PNG raw body を送信する `POST` 実処理を実装する
- [ ] T012 [US1] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に `200 OK` 成功時の UI summary 返却を実装する
- [ ] T013 [US1] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に node 名、入力項目、026 `POST /upload` との接続例を記載する
- [ ] T014 [US1] `README.md` に repo 内カスタムノードの配置場所と `specs/027-comfyui-post-node/quickstart.md` への導線を追記する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - 送信失敗を判別したい (Priority: P2)

**Goal**: URL 不正、接続失敗、失敗 status を workflow エラーとして明示する

**Independent Test**: 不正 URL、接続不能 URL、`400` または `500` を返す送信先で実行したとき、node が例外で失敗し、原因を判別できる

### Verification for User Story 2

- [ ] T015 [P] [US2] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` に不正 URL、network error、`200` 以外の status を失敗扱いにする契約テストを追加する
- [ ] T016 [US2] `specs/027-comfyui-post-node/quickstart.md` に URL 不正、接続失敗、`400/500` 応答の失敗確認手順を更新する

### Implementation for User Story 2

- [ ] T017 [US2] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に URL 不正と入力不足を即時エラー化する検証処理を実装する
- [ ] T018 [US2] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に network error と `200` 以外の HTTP 応答を `RuntimeError` へ変換する処理を実装する
- [ ] T019 [US2] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に status と本文要約を含む失敗メッセージ整形を実装する
- [ ] T020 [US2] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に失敗時に workflow がエラー終了する前提と代表的な失敗パターンを記載する

**Checkpoint**: User Story 2 が独立して検証可能であること

---

## Phase 5: User Story 3 - 繰り返し使いやすくしたい (Priority: P3)

**Goal**: runtime 配置と再実行導線を整え、同じ workflow で URL を切り替えながら安定利用できるようにする

**Independent Test**: runtime 配置後に ComfyUI を再起動し、URL を変えて複数回実行しても毎回同じ node から送信できる

### Verification for User Story 3

- [ ] T021 [P] [US3] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` に複数回実行時の再送と複数画像バッチ拒否のテストを追加する
- [ ] T022 [US3] `specs/027-comfyui-post-node/quickstart.md` に symlink/copy 配置と ComfyUI 再起動、URL 切り替え再実行の確認手順を更新する

### Implementation for User Story 3

- [ ] T023 [US3] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に単一画像のみ許可し、複数画像バッチを拒否する処理を実装する
- [ ] T024 [US3] `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` に URL 入力ごとに独立送信する再実行前提の node 実行ロジックを仕上げる
- [ ] T025 [US3] `comfyui/custom_node/comfyui-photopainter-custom/README.md` に `comfyui-data/custom_nodes/comfyui-photopainter-custom` への symlink/copy 配置手順を記載する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 最終確認と横断調整

- [ ] T026 [P] `comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py` と `comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py` を最終仕様に合わせて整理する
- [ ] T027 `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` の import、例外文言、コメントを整理して可読性を整える
- [ ] T028 `specs/027-comfyui-post-node/quickstart.md` の手順を実際の最終ファイル構成に合わせて通し確認する
- [ ] T029 `README.md` と `comfyui/custom_node/comfyui-photopainter-custom/README.md` の説明が重複や矛盾なく整合していることを確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Phase 6)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 完了後に開始可能
- **User Story 2 (P2)**: User Story 1 完了後に開始推奨
- **User Story 3 (P3)**: User Story 1 完了後に開始可能

### Within Each User Story

- 契約・ロジック検証を先に追加し、期待動作を固定してから実装する
- `__init__.py` の共通 helper を壊さない範囲で story ごとの責務を積み上げる
- `quickstart.md` / README 更新を実装直後に行い、手順とコードの乖離を残さない

### Parallel Opportunities

- `[P]` 付き Setup / Foundational タスクは並列実行可能
- US1 の契約テスト追加と README 更新は並列進行可能
- US2 の契約テスト追加と quickstart 更新は並列進行可能
- US3 の再実行テスト追加と quickstart 更新は並列進行可能

---

## Parallel Example: User Story 1

```bash
Task: "T009 [US1] 契約テストを comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py に追加する"
Task: "T010 [US1] 026 成功確認手順を specs/027-comfyui-post-node/quickstart.md に反映する"
```

---

## Parallel Example: User Story 2

```bash
Task: "T015 [US2] 失敗系契約テストを comfyui/custom_node/comfyui-photopainter-custom/tests/test_contract.py に追加する"
Task: "T016 [US2] 失敗確認手順を specs/027-comfyui-post-node/quickstart.md に反映する"
```

---

## Parallel Example: User Story 3

```bash
Task: "T021 [US3] 再実行と複数画像拒否のテストを comfyui/custom_node/comfyui-photopainter-custom/tests/test_node_logic.py に追加する"
Task: "T022 [US3] runtime 配置と再実行手順を specs/027-comfyui-post-node/quickstart.md に反映する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. 026 `POST /upload` への疎通を独立検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して送信成功導線を固める
3. User Story 2 を追加して失敗導線を固める
4. User Story 3 を追加して runtime 配置と再実行導線を固める
5. Polish で文書とテストを最終整合する

### Parallel Team Strategy

1. 1 名が Foundational の node 骨格と helper を整備する
2. その後、別担当が US2 の失敗導線、別担当が US3 の runtime / 再実行導線を進める
3. 最後に 1 名が README と quickstart、最終テスト整理をまとめる

---

## Notes

- `[P]` は別ファイルまたは実装依存の薄い並列タスクを示す
- `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` への集中編集が多いため、story 間で同時編集する場合は順序を守る
- runtime の `comfyui-data/custom_nodes/` 側は install 先であり、repo 管理ソースの主置き場ではない

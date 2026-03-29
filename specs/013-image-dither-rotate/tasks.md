# タスク: 画像ディザリング回転配信

**Input**: `/specs/013-image-dither-rotate/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Path Conventions

- サーバ実装: `server/src/main.rs`
- feature 成果物: `specs/013-image-dither-rotate/`
- 参照実装: `ref/convert.py`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 画像変換付き配信に必要な依存とテスト入口を揃える

- [ ] T001 `server/Cargo.toml` に画像読込・画像変換・BMP 出力に必要な依存を追加する
- [ ] T002 `server/src/main.rs` の既存配信責務を、入力画像読込・変換・配信へ分解する設計コメントまたは関数境界へ整理する
- [ ] T003 [P] 画像処理 fixture と比較方針を `specs/013-image-dither-rotate/quickstart.md` に反映する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story が依存する変換パイプラインの骨格を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T004 `server/src/main.rs` に `image.png` 入力、変換済み RGB 画像、失敗応答を表す共通データ構造を追加する
- [ ] T005 [P] `server/src/main.rs` に入力画像読込と失敗応答の共通ヘルパーを追加する
- [ ] T006 [P] `server/src/main.rs` に変換済み画像を 24bit BMP 応答へ変換する共通ヘルパーを追加する
- [ ] T007 `specs/013-image-dither-rotate/contracts/transformed-bmp-response-contract.md` と `specs/013-image-dither-rotate/research.md` の用語を実装用語へ揃える
- [ ] T008 Allowed Scope / Forbidden Scope の境界を `specs/013-image-dither-rotate/plan.md` と `specs/013-image-dither-rotate/tasks.md` で確認する

**Checkpoint**: 変換パイプラインの入出力と失敗系の共通土台が固まっていること

---

## Phase 3: User Story 1 - 変換済み BMP を取得したい (Priority: P1)

**Goal**: `image.png` から変換済み 24bit BMP を `/` と `/image.bmp` で取得できるようにする

**Independent Test**: 既知の `image.png` を配置してサーバを起動し、`/` と `/image.bmp` が同一の 24bit BMP を返すことを確認する

### Verification for User Story 1

- [ ] T009 [P] [US1] 変換済み BMP 応答の契約テストを `server/src/main.rs` に追加する
- [ ] T010 [P] [US1] 入力画像読込テストを `server/src/main.rs` に追加し、`image.png` の存在時に読めることを確認する
- [ ] T011 [P] [US1] 24bit BMP エンコードテストを `server/src/main.rs` に追加し、Content-Type と BMP ヘッダを確認する
- [ ] T012 [US1] 取得確認手順を `specs/013-image-dither-rotate/quickstart.md` に反映する

### Implementation for User Story 1

- [ ] T013 [US1] `image.png` を変換対象として解決する処理を `server/src/main.rs` に実装する
- [ ] T014 [US1] 変換済み RGB 画像を 24bit BMP として返すレスポンス生成を `server/src/main.rs` に実装する
- [ ] T015 [US1] `/` と `/image.bmp` が同一の変換済み BMP を返すよう `server/src/main.rs` を更新する
- [ ] T016 [US1] `server/run.sh` と `specs/013-image-dither-rotate/contracts/transformed-bmp-response-contract.md` に `image.png` 入力前提を反映する

**Checkpoint**: User Story 1 が単独で検証可能であること

---

## Phase 4: User Story 2 - 参照変換と同等の見た目を維持したい (Priority: P2)

**Goal**: 彩度強調、参照相当ディザリング、右 90 度回転の順で変換した結果が参照品質と整合するようにする

**Independent Test**: 同じ `image.png` について、サーバ出力が参照変換と同等のディザリング傾向と向きを持つことを確認する

### Verification for User Story 2

- [ ] T017 [P] [US2] 彩度変換の単体テストを `server/src/main.rs` に追加し、代表ピクセルの色変化を確認する
- [ ] T018 [P] [US2] ディザリングの単体テストを `server/src/main.rs` に追加し、出力色が参照パレット内に収まることを確認する
- [ ] T019 [P] [US2] 右 90 度回転の単体テストを `server/src/main.rs` に追加し、座標変換が正しいことを確認する
- [ ] T020 [P] [US2] 参照変換比較テストを `server/src/main.rs` に追加し、fixture 画像で出力傾向を比較する
- [ ] T021 [US2] 参照比較手順を `specs/013-image-dither-rotate/quickstart.md` に反映する

### Implementation for User Story 2

- [ ] T022 [US2] 彩度強調処理を `server/src/main.rs` に実装する
- [ ] T023 [US2] `ref/convert.py` 相当のディザリング処理を `server/src/main.rs` に実装する
- [ ] T024 [US2] 右 90 度回転処理を `server/src/main.rs` に実装する
- [ ] T025 [US2] 変換順序を「彩度強調 → ディザリング → 回転 → BMP 化」に統合して `server/src/main.rs` に実装する
- [ ] T026 [US2] パレット構成の扱いを `specs/013-image-dither-rotate/research.md` と `specs/013-image-dither-rotate/plan.md` に明記する

**Checkpoint**: User Story 2 が User Story 1 を壊さず独立検証可能であること

---

## Phase 5: User Story 3 - 入力画像の問題を切り分けたい (Priority: P3)

**Goal**: `image.png` 未配置または変換不能時に、入力画像起因の失敗として判別できるようにする

**Independent Test**: `image.png` を未配置または変換不能にした状態でサーバを起動し、`/` と `/image.bmp` が入力画像問題と判別できる失敗応答を返すことを確認する

### Verification for User Story 3

- [ ] T027 [P] [US3] `image.png` 未配置時の失敗テストを `server/src/main.rs` に追加する
- [ ] T028 [P] [US3] 変換不能入力時の失敗テストを `server/src/main.rs` に追加する
- [ ] T029 [P] [US3] `/` と `/image.bmp` の失敗応答一致テストを `server/src/main.rs` に追加する
- [ ] T030 [US3] 失敗応答確認手順を `specs/013-image-dither-rotate/quickstart.md` に反映する

### Implementation for User Story 3

- [ ] T031 [US3] `image.png` 未配置時に入力画像未配置と判別できる失敗応答を `server/src/main.rs` に実装する
- [ ] T032 [US3] 変換不能入力時に変換失敗と判別できる失敗応答を `server/src/main.rs` に実装する
- [ ] T033 [US3] 失敗応答の文言とステータスを `specs/013-image-dither-rotate/contracts/transformed-bmp-response-contract.md` に反映する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 通し確認と横断品質の整合を取る

- [ ] T034 [P] 入力画像差し替え後の次回取得反映テストを `server/src/main.rs` に追加する
- [ ] T035 `server/run.sh` の起動案内と `specs/013-image-dither-rotate/quickstart.md` の手順を実装結果へ合わせて更新する
- [ ] T036 `specs/013-image-dither-rotate/quickstart.md` の通し手順を実行し、参照比較と失敗系の確認結果を反映する
- [ ] T037 `specs/013-image-dither-rotate/research.md`、`specs/013-image-dither-rotate/plan.md`、`specs/013-image-dither-rotate/contracts/transformed-bmp-response-contract.md` の記述差分を解消する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Story 1 (Phase 3)**: Foundational 完了後に開始する
- **User Story 2 (Phase 4)**: User Story 1 完了後に開始する
- **User Story 3 (Phase 5)**: User Story 2 完了後に開始する
- **Polish (Phase 6)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: まず PNG 入力から変換済み BMP を返す最小配信を成立させる
- **User Story 2 (P2)**: User Story 1 の配信導線の上で画像品質を参照変換へ寄せる
- **User Story 3 (P3)**: 変換処理が成立した後に失敗系切り分けを強化する

### Within Each User Story

- 自動テストは実装前に追加し、失敗を確認してから処理本体を実装する
- 画像処理は「彩度変換」「ディザリング」「回転」「BMP 化」を分離してテストする
- contract と quickstart は各 story 完了時点の挙動へ更新する

### Parallel Opportunities

- Phase 1 の `T003` は `T001` と並列で進められる
- User Story 2 の画像処理単体テスト `T017` `T018` `T019` `T020` は並列化しやすい
- User Story 3 の失敗系テスト `T027` `T028` `T029` は並列実行可能

---

## Parallel Example: User Story 2

```bash
Task: "彩度変換の単体テストを server/src/main.rs に追加し、代表ピクセルの色変化を確認する"
Task: "ディザリングの単体テストを server/src/main.rs に追加し、出力色が参照パレット内に収まることを確認する"
Task: "右 90 度回転の単体テストを server/src/main.rs に追加し、座標変換が正しいことを確認する"
Task: "参照変換比較テストを server/src/main.rs に追加し、fixture 画像で出力傾向を比較する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 と Phase 2 を完了する
2. Phase 3 で PNG 入力から変換済み BMP を返す最小配信を実装する
3. User Story 1 を独立検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して変換済み BMP 配信を成立させる
3. User Story 2 を追加して画像品質を参照変換へ揃える
4. User Story 3 を追加して失敗系切り分けを完成させる
5. Polish で差し替え反映と通し確認を行う

### Parallel Team Strategy

1. 1 人が入出力と BMP 応答、別の 1 人が画像処理単体テスト群を担当する
2. User Story 2 では彩度変換、ディザリング、回転のテストを並列で作る
3. story 完了後に quickstart と contract 更新を別担当で詰める

---

## Notes

- 今回はユーザー要望に合わせ、画像処理ごとの自動テストを明示的に含めている
- `ref/convert.py` は参照専用であり、直接変更しない
- `[P]` は別観点または別ファイルで安全に並列化できるタスクを示す

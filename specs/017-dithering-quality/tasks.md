# タスク: ディザリングアルゴリズムの改善

**Input**: `/specs/017-dithering-quality/` の設計文書
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスクを含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story（`US1`〜`US4`）
- タスク記述には正確なファイルパスを含める

---

## Phase 1: Setup

**Purpose**: 変更対象の現状把握と実装境界の確認

- [ ] T001 `server/src/main.rs` の `apply_reference_dither`、`squared_distance`、`nearest_palette_color`、`AppState` 定義を読んで現状を確認する
- [ ] T002 `server/src/main.rs` の `main()` 関数と環境変数読み込み箇所（`PORT`、`CONTENT_DIR`）を読んで追加位置を確認する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: US1〜US3 に共通する基盤の実装

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T003 `server/src/main.rs` の `AppState` に `use_lab: bool` と `use_atkinson: bool` フィールドを追加する
- [ ] T004 `server/src/main.rs` の `main()` で `DITHER_USE_LAB` と `DITHER_USE_ATKINSON` 環境変数を読み込み `AppState` に格納する
- [ ] T005 `server/src/main.rs` に `rgb_to_lab(pixel: [u8; 3]) -> [f32; 3]` 関数を追加する（sRGB→XYZ→CIE Lab、D65光源）
- [ ] T006 `server/src/main.rs` に `lab_squared_distance(pixel: [f32; 3], candidate: [u8; 3]) -> f32` 関数を追加する
- [ ] T007 `cargo build` でビルドエラーがないことを確認する

**Checkpoint**: T003〜T007 完了後に user story 実装へ進む

---

## Phase 3: User Story 1 - B案（Atkinsonアルゴリズム）の評価 (Priority: P1)

**Goal**: `DITHER_USE_ATKINSON=1` で起動したサーバーが Atkinson ディザリングを使用して画像を生成できる

**Independent Test**: `DITHER_USE_ATKINSON=1 cargo run --release` でサーバーを起動し、ePaper に画像を表示して現行と異なるディザリングパターンを目視確認する

### Implementation for User Story 1

- [ ] T008 [US1] `server/src/main.rs` の `apply_reference_dither` を `AppState`（または `use_atkinson: bool` 引数）を受け取るよう変更し、`use_atkinson` が `true` のとき Atkinson 係数（6隣接ピクセルに各 1/8）で誤差拡散するよう実装する
- [ ] T009 [US1] `cargo test` を実行し、既存のディザリングテストが全通過することを確認する（デフォルトは Floyd-Steinberg のまま）
- [ ] T010 [US1] `DITHER_USE_ATKINSON=1 cargo run --release` でサーバーを起動し、実機で画像を表示して現行との違いを目視確認する

**Checkpoint**: B案が独立して動作し、デフォルト動作が変わっていないこと

---

## Phase 4: User Story 2 - A案（CIE Lab色空間）の評価 (Priority: P1)

**Goal**: `DITHER_USE_LAB=1` で起動したサーバーが CIE Lab 色距離で最近傍パレット色を選択できる

**Independent Test**: `DITHER_USE_LAB=1 cargo run --release` でサーバーを起動し、ePaper に画像を表示して色選択の自然さを目視確認する

### Implementation for User Story 2

- [ ] T011 [US1] [US2] `server/src/main.rs` の `nearest_palette_color` を `use_lab: bool` 引数を受け取るよう変更し、`use_lab` が `true` のとき `lab_squared_distance` を使用するよう実装する
- [ ] T012 [US2] `cargo test` を実行し、既存テストが全通過することを確認する（デフォルトは RGB のまま）
- [ ] T013 [US2] `DITHER_USE_LAB=1 cargo run --release` でサーバーを起動し、実機で画像を表示して色選択の変化を目視確認する

**Checkpoint**: A案が独立して動作し、デフォルト動作が変わっていないこと

---

## Phase 5: User Story 3 - A+B組み合わせの評価 (Priority: P2)

**Goal**: `DITHER_USE_LAB=1 DITHER_USE_ATKINSON=1` で起動したサーバーが両方を同時に使用できる

**Independent Test**: 両フラグ有効でサーバーを起動し、ePaper に画像を表示して単独案との違いを目視確認する

### Implementation for User Story 3

- [ ] T014 [US3] `server/src/main.rs` の `apply_reference_dither` と `nearest_palette_color` の呼び出し経路を確認し、A案・B案フラグが組み合わせ時も正しく伝達されることを確認する
- [ ] T015 [US3] `DITHER_USE_LAB=1 DITHER_USE_ATKINSON=1 cargo run --release` でサーバーを起動し、実機で画像を表示して A案のみ・B案のみ・現行との違いを目視確認する

**Checkpoint**: 4 パターン（現行・B案・A案・A+B）が全て動作すること

---

## Phase 6: User Story 4 - 評価結果に基づくクリーンアップ (Priority: P3)

**Goal**: 採用アルゴリズムを正式採用し、評価用コードを除去してコードをクリーンにする

**Independent Test**: クリーンアップ後に `cargo run --release`（フラグなし）で起動し、採用アルゴリズムで画像が正常に表示されること。`grep -r "DITHER_USE" server/src/` で何も出力されないこと

### Implementation for User Story 4

- [ ] T016 [US4] 採用アルゴリズムを確定し、`server/src/main.rs` から不採用アルゴリズムのコードブロックを削除する
- [ ] T017 [US4] `server/src/main.rs` から `use_lab`・`use_atkinson` フィールドと環境変数読み込みを削除する
- [ ] T018 [US4] `server/src/main.rs` の `AppState` を元の形に戻し、フラグを受け取っていた関数シグネチャを整理する
- [ ] T019 [US4] `cargo test` を実行し、全テストが通過することを確認する
- [ ] T020 [US4] `grep -r "DITHER_USE" server/src/` で出力がないことを確認し、`cargo run --release` で実機表示品質がクリーンアップ前と同等であることを確認する

**Checkpoint**: 評価用コードが完全に除去され、採用アルゴリズムのみが残っていること

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: コードコメントの更新と最終確認

- [ ] T021 `server/src/main.rs` の `apply_reference_dither` と `nearest_palette_color` 付近のコメントを採用アルゴリズムに合わせて更新する
- [ ] T022 `cargo test` で全テスト通過を最終確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後
- **US1 (Phase 3)**: Foundational 完了後
- **US2 (Phase 4)**: Foundational 完了後（US1 と並列実行可能）
- **US3 (Phase 5)**: US1・US2 両方完了後
- **US4 (Phase 6)**: US3 完了・採用決定後
- **Polish (Phase 7)**: US4 完了後

### User Story Dependencies

- **US1 (P1)**: Foundational 後に開始可能
- **US2 (P1)**: Foundational 後に開始可能（US1 と並列）
- **US3 (P2)**: US1・US2 完了後
- **US4 (P3)**: US3 完了・評価結果確定後

### Parallel Opportunities

- T001 と T002 は並列実行可能（同一ファイルの別箇所）
- T005 と T006 は並列実行可能（独立した関数）
- US1（T008〜T010）と US2（T011〜T013）は並列実行可能（別フラグ経路）

---

## Parallel Example: US1 と US2

```
[Foundational 完了後、並列]
  US1: T008 Atkinson 誤差拡散の実装 (server/src/main.rs)
  US2: T011 Lab 色距離の適用 (server/src/main.rs) ← T008 と同ファイルだが独立した関数
[両方完了後]
  US3: T014-T015 A+B 組み合わせ確認
```

**注意**: US1 と US2 は同一ファイル（`server/src/main.rs`）を変更するため、マージコンフリクトに注意。1人で実施する場合は US1 → US2 の順が安全。

---

## Implementation Strategy

### MVP First (US1 のみ)

1. Phase 1: Setup 完了（T001-T002）
2. Phase 2: Foundational 完了（T003-T007）
3. Phase 3: US1 実装（T008）・確認（T009-T010）
4. B案の効果を評価してから US2 へ進む

### Incremental Delivery

1. Setup + Foundational 完了
2. US1（B案）実装・実機評価
3. US2（A案）実装・実機評価
4. US3（A+B）確認・実機評価
5. 採用決定 → US4 クリーンアップ
6. Polish

---

## Notes

- US1 と US2 は同一ファイルの変更だが独立した関数への変更であるため論理的には並列可能
- T016（採用決定）はユーザーの実機評価結果に依存する人間判断タスク
- クリーンアップ（US4）は評価フェーズとは別コミットで行うことを推奨

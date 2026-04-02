# タスク: POST画像保存

**Input**: `/specs/026-post-image-upload/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Path Conventions

- サーバ本体: `server/src/`
- 画像変換・保存補助: `server/src/image_pipeline/` または upload 専用モジュール
- HTTP レベルの回帰テスト: `server/src/routes.rs`
- 設定・定数: `server/src/config.rs`
- 応答 helper: `server/src/response.rs`
- ログ: `server/src/logging.rs`
- 文書: `server/README.md` と `specs/026-post-image-upload/quickstart.md`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: upload 機能を追加できる依存とファイル構成を整える

- [ ] T001 `server/Cargo.toml` に `axum` の `multipart` feature を追加し、upload 実装に必要な依存条件を更新する
- [ ] T002 `server/src/main.rs` と `server/src/app.rs` の module 宣言・配線を見直し、upload 用モジュールを追加できる構成を整える
- [ ] T003 [P] `server/README.md` に `POST /upload` を扱う予定の起動・運用前提を追記するための節を準備する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に共通する upload 基盤を先に整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T004 `server/src/config.rs` に upload 用のターゲット寸法定数、temporary path helper、関連メッセージ定数を追加する
- [ ] T005 [P] `server/src/image_pipeline/upload.rs` を新規作成し、`UploadRequest`、`UploadCandidate`、`NormalizedImage`、`UploadResult` の基礎型を定義する
- [ ] T006 [P] `server/src/image_pipeline/mod.rs` に upload モジュールの公開と既存 pipeline から参照する export を追加する
- [ ] T007 `server/src/logging.rs` に `POST /upload` 用の outcome 分類を追加し、既存 request logging へ統合できるようにする
- [ ] T008 `server/src/response.rs` に upload 成功・入力不正・保存失敗向け text response helper を追加する
- [ ] T009 `specs/026-post-image-upload/contracts/upload-endpoint-contract.md` と `specs/026-post-image-upload/plan.md` を見直し、実装境界が Allowed Scope / Forbidden Scope から外れていないことを確認する

**Checkpoint**: upload の共通型、定数、response、logging 導線が揃っていること

---

## Phase 3: User Story 1 - 新しい画像をアップロードしたい (Priority: P1)

**Goal**: 有効な画像を `POST /upload` で受け取り、PNG かつ 480x800 の `image.png` として保存できるようにする

**Independent Test**: 既存画像がある状態で、PNG、JPG/JPEG、GIF、BMP、WebP を raw body と multipart/form-data の両形式でそれぞれ POST し、保存結果が `image.png` に正規化され、480x800 への変換はアスペクト比維持の中央クロップで行われ、`GET /image.bmp` と `GET /image.bin` が更新後画像を入力として扱うことを確認する

### Verification for User Story 1

- [ ] T010 [P] [US1] `server/src/image_pipeline/upload.rs` に PNG、JPG/JPEG、GIF、BMP、WebP の decode、PNG 正規化、480x800 中央クロップの単体テストを追加する
- [ ] T011 [P] [US1] `server/src/routes.rs` に raw body と multipart/form-data の両方で `POST /upload` が成功し、`GET /image.bmp` と `GET /image.bin` に反映される HTTP テストを追加する
- [ ] T012 [US1] `specs/026-post-image-upload/quickstart.md` に raw body と multipart の成功確認手順を実装に合わせて更新する

### Implementation for User Story 1

- [ ] T013 [P] [US1] `server/src/image_pipeline/upload.rs` に PNG、JPG/JPEG、GIF、BMP、WebP の受信バイト列 decode、形式判定、PNG encode を実装する
- [ ] T014 [P] [US1] `server/src/image_pipeline/upload.rs` に 480x800 へのアスペクト比維持リサイズと中央クロップ処理を実装する
- [ ] T015 [US1] `server/src/image_pipeline/upload.rs` に temporary file 経由で `image.png` を安全に置換する保存処理を実装する
- [ ] T016 [US1] `server/src/routes.rs` に `POST /upload` handler を追加し、raw body と multipart/form-data を `Content-Type` に応じて受理できるようにする
- [ ] T017 [US1] `server/src/routes.rs` と `server/src/response.rs` に upload 成功時の応答生成と既存 GET route 反映確認を実装する

**Checkpoint**: User Story 1 が単独で検証可能であり、upload 成功後に既存 GET route から更新結果を利用できること

---

## Phase 4: User Story 2 - 不正なアップロードを判別したい (Priority: P2)

**Goal**: 無効な入力や malformed request に対して保存を拒否し、既存画像を保護したまま失敗理由を判別できるようにする

**Independent Test**: 壊れたファイル、空の本文、PNG/JPG/JPEG/GIF/BMP/WebP 以外の形式、画像として扱えない内容、または multipart/form-data 内に有効な画像ファイルが含まれない要求を POST し、既存の `image.png` が保持され、`400` / `415` / `500` の失敗理由が判別できることを確認する

### Verification for User Story 2

- [ ] T018 [P] [US2] `server/src/routes.rs` に空 body と multipart 構造不正で `400`、対応外形式と decode 失敗で `415`、multipart の画像不足で `400` を検証する HTTP テストを追加する
- [ ] T019 [P] [US2] `server/src/image_pipeline/upload.rs` に保存失敗時の rollback と既存画像維持を検証する単体テストを追加する
- [ ] T020 [US2] `specs/026-post-image-upload/quickstart.md` に失敗系確認手順と期待結果を実装に合わせて更新する

### Implementation for User Story 2

- [ ] T021 [P] [US2] `server/src/image_pipeline/upload.rs` に `400` / `415` / `500` へ対応する invalid payload、unsupported media、save failure の分類処理を実装する
- [ ] T022 [US2] `server/src/response.rs` に upload 失敗時の `400` / `415` / `500` と判別可能な文言を返す処理を実装する
- [ ] T023 [US2] `server/src/routes.rs` に multipart 内の画像 file 特定失敗、decode 失敗、空 body を失敗応答へ変換する処理を実装する
- [ ] T024 [US2] `server/src/logging.rs` と `server/src/routes.rs` に upload 失敗時の outcome 記録を追加し、既存画像保護と同時にログで判別できるようにする

**Checkpoint**: User Story 2 が単独で検証可能であり、失敗時に `image.png` が維持されること

---

## Phase 5: User Story 3 - 更新結果をすぐ反映したい (Priority: P3)

**Goal**: 連続した更新要求でも最後に成功した画像だけが現在画像となり、再起動なしで直後の GET route に反映されるようにする

**Independent Test**: サーバ起動中に解像度の異なる画像を含む複数回の画像を POST し、各回の直後に取得結果が最後に成功した 480x800 の保存済み画像へ切り替わることを確認する

### Verification for User Story 3

- [ ] T025 [P] [US3] `server/src/routes.rs` に連続 upload 後の GET route 反映を検証する HTTP テストを追加する
- [ ] T026 [P] [US3] `server/src/image_pipeline/upload.rs` に 2 回連続更新時の最終保存結果と中央クロップ一貫性を検証する単体テストを追加する
- [ ] T027 [US3] `specs/026-post-image-upload/quickstart.md` に連続更新と即時反映の確認手順を追加する

### Implementation for User Story 3

- [ ] T028 [P] [US3] `server/src/image_pipeline/upload.rs` に既存画像の有無に依存しない初回保存と連続更新の置換処理を実装する
- [ ] T029 [US3] `server/src/routes.rs` に upload 成功直後から既存 GET route が新しい `image.png` を参照する統合処理を仕上げる
- [ ] T030 [US3] `server/src/logging.rs` と `server/src/routes.rs` に連続成功更新でも一貫した request log が残るよう調整する

**Checkpoint**: すべての user story が独立検証可能であり、連続更新後も最後の成功結果だけが使われること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 複数 story にまたがる最終調整

- [ ] T031 [P] `server/README.md` に `POST /upload` の request 形式、正規化規則、失敗時挙動を追記する
- [ ] T032 `server/src/routes.rs`、`server/src/response.rs`、`server/src/logging.rs` のコード整理を行い、`GET /image.bmp` と `GET /image.bin` の既存契約を壊していないことを確認する
- [ ] T033 [P] `specs/026-post-image-upload/contracts/upload-endpoint-contract.md` と `specs/026-post-image-upload/quickstart.md` を実装結果に合わせて仕上げる
- [ ] T034 `server/README.md` と `specs/026-post-image-upload/quickstart.md` の手順どおりに手動確認し、必要なら文言を補正する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Phase 6)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の upload 基本導線を前提に進める
- **User Story 3 (P3)**: User Story 1 の保存成功導線を前提に進める

### Within Each User Story

- 検証タスクを先に追加し、期待挙動を固定してから実装に進む
- `server/src/image_pipeline/upload.rs` の純粋ロジックを `server/src/routes.rs` の HTTP 統合より先に実装する
- `server/src/response.rs` と `server/src/logging.rs` の反映は handler 実装と同じ phase 内で完了する
- 文書更新は該当 story の実装確認に必要な範囲で同 phase 内に反映する

### Parallel Opportunities

- `[P]` 付き Setup / Foundational タスクは並列実行可能
- US1 では decode/normalize 実装と HTTP テスト作成を別担当で並列化できる
- US2 では rollback テストと失敗応答・失敗ログの実装準備を並列化できる
- US3 では連続更新テストと quickstart 追記を並列化できる

---

## Parallel Example: User Story 1

```bash
Task: "T010 [US1] server/src/image_pipeline/upload.rs に画像正規化の単体テストを追加する"
Task: "T011 [US1] server/src/routes.rs に POST /upload 成功の HTTP テストを追加する"
Task: "T013 [US1] server/src/image_pipeline/upload.rs に decode と PNG 正規化を実装する"
Task: "T014 [US1] server/src/image_pipeline/upload.rs に 480x800 中央クロップ処理を実装する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. `POST /upload` の成功系と既存 GET route 反映を独立検証する
5. 以降の失敗系と連続更新は次の increment として進める

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して upload 成功導線を確立する
3. User Story 2 を追加して失敗時保護と失敗分類を固める
4. User Story 3 を追加して連続更新と即時反映を完成させる
5. Polish で文書と cross-cutting な回帰確認を仕上げる

### Parallel Team Strategy

1. 1 人が Setup + Foundational を進める
2. US1 では upload core と HTTP テストを別担当で分担する
3. US2 では rollback / error response / logging を役割分担する
4. US3 では連続更新テストとドキュメント更新を並列で進める

---

## Notes

- 全 34 タスクはチェックボックス、Task ID、必要な `[P]`、story label、ファイルパスを含む形式に統一した
- `server/src/image_pipeline/upload.rs` は現時点では新規想定パスであり、実装時に `image_pipeline/` 配下へ作成する
- 既存 GET route 契約を守るため、cross-story な変更は `server/src/routes.rs`、`server/src/response.rs`、`server/src/logging.rs` に限定して追跡する

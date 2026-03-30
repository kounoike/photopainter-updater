# タスク: HTTPサーバ構成整理

**Input**: `/specs/018-http-server-refactor/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Path Conventions

- server 実装: `server/src/`
- server テスト: `server/src/` 内の `#[cfg(test)]` モジュール
- feature 文書: `specs/018-http-server-refactor/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 依存追加と分割先ファイルの土台を準備する

- [ ] T001 `server/Cargo.toml` に `envconfig`、`tracing`、`tracing-subscriber` を追加する
- [ ] T002 [P] `server/src/config.rs` と `server/src/logging.rs` を新規作成して設定/ログの受け皿を用意する
- [ ] T003 [P] `server/src/app.rs`、`server/src/routes.rs`、`server/src/response.rs`、`server/src/image_pipeline/mod.rs` を新規作成して責務分割の土台を用意する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に先行する共通モデルと配線を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T004 `server/src/config.rs` に `ServerConfig` と `DitherOptions` の設定読込・既定値・入力検証を実装する
- [ ] T005 [P] `server/src/logging.rs` に `AccessLogEvent`、失敗分類、`tracing` 初期化関数を実装する
- [ ] T006 [P] `server/src/response.rs` に BMP/Binary/Text の HTTP response helper を移設する
- [ ] T007 `server/src/app.rs` に `AppState`、起動メッセージ生成、listener/router 起動配線を実装する
- [ ] T008 `server/src/main.rs` を最小の起動エントリへ整理し、`config.rs`、`logging.rs`、`app.rs`、`routes.rs`、`response.rs`、`image_pipeline` を参照する構成へ置き換える
- [ ] T009 `specs/018-http-server-refactor/contracts/server-runtime-contract.md` と `specs/018-http-server-refactor/plan.md` を確認し、route 名と起動導線を維持する実装境界を `specs/018-http-server-refactor/quickstart.md` に反映する

**Checkpoint**: 基盤完了後に user story 実装へ進む

---

## Phase 3: User Story 1 - 変更しやすい構成で保守したい (Priority: P1)

**Goal**: 配信、変換、応答生成の責務を分離し、変更箇所を局所化できるようにする

**Independent Test**: サーバコードの主要責務が分離された状態で、画像配信と既存の変換処理が従来どおり動作することを確認する。

### Verification for User Story 1

- [ ] T010 [US1] `specs/018-http-server-refactor/quickstart.md` に責務分離後の回帰確認観点が揃っているか見直して必要なら追記する
- [ ] T011 [US1] `server/src/routes.rs` と `server/src/image_pipeline/mod.rs` の `#[cfg(test)]` モジュールへ route 応答と変換回帰の自動テストを再配置する

### Implementation for User Story 1

- [ ] T012 [P] [US1] `server/src/image_pipeline/load.rs` と `server/src/image_pipeline/dither.rs` に入力画像読込とディザ処理を移設する
- [ ] T013 [P] [US1] `server/src/image_pipeline/bmp.rs` と `server/src/image_pipeline/binary.rs` に BMP/Binary 生成処理を移設する
- [ ] T014 [US1] `server/src/image_pipeline/mod.rs` に `ImagePipelineRequest` と `ImagePipelineResult` を実装して変換 API を統一する
- [ ] T015 [US1] `server/src/routes.rs` に `/`、`/image.bmp`、`/image.bin`、fallback handler と router 構築を実装する
- [ ] T016 [US1] `server/src/main.rs` と `server/src/app.rs` から旧 `main.rs` 内の重複処理を削除し、責務分離後の呼び出しへ統合する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - 起動設定を一貫して扱いたい (Priority: P2)

**Goal**: 起動設定の既定値適用と不正値案内を単一の規則で扱えるようにする

**Independent Test**: 既定値起動、不正な設定値、明示的な設定指定の各条件でサーバを起動し、挙動と案内が一貫していることを確認する。

### Verification for User Story 2

- [ ] T017 [US2] `server/src/config.rs` の `#[cfg(test)]` モジュールに既定値、環境変数 override、不正値入力の単体テストを追加する
- [ ] T018 [US2] `specs/018-http-server-refactor/quickstart.md` に `PORT`、`CONTENT_DIR`、ディザ関連設定の確認手順を必要に応じて補強する

### Implementation for User Story 2

- [ ] T019 [US2] `server/src/config.rs` に `envconfig` ベースの環境変数マッピングと起動エラー整形を実装する
- [ ] T020 [US2] `server/src/app.rs` に `ServerConfig` から `AppState` を組み立てる初期化処理と起動時の設定要約生成を実装する
- [ ] T021 [US2] `server/run.sh` を更新し、既存引数互換を維持したまま設定読込後の起動案内が新構成と整合するようにする

**Checkpoint**: User Story 2 が独立して検証可能であること

---

## Phase 5: User Story 3 - 実行状況を読みやすく確認したい (Priority: P3)

**Goal**: 起動時とリクエスト時の情報を同一導線で読みやすく追えるようにする

**Independent Test**: サーバ起動とリクエスト処理を実行し、主要な実行状況が一貫した形式で確認できることを確かめる。

### Verification for User Story 3

- [ ] T022 [US3] `server/src/logging.rs` と `server/src/routes.rs` の `#[cfg(test)]` モジュールにアクセスログの成功/失敗分類を確認するテストを追加する
- [ ] T023 [US3] `specs/018-http-server-refactor/quickstart.md` と `specs/018-http-server-refactor/contracts/server-runtime-contract.md` に起動ログ/アクセスログ確認手順の最終形を反映する

### Implementation for User Story 3

- [ ] T024 [US3] `server/src/logging.rs` に起動ログ、アクセスログ、失敗分類ログの `tracing` 出力を実装する
- [ ] T025 [US3] `server/src/routes.rs` に request 単位の `AccessLogEvent` 生成と正常系/失敗系の記録処理を統合する
- [ ] T026 [US3] `server/src/app.rs` と `server/src/main.rs` に startup log 初期化と待受情報の出力配線を統合する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 横断的な仕上げと最終回帰確認

- [ ] T027 [P] `server/src/main.rs`、`server/src/app.rs`、`server/src/routes.rs`、`server/src/config.rs`、`server/src/logging.rs`、`server/src/response.rs`、`server/src/image_pipeline/` 配下の未使用コードと import を整理する
- [ ] T028 `specs/018-http-server-refactor/quickstart.md` の手順に沿って `server/run.sh` と主要 route の手動確認結果を反映する
- [ ] T029 `server/README.md` または既存の server 向け運用文書に設定項目説明と責務分割後の変更対象の探し方を反映する
- [ ] T030 `server/Cargo.toml` と `server/src/` 配下を対象に `cargo fmt` / `cargo test` の最終実行を完了する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能。最小 MVP
- **User Story 2 (P2)**: Foundational 後に開始可能だが、`AppState` と起動配線の土台として User Story 1 の分割結果を利用する
- **User Story 3 (P3)**: Foundational 後に開始可能だが、設定と route が整理された後に仕上げると衝突が少ない

### Within Each User Story

- 自動テストは対象 story の実装変更前または同時に追加して回帰基準を固定する
- `image_pipeline` の分離を route 統合より先に進める
- 設定モデルを起動配線より先に固める
- ログ整形を event 定義の後で統合する

### Parallel Opportunities

- **Setup**: `T002` と `T003` は並列実行可能
- **Foundational**: `T005` と `T006` は並列実行可能
- **User Story 1**: `T012` と `T013` は並列実行可能
- **Polish**: `T027` は最終確認前の整理として並列実行可能

---

## Parallel Example: User Story 1

```bash
Task: "T012 [US1] server/src/image_pipeline/load.rs と server/src/image_pipeline/dither.rs に入力画像読込とディザ処理を移設する"
Task: "T013 [US1] server/src/image_pipeline/bmp.rs と server/src/image_pipeline/binary.rs に BMP/Binary 生成処理を移設する"
Task: "T011 [US1] server/src/routes.rs と server/src/image_pipeline/mod.rs の #[cfg(test)] モジュールへ route 応答と変換回帰の自動テストを再配置する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. `cargo test` で route 応答と変換回帰を確認する

### Incremental Delivery

1. Setup + Foundational でモジュール分割の土台を作る
2. User Story 1 で責務分離と route 維持を完了する
3. User Story 2 で設定読込と起動案内を統一する
4. User Story 3 で `tracing` ベースのログ導線を完成させる
5. Polish で文書と最終検証を閉じる

### Parallel Team Strategy

1. 1 人が Setup + Foundational を進める
2. User Story 1 では変換処理分離と route 統合を分担する
3. User Story 2 と 3 は `config/app` 系と `logging/routes` 系で分担する

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[US1]` から `[US3]` は spec の user story と 1 対 1 に対応する
- 既存 route 契約と `server/run.sh` の導線維持を最優先とし、Forbidden Scope への変更は含めない

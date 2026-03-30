# タスク: デフォルト画像フォーマットを .bin に変更

**Input**: `/specs/016-default-bin-format/` の設計文書
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

---

## Phase 1: Setup

**Purpose**: 変更対象ファイルの現状確認と実装境界の確認

- [x] T001 `server/src/main.rs` のルーティング定義（L170-180 付近）を読んで現状を確認する
- [x] T002 `firmware/main/config.cc` の `HasBinSuffix` と `IsBinaryImageUrl`（L35-106 付近）を読んで現状を確認する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に先行する確認

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [x] T003 Allowed Scope の確認 — `/image.bin` ルート自体は変更しないこと、BMP 変換ロジックは変更しないことを確認する
- [x] T004 `HasBinSuffix` が `IsBinaryImageUrl` 以外から使われていないことを `firmware/main/` 全体で grep 確認する

**Checkpoint**: T003・T004 完了後に実装へ進む

---

## Phase 3: User Story 1 - ルートエンドポイントが .bin を返す (Priority: P1)

**Goal**: サーバーの `/` ルートへのリクエストが BMP ではなくバイナリフレーム形式で返るようになる

**Independent Test**: `curl -I http://localhost:<port>/` で `Content-Type: application/vnd.photopainter-frame` が返ることを確認する

### Implementation for User Story 1

- [x] T005 [US1] `server/src/main.rs` の `.route("/", get(serve_image))` を `.route("/", get(serve_binary_image))` に変更する（L173）
- [x] T006 [US1] サーバーをビルドして起動し、`GET /` のレスポンスヘッダーが `application/vnd.photopainter-frame` であることを確認する
- [x] T007 [US1] `GET /image.bmp` が引き続き `image/bmp` を返すことを確認する（BMP フォールバック維持）

**Checkpoint**: `GET /` がバイナリフレームを返し、`GET /image.bmp` が BMP を返すこと

---

## Phase 4: User Story 2 - ファームウェアがデフォルトで .bin を扱う (Priority: P1)

**Goal**: ファームウェアが URL に `.bmp` サフィックスがない限りバイナリパスを選択するようになる

**Independent Test**: `IsBinaryImageUrl("http://server/")` が `true`、`IsBinaryImageUrl("http://server/image.bmp")` が `false`、`IsBinaryImageUrl("http://server/image.bin")` が `true` を返すことを確認する

### Implementation for User Story 2

- [x] T008 [US2] `firmware/main/config.cc` に `HasBmpSuffix` 関数を追加する（`HasBinSuffix` の直後、`.bmp` サフィックス判定）
- [x] T009 [US2] `firmware/main/config.cc` の `IsBinaryImageUrl` を `!HasBmpSuffix(image_url)` に変更する（L105）
- [x] T010 [US2] ファームウェアをビルドして、`image_url = "http://<server>/"` の設定でバイナリパスが選択されることをログで確認する
- [x] T011 [US2] `image_url = "http://<server>/image.bmp"` の設定で BMP パスが選択されることをログで確認する（後方互換確認）

**Checkpoint**: ファームウェアがデフォルトでバイナリパスを使用し、`.bmp` URL の場合のみ BMP パスを使用すること

---

## Phase 5: User Story 3 - 既存の image.bin ルートとの整合性 (Priority: P2)

**Goal**: `/` と `/image.bin` が同一フォーマット・同一内容を返す

**Independent Test**: `GET /` と `GET /image.bin` のレスポンスボディが同一であることを確認する

### Implementation for User Story 3

- [x] T012 [US3] `GET /` と `GET /image.bin` に同時リクエストを送り、両レスポンスの `Content-Type` が一致することを確認する
- [x] T013 [US3] 両エンドポイントのレスポンスボディ（バイナリフレームデータ）が同一内容であることを確認する（同一タイミングで同一画像が返ること）

**Checkpoint**: `/` と `/image.bin` が完全に同等の動作をすること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: ドキュメント更新と不要コード整理

- [x] T014 [P] `server/src/main.rs` のコメント（もしあれば）を更新し、`/` ルートが `.bin` を返すことを明記する
- [x] T015 [P] `firmware/main/config.cc` の `IsBinaryImageUrl` 付近のコメントを更新し、デフォルト `.bin` 判定ロジックを説明する
- [x] T016 `HasBinSuffix` が不要になった場合、`firmware/main/config.cc` から削除する（T004 の確認結果に基づく）

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後
- **US1 (Phase 3)**: Foundational 完了後に開始可能
- **US2 (Phase 4)**: Foundational 完了後に開始可能（US1 と並列可能）
- **US3 (Phase 5)**: US1 完了後（サーバー変更が前提）
- **Polish (Phase 6)**: US1・US2 完了後

### User Story Dependencies

- **US1 (P1)**: Foundational 後に開始可能
- **US2 (P1)**: Foundational 後に開始可能（US1 と並列実行可能 — 別ファイル）
- **US3 (P2)**: US1 完了後（`/` ルート変更が完了していること）

### Parallel Opportunities

- T001 と T002 は並列実行可能（別ファイル）
- US1（T005-T007）と US2（T008-T011）は並列実行可能（サーバーとファームウェアは独立）
- T014 と T015 は並列実行可能（別ファイル）

---

## Parallel Example: US1 と US2

```
[並列]
  Task US1: T005 server/src/main.rs のルート変更
  Task US2: T008 firmware/main/config.cc に HasBmpSuffix 追加
            T009 IsBinaryImageUrl を変更
[US1 完了後]
  Task US3: T012-T013 両エンドポイントの整合性確認
```

---

## Implementation Strategy

### MVP First (User Story 1 のみ)

1. Phase 1: Setup 完了（T001-T002）
2. Phase 2: Foundational 完了（T003-T004）
3. Phase 3: US1 実装（T005）と確認（T006-T007）
4. US1 を独立検証する

### Incremental Delivery

1. Setup + Foundational 完了
2. US1 と US2 を並列実装して各々独立検証
3. US3 で両エンドポイントの整合性確認
4. Polish でドキュメント・不要コード整理

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために必須
- T016（`HasBinSuffix` 削除）は T004 の grep 結果に依存する条件付きタスク

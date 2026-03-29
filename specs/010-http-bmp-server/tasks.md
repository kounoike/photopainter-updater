# タスク: BMP配信HTTPサーバ

**Input**: `/workspaces/photopainter-updater/specs/010-http-bmp-server/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3`)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Rust サーバの最小プロジェクト構成を作る

- [X] T001 `server/Cargo.toml` に Rust + `axum` の最小サーバ定義を追加する
- [X] T002 [P] `server/src/main.rs` を新規作成し、`axum` アプリケーションの起動骨格を用意する
- [X] T003 [P] `.gitignore` と `server/contents/.gitignore` を確認し、Rust build 成果物と配信画像が適切に除外されるよう整える

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に共通する起動・配信基盤を固める

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [X] T004 `server/src/main.rs` に `server/contents/image.bmp` を参照する共通ファイル解決処理を実装する
- [X] T005 [P] `server/run.sh` を Rust サーバ起動導線に差し替える
- [X] T006 [P] `specs/010-http-bmp-server/contracts/root-bmp-response-contract.md` を基準に `GET /` と `GET /image.bmp` の応答条件を実装へ対応付ける
- [X] T007 `specs/010-http-bmp-server/quickstart.md` の確認手順が実装方針と一致するよう見直す

**Checkpoint**: 起動導線と配信元パスの前提が固まっていること

---

## Phase 3: User Story 1 - 画像取得用の URL で画像を取得したい (Priority: P1)

**Goal**: `GET /` と `GET /image.bmp` の両方で `image.bmp` を `image/bmp` として返せるようにする

**Independent Test**: `server/contents/image.bmp` を配置した状態でサーバを起動し、`GET /` と `GET /image.bmp` が `200 OK` と `image/bmp` を返すことを確認する

### Verification for User Story 1

- [X] T008 [US1] `specs/010-http-bmp-server/quickstart.md` の「画像ありの確認」に沿った手動確認を記録する

### Implementation for User Story 1

- [X] T009 [US1] `server/src/main.rs` に `GET /` と `GET /image.bmp` の handler を実装し、同じ `server/contents/image.bmp` を返す
- [X] T010 [US1] `server/src/main.rs` に両 route の成功時 `Content-Type: image/bmp` を付与する
- [X] T011 [US1] `server/run.sh` から起動したサーバが `GET /` と `GET /image.bmp` を受けられるよう起動処理を完成させる

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - 画像未配置時の状態を判別したい (Priority: P2)

**Goal**: `image.bmp` が無いときに未配置と分かる失敗応答を返せるようにする

**Independent Test**: `server/contents/image.bmp` が無い状態でサーバを起動し、`GET /` と `GET /image.bmp` が `404 Not Found` と説明文を返すことを確認する

### Verification for User Story 2

- [X] T012 [US2] `specs/010-http-bmp-server/quickstart.md` の「画像未配置の確認」に沿った手動確認を記録する

### Implementation for User Story 2

- [X] T013 [US2] `server/src/main.rs` に両 route 共通の `image.bmp` 未配置時 `404 Not Found` 応答を実装する
- [X] T014 [US2] `server/src/main.rs` に両 route 共通で画像未配置と判別できる `text/plain; charset=utf-8` の説明文を実装する

**Checkpoint**: User Story 2 が独立して検証可能であること

---

## Phase 5: User Story 3 - 後から画像を差し替えて使いたい (Priority: P3)

**Goal**: サーバ再起動なしで `image.bmp` の差し替えが次回アクセスから反映されるようにする

**Independent Test**: サーバ起動中に `server/contents/image.bmp` を別ファイルへ差し替え、次回 `GET /` と `GET /image.bmp` で新しい内容が返ることを確認する

### Verification for User Story 3

- [X] T015 [US3] `specs/010-http-bmp-server/quickstart.md` の「差し替え反映の確認」に沿った手動確認を記録する

### Implementation for User Story 3

- [X] T016 [US3] `server/src/main.rs` をリクエストごとにファイルを読み直す構成にして両 route の差し替え反映を保証する
- [X] T017 [US3] `specs/010-http-bmp-server/contracts/root-bmp-response-contract.md` と実装が差し替え時の挙動まで一致するよう確認する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: ドキュメントと運用導線を仕上げる

- [X] T018 [P] `docs/firmware.md` にローカル HTTP サーバとしての利用前提が必要なら追記する
- [X] T019 `server/run.sh`、`specs/010-http-bmp-server/quickstart.md`、`specs/010-http-bmp-server/contracts/root-bmp-response-contract.md` の表現を統一する
- [X] T020 `cargo test` または同等の検証と `specs/010-http-bmp-server/quickstart.md` の手動確認結果をまとめる

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Phase 6)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の route 実装を前提に開始する
- **User Story 3 (P3)**: User Story 1 の成功配信実装を前提に開始する

### Within Each User Story

- 手動確認手順を先に確定する
- `server/src/main.rs` の route 実装を先に行う
- `server/run.sh` と契約文書の整合を最後に確認する

### Parallel Opportunities

- Setup では `T002` と `T003` を並列化できる
- Foundational では `T005` と `T006` を並列化できる
- Polish では `T018` と一部レビュー作業を並列化できる

---

## Parallel Example: User Story 1

```bash
Task: "`specs/010-http-bmp-server/quickstart.md` の「画像ありの確認」に沿った手動確認を記録する"
Task: "`server/src/main.rs` に `GET /` と `GET /image.bmp` の handler を実装し、同じ `server/contents/image.bmp` を返す"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. `GET /` と `GET /image.bmp` で BMP が返ることを確認する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して最小配信を成立させる
3. User Story 2 を追加して未配置時の切り分けを可能にする
4. User Story 3 を追加して差し替え運用を成立させる
5. Polish で文書と検証結果を整える

### Parallel Team Strategy

1. 1 人が Rust サーバ骨格と起動導線を整える
2. もう 1 人が contract と quickstart を整える
3. story ごとの配信挙動を段階的に統合する

---

## Notes

- `[P]` は別ファイルで編集衝突しにくい独立タスクを示す
- `[US1]`、`[US2]`、`[US3]` は traceability のために必須
- 画像変換や telemetry は今回のタスクに含めない

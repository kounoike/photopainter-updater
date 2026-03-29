# タスク: run.sh 配信設定改善

**Input**: `/specs/012-fix-run-access-path/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 実装対象と検証入口を feature 文書に揃える

- [X] T001 現行の起動仕様と配信挙動を `server/run.sh`、`server/src/main.rs`、`specs/012-fix-run-access-path/quickstart.md` で再確認する
- [X] T002 feature の実装方針に合わせて変更対象を `server/run.sh`、`server/src/main.rs`、`specs/012-fix-run-access-path/quickstart.md` に限定する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story が共有する起動設定の受け渡し基盤を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [X] T003 `server/run.sh` で配信元ディレクトリ入力と既定ディレクトリ解決の方針を整理する
- [X] T004 [P] `server/run.sh` の起動前検証で `cargo`、`PORT`、配信元ディレクトリ失敗時のメッセージ要件を整理する
- [X] T005 `server/src/main.rs` で配信元ディレクトリまたは代表ファイルを環境変数から受け取れる前提へ整理する
- [X] T006 Allowed Scope / Forbidden Scope の境界を `specs/012-fix-run-access-path/plan.md` と `specs/012-fix-run-access-path/tasks.md` で確認する

**Checkpoint**: 起動設定の入力、既定値、失敗条件の設計が固まっていること

---

## Phase 3: User Story 1 - LAN 端末から配信へアクセスしたい (Priority: P1)

**Goal**: `run.sh` 起動後に利用者が別端末から到達可能だと分かる案内と待受整合を提供する

**Independent Test**: `server/run.sh` でサーバを起動し、起動案内に localhost 専用ではない利用方法が示され、同一ネットワーク上の別端末から配信内容を取得できることを確認する

### Verification for User Story 1

- [X] T007 [US1] LAN 到達確認の手順を `specs/012-fix-run-access-path/quickstart.md` に反映する
- [X] T008 [P] [US1] 起動案内の期待値を検証する自動テストを `server/src/main.rs` に追加または更新する
- [X] T009 [US1] `server/run.sh` 実行結果の案内文を確認する手順を `specs/012-fix-run-access-path/quickstart.md` に追加する

### Implementation for User Story 1

- [X] T010 [US1] 外部端末利用を誤解させない起動案内へ `server/run.sh` を更新する
- [X] T011 [US1] `server/src/main.rs` の起動ログを `server/run.sh` の案内方針と整合する内容へ更新する
- [X] T012 [US1] localhost と LAN 利用時の接続先説明を `specs/012-fix-run-access-path/contracts/run-script-invocation-contract.md` に反映する

**Checkpoint**: User Story 1 が単独で検証可能であること

---

## Phase 4: User Story 2 - 配信元ディレクトリを切り替えたい (Priority: P2)

**Goal**: 起動時に任意ディレクトリを指定して配信元を切り替えられるようにする

**Independent Test**: 任意ディレクトリを指定して `server/run.sh` を起動し、指定したディレクトリの内容が配信されることを確認する

### Verification for User Story 2

- [X] T013 [US2] 任意ディレクトリ指定での起動手順を `specs/012-fix-run-access-path/quickstart.md` に反映する
- [X] T014 [P] [US2] 配信元 override の挙動を検証する自動テストを `server/src/main.rs` に追加または更新する
- [X] T015 [US2] 任意ディレクトリ指定で `server/run.sh` の起動結果を確認する手順を `specs/012-fix-run-access-path/quickstart.md` に追加する

### Implementation for User Story 2

- [X] T016 [US2] 配信元ディレクトリ指定を受け付ける起動インターフェースへ `server/run.sh` を更新する
- [X] T017 [US2] 指定ディレクトリを実際の配信元として解決する処理を `server/src/main.rs` に実装する
- [X] T018 [US2] 既定ディレクトリと override 優先順位を `specs/012-fix-run-access-path/contracts/run-script-invocation-contract.md` に反映する

**Checkpoint**: User Story 2 が User Story 1 を壊さず独立検証可能であること

---

## Phase 5: User Story 3 - 実行場所に依存せず起動したい (Priority: P3)

**Goal**: どのカレントディレクトリから `run.sh` を呼んでも同じ配信設定で起動できるようにする

**Independent Test**: リポジトリ外または別ディレクトリから `server/run.sh` を起動し、既定ディレクトリ利用時も任意ディレクトリ指定時も同じ結果になることを確認する

### Verification for User Story 3

- [X] T019 [US3] 別ディレクトリからの起動確認手順を `specs/012-fix-run-access-path/quickstart.md` に反映する
- [X] T020 [P] [US3] カレントディレクトリ非依存の解決を検証する自動テストを `server/src/main.rs` に追加または更新する
- [X] T021 [US3] 別カレントディレクトリから `server/run.sh` を実行した結果を確認する手順を `specs/012-fix-run-access-path/quickstart.md` に追加する

### Implementation for User Story 3

- [X] T022 [US3] スクリプト基準で既定ディレクトリを解決する処理へ `server/run.sh` を更新する
- [X] T023 [US3] 配信元指定の絶対パス化または正規化処理を `server/run.sh` と必要に応じて `server/src/main.rs` に実装する
- [X] T024 [US3] 実行場所非依存の制約と失敗条件を `specs/012-fix-run-access-path/contracts/run-script-invocation-contract.md` に反映する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 横断確認と最終手順の整合を取る

- [X] T025 [P] `server/src/main.rs` と `server/run.sh` のエラーメッセージ整合を確認し必要なら調整する
- [X] T026 `specs/012-fix-run-access-path/quickstart.md` の通し手順を実行し、結果に合わせて文言を更新する
- [X] T027 空の配信元ディレクトリを使ったときの挙動を `server/run.sh`、`server/src/main.rs`、`specs/012-fix-run-access-path/quickstart.md` で確認し必要なら文言を更新する
- [X] T028 `specs/012-fix-run-access-path/research.md` と `specs/012-fix-run-access-path/plan.md` の決定内容と実装結果の差分を確認する

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

- **User Story 1 (P1)**: 基本の案内と待受整合を最初に確立する
- **User Story 2 (P2)**: User Story 1 の起動導線を前提に配信元 override を追加する
- **User Story 3 (P3)**: User Story 2 の配信元指定を保ったまま実行場所非依存を固める

### Within Each User Story

- 検証手順または自動テストを先に定義し、受け入れ条件を固定してから実装する
- `server/run.sh` の起動インターフェース変更を先に行い、その後 `server/src/main.rs` の受け取り側を合わせる
- contract と quickstart は各 story 完了時点の挙動へ更新する

### Parallel Opportunities

- Phase 2 の `T004` は `T003` と並列で進めやすい
- 各 story の verification タスクは implementation タスクと別ファイルで並列化できる
- Polish の `T022` は `T023` と並列実行可能

---

## Parallel Example: User Story 1

```bash
Task: "起動案内の期待値を検証する自動テストを server/src/main.rs に追加または更新する"
Task: "LAN 到達確認の手順を specs/012-fix-run-access-path/quickstart.md に反映する"
```

---

## Parallel Example: User Story 2

```bash
Task: "配信元 override の挙動を検証する自動テストを server/src/main.rs に追加または更新する"
Task: "任意ディレクトリ指定での起動手順を specs/012-fix-run-access-path/quickstart.md に反映する"
```

---

## Parallel Example: User Story 3

```bash
Task: "カレントディレクトリ非依存の解決を検証する自動テストを server/src/main.rs に追加または更新する"
Task: "別ディレクトリからの起動確認手順を specs/012-fix-run-access-path/quickstart.md に反映する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 と Phase 2 を完了する
2. Phase 3 で起動案内と LAN 到達性の整合を実装する
3. User Story 1 を単独検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して LAN 利用導線を確定する
3. User Story 2 を追加して配信元 override を実装する
4. User Story 3 を追加して実行場所非依存を確定する
5. Polish で通し確認と文書整合を取る

### Parallel Team Strategy

1. 1 人が `server/run.sh`、別の 1 人が `server/src/main.rs` の検証準備を担当する
2. 各 story で quickstart / contract 更新を別担当へ分ける
3. story 完了時に通し起動確認をまとめて実施する

---

## Notes

- `[P]` は別ファイルまたは独立した検証観点で並列実行可能なタスクを示す
- 各 story は `server/run.sh` を中心にしつつ、必要最小限で `server/src/main.rs` と feature 文書を更新する
- 今回の tasks は route 追加、画像生成、`firmware/` 変更を含まない

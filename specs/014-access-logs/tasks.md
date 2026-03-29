# タスク: HTTP アクセスログ追加

**Input**: `/specs/014-access-logs/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Path Conventions

- サーバ実装: `server/src/main.rs`
- 起動導線: `server/run.sh`
- feature 成果物: `specs/014-access-logs/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: アクセスログ追加に必要な観測対象と確認導線を揃える

- [ ] T001 `server/src/main.rs` の既存リクエスト処理とレスポンス生成箇所を整理し、アクセスログ追加位置を決める
- [ ] T002 `specs/014-access-logs/quickstart.md` に正常系・失敗系・未定義 path のログ確認観点を反映する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story が使う共通ログ出力基盤を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T003 `server/src/main.rs` にアクセスログ行と応答結果情報を表す共通構造を追加する
- [ ] T004 `server/src/main.rs` に時刻・アクセス元・method・path・status を整形する共通ログヘルパーを追加する
- [ ] T005 `specs/014-access-logs/contracts/access-log-output-contract.md` と `specs/014-access-logs/research.md` の用語を実装用語へ揃える
- [ ] T006 Allowed Scope / Forbidden Scope の境界を `specs/014-access-logs/plan.md` と `specs/014-access-logs/tasks.md` で確認する

**Checkpoint**: すべての route が同じ形式でアクセスログを出せる土台が揃っていること

---

## Phase 3: User Story 1 - リクエスト到達を確認したい (Priority: P1)

**Goal**: `/` と `/image.bmp` へのアクセスごとに 1 件のアクセスログを確認できるようにする

**Independent Test**: サーバ起動中に `/` と `/image.bmp` へアクセスし、各リクエストで path を含むアクセスログが 1 行ずつ出力されることを確認する

### Verification for User Story 1

- [ ] T007 [US1] `/` と `/image.bmp` のアクセスログ出力テストを `server/src/main.rs` に追加する
- [ ] T008 [US1] 1 リクエスト 1 行のログ形式テストを `server/src/main.rs` に追加する
- [ ] T009 [US1] 正常系ログ確認手順を `specs/014-access-logs/quickstart.md` に反映する

### Implementation for User Story 1

- [ ] T010 [US1] 正常系レスポンスでアクセスログを出力する処理を `server/src/main.rs` に実装する
- [ ] T011 [US1] `/` と `/image.bmp` の両 route が同じログ形式を使うよう `server/src/main.rs` を更新する
- [ ] T012 [US1] `server/run.sh` にアクセスログ確認前提の起動案内を反映する

**Checkpoint**: User Story 1 が単独で検証可能であること

---

## Phase 4: User Story 2 - 応答結果も把握したい (Priority: P2)

**Goal**: 正常系と失敗系の応答結果をログから切り分けられるようにする

**Independent Test**: 正常系、入力画像未配置、変換不能の各ケースでアクセスし、ログに応答結果とステータスが含まれることを確認する

### Verification for User Story 2

- [ ] T013 [US2] 正常系と入力画像未配置のログ結果テストを `server/src/main.rs` に追加する
- [ ] T014 [US2] 変換不能入力時のログ結果テストを `server/src/main.rs` に追加する
- [ ] T015 [US2] 失敗系ログ確認手順を `specs/014-access-logs/quickstart.md` に反映する

### Implementation for User Story 2

- [ ] T016 [US2] 正常系と失敗系で status を含むアクセスログを出力する処理を `server/src/main.rs` に実装する
- [ ] T017 [US2] 失敗種別が分かる応答結果情報を `server/src/main.rs` に追加する
- [ ] T018 [US2] 失敗系ログの期待内容を `specs/014-access-logs/contracts/access-log-output-contract.md` に反映する

**Checkpoint**: User Story 2 が User Story 1 を壊さず独立検証可能であること

---

## Phase 5: User Story 3 - ログで利用状況を追いたい (Priority: P3)

**Goal**: 複数回アクセスや未定義 path も含めて区別可能なアクセスログを追えるようにする

**Independent Test**: 複数回アクセスと未定義 path へのアクセスを行い、各アクセスが個別のログ行として区別できることを確認する

### Verification for User Story 3

- [ ] T019 [US3] 未定義 path のアクセスログテストを `server/src/main.rs` に追加する
- [ ] T020 [US3] 複数回アクセス時のログ識別テストを `server/src/main.rs` に追加する
- [ ] T021 [US3] 取得可能な場合にアクセス元情報がログへ含まれることを確認するテストを `server/src/main.rs` に追加する
- [ ] T022 [US3] 未定義 path と複数回アクセス、アクセス元情報の確認手順を `specs/014-access-logs/quickstart.md` に反映する

### Implementation for User Story 3

- [ ] T023 [US3] 未定義 path でもアクセスログを出力する処理を `server/src/main.rs` に実装する
- [ ] T024 [US3] 複数回アクセスを識別しやすい時刻付きログ出力を `server/src/main.rs` に実装する
- [ ] T025 [US3] アクセス元情報の出力とフォールバックを `server/src/main.rs` に実装する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 通し確認と成果物整合を取る

- [ ] T026 `/` と `/image.bmp` の body / Content-Type / 失敗応答がログ追加後も従来どおりであることを確認するテストを `server/src/main.rs` に追加する
- [ ] T027 `specs/014-access-logs/research.md`、`specs/014-access-logs/plan.md`、`specs/014-access-logs/contracts/access-log-output-contract.md` の記述差分を解消する
- [ ] T028 `specs/014-access-logs/quickstart.md` の通し手順を実行し、確認結果を反映する
- [ ] T029 `server/src/main.rs` のアクセスログ実装について不要な重複を整理する

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

- **User Story 1 (P1)**: まず基本のアクセスログ出力を成立させる
- **User Story 2 (P2)**: User Story 1 のログ導線の上で応答結果の切り分けを追加する
- **User Story 3 (P3)**: User Story 2 の結果情報を維持したまま未定義 path と複数回アクセスの追跡性を高める

### Within Each User Story

- 自動テストは実装前に追加し、失敗を確認してから処理本体を実装する
- 共通ログ形式を壊さないよう、各 story の実装は既存ログ出力に追記する形で進める
- quickstart と contract は各 story 完了時点の挙動へ更新する

### Parallel Opportunities

- Phase 1 の `T002` は `T001` と並列で進められる
- `server/src/main.rs` を更新する task は原則直列で進める
- 文書更新 task は実装 task と別ファイルの範囲で並列化できる

---

## Parallel Example: User Story 1

```bash
Task: "正常系ログ確認手順を specs/014-access-logs/quickstart.md に反映する"
Task: "server/run.sh にアクセスログ確認前提の起動案内を反映する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 と Phase 2 を完了する
2. Phase 3 で `/` と `/image.bmp` の基本アクセスログ出力を実装する
3. User Story 1 を独立検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して基本アクセスログを成立させる
3. User Story 2 を追加して成功/失敗の切り分けを可能にする
4. User Story 3 を追加して未定義 path と複数回アクセスの追跡性を高める
5. Polish で成果物整合と通し確認を行う

### Parallel Team Strategy

1. 1 人が `server/src/main.rs` の共通ログ基盤を担当し、別の 1 人が quickstart / contract 更新を担当する
2. `server/src/main.rs` の変更は直列で進め、文書更新だけ並列化する
3. story 完了ごとに quickstart を更新して運用確認を早めに回す

---

## Notes

- 今回は観測性改善が主目的なので、各 story にログ確認タスクを含めている
- `server/src/main.rs` に変更が集中するため、`[P]` は文書タスクなど衝突しない範囲に限定する
- 既存 route 契約を壊さないことを最優先とする

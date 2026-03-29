# タスク: 独自画像転送形式追加

**Input**: `/specs/015-custom-transfer-format/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Path Conventions

- server 実装: `server/src/main.rs`
- firmware 実装: `firmware/main/display_update.cc`、`firmware/main/display_update.h`、`firmware/main/update_job.cc`
- feature 成果物: `specs/015-custom-transfer-format/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 独自形式追加の対象箇所と確認導線を揃える

- [ ] T001 `server/src/main.rs`、`firmware/main/display_update.cc`、`firmware/main/update_job.cc` の現行 BMP 経路、SD 保存依存箇所、`image_url` 末尾判定箇所を整理する
- [ ] T002 `specs/015-custom-transfer-format/quickstart.md` に BMP 維持確認、`/image.bin` 確認、保存なし更新確認の観点を反映する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: server と firmware が共有する独自形式契約と共通基盤を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T003 `server/src/main.rs` に独自形式ヘッダと packed payload を表す共通構造を追加する
- [ ] T004 `firmware/main/display_update.h` に独自形式受信・検証・直接描画の公開インターフェースを定義する
- [ ] T005 `specs/015-custom-transfer-format/contracts/image-bin-transfer-contract.md` と `specs/015-custom-transfer-format/data-model.md` の用語を実装用語へ揃える
- [ ] T006 `firmware/main/update_job.cc` と `server/src/main.rs` で Allowed Scope / Forbidden Scope の実装境界を確認する

**Checkpoint**: `/image.bin` の契約と firmware 側の受け口が定義済みであること

---

## Phase 3: User Story 1 - 保存なしで表示更新したい (Priority: P1)

**Goal**: firmware が `image_url` 末尾 `.bin` のときだけ独自形式経路を使って SD カード保存なしに更新できるようにする

**Independent Test**: firmware が `.bin` で終わる `image_url` を使う状態で更新を実行し、中間 BMP ファイルなしで表示更新が完了することを確認する

### Verification for User Story 1

- [ ] T007 [US1] `/image.bin` の成功応答と payload 長検証テストを `server/src/main.rs` に追加する
- [ ] T008 [US1] 独自形式のヘッダ検証と payload 完了判定テストを `firmware/main/display_update.cc` に追加する
- [ ] T009 [US1] `.bin` 末尾 URL での保存なし更新確認手順を `specs/015-custom-transfer-format/quickstart.md` に反映する

### Implementation for User Story 1

- [ ] T010 [US1] `/image.bin` 応答を生成する処理を `server/src/main.rs` に実装する
- [ ] T011 [US1] 独自形式を受信して RAM 上で検証する処理を `firmware/main/display_update.cc` と `firmware/main/display_update.h` に実装する
- [ ] T012 [US1] 検証済み payload を直接表示バッファへ反映する処理を `firmware/main/display_update.cc` に実装する
- [ ] T013 [US1] `firmware/main/update_job.cc` で `image_url` 末尾が `.bin` のときだけ SD カード保存経路を通さない更新フローへ切り替える

**Checkpoint**: User Story 1 が単独で検証可能であること

---

## Phase 4: User Story 2 - 既存の BMP クライアントを壊したくない (Priority: P2)

**Goal**: `image_url` 末尾判定による経路選択を追加しつつ `/` と `/image.bmp` の BMP 互換を維持する

**Independent Test**: `image_url` が `.bin` 以外なら BMP 経路、`.bin` なら独自形式経路を使うことを確認し、加えて `/` と `/image.bmp` が従来どおり BMP を返すことを確認する

### Verification for User Story 2

- [ ] T014 [US2] `/`、`/image.bmp`、`/image.bin` の route 切り分けテストを `server/src/main.rs` に追加する
- [ ] T015 [US2] BMP 経路の Content-Type と body 維持テストを `server/src/main.rs` に追加する
- [ ] T016 [US2] `image_url` 末尾判定と既存 BMP 経路維持手順を `specs/015-custom-transfer-format/quickstart.md` に反映する

### Implementation for User Story 2

- [ ] T017 [US2] `server/src/main.rs` で `/` と `/image.bmp` の既存 BMP 応答を維持したまま `/image.bin` route を統合する
- [ ] T018 [US2] `server/run.sh` に `/image.bin` の案内を追加しつつ BMP 取得先維持を明記する
- [ ] T019 [US2] `firmware/main/update_job.cc` と `firmware/main/config.cc` に `image_url` 末尾 `.bin` 判定による経路選択を実装し、`specs/015-custom-transfer-format/contracts/image-bin-transfer-contract.md` にも反映する

**Checkpoint**: User Story 1 と 2 が独立検証可能であること

---

## Phase 5: User Story 3 - 失敗時に安全に切り分けたい (Priority: P3)

**Goal**: 独自形式の途中中断、不正内容、入力画像問題を安全に停止し、切り分けられるようにする

**Independent Test**: 通信失敗、入力画像失敗、形式不整合を個別に発生させ、成功扱いにならず理由を区別できることを確認する

### Verification for User Story 3

- [ ] T020 [US3] `/image.bin` の空応答・不正ヘッダ・checksum 不一致テストを `server/src/main.rs` に追加する
- [ ] T021 [US3] firmware 側の通信失敗、入力失敗、形式失敗の判定テストを `firmware/main/display_update.cc` に追加する
- [ ] T022 [US3] 失敗系確認手順を `specs/015-custom-transfer-format/quickstart.md` に反映する

### Implementation for User Story 3

- [ ] T023 [US3] `server/src/main.rs` で入力画像未配置や変換不能時の `/image.bin` 応答を切り分け可能な失敗にする
- [ ] T024 [US3] `firmware/main/display_update.cc` で magic/version/length/checksum 不整合時の失敗処理を実装する
- [ ] T025 [US3] `firmware/main/update_job.cc` で独自形式経路の失敗分類を通信、入力、形式の観点で記録する
- [ ] T026 [US3] `specs/015-custom-transfer-format/contracts/image-bin-transfer-contract.md` に失敗応答期待値を反映する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 通し確認と成果物整合を取る

- [ ] T027 同じ入力画像で BMP 経路と独自形式経路の最終表示が一致する確認を `specs/015-custom-transfer-format/quickstart.md` に反映する
- [ ] T028 `specs/015-custom-transfer-format/research.md`、`specs/015-custom-transfer-format/plan.md`、`specs/015-custom-transfer-format/contracts/image-bin-transfer-contract.md` の記述差分を解消する
- [ ] T029 `server/src/main.rs` と `firmware/main/display_update.cc` の重複ロジックを整理する
- [ ] T030 `specs/015-custom-transfer-format/quickstart.md` の通し手順を実行し、確認結果を反映する

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

- **User Story 1 (P1)**: まず保存なし更新の成立を優先する
- **User Story 2 (P2)**: User Story 1 の独自形式追加を前提に、既存 BMP 互換を固定する
- **User Story 3 (P3)**: User Story 1 と 2 の経路を前提に失敗切り分けを強化する

### Within Each User Story

- 検証タスクは実装前に追加し、期待する失敗を確認してから処理本体を実装する
- server の route / contract 変更と firmware の更新経路変更は契約を壊さない順序で進める
- quickstart と contract は各 story 完了時点の挙動へ更新する

### Parallel Opportunities

- Phase 1 の `T002` は `T001` と並列で進められる
- `T007` と `T008` は server / firmware の別ファイルなので並列化できる
- `T014` と `T015` は同じ `server/src/main.rs` に触るため直列が望ましい
- `T020` と `T021` は server / firmware の別ファイルなので並列化できる

---

## Parallel Example: User Story 1

```bash
Task: "/image.bin の成功応答と payload 長検証テストを server/src/main.rs に追加する"
Task: "独自形式のヘッダ検証と payload 完了判定テストを firmware/main/display_update.cc に追加する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 と Phase 2 を完了する
2. Phase 3 で `/image.bin` と保存なし更新経路を実装する
3. User Story 1 を独立検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して保存なし更新を成立させる
3. User Story 2 を追加して BMP 互換を固定する
4. User Story 3 を追加して失敗切り分けを強化する
5. Polish で成果物整合と通し確認を行う

### Parallel Team Strategy

1. 1 人が server 側 route / payload 生成を担当し、別の 1 人が firmware 側受信 / 描画を担当する
2. 契約変更は `contracts/` と `quickstart.md` を別担当で並行更新できる
3. story 完了ごとに quickstart を更新し、実機または手動検証を早めに回す

---

## Notes

- 今回は server と firmware の両側変更なので、契約文書と quickstart を各 story で更新する
- `server/src/main.rs` と `firmware/main/display_update.cc` に変更が集中するため、同一ファイル上のタスクは原則直列で進める
- `/` と `/image.bmp` の BMP 互換維持を最優先とし、`/image.bin` は追加導線として実装する

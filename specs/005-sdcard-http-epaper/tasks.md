# タスク: SDカード設定 HTTP e-paper 更新ファーム

**Input**: `/specs/005-sdcard-http-epaper/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Path Conventions

- 新規実装: `firmware/`
- feature 設計成果物: `specs/005-sdcard-http-epaper/`
- 参照専用コード: `xiaozhi-esp32/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: `firmware/` 配下の実装土台と参照境界を確定する

- [ ] T001 `firmware/CMakeLists.txt`、`firmware/sdkconfig.defaults`、`firmware/main/CMakeLists.txt`、`firmware/main/main.cc` と、`firmware/main/` 配下の設定読込・更新ジョブ・表示更新・失敗制御用ファイルを置ける初期ディレクトリ構成を作成し、実装先を `firmware/` に固定する
- [ ] T002 `specs/005-sdcard-http-epaper/plan.md` と `AGENTS.md` に沿って、`xiaozhi-esp32/` を参照専用とする実装境界を確認する
- [ ] T003 [P] `xiaozhi-esp32/components/sdcard_bsp`、`button_bsp`、`epaper_port`、HTTP 関連部品の参照メモを `specs/005-sdcard-http-epaper/research.md` と照合する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に共通する設定読込・更新制御・失敗制御の基盤を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T004 `specs/005-sdcard-http-epaper/contracts/config-and-update-contract.md` を基に、`wifi_ssid`、`wifi_password`、`image_url` の必須項目を持つ `config.json` 読込契約を反映する設定モデルを `firmware/` 配下に実装する
- [ ] T005 [P] `specs/005-sdcard-http-epaper/data-model.md` を基に、`firmware/` 配下に更新ジョブ状態と失敗状態の共通モデルを実装する
- [ ] T006 [P] `firmware/` 配下に起動時更新と BOOT ボタン更新が重複しない直列実行制御を実装する
- [ ] T007 `firmware/` 配下に失敗時シャットダウンの共通制御と失敗理由の保持経路を実装する
- [ ] T008 `specs/005-sdcard-http-epaper/quickstart.md` に沿って、正常系/失敗系の手動確認観点を実装境界に対応づける

**Checkpoint**: 基盤完了後に user story 実装へ進む

---

## Phase 3: User Story 1 - 起動時に最新画像を表示できる (Priority: P1)

**Goal**: 起動時に `config.json` を読み、WiFi 接続、HTTP 画像取得、e-paper 更新を完了できるようにする

**Independent Test**: SDカードルートに正しい `config.json` を置いて起動し、60 秒以内に画像更新が完了することを確認する

### Verification for User Story 1

- [ ] T009 [US1] User Story 1 の手動確認手順を `specs/005-sdcard-http-epaper/quickstart.md` に実装観点付きで具体化する

### Implementation for User Story 1

- [ ] T010 [P] [US1] `firmware/` 配下に SDカードルートの `config.json` を読み込み、`wifi_ssid`、`wifi_password`、`image_url` の必須項目を検証する処理を実装する
- [ ] T011 [P] [US1] `firmware/` 配下に起動時 WiFi 接続処理と画像取得前提確認処理を実装する
- [ ] T012 [US1] `firmware/` 配下に起動時の HTTP 画像取得処理と画像妥当性確認処理を実装する
- [ ] T013 [US1] `firmware/` 配下に e-paper 表示更新処理を実装し、起動時更新フローへ統合する
- [ ] T014 [US1] `firmware/` 配下に起動時更新の成功経路をまとめ、`config.json` 読込から表示更新までを一連化する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - BOOTボタンで手動更新できる (Priority: P2)

**Goal**: 起動後に BOOT ボタン押下で画像再取得と e-paper 再更新を実行できるようにする

**Independent Test**: 起動後に BOOT ボタンを押し、60 秒以内に画像再取得と表示更新が完了することを確認する

### Verification for User Story 2

- [ ] T015 [US2] User Story 2 の手動確認手順を `specs/005-sdcard-http-epaper/quickstart.md` に追記する

### Implementation for User Story 2

- [ ] T016 [P] [US2] `firmware/` 配下に BOOT ボタン押下の監視と更新要求発行処理を実装する
- [ ] T017 [P] [US2] `firmware/` 配下に手動更新トリガから既存更新ジョブを再利用する処理を実装する
- [ ] T018 [US2] `firmware/` 配下で BOOT ボタン連打時に更新ジョブが重複しない保護を追加する
- [ ] T019 [US2] `firmware/` 配下に BOOT ボタン更新の成功経路を統合し、起動後再更新を完結させる

**Checkpoint**: User Story 1 と 2 が独立検証可能であること

---

## Phase 5: User Story 3 - 設定不備や取得失敗時でも原因を切り分けられる (Priority: P3)

**Goal**: 設定不備、WiFi 失敗、HTTP 失敗、画像不正時に、原因を切り分け可能なまま終了しシャットダウンする

**Independent Test**: 各失敗系を個別に発生させ、失敗種別を判断できる状態で更新処理が終了しシャットダウンすることを確認する

### Verification for User Story 3

- [ ] T020 [US3] User Story 3 の手動確認手順を `specs/005-sdcard-http-epaper/quickstart.md` に追記する

### Implementation for User Story 3

- [ ] T021 [P] [US3] `firmware/` 配下に `config.json` 欠落・不正形式・必須項目不足時の失敗処理を実装する
- [ ] T022 [P] [US3] `firmware/` 配下に WiFi 接続失敗と HTTP 取得失敗時の失敗分類処理を実装する
- [ ] T023 [P] [US3] `firmware/` 配下に画像不正時の失敗分類と更新中断処理を実装する
- [ ] T024 [US3] `firmware/` 配下に失敗種別を残してシャットダウンする最終処理を統合する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 仕上げ、文書整合、総合確認

- [ ] T025 [P] `firmware/` 配下の実装と `specs/005-sdcard-http-epaper/spec.md`、`plan.md`、`tasks.md` の整合性を確認する
- [ ] T026 `specs/005-sdcard-http-epaper/quickstart.md` に従って正常系 2 本と失敗系 4 本の手動確認を実施する
- [ ] T027 [P] `specs/005-sdcard-http-epaper/contracts/config-and-update-contract.md`、`specs/005-sdcard-http-epaper/quickstart.md`、`docs/firmware-http-epaper.md` を見直し、`config.json` 仕様、SDカード配置方法、起動時更新、BOOT ボタン更新、失敗時運用説明を更新する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Phase 6)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の更新ジョブが動作した後に開始する
- **User Story 3 (P3)**: User Story 1 と 2 の基本経路が揃った後に開始する

### Within Each User Story

- 手動確認手順を先に明示してから実装する
- 設定/状態モデルを先に使い、更新フローへ統合する
- BOOT ボタン更新は起動時更新の既存経路を再利用する
- 失敗時処理は個別失敗分類を先に作ってからシャットダウン統合を行う

### Parallel Opportunities

- `[P]` 付き Setup / Foundational タスクは並列実行可能
- User Story 1 では設定読込と WiFi 接続準備を並列に進められる
- User Story 2 ではボタン監視と再利用経路の整備を並列に進められる
- User Story 3 では失敗分類ごとの処理を並列に進められる

---

## Parallel Example: User Story 1

```bash
Task: "`firmware/` 配下に `config.json` 読込処理と必須項目検証処理を実装する"
Task: "`firmware/` 配下に起動時 WiFi 接続処理と画像取得前提確認処理を実装する"
```

## Parallel Example: User Story 3

```bash
Task: "`firmware/` 配下に `config.json` 欠落・不正形式・必須項目不足時の失敗処理を実装する"
Task: "`firmware/` 配下に WiFi 接続失敗と HTTP 取得失敗時の失敗分類処理を実装する"
Task: "`firmware/` 配下に画像不正時の失敗分類と更新中断処理を実装する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. User Story 1 を独立検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して独立検証する
3. User Story 2 を追加して独立検証する
4. User Story 3 を追加して独立検証する
5. 最後に Polish で総合確認する

### Parallel Team Strategy

1. 1 人が `firmware/` の基盤と更新ジョブ共通部を担当する
2. 1 人が起動時/BOOT 更新の成功経路を担当する
3. 1 人が失敗分類とシャットダウン経路を担当する

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために必須
- 今回の feature では自動テスト追加ではなく、手動確認手順を必須とする
- `xiaozhi-esp32/` は参照専用であり、変更先は `firmware/` のみとする

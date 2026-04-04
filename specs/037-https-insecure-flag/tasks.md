# タスク: Config Insecure HTTPS

**Input**: `/specs/037-https-insecure-flag/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: `config.txt` の HTTPS / insecure 契約と実装境界を固定する

- [ ] T001 `specs/037-https-insecure-flag/spec.md`、`plan.md`、`contracts/insecure-https-config-contract.md` を照合し、`http://` 維持・`https://` 許可・`insecure` 任意 boolean の前提を実装開始条件として確定する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 設定読込と通信方針判定の責務分担を揃える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T002 `firmware/main/config.h` に `insecure` を含む `FirmwareConfig` 拡張方針を反映し、`specs/037-https-insecure-flag/data-model.md` と整合させる
- [ ] T003 [P] `firmware/main/config.cc` に `http://` / `https://` 判定、`insecure` 正規化、型不正検出を追加する設計を反映する
- [ ] T004 [P] `firmware/main/display_update.cc` と `firmware/main/display_update.h` に transport policy 受け渡しを追加する更新範囲を整理する

**Checkpoint**: config schema と HTTP client 設定責務が確定していること

---

## Phase 3: User Story 1 - HTTPS 検証省略で更新する (Priority: P1)

**Goal**: `insecure: true` のときだけ証明書未検証 HTTPS で画像取得を成功させる

**Independent Test**: `https://` の `image_url` と `insecure: true` を設定した状態で `idf.py build` が通り、実機または手動検証で証明書未検証 HTTPS 更新が成功することを確認する

### Verification for User Story 1

- [ ] T005 [US1] `specs/037-https-insecure-flag/quickstart.md` に証明書未検証 HTTPS 更新の確認手順を実装前提として整理する

### Implementation for User Story 1

- [ ] T006 [US1] `firmware/main/config.h` と `firmware/main/config.cc` に `insecure` の保持と `https://` 受理を実装する
- [ ] T007 [US1] `firmware/main/display_update.h` と `firmware/main/display_update.cc` に `insecure: true` 時の HTTPS 証明書未検証 client 設定分岐を実装する
- [ ] T008 [US1] `firmware/main/update_job.cc` に `insecure` を含む画像取得呼び出しを反映し、BMP / binary 両経路で未検証 HTTPS を使えるようにする

**Checkpoint**: `insecure: true` の HTTPS 更新が独立して成功できること

---

## Phase 4: User Story 2 - 既定では安全側を維持する (Priority: P2)

**Goal**: `insecure` 未設定または `false` では、HTTP 回帰を壊さず通常の HTTPS 証明書検証を維持する

**Independent Test**: `http://` の既存更新が維持され、`https://` + `insecure: false` は検証可能サーバーで成功し、検証不能サーバーでは失敗扱いになることを確認する

### Verification for User Story 2

- [ ] T009 [US2] `specs/037-https-insecure-flag/quickstart.md` に HTTP 回帰確認と検証付き HTTPS 確認手順を整理する

### Implementation for User Story 2

- [ ] T010 [US2] `firmware/main/display_update.cc` に certificate bundle を使う通常 HTTPS client 設定を実装する
- [ ] T011 [US2] `firmware/main/config.cc` と `firmware/main/update_job.cc` で `insecure` 未設定時を `false` として扱い、HTTP 経路の既存挙動を維持する
- [ ] T012 [US2] `docs/firmware-http-epaper.md` に `https://` 利用時の既定挙動、`insecure` 未設定時の安全側動作、HTTP 後方互換を追記する

**Checkpoint**: HTTP と検証付き HTTPS の既定挙動が独立して確認できること

---

## Phase 5: User Story 3 - 設定ミスを判別する (Priority: P3)

**Goal**: `insecure` の型不正や無効設定を通信開始前に config error として切り分ける

**Independent Test**: `config.txt` の `insecure` に boolean 以外を設定したとき、WiFi 接続や HTTP 通信前に設定不備として失敗することを確認する

### Verification for User Story 3

- [ ] T013 [US3] `specs/037-https-insecure-flag/quickstart.md` に `insecure` 型不正時の確認手順を整理する

### Implementation for User Story 3

- [ ] T014 [US3] `firmware/main/config.cc` に `insecure` 型不正時の検出とエラー詳細生成を実装する
- [ ] T015 [US3] `firmware/main/update_job.cc` と `docs/firmware-http-epaper.md` に config error と通信失敗の切り分け観点を反映する

**Checkpoint**: `insecure` 設定不備が画像取得失敗と区別されること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 最終整合とビルド回帰確認を行う

- [ ] T016 [P] `firmware/` で `idf.py build` を実行し、`firmware/main/config.*`、`display_update.*`、`update_job.cc` の変更がビルド可能であることを確認する
- [ ] T017 `specs/037-https-insecure-flag/plan.md`、`contracts/insecure-https-config-contract.md`、`tasks.md` の記述整合を確認する
- [ ] T018 `docs/firmware-http-epaper.md` と `specs/037-https-insecure-flag/quickstart.md` の確認手順が一致することを確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能。MVP
- **User Story 2 (P2)**: User Story 1 の HTTPS 経路追加を前提に開始する
- **User Story 3 (P3)**: Foundational 後に開始可能だが、設定仕様確定後に着手すると手戻りが少ない

### Parallel Opportunities

- Phase 2 の T003 と T004 は並列実行可能
- Polish の T016 と T017 は並列実行可能

## Parallel Example: User Story 1

```bash
Task: "`specs/037-https-insecure-flag/quickstart.md` に証明書未検証 HTTPS 更新の確認手順を整理する"
Task: "`firmware/main/config.h` と `firmware/main/config.cc` に `insecure` の保持と `https://` 受理を実装する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. 証明書未検証 HTTPS 更新を独立検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して未検証 HTTPS を成立させる
3. User Story 2 を追加して既定安全挙動と HTTP 回帰を固める
4. User Story 3 を追加して設定不備の切り分けを固める
5. 最後に build と文書整合を確認する

### Parallel Team Strategy

1. 1 人が Foundational を完了する
2. その後 US1 / US3 を別担当で進める
3. US2 は HTTPS 既定挙動と文書整合の担当として後続でまとめる

---

## Notes

- 実装対象は `firmware/`、`docs/firmware-http-epaper.md`、feature artifact に限定する
- `xiaozhi-esp32/` は参照専用で変更しない
- `insecure` は HTTPS の証明書検証例外にのみ影響し、route 選択や他 failure category の意味は変えない

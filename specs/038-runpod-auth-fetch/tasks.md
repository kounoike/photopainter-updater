# タスク: RunPod Authenticated Fetch

**Input**: `/specs/038-runpod-auth-fetch/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 外部認証付き取得の契約と実装境界を固定する

- [ ] T001 `specs/038-runpod-auth-fetch/spec.md`、`plan.md`、`contracts/authenticated-fetch-contract.md` を照合し、`bearer_token`・`insecure`・`https://` の前提を実装開始条件として確定する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 設定読込、認証条件、通信方針の責務分担を揃える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T002 `firmware/main/config.h` に `insecure` と `bearer_token` を含む `FirmwareConfig` の新規フィールド定義と関連関数宣言を追加し、`specs/038-runpod-auth-fetch/data-model.md` と整合させる
- [ ] T003 [P] `firmware/main/config.cc` に `http://` / `https://` の URL scheme 検証、`insecure` の既定値 `false` 正規化、`bearer_token` の型不正・空文字検出を実装する
- [ ] T004 [P] `firmware/main/display_update.h` と `firmware/main/display_update.cc` に `bearer_token` と HTTPS transport policy を受け取るダウンロード API 変更を実装する

**Checkpoint**: config schema と HTTP client 設定責務が確定していること

---

## Phase 3: User Story 1 - Bearer 認証付き HTTPS で更新する (Priority: P1)

**Goal**: Bearer トークン付き HTTPS 更新元から画像取得して表示更新を成功させる

**Independent Test**: `https://` の `image_url` と有効な `bearer_token` を設定した状態で `idf.py build` が通り、手動検証で Bearer 認証付き HTTPS 更新が成功することを確認する

### Verification for User Story 1

- [ ] T005 [US1] `specs/038-runpod-auth-fetch/quickstart.md` に Bearer 認証付き HTTPS 更新の確認手順を実装前提として整理する

### Implementation for User Story 1

- [ ] T006 [US1] `firmware/main/config.h` と `firmware/main/config.cc` に `bearer_token` 保持、`https://` 受理、空文字拒否を実装する
- [ ] T007 [US1] `firmware/main/display_update.h` と `firmware/main/display_update.cc` に `Authorization: Bearer <token>` ヘッダ付与を実装する
- [ ] T008 [US1] `firmware/main/update_job.cc` に認証設定を含む画像取得呼び出しを反映し、BMP / binary 両経路で Bearer 認証付き HTTPS を使えるようにする

**Checkpoint**: Bearer 認証付き HTTPS 更新が独立して成功できること

---

## Phase 4: User Story 2 - 外部 HTTPS 運用の例外設定を扱う (Priority: P2)

**Goal**: `insecure: true` のときだけ未検証 HTTPS でも Bearer 認証付き更新を継続できるようにする

**Independent Test**: `https://`、有効な `bearer_token`、`insecure: true` を設定した状態で、通常の証明書検証では通過できない更新元に対して認証付き更新が成功することを確認する

### Verification for User Story 2

- [ ] T009 [US2] `specs/038-runpod-auth-fetch/quickstart.md` に未検証 HTTPS と安全側既定値の確認手順を整理する

### Implementation for User Story 2

- [ ] T010 [US2] `firmware/main/display_update.cc` に certificate bundle を使う通常 HTTPS 経路と `insecure: true` の未検証 HTTPS 経路を両立する client 設定を実装し、BMP / binary 両経路で verified HTTPS を維持する
- [ ] T011 [US2] `firmware/main/config.cc` と `firmware/main/update_job.cc` で `insecure` 未設定時を `false` として扱い、HTTP 回帰と安全側既定値を維持する
- [ ] T012 [US2] `docs/firmware-http-epaper.md` に `bearer_token` と `insecure` を併用する外部 HTTPS 運用手順と注意点を追記する

**Checkpoint**: 未検証 HTTPS 例外設定と既定安全挙動が独立して確認できること

---

## Phase 5: User Story 3 - 認証設定ミスを判別する (Priority: P3)

**Goal**: `bearer_token` と `insecure` の設定不備、認証拒否、通信失敗を切り分ける

**Independent Test**: `bearer_token` の型不正、空文字、無効 token、`insecure` 型不正の各ケースで、通信開始前の設定不備と通信後の認証失敗が区別されることを確認する

### Verification for User Story 3

- [ ] T013 [US3] `specs/038-runpod-auth-fetch/quickstart.md` に設定不備と認証失敗の確認手順を整理する

### Implementation for User Story 3

- [ ] T014 [US3] `firmware/main/config.cc` に `bearer_token` 型不正、空文字、`insecure` 型不正の検出とエラー詳細生成を実装する
- [ ] T015 [US3] `firmware/main/update_job.cc` と `docs/firmware-http-epaper.md` に config error、認証失敗、通信失敗の切り分け観点を反映する

**Checkpoint**: 設定不備と認証失敗が画像取得失敗全般と区別されること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 最終整合とビルド回帰確認を行う

- [ ] T016 [P] `firmware/` で `idf.py build` を実行し、`firmware/main/config.*`、`display_update.*`、`update_job.cc` の変更がビルド可能であることを確認する
- [ ] T017 `specs/038-runpod-auth-fetch/plan.md`、`contracts/authenticated-fetch-contract.md`、`tasks.md` の記述整合を確認する
- [ ] T018 `docs/firmware-http-epaper.md` と `specs/038-runpod-auth-fetch/quickstart.md` の確認手順が一致することを確認する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Final Phase)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能。MVP
- **User Story 2 (P2)**: User Story 1 の Bearer 認証付き HTTPS 経路追加を前提に開始する
- **User Story 3 (P3)**: Foundational 後に開始可能だが、US1 / US2 の設定項目確定後に着手すると手戻りが少ない

### Parallel Opportunities

- Phase 2 の T003 と T004 は並列実行可能
- Polish の T016 と T017 は並列実行可能

## Parallel Example: User Story 1

```bash
Task: "`specs/038-runpod-auth-fetch/quickstart.md` に Bearer 認証付き HTTPS 更新の確認手順を整理する"
Task: "`firmware/main/config.h` と `firmware/main/config.cc` に `bearer_token` 保持、`https://` 受理、空文字拒否を実装する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. Bearer 認証付き HTTPS 更新を独立検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して Bearer 認証付き HTTPS を成立させる
3. User Story 2 を追加して未検証 HTTPS 例外設定と安全側既定値を固める
4. User Story 3 を追加して設定不備と認証失敗の切り分けを固める
5. 最後に build と文書整合を確認する

### Parallel Team Strategy

1. 1 人が Foundational を完了する
2. その後 US1 / US3 を別担当で進める
3. US2 は HTTPS 検証分岐と文書整合の担当として後続でまとめる

---

## Notes

- 実装対象は `firmware/`、`docs/firmware-http-epaper.md`、feature artifact に限定する
- `xiaozhi-esp32/` は参照専用で変更しない
- `bearer_token` は Bearer 認証ヘッダにのみ使い、秘密情報保護の高度化は今回扱わない
- `insecure` は HTTPS の証明書検証例外にのみ影響し、route 選択や他 failure category の意味は変えない

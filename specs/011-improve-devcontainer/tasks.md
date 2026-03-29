# タスク: Devcontainer 起動改善

**Input**: `/home/kounoike/ghq/github.com/kounoike/photopainter-updater/specs/011-improve-devcontainer/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3`)
- タスク記述には正確なファイルパスを含める

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: devcontainer 改善の対象ファイルと検証導線を揃える

- [X] T001 `.devcontainer/devcontainer.json`、`.devcontainer/Dockerfile`、`docs/firmware.md` の現状差分観点を `specs/011-improve-devcontainer/plan.md` に照らして整理する
- [X] T002 [P] `specs/011-improve-devcontainer/quickstart.md` を実装前提の確認手順として見直し、`codex` と `claude` の確認観点を不足なく揃える
- [X] T003 [P] `specs/011-improve-devcontainer/contracts/devcontainer-readiness-contract.md` を実装対象ファイルへ対応付け、接続直後利用・認証保持・初期化の契約境界を明確化する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に共通する devcontainer 基盤変更を先に整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [X] T004 `.devcontainer/Dockerfile` に主要 CLI を image build 時点で利用可能にする共通セットアップを実装する
- [X] T005 `.devcontainer/devcontainer.json` から長い追加セットアップ待ちを発生させる構成を除去または置換し、接続直後利用の前提へ合わせる
- [X] T006 [P] `.devcontainer/devcontainer.json` に認証キャッシュ用の永続領域を定義し、Git 管理対象と分離した保持先を共通基盤として追加する
- [X] T007 [P] `.devcontainer/devcontainer.json` と `.devcontainer/.env.example` を見直し、追加した永続化構成が既存 ESP-IDF 前提を壊さないことを確認する

**Checkpoint**: devcontainer の build/attach と認証保持の前提が固まっていること

---

## Phase 3: User Story 1 - 初回セットアップ待ち時間を減らしたい (Priority: P1)

**Goal**: devcontainer 新規作成直後に `codex` と `claude` を追加セットアップなしで利用可能にする

**Independent Test**: devcontainer を新規作成し、追加の手動インストール待ちなしで `codex` と `claude` が利用可能になっていることを確認する

### Verification for User Story 1

- [X] T008 [US1] `specs/011-improve-devcontainer/quickstart.md` の「新規作成直後の利用確認」に沿った手動確認結果を記録する

### Implementation for User Story 1

- [X] T009 [US1] `.devcontainer/Dockerfile` に `codex` と `claude` を接続直後利用可能にするインストール処理を実装する
- [X] T010 [US1] `.devcontainer/devcontainer.json` を接続直後に `codex` と `claude` を利用できる構成へ更新する
- [X] T011 [US1] `.devcontainer/devcontainer.json` と `.devcontainer/Dockerfile` の役割分担を整理し、初回作成時の追加待機が発生しないことを確認する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - コンテナ再生成後も認証状態を維持したい (Priority: P2)

**Goal**: devcontainer の rebuild/recreate 後も `codex` と `claude` の認証状態を再利用できるようにする

**Independent Test**: 認証済み状態で devcontainer を再生成し、再接続後に `codex` と `claude` が再認証なしで利用継続できることを確認する

### Verification for User Story 2

- [X] T012 [US2] `specs/011-improve-devcontainer/quickstart.md` の「初回認証と再利用確認」と「Git 管理対象からの分離確認」に沿った手動確認結果を記録する

### Implementation for User Story 2

- [X] T013 [US2] `.devcontainer/devcontainer.json` に `codex` と `claude` の認証状態を再生成後も再利用できる mount/保持設定を実装する
- [X] T014 [US2] `.devcontainer/devcontainer.json` の永続化対象を `codex` と `claude` の認証継続に必要な最小範囲へ絞り、ワークスペースの Git 管理対象から分離する
- [X] T015 [US2] `.devcontainer/.env.example` と必要な関連設定を更新し、認証保持構成の前提を明示する

**Checkpoint**: User Story 2 が独立して検証可能であること

---

## Phase 5: User Story 3 - 認証キャッシュの扱いを安全に把握したい (Priority: P3)

**Goal**: 開発者が `codex` と `claude` の保持対象、初期化方法、再認証が必要な条件を文書だけで判断できるようにする

**Independent Test**: 開発者向け手順を読み、保持対象・初期化方法・再認証が必要になる条件を判断できることを確認する

### Verification for User Story 3

- [X] T016 [US3] `specs/011-improve-devcontainer/quickstart.md` の「初期化手順の確認」に沿って、永続領域の初期化手順と再認証復帰条件の文書確認結果を記録する

### Implementation for User Story 3

- [X] T017 [US3] `docs/firmware.md` に `codex` と `claude` の初回認証、再生成後の再利用、永続領域を消去する意図的な初期化方法を追記する
- [X] T018 [P] [US3] `docs/firmware-http-epaper.md` の devcontainer 前提説明を見直し、認証保持運用と矛盾しない表現へ更新する
- [X] T019 [US3] `specs/011-improve-devcontainer/contracts/devcontainer-readiness-contract.md` と `specs/011-improve-devcontainer/quickstart.md` を実装後の運用説明に合わせて整合させる

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 実装・文書・検証結果を横断的に整える

- [X] T020 [P] `specs/011-improve-devcontainer/spec.md`、`specs/011-improve-devcontainer/plan.md`、`specs/011-improve-devcontainer/tasks.md` の記述が最終実装方針と矛盾しないことを確認する
- [X] T021 `.devcontainer/devcontainer.json`、`.devcontainer/Dockerfile`、`docs/firmware.md`、`docs/firmware-http-epaper.md` の表現を統一し、認証情報を Git 管理対象へ保存しない方針を再確認する
- [X] T022 `specs/011-improve-devcontainer/quickstart.md` の 4 手順を通した手動検証結果をまとめ、残課題があれば `specs/011-improve-devcontainer/plan.md` に記録する

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Phase 6)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 後に開始可能
- **User Story 2 (P2)**: User Story 1 の CLI 提供構成を前提に開始する
- **User Story 3 (P3)**: User Story 2 の認証保持構成を前提に開始する

### Within Each User Story

- 手動確認手順を先に固定する
- `.devcontainer` の設定変更を先に実装する
- 関連文書と契約の整合確認を最後に行う

### Parallel Opportunities

- Setup では `T002` と `T003` を並列化できる
- Foundational では `T006` と `T007` を並列化できる
- User Story 3 では `T017` と `T018` を並列化できる
- Polish では `T020` と `T022` を並列化できる

---

## Parallel Example: User Story 2

```bash
Task: "`specs/011-improve-devcontainer/quickstart.md` の「初回認証と再利用確認」と「Git 管理対象からの分離確認」に沿った手動確認結果を記録する"
Task: "`.devcontainer/devcontainer.json` に対象 CLI の認証状態を再生成後も再利用できる mount/保持設定を実装する"
Task: "`.devcontainer/.env.example` と必要な関連設定を更新し、認証保持構成の前提を明示する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. devcontainer 新規作成直後に主要 CLI を利用できることを確認する

### Incremental Delivery

1. Setup + Foundational を完了する
2. User Story 1 を追加して接続直後利用を成立させる
3. User Story 2 を追加して認証保持を成立させる
4. User Story 3 を追加して運用手順を完成させる
5. Polish で文書と検証結果を整える

### Parallel Team Strategy

1. 1 人が `.devcontainer/Dockerfile` と `.devcontainer/devcontainer.json` の基盤変更を進める
2. もう 1 人が quickstart と contract の検証観点を整える
3. 認証保持実装後に文書更新担当が `docs/firmware.md` と `docs/firmware-http-epaper.md` を仕上げる

---

## Notes

- `[P]` は別ファイルまたは独立依存で編集衝突しにくいタスクを示す
- `[US1]`、`[US2]`、`[US3]` は story traceability のため必須
- アプリケーション本体、外部サービス側の認証仕様、Git 管理対象への認証保存は今回のタスクに含めない

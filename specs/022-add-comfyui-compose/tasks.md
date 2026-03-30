---
description: "機能実装のためのタスクリスト"
---

# タスク: ComfyUI Docker Compose 統合

**Input**: `/specs/022-add-comfyui-compose/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`quickstart.md`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能（別ファイル・独立依存）
- **[Story]**: 対応する user story (`US1`, `US2`, `US3`)
- タスク記述には正確なファイルパスを含める

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: `.gitignore` と `.env.example` の基盤整備

- [x] T001 [P] `.gitignore` にリポジトリルートで `.env` と `comfyui-data/` を追加する（`.gitignore`）
- [x] T002 [P] `.env.example` をリポジトリルートに作成する（`COMFYUI_PORT=18188`、`COMFYUI_DATA_DIR=./comfyui-data`、`COMFYUI_CLI_ARGS=--fast` のコメント付きテンプレート）（`.env.example`）

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: `compose.yml` の骨格とネットワーク定義を作成し、全 user story の基盤とする

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [x] T003 `compose.yml` をリポジトリルートに新規作成し、`networks: photopainter` (driver: bridge) のみを定義したスケルトンを作成する（`compose.yml`）
- [x] T004 `docker compose config` を実行し `compose.yml` が正しくパースされることを確認する（コマンドライン検証）

**Checkpoint**: `docker compose config` が警告・エラーなく通過すること

---

## Phase 3: User Story 1 - ComfyUI をローカルで起動する (Priority: P1)

**Goal**: `docker compose up` 一発で ComfyUI Web UI にアクセスできる

**Independent Test**: `docker compose up` → `http://localhost:18188` でブラウザアクセス成功 → `docker compose down` で正常停止

### Implementation for User Story 1

- [x] T005 [US1] `compose.yml` の comfyui サービス基本定義を追加する（image: `yanwk/comfyui-boot:cu130-slim-v2`、container_name: `photopainter-comfyui`、ports: `0.0.0.0:${COMFYUI_PORT:-18188}:8188`、networks: photopainter、restart: `unless-stopped`）（`compose.yml`）
- [x] T006 [US1] `compose.yml` の comfyui サービスに healthcheck を追加する（test: `curl -f http://localhost:8188/system_stats`、interval: 30s、timeout: 10s、retries: 3、start_period: 60s）（`compose.yml`）
- [x] T007 [US1] `docker compose config` で US1 の設定が正しく解決されることを確認する（コマンドライン検証）

### Verification for User Story 1

- [ ] T008 [US1] 手動確認: `docker compose up` を実行して `http://localhost:18188` にブラウザでアクセスし、ComfyUI Web UI が表示されること、`docker compose down` で正常停止することを `quickstart.md` の手順に沿って検証する（**GPU実機環境で実施**）

**Checkpoint**: Web UI アクセス成功・正常停止が確認できること

---

## Phase 4: User Story 2 - モデル・カスタムノード・生成画像を永続保存する (Priority: P2)

**Goal**: コンテナ削除後もモデル・カスタムノード・依存ライブラリ・生成画像が保持される

**Independent Test**: ダミーファイルを `comfyui-data/models/` に配置 → `docker compose down` → `docker compose up` → ファイルが残っていること

### Implementation for User Story 2

- [x] T009 [US2] `compose.yml` の comfyui サービスに必須 bind mount を追加する（`${COMFYUI_DATA_DIR:-./comfyui-data}/models:/root/ComfyUI/models`、`custom_nodes:/root/ComfyUI/custom_nodes`、`dot-local:/root/.local`、`output:/root/ComfyUI/output`）（`compose.yml`）
- [x] T010 [US2] `compose.yml` の comfyui サービスに推奨 bind mount を追加する（`user:/root/ComfyUI/user`、`input:/root/ComfyUI/input`、`dot-cache:/root/.cache`）（`compose.yml`）
- [x] T011 [US2] `docker compose config` で全ボリューム変数が正しく展開されることを確認する（コマンドライン検証）

### Verification for User Story 2

- [ ] T012 [US2] 手動確認（モデル永続化）: ダミーファイルを `${COMFYUI_DATA_DIR:-./comfyui-data}/models/` に配置 → `docker compose down` → `docker compose up` → ファイルが残っていることを確認する（**GPU実機環境で実施**）
- [ ] T013 [US2] 手動確認（出力永続化）: ComfyUI でサンプル画像を生成 → `docker compose down` → `comfyui-data/output/` に画像ファイルが残っていることを確認する（**GPU実機環境で実施**）
- [ ] T013b [US2] 手動確認（カスタムノード・依存ライブラリ永続化）: ComfyUI Manager でカスタムノードをインストール → `docker compose down && docker compose up` → `comfyui-data/custom_nodes/` にノードのディレクトリが残存していること、かつ `comfyui-data/dot-local/` に pip 依存ファイルが残存していることを確認する（**GPU実機環境で実施**）

**Checkpoint**: コンテナ再作成後もモデル・生成画像・カスタムノード・依存ライブラリが保持されること

---

## Phase 5: User Story 3 - GPU アクセラレーションを利用する (Priority: P3)

**Goal**: NVIDIA GPU がコンテナに渡され、ComfyUI が GPU で動作する

**Independent Test**: `docker exec photopainter-comfyui nvidia-smi` で GPU が認識されること

### Implementation for User Story 3

- [x] T014 [US3] `compose.yml` の comfyui サービスに GPU 設定を追加する（`deploy.resources.reservations.devices`: driver: nvidia, count: all, capabilities: [gpu]）（`compose.yml`）
- [x] T015 [US3] `compose.yml` の comfyui サービスに環境変数を追加する（`NVIDIA_VISIBLE_DEVICES=all`、`NVIDIA_DRIVER_CAPABILITIES=compute,utility`、`CLI_ARGS=${COMFYUI_CLI_ARGS:---fast}`）（`compose.yml`）

### Verification for User Story 3

- [ ] T016 [US3] 手動確認: `docker compose up` 後に `docker exec photopainter-comfyui nvidia-smi` を実行して GPU が認識されること、ComfyUI Web UI の System Stats で GPU が表示されることを確認する（**GPU実機環境で実施**）

**Checkpoint**: `nvidia-smi` が GPU 情報を返すこと

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 最終整合確認・ドキュメント更新・エンドツーエンド検証

- [x] T017 [P] `quickstart.md` の手順が最終的な `compose.yml` と完全に整合していることを確認し、差分があれば `quickstart.md` を更新する。あわせて `research.md` の主要設計決定（GPU 設定・ボリューム戦略・ネットワーク設計）が `compose.yml` に反映されていることを確認する（`specs/022-add-comfyui-compose/quickstart.md`）
- [x] T018 [P] `README.md` に ComfyUI Docker Compose の起動方法への参照または簡易手順を追記する（既存の `README.md`）
- [ ] T019 エンドツーエンド全フロー手動検証: 起動 → GPU 確認 → モデル配置 → カスタムノードインストール → 画像生成 → `docker compose down` → 再起動 → データ永続確認（モデル・カスタムノード・依存ライブラリ・生成画像すべて）の全サイクルを通して検証する（**GPU実機環境で実施**）

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup（T001–T002）完了後に開始
- **US1 (Phase 3)**: Foundational（T003–T004）完了後に開始
- **US2 (Phase 4)**: US1（T005–T008）完了後に開始（基本起動が前提）
- **US3 (Phase 5)**: US2（T009–T013）完了後に開始（ボリューム確認が前提）
- **Polish (Phase 6)**: US3（T014–T016）完了後に開始

### User Story Dependencies

```
Setup → Foundational → US1 → US2 → US3 → Polish
```

US1 は起動確認、US2 は永続化確認、US3 は GPU 確認の順に進める。  
US2 は US1 の compose.yml 基本定義に追記する形で実装するため、US1 完了が前提。

### Within Each User Story

- 実装タスク完了後に検証タスクを実行する
- `compose.yml` への変更後は常に `docker compose config` でパース確認してから起動テストに進む

### Parallel Opportunities

- T001 と T002 は同時実行可能（別ファイル）（両者に `[P]` マーカーあり）
- T017 と T018 は同時実行可能（別ファイル）

---

## Parallel Example: Phase 1

```bash
# 同時実行可能
Task T001: ".gitignore に .env と comfyui-data/ を追加"
Task T002: ".env.example をリポジトリルートに作成"
```

## Parallel Example: Phase 6

```bash
# 同時実行可能
Task T017: "quickstart.md の整合確認・更新"
Task T018: "README.md に起動方法を追記"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: T001, T002（Setup）を完了する
2. Phase 2: T003, T004（Foundational）を完了する
3. Phase 3: T005–T008（US1）を完了する
4. `docker compose up` → `http://localhost:18188` アクセスを独立検証する

### Incremental Delivery

1. Setup + Foundational を完了する
2. US1（基本起動）を追加して独立検証する
3. US2（永続化）を追加してコンテナ再起動での永続確認を行う
4. US3（GPU）を追加して `nvidia-smi` で GPU 認識を確認する
5. 前段の user story が壊れていないことを確認する

---

## Notes

- `compose.yml` は単一ファイルのため、各 phase で追記する形で実装する
- `[P]` は別ファイルへの操作であり並列実行可能なタスクを示す
- `[Story]` は traceability のために必須
- GPU 設定（T014）は `docker compose config` では検証できない（実際の GPU 環境での `docker compose up` が必要）

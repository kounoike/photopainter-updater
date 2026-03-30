# タスク: Ollama Docker Compose 追加

**Input**: `/specs/023-add-ollama-compose/` の設計文書  
**Prerequisites**: `plan.md`、`spec.md`、`research.md`、`data-model.md`、`contracts/`

**記述ルール**: この文書は日本語で記述する。各 user story には少なくとも 1 つの検
証タスク、または明示的な手動確認手順を含める。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能
- **[Story]**: 対応する user story (`US1`, `US2`, `US3` など)
- タスク記述には正確なファイルパスを含める

## Path Conventions

- Compose 定義: `compose.yml`
- 環境変数テンプレート: `.env.example`
- ルート導線: `README.md`
- feature 成果物: `specs/023-add-ollama-compose/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: feature 実装に必要な compose/documentation 対象を揃える

- [ ] T001 実装対象と検証対象を `specs/023-add-ollama-compose/plan.md` と `specs/023-add-ollama-compose/contracts/ollama-compose-runtime-contract.md` に照らして確認する
- [ ] T002 [P] 既存の ComfyUI compose 変数と README 導線を `compose.yml`、`.env.example`、`README.md` で確認し、変更境界を明確にする

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全 user story に先行する Ollama 追加の共通定義を整える

**CRITICAL**: この phase 完了まで user story 実装を開始しない

- [ ] T003 `compose.yml` に `ollama` サービス、`photopainter` ネットワーク参加、`/root/.ollama` 永続化を追加する
- [ ] T004 [P] `.env.example` に `OLLAMA_DATA_DIR` の説明とデフォルト値を追加する
- [ ] T005 [P] `specs/023-add-ollama-compose/quickstart.md` の前提条件と起動確認手順を実装方針に合わせて更新する

**Checkpoint**: Ollama サービス定義と環境変数が共通基盤として成立している

---

## Phase 3: User Story 1 - Ollama を compose から起動する (Priority: P1)

**Goal**: Ollama を Docker Compose から起動し、Compose 内ネットワークから API 到達確認できるようにする

**Independent Test**: `docker compose up -d ollama` と `docker compose exec comfyui curl -fsS http://ollama:11434/api/version` で起動と内部疎通を確認できれば完了

### Verification for User Story 1

- [ ] T006 [US1] 起動と内部疎通の検証手順を `specs/023-add-ollama-compose/quickstart.md` に明記する
- [ ] T007 [US1] `docker compose config`、`docker compose up -d ollama`、`docker compose exec ollama ollama list`、`docker compose exec comfyui curl -fsS http://ollama:11434/api/version` を実行して結果を確認する

### Implementation for User Story 1

- [ ] T008 [US1] `compose.yml` の `ollama` サービスに `image`、`container_name`、`restart`、`volumes`、`networks` を実装する
- [ ] T009 [US1] `compose.yml` で Ollama をホストへ公開しないことを明示し、`ports` を追加しない構成にする
- [ ] T010 [P] [US1] `README.md` に Ollama 起動の短い導線と詳細手順へのリンクを追加する

**Checkpoint**: User Story 1 が独立して検証可能であること

---

## Phase 4: User Story 2 - モデルを再利用する (Priority: P2)

**Goal**: Ollama のモデル保存領域を永続化し、コンテナ再作成後も再利用できるようにする

**Independent Test**: モデル pull 後に `docker compose down` と `docker compose up -d ollama` を行い、`docker compose exec ollama ollama list` で保持を確認できれば完了

### Verification for User Story 2

- [ ] T011 [US2] モデル pull と再作成後確認の手順を `specs/023-add-ollama-compose/quickstart.md` に記載する
- [ ] T012 [US2] `docker compose exec ollama ollama pull gemma3:1b`、`docker compose down`、`docker compose up -d ollama`、`docker compose exec ollama ollama list` を実行して永続化を確認する

### Implementation for User Story 2

- [ ] T013 [US2] `.env.example` に `OLLAMA_DATA_DIR` の保存先説明と編集例を追記する
- [ ] T014 [US2] `compose.yml` の `ollama` サービスで `${OLLAMA_DATA_DIR:-./ollama-data}:/root/.ollama` bind mount を実装する
- [ ] T015 [P] [US2] `specs/023-add-ollama-compose/data-model.md` と `specs/023-add-ollama-compose/contracts/ollama-compose-runtime-contract.md` の永続化前提を実装結果に合わせて更新する

**Checkpoint**: User Story 2 が独立して検証可能であること

---

## Phase 5: User Story 3 - ComfyUI と共存させる (Priority: P3)

**Goal**: 既存 ComfyUI の利用手順を壊さず、同一 compose 上で Ollama を共存させる

**Independent Test**: `docker compose config` と `docker compose up -d` の結果、および `README.md` / `quickstart.md` を確認して ComfyUI 手順が維持されていれば完了

### Verification for User Story 3

- [ ] T016 [US3] ComfyUI 維持と Ollama 追加後の利用手順を `README.md` と `specs/023-add-ollama-compose/quickstart.md` で確認する
- [ ] T017 [US3] `docker compose config` と `docker compose up -d` を実行し、`docker compose ps` で `comfyui` と `ollama` の共存を確認する

### Implementation for User Story 3

- [ ] T018 [US3] `README.md` の ComfyUI セクションを壊さずに Ollama セクションまたは導線を追加する
- [ ] T019 [US3] `compose.yml` の既存 `comfyui` 定義を保持したまま `ollama` を同一 `photopainter` ネットワークへ統合する
- [ ] T020 [P] [US3] `specs/023-add-ollama-compose/research.md` と `specs/023-add-ollama-compose/plan.md` に共存判断と制約が実装どおりであることを反映する

**Checkpoint**: すべての user story が独立検証可能であること

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 全 story を横断して最終整合と実行結果を揃える

- [ ] T021 `specs/023-add-ollama-compose/quickstart.md` のコマンド列を実際の実装に合わせて最終確認する
- [ ] T022 `specs/023-add-ollama-compose/spec.md`、`plan.md`、`tasks.md` の整合を確認し、必要なら文言を同期する
- [ ] T023 実行した `docker compose` 検証結果と残リスクを feature 配下の成果物または最終報告へまとめる

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 直ちに開始可能
- **Foundational (Phase 2)**: Setup 完了後に開始し、全 story をブロックする
- **User Stories (Phase 3+)**: Foundational 完了後に開始する
- **Polish (Phase 6)**: 対象 story 完了後に開始する

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 完了後に開始可能
- **User Story 2 (P2)**: User Story 1 の `ollama` サービス定義完了後に開始可能
- **User Story 3 (P3)**: User Story 1 完了後に開始可能

### Within Each User Story

- 検証手順は実装前に文書へ反映し、実装後にコマンド実行で確認する
- `compose.yml` の構成確定を `.env.example` と `README.md` より先に行う
- 永続化設定は起動成功後に確認する
- 共存確認は Ollama 単体起動確認のあとにまとめて行う

### Parallel Opportunities

- Phase 2 では `T004` と `T005` を並列で進められる
- User Story 1 では `T010` は `T008`/`T009` と別ファイルなので並列可能
- User Story 2 では `T015` は `compose.yml` 実装完了後に別文書更新として並列可能
- User Story 3 では `T020` は runtime 実装確認後に別文書更新として並列可能

---

## Parallel Example: User Story 1

```bash
Task: "`compose.yml` の `ollama` サービスに runtime 定義を追加する"
Task: "`README.md` に Ollama 起動導線を追加する"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup を完了する
2. Phase 2: Foundational を完了する
3. Phase 3: User Story 1 を完了する
4. 起動と内部疎通を確認する

### Incremental Delivery

1. Ollama の compose 起動を成立させる
2. 永続化を追加して再作成後保持を確認する
3. ComfyUI 共存と README 導線を整える
4. 最後に文書整合と検証結果を揃える

### Parallel Team Strategy

1. 1 人が `compose.yml` と `.env.example` を担当する
2. もう 1 人が `README.md` と `quickstart.md` を担当する
3. runtime 実装後に検証結果と spec 成果物の同期を分担する

---

## Notes

- `[P]` は別ファイル・独立依存の並列タスクを示す
- `[Story]` は traceability のために user story phase で必須
- 実装順は `compose.yml` を起点にし、README と feature 文書は追随更新とする

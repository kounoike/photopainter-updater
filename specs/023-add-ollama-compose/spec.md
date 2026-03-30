# 機能仕様: Ollama Docker Compose 追加

**Feature Branch**: `023-add-ollama-compose`  
**Created**: 2026-03-30  
**Status**: Draft  
**Input**: ユーザー記述: "compose.ymlにollamaを追加して"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Clarifications

### Session 2026-03-30

- Q: Ollama の接続公開範囲をどうするか → A: Compose では外部公開方法を決めず、内部ネットワーク専用にする

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Ollama を compose から起動する (Priority: P1)

開発者として、既存の `compose.yml` に追加された Ollama サービスを使って、Docker Compose から Ollama を起動したい。これにより、ローカルに別途手作業で Ollama を立ち上げずに、同じ運用導線で LLM 実行基盤を使える。

**Why this priority**: Ollama サービスを起動できなければ、この feature の価値が成立しないため。

**Independent Test**: `docker compose up -d ollama` で Ollama を起動し、`docker compose exec ollama ollama list` で応答確認できれば完了。

**Acceptance Scenarios**:

1. **Given** Docker Compose が利用可能な環境で, **When** `compose.yml` を使って Ollama を起動する, **Then** Ollama サービスが正常起動する
2. **Given** Ollama が起動している状態で, **When** `docker compose exec ollama ollama list` を実行する, **Then** 応答を確認できる

---

### User Story 2 - モデルを再利用する (Priority: P2)

開発者として、Ollama で取得したモデルをコンテナ再作成後も再利用したい。これにより、毎回モデルを取り直す無駄を避けられる。

**Why this priority**: モデル永続化がないと、Compose 統合の運用価値が大きく下がるため。

**Independent Test**: モデル格納先をホストへ永続化した状態でコンテナを再作成し、取得済みモデルが残っていれば完了。

**Acceptance Scenarios**:

1. **Given** Ollama でモデルを取得済みである, **When** コンテナを停止・再作成する, **Then** 取得済みモデルが引き続き利用できる
2. **Given** ホスト側の永続ディレクトリを変更したい, **When** 設定を変更する, **Then** Compose 定義を大きく書き換えずに保存先を変更できる

---

### User Story 3 - ComfyUI と共存させる (Priority: P3)

開発者として、既存の ComfyUI 用 `compose.yml` を壊さずに Ollama を追加したい。これにより、画像生成基盤と LLM 基盤を同じ Compose 管理下で段階的に育てられる。

**Why this priority**: 既存の ComfyUI 運用を壊すと feature 追加ではなく回帰になるため。

**Independent Test**: `compose.yml` を検証し、ComfyUI と Ollama の両方が定義され、既存 ComfyUI の利用手順が維持されていることを確認する。

**Acceptance Scenarios**:

1. **Given** 既存の ComfyUI サービスが定義されている, **When** Ollama サービスを追加した `compose.yml` を確認する, **Then** ComfyUI の既存設定が保持されている
2. **Given** 将来別サービスを追加する可能性がある, **When** Compose 構成を確認する, **Then** Ollama 追加が既存ネットワークや運用方針と矛盾しない

### Edge Cases

- Ollama のデータ保存先が未作成でも、Docker が扱えるか、または手順書で事前準備が分かること
- ComfyUI のみ使いたい利用者がいても、既存の ComfyUI 起動方法が不必要に複雑化しないこと
- GPU 有無や利用方針が ComfyUI と Ollama で異なる場合でも、サービスごとの責務が混線しないこと
- Ollama を外部公開しない前提でも、Compose 内からの疎通確認方法が明確であること
- Ollama をホストへ公開しない前提でも、利用者が内部接続先 `http://ollama:11434` を誤解しないこと

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST 既存の `compose.yml` に Ollama サービスを追加し、Docker Compose から起動できるようにする
- **FR-002**: System MUST Ollama API を Compose 内ネットワーク専用とし、ホストへのポート公開を必須要件にしない
- **FR-003**: Users MUST be able to Ollama の保存データをホスト側へ永続化し、コンテナ再作成後もモデルを再利用できる
- **FR-004**: System MUST Ollama のホスト側保存先や必要な設定を、既存 Compose 運用と同じ方針で調整できるようにする
- **FR-005**: System MUST ComfyUI の既存設定と利用手順を壊さずに Ollama を追加する
- **FR-006**: System MUST `compose.yml` の検証方法と Ollama の起動・停止・Compose 内からの接続確認手順を文書化する
- **FR-007**: System MUST Ollama 追加に伴う運用上の前提、永続化方針、既存サービスとの共存方針を成果物へ記録する

### Key Entities *(include if feature involves data)*

- **Ollama サービス**: Docker Compose で管理される LLM 実行サービス。Compose 内ネットワーク向け API とモデル保存領域を持つ。
- **モデル保存領域**: Ollama が取得したモデルや関連データを保持するホスト側ディレクトリ。
- **Compose 構成**: ComfyUI と Ollama を同一 `compose.yml` で扱うためのサービス定義、ポート、ネットワーク、ボリューム設定。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- リポジトリルートの `compose.yml` 更新
- 必要に応じた `.env.example`、`README.md`、関連手順書の更新
- `specs/023-add-ollama-compose/` 配下の成果物作成
- Ollama 追加に伴う Compose 運用ルールの文書化

### Forbidden Scope

- 既存 ComfyUI サービスの用途変更
- firmware や `server/` 本体への機能追加
- Ollama を利用するアプリケーションロジックの実装
- `xiaozhi-esp32/` 配下の変更

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `docker compose` による検証と起動手順を使って、Ollama サービスを追加した Compose 構成を 5 分以内に立ち上げ確認できる
- **SC-002**: Ollama で取得したモデルが、コンテナ停止・再作成後も保持されることを確認できる
- **SC-003**: 既存の ComfyUI 利用手順を維持したまま、Ollama の起動・Compose 内接続確認手順を追加で説明できる
- **SC-004**: 成果物を読めば、Ollama 追加後の保存先、接続先、既存サービスとの関係を誤解なく説明できる

## Assumptions

- 既存の `compose.yml` は ComfyUI 用として継続利用する
- Ollama 自体の詳細なモデル運用やアプリ統合は今回のスコープ外とする
- Docker Compose v2 系の運用を前提にする
- Ollama の利用確認は Compose 内ネットワークからの API 到達確認で十分とする

## Documentation Impact

- `specs/023-add-ollama-compose/spec.md` に Ollama 追加の目的、境界、成功条件を記載する
- 後続 phase で `plan.md`、`tasks.md`、必要に応じて `research.md`、`quickstart.md`、`contracts/` を作成・更新する
- 実装時には `compose.yml` と `README.md`、必要なら `.env.example` の手順同期が必要になる

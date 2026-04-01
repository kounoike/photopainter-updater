# 機能仕様: AI Toolkit 試用環境

**Feature Branch**: `025-ai-toolkit-env`  
**Created**: 2026-04-01  
**Status**: Draft  
**Input**: ユーザー記述: "AI Toolkitを試すための環境を作る"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Clarifications

### Session 2026-04-01

- Q: `AI Toolkit` は何を指すか? → A: `ostris/ai-toolkit` を指す
- Q: どこで起動するか? → A: このリポジトリの `compose.yml` に `ai-toolkit` サービスを追加して起動する
- Q: 何をもって試せたとみなすか? → A: `ai-toolkit` サービスが起動し、Web UI に到達できること

## User Scenarios & Testing *(mandatory)*

### User Story 1 - AI Toolkit を起動して触り始める (Priority: P1)

開発者として、`ostris/ai-toolkit` をこのリポジトリの Docker Compose から起動し、Web UI へ到達したい。これにより、別リポジトリを個別にセットアップせず、この作業環境の中で AI Toolkit を試し始められる。

**Why this priority**: AI Toolkit のサービスを起動できなければ、この feature の価値が成立しないため。

**Independent Test**: 利用者が `.env.example` と `README.md` の案内に従って `docker compose up -d ai-toolkit` を実行し、ブラウザで AI Toolkit Web UI へ到達できれば完了。

**Acceptance Scenarios**:

1. **Given** 利用者がこのリポジトリを取得した直後で, **When** AI Toolkit 用の案内に従って設定を準備する, **Then** `ai-toolkit` サービス起動に必要な前提条件が分かる
2. **Given** 利用者が `ai-toolkit` サービスを起動した状態で, **When** 指定された URL へアクセスする, **Then** AI Toolkit Web UI に到達できる

---

### User Story 2 - 試用データと設定を保持して再開する (Priority: P2)

開発者として、AI Toolkit の設定や入出力データを保持したまま再起動したい。これにより、毎回最初から環境を作り直さず、同じ試行を継続できる。

**Why this priority**: 試用のたびに設定や出力が消えると、継続利用の価値が大きく下がるため。

**Independent Test**: 利用者が AI Toolkit の保存先を使って起動・停止・再起動を行い、設定や出力の保存先が維持されることを確認できれば完了。

**Acceptance Scenarios**:

1. **Given** 利用者が AI Toolkit 用の保存先を準備済みで, **When** サービスを停止して再起動する, **Then** 設定ファイルや出力先の参照位置が変わらない
2. **Given** 利用者が別タイミングで同じ `.env` を使って再起動する, **When** AI Toolkit を再度開く, **Then** 同じ保存先前提で試用を再開できる

---

### User Story 3 - 既存 Compose 導線を壊さずに共存させる (Priority: P3)

開発者として、AI Toolkit を追加しても既存の ComfyUI と Ollama の利用導線は壊したくない。これにより、画像生成や LLM 推論の既存環境を残したまま、新しい学習ツールを同じ Compose プロジェクト内で試せる。

**Why this priority**: 既存の Compose 運用を壊すと、新規試用より回帰リスクの方が大きくなるため。

**Independent Test**: 利用者が `README.md` を確認したとき、ComfyUI / Ollama の既存導線が残りつつ、AI Toolkit が追加サービスとして理解できれば完了。

**Acceptance Scenarios**:

1. **Given** 既存の ComfyUI または Ollama の利用手順を使っている利用者がいる, **When** AI Toolkit が追加された `README.md` を読む, **Then** 既存導線が置き換えられていないと分かる
2. **Given** 利用者が AI Toolkit だけを起動したい, **When** Compose 手順を確認する, **Then** 既存サービスと独立して起動方法を判断できる

### Edge Cases

- Docker や GPU 前提が満たされない場合でも、何が不足しているかを利用者が判断できること
- AI Toolkit 用の保存先ディレクトリが未作成でも、事前準備方法または自動生成前提を理解できること
- AI Toolkit の UI 公開に認証を設定したい場合でも、最低限の設定位置が分かること
- AI Toolkit を使わない利用者が既存の ComfyUI / Ollama 導線だけを参照しても混乱しないこと

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST `compose.yml` に `ostris/ai-toolkit` を起動する `ai-toolkit` サービスを追加し、Docker Compose から起動できるようにしなければならない
- **FR-002**: System MUST AI Toolkit Web UI へ到達するためのポート、起動手順、アクセス方法を利用者へ明示しなければならない
- **FR-003**: Users MUST be able to `.env` と `docker compose up -d ai-toolkit` を使って AI Toolkit を起動し、Web UI 到達可否を判断できなければならない
- **FR-004**: System MUST AI Toolkit の設定、入力、出力、永続データの保存先を継続利用できるようにしなければならない
- **FR-005**: System MUST AI Toolkit の主要な環境変数や認証設定の入口を `.env.example` と関連手順へ記載しなければならない
- **FR-006**: System MUST `README.md` に AI Toolkit の追加導線を記載し、詳細手順を feature 配下の文書へ誘導しなければならない
- **FR-007**: System MUST 既存 ComfyUI / Ollama の導線を壊さず、AI Toolkit が追加サービスであることを明示しなければならない
- **FR-008**: System MUST AI Toolkit 起動時の前提不足や起動失敗時に、利用者が最初の確認先を判断できるようにしなければならない

### Key Entities *(include if feature involves data)*

- **AI Toolkit サービス**: `compose.yml` に追加される `ostris/ai-toolkit` ベースのサービス。Web UI と学習関連の保存先を持つ。
- **AI Toolkit 保存先**: 設定、入力データ、出力データ、DB、必要なキャッシュを保持するホスト側ディレクトリやファイル。
- **AI Toolkit 環境変数**: ポート、認証、保存先など、起動条件を切り替えるための `.env` 設定。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `compose.yml` への `ai-toolkit` サービス追加
- `.env.example` の AI Toolkit 向け環境変数追加
- `README.md` の AI Toolkit 導線追加
- `specs/025-ai-toolkit-env/` 配下の成果物更新
- AI Toolkit の起動、保存先、UI 到達確認、復帰方法の文書化

### Forbidden Scope

- `firmware/` や `server/` 本体への AI Toolkit 統合実装
- `xiaozhi-esp32/` 配下の変更
- 既存 ComfyUI / Ollama の導線削除や置換
- AI Toolkit 自体のソースコード改変
- 試用範囲を超える本番運用設計やクラウド移行設計

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 新規利用者が案内を読み始めてから 15 分以内に、`docker compose up -d ai-toolkit` の実行と Web UI 到達可否を自力で判断できる
- **SC-002**: 同じ利用者が同じ `.env` と保存先設定で別タイミングに再起動しても、同じ手順で AI Toolkit を再開できる
- **SC-003**: AI Toolkit を使わない利用者が `README.md` を読んだとき、既存 ComfyUI / Ollama 導線がそのまま維持されていると理解できる
- **SC-004**: 起動失敗時に、利用者が追加質問なしで少なくとも最初の確認先を 1 つ特定できる

## Assumptions

- AI Toolkit の起動対象は `ostris/ai-toolkit` の既存コンテナイメージを利用する
- AI Toolkit の UI は Docker Compose で起動したサービスへブラウザで到達して利用する
- 保存先はホスト側へ永続化し、利用者が再起動後も同じ場所を使える前提とする
- 既存の ComfyUI / Ollama Compose 運用は継続利用し、AI Toolkit は追加サービスとして共存させる
- AI Toolkit 自体の学習設定内容やモデル選定は今回の主目的ではなく、「起動して試せる環境」を優先する

## Documentation Impact

- `specs/025-ai-toolkit-env/spec.md` に AI Toolkit サービス追加の目的、境界、成功条件を記載する
- 後続 phase で `plan.md`、`tasks.md`、`research.md`、`quickstart.md` を AI Toolkit 実体に合わせて更新する
- 実装時には `compose.yml`、`.env.example`、`README.md`、関連 quickstart を同期する必要がある

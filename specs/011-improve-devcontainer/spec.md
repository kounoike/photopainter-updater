# 機能仕様: Devcontainer 起動改善

**Feature Branch**: `011-improve-devcontainer`  
**Created**: 2026-03-30  
**Status**: Draft  
**Input**: ユーザー記述: "devcontainerの改善。postCreateは遅いのでDockerfileに入れちゃう。codexとかの認証情報がクリアされちゃうからボリュームマウントか何かでキャッシュする"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 初回セットアップ待ち時間を減らしたい (Priority: P1)

開発者は devcontainer を作成した直後に長い追加セットアップ待ちを挟まず、すぐに作業開始できる状態へ入りたい。

**Why this priority**: 開発開始のたびに長い待ち時間が発生すると、環境再作成や新規参加者の立ち上がりコストが高くなるため。

**Independent Test**: devcontainer を新規作成し、追加の手動インストール待ちなしで `codex` と `claude` が利用可能になっていることを確認する。

**Acceptance Scenarios**:

1. **Given** 開発者が新しい devcontainer を作成する, **When** コンテナ利用可能状態になるまで待つ, **Then** 長い追加セットアップ待ちをせずに作業を開始できる。
2. **Given** 開発者が devcontainer に接続した直後, **When** `codex` または `claude` を利用する, **Then** 事前準備済みで即座に利用できる。

---

### User Story 2 - コンテナ再生成後も認証状態を維持したい (Priority: P2)

開発者は devcontainer を rebuild または recreate しても、Codex や関連 CLI の認証状態が毎回消えず、継続して利用したい。

**Why this priority**: 認証し直しが毎回必要だと日常運用が不安定になり、作業再開までの時間も増えるため。

**Independent Test**: 認証済み状態で devcontainer を再生成し、再接続後に `codex` と `claude` が再認証なしで利用継続できることを確認する。

**Acceptance Scenarios**:

1. **Given** 開発者が `codex` または `claude` を一度認証済みの状態, **When** devcontainer を rebuild または recreate する, **Then** 再接続後も認証済み状態が維持される。
2. **Given** 認証情報を保持する仕組みが有効な状態, **When** 開発者が複数回 devcontainer を再生成する, **Then** 毎回同じ認証情報を再利用できる。

---

### User Story 3 - 認証キャッシュの扱いを安全に把握したい (Priority: P3)

開発者はどの情報が保持され、どのタイミングで初期化されるかを理解したうえで devcontainer を運用したい。

**Why this priority**: 永続化の範囲が不明だと、不要な情報保持や逆に意図しない消失が発生しやすいため。

**Independent Test**: 開発者向け手順を読み、保持対象・初期化方法・再認証が必要になる条件を判断できることを確認する。

**Acceptance Scenarios**:

1. **Given** 開発者が devcontainer の利用手順を参照する, **When** 認証キャッシュ運用の説明を確認する, **Then** 保持対象と初期化方法を判断できる。

### Edge Cases

- 認証情報の保存領域がまだ存在しない初回起動でも、通常手順で認証を完了できること。
- 開発者が意図的に認証キャッシュを初期化したい場合、文書化された明確な手順で永続領域を消去し、その後に通常の初回認証へ戻れること。
- 認証保持の導入によって、ワークスペース本体のソース管理対象へ機密情報を書き込まないこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST devcontainer 利用可能直後に `codex` と `claude` を利用できる状態を提供しなければならない。
- **FR-002**: System MUST 開発者が毎回長い追加セットアップを待たずに作業を開始できるようにしなければならない。
- **FR-003**: System MUST devcontainer の rebuild または recreate 後も、`codex` と `claude` の認証状態を再利用できるようにしなければならない。
- **FR-004**: System MUST 認証情報や関連キャッシュを、ワークスペース内のソース管理対象とは分離した保持領域に保存しなければならない。
- **FR-005**: System MUST `codex` と `claude` について、初回認証、再利用、意図的な初期化の手順を開発者が実行判断できるよう文書化しなければならない。

### Key Entities *(include if feature involves data)*

- **開発 CLI 認証状態**: `codex` と `claude` が継続利用に必要とするログイン済み情報や関連設定。
- **認証キャッシュ領域**: コンテナ再生成後も保持される、認証状態や関連キャッシュを保存する永続化先。
- **devcontainer 初期セットアップ**: 開発者がコンテナ接続後に作業開始可能になるまでの準備状態。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- devcontainer 初期セットアップ時間の短縮
- 主要な開発 CLI の事前利用可能化
- devcontainer 再生成後も残る認証キャッシュ保持
- 認証キャッシュ運用に関する開発文書の更新

### Forbidden Scope

- アプリケーション本体の機能追加や firmware/server 実装変更
- 開発 CLI 自体の機能変更
- チーム外部サービス側の認証仕様変更
- ワークスペースの Git 管理対象へ認証情報を保存する運用

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 開発者は新規 devcontainer 作成後、追加セットアップ待ちなしで `codex` と `claude` をそのまま起動できる。
- **SC-002**: 開発者は devcontainer を再生成しても、少なくとも 1 回の再接続確認で `codex` と `claude` の再認証不要を判断できる。
- **SC-003**: 開発者は認証キャッシュの保持対象と初期化方法を文書だけで判断できる。
- **SC-004**: 開発者は再生成前後で同じ作業開始手順を維持でき、環境再作成が日常運用の妨げにならない。

## Assumptions

- 対象はこのリポジトリの `.devcontainer` を使ってローカル開発する開発者である。
- 保持対象は `codex` と `claude` の認証関連情報であり、アプリケーション実行時データは対象外とする。
- devcontainer の再生成は開発者が日常的に行う前提で、毎回の再認証は避ける価値がある。

## Documentation Impact

- `specs/011-improve-devcontainer/` 配下の後続成果物を作成する必要がある。
- 実装時には `.devcontainer` の利用手順、認証保持方法、初期化方法を案内する文書更新が必要になる。

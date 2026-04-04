# 機能仕様: Release Drafter 導入

**Feature Branch**: `033-release-drafter`  
**Created**: 2026-04-04  
**Status**: Draft  
**Input**: ユーザー記述: "github actionsでrelease-drafter導入して"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

## Clarifications

### Session 2026-04-04

- Q: release draft はどの契機で更新するか → A: `main` への `push` のみ

### User Story 1 - 次回リリースの下書きを自動更新したい (Priority: P1)

リポジトリ管理者は、main へ変更が入るたびに、次回リリース向けの draft を自動で更新し、手で changelog をまとめ直す手間を減らしたい。

**Why this priority**: release drafter 導入の主目的は、変更履歴の収集と release note 下書き更新を自動化することだから。

**Independent Test**: main 向けの pull request 情報をもとに release draft が生成または更新され、次回リリース候補として確認できることを確認する。

**Acceptance Scenarios**:

1. **Given** repository に main 向けの変更履歴がある, **When** `main` への `push` を契機に release draft 更新処理が実行される, **Then** 次回リリース向けの draft が生成または更新される。
2. **Given** 新しい pull request が main に取り込まれる, **When** merge 後の `main` への `push` を契機に release draft 更新処理が再実行される, **Then** draft へ新しい変更が反映される。

---

### User Story 2 - 変更種別ごとに整理された下書きを見たい (Priority: P2)

リポジトリ管理者は、集められた pull request 情報を種類ごとに整理された release draft として見て、公開前レビューをしやすくしたい。

**Why this priority**: 単に変更が並ぶだけではレビューや公開判断がしづらく、分類された draft の方が運用価値が高いため。

**Independent Test**: 複数種類の pull request 情報を含む状態で draft を確認し、分類ルールに従って見出しや項目が整理されることを確認する。

**Acceptance Scenarios**:

1. **Given** repository に複数の変更種別がある, **When** release draft が更新される, **Then** 管理者は分類済みの変更一覧を確認できる。
2. **Given** pull request に分類用 metadata が付いている, **When** release draft が生成される, **Then** 対応する区分へ反映される。

---

### User Story 3 - 導入後の運用方法を把握したい (Priority: P3)

リポジトリ管理者は、release drafter 導入後に何を見ればよいか、どう使うかを repository 内の手順から把握したい。

**Why this priority**: 自動化を入れても、設定ファイルや確認方法が文書化されていないと運用が定着しないため。

**Independent Test**: repository 内文書を参照し、管理者が release draft の更新契約と確認方法を理解できることを確認する。

**Acceptance Scenarios**:

1. **Given** 管理者が repository の運用手順を参照する, **When** release draft の扱い方を探す, **Then** 設定場所と確認方法を把握できる。
2. **Given** release draft が期待どおり更新されない, **When** 管理者が repository 内手順を見る, **Then** 確認対象や前提条件を追える。

### Edge Cases

- 分類対象外の pull request が含まれても、release draft 更新自体は止まらず、既定の扱いで一覧化されること。
- release draft がまだ存在しない初回実行でも、新規 draft を作成できること。
- `main` への `push` 以外の branch や event では、不要な draft 更新を発生させないこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST repository の変更履歴をもとに、次回リリース向け draft を自動生成または更新できなければならない。
- **FR-002**: System MUST main 向けの新しい変更が取り込まれた後、既存 draft へ変更内容を反映できなければならない。
- **FR-003**: Users MUST be able to repository 上で次回リリースの draft 内容を確認できなければならない。
- **FR-004**: System MUST pull request の分類情報に応じて release draft 内の項目を整理しなければならない。
- **FR-005**: System MUST 分類対象外の変更も release draft から欠落させず、既定の扱いで掲載しなければならない。
- **FR-006**: System MUST release draft 更新の対象を `main` への `push` に限定し、不要な更新を避けなければならない。
- **FR-007**: System MUST repository 内文書で release draft の設定場所、更新契約、確認方法を案内しなければならない。

### Key Entities *(include if feature involves data)*

- **Release draft**: 次回リリース候補として repository 上に維持される下書きリリース。
- **Pull request metadata**: 各変更を release draft 内で整理するための分類情報。
- **更新契約**: どの変更契機で draft を生成・更新するかを定める運用ルール。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- repository の release draft 自動更新設定追加
- release draft の分類ルールと既定扱いの定義
- release draft 導入に伴う repository 内文書更新

### Forbidden Scope

- 実際の versioning policy 全面変更
- リリース publish 作業そのものの自動化
- repository 外サービスを前提とする新しい配布導線追加
- 既存 CI/CD 全体の作り直し

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 管理者は main 向け変更の反映後、追加の手作業なしで次回リリース用 draft を確認できる。
- **SC-002**: release draft には分類済みの変更一覧が表示され、管理者が主要変更を目視で追える。
- **SC-003**: 分類対象外の変更があっても、release draft から変更項目が欠落しない。
- **SC-004**: repository 内文書を読んだ管理者が、release draft の設定場所と確認方法を追加説明なしで理解できる。

## Assumptions

- この repository は GitHub 上で運用され、pull request ベースで main へ変更が取り込まれる。
- pull request には分類に使える label などの metadata を付与できる前提とする。
- release draft 更新契機は `main` への `push` のみを対象とする。
- 今回は release draft の維持までを対象とし、publish 操作の自動化は含めない。

## Documentation Impact

- GitHub 運用手順または README 系文書に release draft の確認方法を追記する必要がある。
- release draft の分類ルールや既定扱いを repository 内で参照できるようにする必要がある。

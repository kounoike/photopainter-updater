# 機能仕様: Release 時の GHCR image publish

**Feature Branch**: `034-ghcr-release-publish`  
**Created**: 2026-04-04  
**Status**: Draft  
**Input**: ユーザー記述: "draftからリリースするときにserverをdocker buildしてghcrにプッシュする。他のイメージもpushする予定だからそれも考慮して"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - release 時に server image を自動公開したい (Priority: P1)

リポジトリ管理者は、GitHub の draft release を正式 release にするとき、`server` image を手作業なしで build して GHCR に公開したい。

**Why this priority**: まず `server` image の公開を自動化することが今回の主目的であり、運用負荷と手順漏れを直接減らせるため。

**Independent Test**: draft release を publish した想定で release 用自動処理を実行し、`server` image が期待する repository 名と release version を含むタグで公開対象として扱われることを確認する。

**Acceptance Scenarios**:

1. **Given** draft release が公開待ちで存在する, **When** 管理者がその draft を正式 release として publish する, **Then** `server` image の build と GHCR への publish が自動で開始される。
2. **Given** release version が確定している, **When** `server` image が公開される, **Then** 管理者は release version に対応する image tag を GHCR 上で確認できる。

---

### User Story 2 - 追加 image を同じ仕組みで載せたい (Priority: P2)

リポジトリ管理者は、将来 `comfyui` など別の image を公開対象へ追加するとき、release 用 workflow 全体を作り直さずに同じ枠組みへ載せたい。

**Why this priority**: 今後 publish 対象が増える前提があるため、`server` 専用の作り捨てではすぐに運用負債になるため。

**Independent Test**: release publish 対象の定義を参照し、`server` がその一覧に含まれ、同じ形式で別 image を追加できることを確認する。

**Acceptance Scenarios**:

1. **Given** release publish 対象を定義する共通の仕組みがある, **When** 管理者が将来の追加 image を検討する, **Then** 既存 `server` と同じ定義単位で追加可否を判断できる。
2. **Given** 現時点では `server` だけを公開対象にする, **When** workflow が実行される, **Then** 今後の追加余地を残しつつ、不要な image まで誤って公開しない。

---

### User Story 3 - release image の運用方法を追いたい (Priority: P3)

リポジトリ管理者は、どの契機で image が公開されるか、どこに公開されるか、失敗時に何を確認すればよいかを repository 内の手順で把握したい。

**Why this priority**: 公開自動化は導入して終わりではなく、運用時に確認導線がないと失敗原因を追えず定着しないため。

**Independent Test**: repository 内文書だけを参照し、管理者が release publish の契機、公開先、対象 image、確認場所を理解できることを確認する。

**Acceptance Scenarios**:

1. **Given** 管理者が release 運用手順を確認する, **When** image 公開の流れを探す, **Then** draft release の publish を契機に GHCR へ公開されることを把握できる。
2. **Given** release publish 後に image を確認したい, **When** 管理者が repository 内文書を見る, **Then** Actions と GHCR のどこを見ればよいかを追える。

### Edge Cases

- draft ではない release の更新や編集が行われても、意図しない再公開や重複 publish を起こさないこと。
- 追加 image 用の定義枠が存在しても、未設定の image は公開対象に含めないこと。
- image の build または publish に失敗した場合、管理者が GitHub Actions 上で失敗を認識できること。
- release version 文字列を image tag に反映できない場合は、曖昧なタグで公開しないこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST GitHub 上の draft release が正式 release として publish されたときにのみ、release image 公開処理を開始しなければならない。
- **FR-002**: System MUST 今回の初期対象として `server` image を build し、GHCR へ publish できなければならない。
- **FR-003**: System MUST 公開された `server` image に対して、release version と対応づけて識別できる tag を付与しなければならない。
- **FR-004**: Users MUST be able to GitHub Actions 上で release image publish の成功または失敗を確認できなければならない。
- **FR-005**: Users MUST be able to GHCR 上で対象 release に対応する `server` image を確認できなければならない。
- **FR-006**: System MUST 現時点の公開対象 image を明示的に定義し、未定義の image を暗黙に publish しないようにしなければならない。
- **FR-007**: System MUST 将来 `server` 以外の image を同じ release publish の仕組みに追加できるよう、公開対象定義を拡張可能な形で保持しなければならない。
- **FR-008**: System MUST release image publish の契機、公開先、対象 image、確認方法を repository 内文書で案内しなければならない。
- **FR-009**: System MUST 既存の release draft 更新運用を壊さず、draft 生成と image publish の責務を混同しないようにしなければならない。

### Key Entities *(include if feature involves data)*

- **Release publish event**: GitHub 上で draft release を正式公開する契機。image build/publish の開始条件になる。
- **Publish target**: release 時に build と publish を行う image 単位。現時点では `server` を必須で含み、将来追加されうる。
- **Image tag set**: release version と公開 image を結びつける識別情報。GHCR 上で対象 release の image を探す基準になる。
- **Publish run result**: GitHub Actions 上で確認できる公開処理の結果。成功/失敗と確認導線を含む。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- release publish 契機で動く GitHub Actions workflow の追加または更新
- `server` image を GHCR へ公開するための repository 内設定追加
- 将来の複数 image 公開を見据えた publish target 定義の整理
- release image 運用に関する README または quickstart 文書更新

### Forbidden Scope

- `server` 以外の image を今回必須で publish 対象に追加すること
- Docker image の中身やアプリケーション挙動自体の改修
- release の publish 以外の配布経路追加
- GHCR 以外の container registry への同時対応
- release draft 作成機能そのものの全面作り直し

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 管理者は draft release を正式公開した後、追加の手作業なしで `server` image の公開処理開始を確認できる。
- **SC-002**: 管理者は公開後 1 つの release version ごとに、対応する `server` image tag を GHCR 上で識別できる。
- **SC-003**: 管理者は repository 内の対象定義を見れば、今回 publish される image と将来追加可能な拡張点を区別できる。
- **SC-004**: repository 内文書だけを読んで、管理者が release publish の契機、公開先、確認場所を追加説明なしで理解できる。

## Assumptions

- GitHub Releases を使って draft release から正式 release への publish を運用する前提とする。
- GHCR への publish に必要な権限や認証は GitHub Actions から利用可能な形で repository 側に設定できる前提とする。
- version は GitHub Release で確定した値を基準に image tag へ反映する前提とする。
- 今回の必須公開対象は `server` のみとし、`comfyui` など他 image は将来追加候補として扱う。

## Documentation Impact

- ルート [README.md](/workspaces/photopainter-updater/README.md) に release image publish の概要と確認場所を追記する必要がある。
- feature 配下の quickstart で release 実行手順、確認手順、対象 image の定義方針を説明する必要がある。

# 機能仕様: HTTPサーバ Compose 統合

**Feature Branch**: `029-compose-http-server`  
**Created**: 2026-04-03  
**Status**: Draft  
**Input**: ユーザー記述: "HTTPサーバをdocker compose内で起動するようにして、server/run.shを廃止して"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - compose だけで HTTP サーバを使いたい (Priority: P1)

運用者は、画像配信用の HTTP サーバを `docker compose` だけで起動し、従来の `server/run.sh` を使わずに利用したい。

**Why this priority**: 起動導線が shell script と compose に分かれていると運用が煩雑になり、既存の ComfyUI / Ollama 導線とも揃わないため。

**Independent Test**: `docker compose up -d` または HTTP サーバ単体起動相当の compose 操作でサーバが起動し、既存の `/`、`/image.bmp`、`/image.bin`、`/upload` が利用できることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が repo ルートで compose を使う状態, **When** HTTP サーバを compose で起動する, **Then** shell script を介さずサーバへ到達できる。
2. **Given** 利用者が既存の HTTP 配信 URL を使う状態, **When** compose で起動したサーバにアクセスする, **Then** 既存の取得 / 更新導線は維持される。

---

### User Story 2 - 既存 compose サービスと共存させたい (Priority: P2)

運用者は、ComfyUI、Ollama、AI Toolkit と同じ compose プロジェクト内で HTTP サーバを管理し、既存導線を壊さずに共存させたい。

**Why this priority**: 画像生成側はすでに compose 運用へ寄っており、HTTP サーバだけが別導線だと全体運用が分裂するため。

**Independent Test**: compose 設定を確認し、既存サービスを壊さずに HTTP サーバサービスが追加されていることを確認する。

**Acceptance Scenarios**:

1. **Given** 既存の ComfyUI / Ollama / AI Toolkit 導線がある状態, **When** HTTP サーバを compose へ追加する, **Then** 既存サービスはそのまま利用できる。
2. **Given** 利用者が HTTP サーバだけ、または他サービスと合わせて起動したい状態, **When** compose を使う, **Then** 必要なサービス単位で管理できる。

---

### User Story 3 - 手順書を一本化したい (Priority: P3)

運用者は、README や feature 手順書を見たときに、HTTP サーバの起動方法が compose 前提で一貫している状態にしたい。

**Why this priority**: 実装だけ compose 化しても、文書に `server/run.sh` が残ると運用時に迷うため。

**Independent Test**: README と server 関連手順を確認し、HTTP サーバ起動導線が compose 前提へ統一され、`server/run.sh` への依存が消えていることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が README を読む状態, **When** HTTP サーバの起動方法を探す, **Then** compose を使う手順だけで理解できる。
2. **Given** 利用者が server 関連の手順書を使う状態, **When** 起動・停止・確認方法を実施する, **Then** `server/run.sh` を必要としない。

---

### Edge Cases

- HTTP サーバを compose へ追加しても、既存の port や既存サービスの公開導線を壊さないこと。
- 画像保存先や content directory の扱いが変わっても、既存データを不意に失わないこと。
- HTTP サーバだけを起動したい場合でも、不要な他サービス起動を強制しないこと。
- `server/run.sh` 廃止後も、利用者が起動方法やログの見方を追跡できること。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST 画像配信用 HTTP サーバを `docker compose` 管理下で起動できなければならない。
- **FR-002**: System MUST compose で起動した HTTP サーバから既存の `/`、`/image.bmp`、`/image.bin`、`/upload` を引き続き利用できなければならない。
- **FR-003**: Users MUST be able to HTTP サーバを他 compose サービスと独立または組み合わせて起動・停止できなければならない。
- **FR-004**: System MUST 既存の ComfyUI、Ollama、AI Toolkit の compose 導線を壊してはならない。
- **FR-005**: System MUST `server/run.sh` への起動依存を廃止しなければならない。
- **FR-006**: System MUST README と関連手順書を compose 起動前提へ更新しなければならない。
- **FR-007**: System MUST HTTP サーバのログ確認方法と起動確認方法を compose 運用前提で案内できなければならない。

### Key Entities *(include if feature involves data)*

- **HTTP サーバサービス**: 画像配信と upload を担う compose 管理対象のサーバ。
- **compose 運用導線**: サービス起動、停止、ログ確認、疎通確認を含む利用者向け手順。
- **server 既存資産**: `server/contents/` や既存エンドポイントなど、compose 化後も維持される server 側資産。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `compose.yml` への HTTP サーバサービス追加または既存構成調整
- HTTP サーバ起動に必要な最小限の container 化対応
- `server/run.sh` 廃止と、それに伴う README / quickstart / server 文書更新

### Forbidden Scope

- HTTP サーバの既存 API 契約変更
- ComfyUI custom node の仕様変更
- firmware 側の通信仕様変更
- 新しいオーケストレーション基盤や本格的な分散運用対応

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は `docker compose` だけで HTTP サーバを起動し、既存配信 / upload endpoint へ到達できる。
- **SC-002**: 既存の compose サービスと共存させても、HTTP サーバ追加により既存利用手順が壊れない。
- **SC-003**: HTTP サーバ起動手順から `server/run.sh` への依存が消え、文書は compose 前提に統一される。
- **SC-004**: 利用者は compose 運用下で、HTTP サーバの起動確認とログ確認を手順書から実施できる。

## Assumptions

- 既存の HTTP サーバ実装自体は継続利用し、主に起動・運用導線を compose へ移す。
- 既存 `compose.yml` は ComfyUI 系サービスの compose プロジェクトとして継続利用する。
- HTTP サーバの配信データは既存 `server/contents/` を基準に扱う。
- compose 化により、利用者は Docker / Docker Compose を使える前提になる。

## Documentation Impact

- README の HTTP サーバ導線を追加または更新する必要がある。
- `server/run.sh` に依存した手順書を見直す必要がある。
- compose 下での起動、停止、ログ確認、疎通確認の手順を記載する必要がある。

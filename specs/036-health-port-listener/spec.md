# 機能仕様: Health Port Listener

**Feature Branch**: `036-health-port-listener`  
**Created**: 2026-04-04  
**Status**: Draft  
**Input**: ユーザー記述: "環境変数PORTで本体のサーバはlistenしつつ、PORT_HEALTHでも/pingだけを提供するサーバをlistenするようにして。PORTとPORT_HEALTHは違う場合もあれば同じ場合もある"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - health check を別 port で受けたい (Priority: P1)

運用者は、通常の server port とは別に `PORT_HEALTH` でも `/ping` だけを受ける listener を持たせ、外部 health check から軽量に到達確認したい。

**Why this priority**: 今回の要求の中心は health check 専用 listener 追加であり、最優先の価値だから。

**Independent Test**: `PORT` と `PORT_HEALTH` を異なる値で起動し、`PORT` 側は既存 route を返し、`PORT_HEALTH` 側は `/ping` だけ `200 OK` を返すことを確認する。

**Acceptance Scenarios**:

1. **Given** `PORT` と `PORT_HEALTH` が異なる, **When** server が起動する, **Then** `PORT` で本体 server、`PORT_HEALTH` で `/ping` 専用 listener が待ち受ける。
2. **Given** `PORT_HEALTH` listener へ `/ping` を送る, **When** health check が実行される, **Then** `200 OK` を受け取れる。

---

### User Story 2 - 同じ port 指定でも壊したくない (Priority: P2)

運用者は、`PORT` と `PORT_HEALTH` が同じ値でも bind 競合を起こさず、main server 側の `/ping` をそのまま使いたい。

**Why this priority**: 同一 port 指定がありうる前提が明示されており、そこで起動不能になるのは許容できないため。

**Independent Test**: `PORT` と `PORT_HEALTH` を同じ値で起動し、起動失敗せず main server だけが listen して `/ping` を返すことを確認する。

**Acceptance Scenarios**:

1. **Given** `PORT` と `PORT_HEALTH` が同じ値である, **When** server が起動する, **Then** 二重 bind を試みずに正常起動する。

---

### User Story 3 - 既存導線と設定方法を維持したい (Priority: P3)

運用者は、既存の `PORT`、`/hello`、画像 route、upload の使い方を保ったまま、`PORT_HEALTH` の意味だけを文書で追いたい。

**Why this priority**: health listener 追加で既存運用や README が分かりにくくなるのを避けたいため。

**Independent Test**: README と quickstart を見て `PORT` と `PORT_HEALTH` の使い分け、既存 route への影響がないことを理解できる。

**Acceptance Scenarios**:

1. **Given** 利用者が server の設定項目と起動確認を見る, **When** `PORT_HEALTH` の説明を読む, **Then** health-only listener の有効条件と既存 route への影響を把握できる。

### Edge Cases

- `PORT_HEALTH` 未指定時は health-only listener を起動しないこと。
- `PORT_HEALTH` が `PORT` と同じ値でも起動失敗しないこと。
- health-only listener では `/ping` 以外を公開しないこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST `PORT` で既存の本体 server を listen し続けなければならない。
- **FR-002**: System MUST `PORT_HEALTH` が `PORT` と異なるとき、`/ping` だけを提供する health-only listener を追加で listen しなければならない。
- **FR-003**: System MUST `PORT_HEALTH` が未指定のとき、health-only listener を起動してはならない。
- **FR-004**: System MUST `PORT` と `PORT_HEALTH` が同じ値のとき、二重 bind を試みず main server の `/ping` を health check 導線として使わなければならない。
- **FR-005**: System MUST health-only listener で `/ping` 以外の route を提供してはならない。
- **FR-006**: System MUST この追加によって既存の `/hello`、`/`、`/image.bmp`、`/image.bin`、`/upload` の契約を変更してはならない。
- **FR-007**: System MUST `PORT_HEALTH` の使い方、未指定時の挙動、同一 port 時の挙動を server 利用文書へ記載しなければならない。

### Key Entities *(include if feature involves data)*

- **Main listener**: `PORT` で既存 route 群を提供する listener。
- **Health listener**: `PORT_HEALTH` で `/ping` のみを提供する補助 listener。
- **Port configuration**: `PORT` と `PORT_HEALTH` の組み合わせによる起動条件。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `server/` 配下での port 設定と listener 起動処理の変更
- health-only router 追加
- route / config / startup message / README の更新

### Forbidden Scope

- 既存画像処理や upload 仕様の変更
- firmware 側の変更
- `/ping` の response 契約変更

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `PORT` と `PORT_HEALTH` が異なるとき、運用者は 2 つの listen 先を起動でき、health port では `/ping` のみを確認できる。
- **SC-002**: `PORT` と `PORT_HEALTH` が同じとき、起動失敗せず main server 上の `/ping` を利用できる。
- **SC-003**: `PORT_HEALTH` 未指定時も既存 server 起動と route 回帰が成功する。
- **SC-004**: README だけで `PORT_HEALTH` の有効条件と使い方を理解できる。

## Assumptions

- health check は `GET /ping` のみを必要とし、`/hello` や画像 route は不要である。
- `PORT_HEALTH` は任意設定で、未指定時は従来どおり `PORT` だけで運用する。
- 同一 port 指定時は新しい listener を増やさず、main listener 上の `/ping` で代替する。

## Documentation Impact

- `server/README.md` に `PORT_HEALTH` の説明を追加する必要がある。
- feature 配下の quickstart で異なる port / 同じ port / 未指定の 3 パターンを説明する必要がある。

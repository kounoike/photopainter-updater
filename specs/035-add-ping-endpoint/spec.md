# 機能仕様: Ping 動作確認エンドポイント

**Feature Branch**: `035-add-ping-endpoint`  
**Created**: 2026-04-04  
**Status**: Draft  
**Input**: ユーザー記述: "http serverにエンドポイントを一つ追加して欲しい。/pingに200を返したい。コンテンツは無くて良いと思う。簡単だからSpecは簡単に書いて全部やっちゃって"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - サーバの到達性だけ確認したい (Priority: P1)

利用者は、画像状態や本文内容に依存せず、HTTP サーバへ到達できるかだけを `GET /ping` で確認したい。

**Why this priority**: 最小の疎通確認 endpoint を 1 つ追加することが今回の目的そのものであり、もっとも重要だから。

**Independent Test**: `GET /ping` を実行し、`200 OK` かつ空 body が返ることを確認する。

**Acceptance Scenarios**:

1. **Given** HTTP サーバが起動している, **When** 利用者が `GET /ping` を実行する, **Then** `200 OK` と空 body を受け取れる。
2. **Given** 画像入力が未配置または異常である, **When** 利用者が `GET /ping` を実行する, **Then** 画像状態に関係なく `200 OK` と空 body を受け取れる。

---

### User Story 2 - 確認手順を増やしすぎたくない (Priority: P2)

利用者は、既存の server 利用文書を見て `/ping` が軽量な疎通確認に使えることをすぐ理解したい。

**Why this priority**: endpoint を追加しても使い方が文書に出ていなければ運用上の価値が薄いため。

**Independent Test**: 利用文書を見て `/ping` が `200 OK` を返す軽量確認用 endpoint だと分かることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が server の確認手順を見る, **When** 起動確認方法を探す, **Then** `/ping` を使った確認方法を把握できる。

---

### User Story 3 - 既存 endpoint は維持したい (Priority: P3)

運用者は、`/ping` を追加しても既存の `/hello`、`/`、`/image.bmp`、`/image.bin`、`/upload`、fallback の契約を壊したくない。

**Why this priority**: 小さな疎通確認改善のために既存導線を壊すのは許容できないため。

**Independent Test**: `/ping` 追加後も既存の主要 endpoint と not found 応答が従来どおりであることを確認する。

**Acceptance Scenarios**:

1. **Given** 既存利用者が server の各 endpoint を利用している, **When** `/ping` が追加される, **Then** 既存 endpoint と fallback の期待挙動は維持される。

### Edge Cases

- `image.png` が未配置でも `/ping` は成功応答を返すこと。
- `/ping` の応答本文は空のままとし、疎通確認以外の情報を追加しないこと。
- 未定義 path は引き続き `404` と既存本文を維持すること。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST `GET /ping` で `200 OK` を返さなければならない。
- **FR-002**: System MUST `/ping` の response body を空にしなければならない。
- **FR-003**: System MUST `/ping` の応答を画像状態に依存させてはならない。
- **FR-004**: Users MUST be able to `/ping` を使って server の到達性確認を完了できなければならない。
- **FR-005**: System MUST `/ping` の追加によって既存の `/hello`、`/`、`/image.bmp`、`/image.bin`、`/upload`、未定義 path の期待挙動を変更してはならない。
- **FR-006**: System MUST server 利用文書に `/ping` を使った確認方法を含めなければならない。

### Key Entities *(include if feature involves data)*

- **Ping endpoint**: server の到達性だけを確認するための軽量 endpoint。
- **疎通確認応答**: `200 OK` と空 body からなる最小の成功 response。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `server/` 配下への `GET /ping` 追加
- `/ping` 用の route test 追加
- `server/README.md` と feature 文書の最小更新

### Forbidden Scope

- 既存 endpoint の意味や response format の変更
- firmware 側の変更
- 新しい認証、監視、運用機能の追加

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は server 起動後 1 回の `GET /ping` で `200 OK` を受け取り、到達性確認を完了できる。
- **SC-002**: `/ping` は画像未配置でも成功し、response body は常に空である。
- **SC-003**: `/ping` 追加後も既存 endpoint と fallback の回帰確認が成功する。
- **SC-004**: 利用文書だけで `/ping` の使い方を追加説明なしに理解できる。

## Assumptions

- 利用者はローカルまたは compose 経由で server へアクセスできる。
- `/ping` は最小の到達性確認専用であり、詳細な状態情報は返さない。
- 既存の `/hello` はそのまま残し、`/ping` は別の軽量導線として追加する。

## Documentation Impact

- `server/README.md` の起動確認に `/ping` を追加する必要がある。
- feature 配下の quickstart で `/ping` の確認手順を記載する必要がある。

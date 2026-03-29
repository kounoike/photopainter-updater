# 機能仕様: BMP配信HTTPサーバ

**Feature Branch**: `010-http-bmp-server`  
**Created**: 2026-03-29  
**Status**: Draft  
**Input**: ユーザー記述: "httpサーバを作る。最初は/に対してimage.bmpを返すだけのもの。image.bmpは必要になったタイミングでこちらが用意する"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 画像取得用の URL で画像を取得したい (Priority: P1)

利用者はローカル HTTP サーバの `/` または `/image.bmp` にアクセスして、PhotoPainter が取得する `image.bmp` をそのまま受け取りたい。

**Why this priority**: まず必要なのは、デバイスが参照できる最小の画像配信 endpoint を成立させることだから。

**Independent Test**: `image.bmp` を配置した状態でサーバを起動し、`/` と `/image.bmp` の両方へのアクセスが `image.bmp` の内容を返すことを確認する。

**Acceptance Scenarios**:

1. **Given** サーバが起動しており `image.bmp` が利用可能な状態, **When** 利用者が `/` にアクセスする, **Then** サーバは `image.bmp` をレスポンスとして返す。
2. **Given** サーバが起動しており `image.bmp` が利用可能な状態, **When** 利用者が `/image.bmp` にアクセスする, **Then** サーバは `/` と同じ `image.bmp` をレスポンスとして返す。

---

### User Story 2 - 画像未配置時の状態を判別したい (Priority: P2)

利用者は `image.bmp` がまだ用意されていない段階でも、サーバが壊れているのか、画像が未配置なのかを判別したい。

**Why this priority**: `image.bmp` は後から用意する前提なので、未配置時の挙動が曖昧だと起動確認や運用切り分けが難しくなるため。

**Independent Test**: `image.bmp` が無い状態でサーバを起動し、`/` と `/image.bmp` へのアクセスが「画像未配置」と分かる失敗応答になることを確認する。

**Acceptance Scenarios**:

1. **Given** サーバは起動しているが `image.bmp` が未配置の状態, **When** 利用者が `/` または `/image.bmp` にアクセスする, **Then** サーバは成功レスポンスを返さず、画像未配置と判別できる応答を返す。

---

### User Story 3 - 後から画像を差し替えて使いたい (Priority: P3)

利用者はサーバ実装を変更せずに、後から `image.bmp` を用意または差し替えて使いたい。

**Why this priority**: 今回のサーバは画像生成機能をまだ持たず、まずは運用で置いた `image.bmp` を返せれば価値があるため。

**Independent Test**: サーバ停止やコード変更なしで `image.bmp` を配置または差し替えた後、`/` と `/image.bmp` の次回アクセス時に新しいファイル内容が返ることを確認する。

**Acceptance Scenarios**:

1. **Given** サーバが `image.bmp` を参照して配信する状態, **When** 利用者が配信元の `image.bmp` を差し替える, **Then** `/` と `/image.bmp` の次回アクセス時に差し替え後の内容が返る。

### Edge Cases

- `image.bmp` が存在しない場合、成功扱いにせず未配置と分かる応答を返すこと。
- `image.bmp` 以外の配信機能や追加の可変 route は今回のスコープに含めないこと。
- PhotoPainter からの利用を想定し、`/` と `/image.bmp` の両方で同じ画像が取得できる構成を維持すること。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST ローカル HTTP サーバとして起動できなければならない。
- **FR-002**: System MUST `GET /` と `GET /image.bmp` の両方に対して同じ `image.bmp` をレスポンスとして返さなければならない。
- **FR-003**: System MUST `image.bmp` が未配置のとき、画像未配置と判別できる失敗応答を返さなければならない。
- **FR-004**: System MUST `image.bmp` の配置または差し替え後、追加の実装変更なしでその内容を次回配信に反映しなければならない。
- **FR-005**: System MUST 今回の範囲では `image.bmp` 配信以外の画像変換、画像生成、telemetry 受信を要求してはならない。

### Key Entities *(include if feature involves data)*

- **配信画像**: サーバが `/` と `/image.bmp` で返す単一の `image.bmp` ファイル。
- **画像取得応答**: `GET /` または `GET /image.bmp` に対して返る BMP 配信レスポンス、または画像未配置時の失敗応答。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- ローカル HTTP サーバの最小起動機能
- `/` と `/image.bmp` での `image.bmp` 配信
- `image.bmp` 未配置時の失敗応答
- この最小機能に対応する利用手順や開発文書の更新

### Forbidden Scope

- 画像変換、ディザリング、6 色インデックス化処理
- 複数 endpoint の追加
- telemetry POST や監視機能
- `firmware/` 側の仕様変更
- `xiaozhi-esp32/` 配下の直接変更

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者はサーバ起動後 1 回のアクセスで `/` または `/image.bmp` から配信画像を取得できる。
- **SC-002**: `image.bmp` 未配置時、利用者は 1 回のアクセスで「サーバ停止」ではなく「画像未配置」と判別できる。
- **SC-003**: 利用者はサーバ実装を変更せずに `image.bmp` を用意または差し替えて運用できる。
- **SC-004**: 初期実装は単一画像配信に限定され、追加機能なしで PhotoPainter の取得先として使える。

## Assumptions

- サーバはローカル LAN 内で利用する。
- `image.bmp` 自体の生成や用意は今回のスコープ外で、必要になったタイミングで利用者が別途配置する。
- まずは単一利用者または少数デバイス向けの最小構成を対象とする。

## Documentation Impact

- `specs/010-http-bmp-server/` 配下の設計成果物を新規作成する必要がある。
- 実装時にはサーバの起動方法と `image.bmp` 配置場所を案内する文書更新が必要になる可能性がある。

# 機能仕様: 画像ディザリング回転配信

**Feature Branch**: `013-image-dither-rotate`  
**Created**: 2026-03-29  
**Status**: Draft  
**Input**: ユーザー記述: "HTTPサーバで画像をディザリング＆回転する。image.pngを読み込み、PhotoShopでいう彩度+70に相当する色変換を行った後、ref/convert.pyと同等のディザリングをして、その結果を更に右に90度回転する。その結果を24bit BMPとして配信する"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 変換済み BMP を取得したい (Priority: P1)

利用者は `image.png` を元に変換された最新の 24bit BMP を、既存の HTTP 取得先から受け取りたい。

**Why this priority**: PhotoPainter が必要とする最終成果物は HTTP で取得できる変換済み BMP であり、これが成立しないと機能価値が生まれないため。

**Independent Test**: 既知の `image.png` を配置してサーバを起動し、`/` または `/image.bmp` へのアクセスが、彩度補正、既存参照と同等のディザリング、右 90 度回転を経た 24bit BMP を返すことを確認する。

**Acceptance Scenarios**:

1. **Given** サーバが起動しており `image.png` が利用可能な状態, **When** 利用者が `/` にアクセスする, **Then** サーバは `image.png` を変換した 24bit BMP を返す。
2. **Given** サーバが起動しており `image.png` が利用可能な状態, **When** 利用者が `/image.bmp` にアクセスする, **Then** サーバは `/` と同じ変換済み 24bit BMP を返す。

---

### User Story 2 - 参照変換と同等の見た目を維持したい (Priority: P2)

利用者は HTTP サーバの出力が既存参照変換と大きくずれず、色味とディザリング方針が期待どおりであることを確認したい。

**Why this priority**: 変換結果が参照変換とずれると、配信自体が成功しても表示品質要件を満たせないため。

**Independent Test**: 同じ `image.png` を入力として、既存参照変換とサーバ出力を比較し、彩度強調、ディザリング傾向、回転後の向きが一致することを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が既知の入力画像で出力結果を比較できる状態, **When** サーバから変換後 BMP を取得する, **Then** 出力は参照変換と同等のディザリング傾向と向きを持つ。

---

### User Story 3 - 入力画像の問題を切り分けたい (Priority: P3)

利用者は `image.png` が未配置、破損、または変換不能な場合に、サーバ障害ではなく入力画像側の問題だと判別したい。

**Why this priority**: 画像変換が配信前提になると、入力画像の問題を切り分けられないと運用判断が難しくなるため。

**Independent Test**: `image.png` を未配置または変換不能にした状態でサーバを起動し、取得時に入力画像問題と判別できる失敗応答になることを確認する。

**Acceptance Scenarios**:

1. **Given** サーバは起動しているが `image.png` が未配置または変換不能な状態, **When** 利用者が `/` または `/image.bmp` にアクセスする, **Then** サーバは成功レスポンスを返さず、入力画像の問題と判別できる応答を返す。

---

### Edge Cases

- `image.png` が存在しない場合、成功扱いにせず入力画像未配置と判別できる失敗応答を返すこと。
- `image.png` が読めても変換処理に必要な条件を満たさない場合、配信停止ではなく変換失敗と判別できる応答を返すこと。
- 同じ入力画像に対しては、連続アクセス時に同じ回転方向と同じ変換方針の BMP が返ること。
- 既存の取得先である `/` と `/image.bmp` の両方が、同一の変換済み結果を返すこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST `image.png` を入力画像として読み込み、取得要求ごとに変換済み画像を配信できなければならない。
- **FR-002**: System MUST 入力画像に対して、彩度を大きく強調した後に既存参照変換と同等のディザリングを行い、その結果を右 90 度回転した配信画像を生成しなければならない。
- **FR-003**: System MUST 変換後の配信画像を 24bit BMP として返さなければならない。
- **FR-004**: System MUST `GET /` と `GET /image.bmp` の両方に対して同一の変換済み 24bit BMP を返さなければならない。
- **FR-005**: System MUST `image.png` が未配置または変換不能な場合、入力画像側の問題と判別できる失敗応答を返さなければならない。
- **FR-006**: Users MUST be able to サーバ実装を再変更せずに `image.png` を差し替え、次回取得時に新しい変換結果を受け取れなければならない。
- **FR-007**: System MUST 今回の範囲では画像変換後の追加編集 UI、複数画像管理、追加 route を要求してはならない。

### Key Entities *(include if feature involves data)*

- **入力画像**: サーバが読み込む `image.png`。変換前の元画像として扱う。
- **変換済み配信画像**: 彩度強調、参照変換と同等のディザリング、右 90 度回転を経た 24bit BMP。
- **画像変換失敗応答**: 入力画像未配置または変換不能時に返る失敗レスポンス。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `image.png` を配信前に変換するサーバ処理
- 既存の BMP 配信導線を維持したまま変換済み結果を返す振る舞い
- 変換失敗時の応答と利用手順の更新

### Forbidden Scope

- 複数画像のアップロード管理や選択 UI
- 追加の HTTP route や画像編集 API
- `firmware/` 側の仕様変更
- `xiaozhi-esp32/` 配下の直接変更

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は `image.png` を用意した状態でサーバ起動後 1 回のアクセスで変換済み 24bit BMP を取得できる。
- **SC-002**: 利用者は同じ入力画像に対して、参照変換と同等のディザリング傾向と右 90 度回転済みの結果を受け取れる。
- **SC-003**: 利用者は `image.png` を差し替えた後、サーバ実装を再変更せずに次回取得時の変換結果へ反映できる。
- **SC-004**: `image.png` 未配置または変換不能時、利用者は 1 回のアクセスで入力画像起因の問題だと判別できる。

## Assumptions

- サーバは既存のローカル HTTP 配信機能を維持したまま画像変換を追加する。
- 入力画像は単一の `image.png` とし、まずは 1 枚の画像を変換して配信できればよい。
- 既存の参照変換は `ref/convert.py` のうち palette と dithering 方針を品質基準として扱い、サイズ調整は今回の必須範囲に含めない。
- 彩度強調の検証は `server/testdata/image-dither-rotate/pre.png` と `server/testdata/image-dither-rotate/post.png` を基準 fixture として行い、代表座標 `(4,4)` `(12,4)` `(4,12)` `(20,12)` `(12,20)` `(4,28)` `(12,28)` `(20,28)` の RGB を各チャネル差 `±6` 以内で比較する。

## Documentation Impact

- 変換済み BMP 配信の利用手順と `image.png` 配置方法を案内する文書更新が必要になる。
- 実装時には参照変換との比較方法を設計成果物へ記載する必要がある。

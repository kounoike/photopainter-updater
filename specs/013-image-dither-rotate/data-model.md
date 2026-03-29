# Data Model: 画像ディザリング回転配信

## Entity: 入力画像

- Purpose: サーバが変換元として読み込む `image.png` を表す。
- Fields:
  - `path`: 入力画像ファイルの保存場所
  - `exists`: 配信時に利用可能か
  - `dimensions`: 元画像の幅と高さ
  - `format`: PNG であること
  - `readable`: デコード可能か

## Entity: 変換パイプライン

- Purpose: 配信前に入力画像へ適用する一連の変換状態を表す。
- Fields:
  - `saturation_boost`: 彩度を大きく強調する補正段階
  - `dither_strategy`: `ref/convert.py` と同等のディザリング方針
  - `rotation`: 右 90 度回転
  - `output_format`: 24bit BMP
  - `deterministic`: 同一入力に対して同じ結果を返す性質

## Entity: 変換済み配信画像

- Purpose: `/` と `/image.bmp` で返す最終レスポンス画像を表す。
- Fields:
  - `content_type`: `image/bmp`
  - `bit_depth`: 24bit
  - `orientation`: 右 90 度回転済み
  - `source`: 元になった `image.png`

## Entity: 画像変換失敗応答

- Purpose: 入力画像未配置または変換失敗時に返す失敗結果を表す。
- Fields:
  - `status`: 失敗 HTTP ステータス
  - `reason`: 未配置または変換不能の区分
  - `message`: 利用者が入力画像起因と判別できる説明文

## Relationships

- `入力画像` は `変換パイプライン` の入力になる。
- `変換パイプライン` が成功した場合のみ `変換済み配信画像` が生成される。
- `入力画像` が利用不能、または `変換パイプライン` が失敗した場合は `画像変換失敗応答` が返る。

## Validation Rules

- 入力は `image.png` として解決できなければならない。
- 出力は常に 24bit BMP として返さなければならない。
- `/` と `/image.bmp` は同じ `変換済み配信画像` または同じ `画像変換失敗応答` を返さなければならない。
- 同じ `入力画像` に対しては、同じ `変換パイプライン` の結果を返さなければならない。

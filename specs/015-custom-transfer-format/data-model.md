# Data Model: 独自画像転送形式追加

## 1. BinaryFrameHeader

- Purpose:
  - `/image.bin` 応答の先頭に付く固定長ヘッダ。payload の妥当性確認と完了判定に使う。
- Fields:
  - `magic`: 独自形式であることを示す識別子。現行は `PPBF`
  - `version`: 形式バージョン。現行は `1`
  - `flags`: packing や将来互換性のための予約情報。現行は `0`
  - `header_length`: 固定長ヘッダ長。現行は `20`
  - `width`: 表示幅。現行は `800`
  - `height`: 表示高。現行は `480`
  - `payload_length`: 後続 payload の期待バイト数。現行は `192000`
  - `payload_checksum`: payload の全 byte を `u32` wrapping sum した整合性確認値
- Validation:
  - `magic` は期待値と一致しなければならない
  - `version` は firmware が理解できる値でなければならない
  - `width` と `height` は e-paper 表示範囲内でなければならない
  - `payload_length` は `width * height / 2` と整合していなければならない

## 2. BinaryFramePayload

- Purpose:
  - e-paper の 6 色 index を 4bit nibble で packed した描画データ
- Fields:
  - `packed_pixels`: 1 byte に 2 画素を持つ配列
- Rules:
  - 各 nibble は既存 e-paper color index と同じ値域を使う
  - payload 全体は header の `payload_length` と一致しなければならない
  - payload は firmware が中間 BMP 再構築なしに扱える並びでなければならない

## 3. TransferEndpointVariant

- Purpose:
  - 同じ入力画像に対する HTTP 取得先ごとの契約差分を表す
- Variants:
  - `bmp_compat_root`: `/`
  - `bmp_compat_image`: `/image.bmp`
  - `binary_epaper`: `/image.bin`
- Rules:
  - `bmp_compat_root` と `bmp_compat_image` は既存互換の BMP 応答でなければならない
  - `binary_epaper` のみが独自形式を返す

## 4. FirmwareUpdateSession

- Purpose:
  - firmware が 1 回の更新要求を処理した結果
- Route selection:
  - `image_url` の文字列末尾が `.bin` なら `binary_epaper` を使う
  - それ以外なら BMP 互換経路を使う
- States:
  - `started`
  - `http_connected`
  - `header_validated`
  - `payload_validated`
  - `displayed`
  - `failed_http`
  - `failed_input`
  - `failed_format`
- Transitions:
  - `started -> http_connected`: HTTP 接続成功
  - `http_connected -> header_validated`: ヘッダ検証成功
  - `header_validated -> payload_validated`: payload 長と checksum が一致
  - `payload_validated -> displayed`: 表示更新完了
  - 任意の途中状態 -> `failed_http`: 通信失敗または空応答
  - 任意の途中状態 -> `failed_input`: server が入力画像起因の失敗応答を返した
  - `header_validated` または `payload_validated` 前後 -> `failed_format`: magic/version/length/checksum 不一致

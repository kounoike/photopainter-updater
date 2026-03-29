# Contract: `/image.bin` 独自転送形式

## Purpose

`/image.bin` は PhotoPainter firmware 向けの専用取得先であり、e-paper 表示バッファへ中間 BMP 保存なしで反映できる応答を返す。

## Endpoint Rules

- Route:
  - `GET /image.bin`
- Existing compatibility:
  - `GET /`
  - `GET /image.bmp`
  - 上記 2 つは従来どおり BMP を返し、この contract の対象外とする

## Success Response

- Status:
  - `200 OK`
- Content meaning:
  - 固定長ヘッダの後ろに、e-paper 用 packed 4bit frame buffer payload が続く
- Required headers:
  - `Content-Type`: binary 独自形式であることが分かる値
  - `Content-Length`: ヘッダ + payload の総バイト数

## Binary Layout

### Header

- `magic`: 4 bytes
  - 独自形式識別子
- `version`: 1 byte
  - 現行 contract version
- `flags`: 1 byte
  - packing や将来拡張用
- `header_length`: 2 bytes
  - 固定長ヘッダの長さ
- `width`: 2 bytes
  - 表示幅
- `height`: 2 bytes
  - 表示高
- `payload_length`: 4 bytes
  - 後続 payload バイト数
- `payload_checksum`: 4 bytes
  - payload 整合性確認値
- `reserved`: 残り bytes
  - 将来拡張用

### Payload

- `packed_pixels`
  - 1 byte に 2 画素を持つ 4bit packed data
  - 各 nibble は既存 e-paper color index を使う
  - firmware はヘッダ検証後、この payload を中間 BMP 再構築なしで表示バッファへ反映できる

## Validation Rules

- `magic` が一致しない場合は形式不整合として失敗する
- `version` が未対応値なら失敗する
- `width` と `height` は e-paper 表示範囲と一致または許容範囲内でなければならない
- `payload_length` は `width * height / 2` と矛盾してはならない
- 実受信バイト数が `payload_length` 未満なら成功扱いにしてはならない
- `payload_checksum` が一致しない場合は成功扱いにしてはならない

## Failure Response Expectations

- 入力画像未配置:
  - 入力画像起因の失敗として判別できる応答
- 入力画像変換不能:
  - 変換失敗として判別できる応答
- 形式生成不能:
  - server 側内部失敗として判別できる応答

## Behavioral Constraints

- `/image.bin` の追加によって `/` と `/image.bmp` の BMP 応答を変更してはならない。
- firmware は `/image.bin` 利用時に SD カード上の中間 BMP 保存を必須としてはならない。
- 同じ入力画像なら、`/image.bin` と BMP 経路は利用者視点で同等の最終表示結果へ到達しなければならない。

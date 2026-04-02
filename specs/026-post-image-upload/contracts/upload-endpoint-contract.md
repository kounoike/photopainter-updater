# Contract: `POST /upload` 画像更新契約

## Purpose

既存の GET 配信契約を壊さずに、外部クライアントが現在画像 `image.png` を HTTP 経由で更新するための request / response 契約を定義する。

## Endpoint Contract

- Method: `POST`
- Path: `/upload`
- Authentication:
  - なし
- Supported request forms:
  - raw body
  - `multipart/form-data`

## Request Contract

### 1. raw body

- Headers:
  - `Content-Type` は画像として decode 可能な内容を示すことが望ましい
- Body:
  - 単一の画像データ
- Behavior:
  - サーバは本文を画像として decode し、PNG、JPG/JPEG、GIF、BMP、WebP のいずれかなら PNG へ正規化する

### 2. multipart/form-data

- Headers:
  - `Content-Type: multipart/form-data; boundary=...`
- Body:
  - 単一の画像ファイル field を含む
- Behavior:
  - サーバは保存対象となる単一画像 field を特定し、その中身を decode する
  - 有効な画像 file field を特定できない場合は失敗とする

## Accepted Media Contract

- Accepted formats:
  - PNG
  - JPG / JPEG
  - GIF
  - BMP
  - WebP
- Rejection rule:
  - 上記以外の形式、または上記を名乗っていても decode 不能なデータは受理しない

## Normalization Contract

- 受理可能な画像は保存前に PNG 形式へ正規化する
- 元画像が 480x800 でない場合は、アスペクト比を維持した拡大縮小の後に中央クロップし、保存結果を 480x800 にする
- 保存後の現在画像は常に `image.png` であり、既存 GET route がそのまま参照できること

## Success Response Contract

- Status:
  - `200 OK`
- Content-Type:
  - `text/plain; charset=utf-8`
- Body expectations:
  - 更新成功と判別できる文言を含む
  - 必要なら PNG 正規化や 480x800 正規化を行ったことが分かる要約を含めてよい

## Failure Response Contract

### Invalid image or malformed request

- `400 Bad Request`:
  - 空 body
  - multipart 構造不正
  - multipart 内に有効な画像 file field が存在しない
- `415 Unsupported Media Type`:
  - PNG/JPG/JPEG/GIF/BMP/WebP 以外の形式
  - 対応対象形式として扱えず decode 不能な画像データ
- Body expectations:
  - 入力不正、対応外形式、画像 decode 不可、multipart 内の画像不足のいずれかを判別できる
- Preservation rule:
  - 既存の `image.png` を変更しない

### Save failure or internal failure

- Status:
  - `500 Internal Server Error`
- Body expectations:
  - 保存失敗または内部失敗と判別できる
- Preservation rule:
  - 既存の `image.png` を変更しない

## Compatibility Contract

- `GET /`
- `GET /image.bmp`
- `GET /image.bin`

上記既存 route は upload 機能追加後も path、利用手順、成功時の応答種別を変更しない。

## Logging Contract

- `POST /upload` も既存のアクセスログ導線で記録する
- 少なくとも method、path、status、remote、成功/失敗分類を 1 request 1 行で追えること

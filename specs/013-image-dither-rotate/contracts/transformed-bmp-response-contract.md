# Contract: Transformed BMP Response

## Purpose

既存の取得先 `/` と `/image.bmp` を維持したまま、`image.png` を変換した 24bit BMP を返す最小契約を定義する。

## Success Response

- Request:
  - Method: `GET`
  - Path: `/`
- Conditions:
  - `image.png` が利用可能で、変換処理が成功する
- Response:
  - Status: `200 OK`
  - Content-Type: `image/bmp`
  - Body: 彩度強調、参照相当ディザリング、右 90 度回転を経た 24bit BMP

- Request:
  - Method: `GET`
  - Path: `/image.bmp`
- Conditions:
  - `image.png` が利用可能で、変換処理が成功する
- Response:
  - Status: `200 OK`
  - Content-Type: `image/bmp`
  - Body: `/` と同一の変換済み 24bit BMP

## Input Image Failure Response

- Request:
  - Method: `GET`
  - Path: `/` または `/image.bmp`
- Conditions:
  - `image.png` が未配置
- Response:
  - Status: 成功以外の失敗ステータス
  - Content-Type: `text/plain; charset=utf-8`
  - Body: 入力画像未配置と判別できる短い説明文

- Request:
  - Method: `GET`
  - Path: `/` または `/image.bmp`
- Conditions:
  - `image.png` は存在するが変換処理に失敗する
- Response:
  - Status: 成功以外の失敗ステータス
  - Content-Type: `text/plain; charset=utf-8`
  - Body: 変換失敗と判別できる短い説明文

## Behavioral Rules

- サーバは取得要求ごとに `image.png` を基準として変換結果を返す。
- `/` と `/image.bmp` は常に同じ結果を返す。
- 出力は 24bit BMP でなければならない。
- 今回の契約は追加 route、複数画像選択、編集 UI を含まない。

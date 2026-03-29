# Contract: Root BMP Response

## Purpose

PhotoPainter と利用者が同じ URL ルールで利用できるよう、`GET /` と `GET /image.bmp` の最小契約を定義する。

## Success Response

- Request:
  - Method: `GET`
  - Path: `/`
- Conditions:
  - `server/contents/image.bmp` が存在する
- Response:
  - Status: `200 OK`
  - Content-Type: `image/bmp`
  - Body: `image.bmp` のバイト列

- Request:
  - Method: `GET`
  - Path: `/image.bmp`
- Conditions:
  - `server/contents/image.bmp` が存在する
- Response:
  - Status: `200 OK`
  - Content-Type: `image/bmp`
  - Body: `/` と同じ `image.bmp` のバイト列

## Missing Image Response

- Request:
  - Method: `GET`
  - Path: `/`
- Conditions:
  - `server/contents/image.bmp` が存在しない
- Response:
  - Status: `404 Not Found`
  - Content-Type: `text/plain; charset=utf-8`
  - Body: 画像未配置と判別できる短い説明文

- Request:
  - Method: `GET`
  - Path: `/image.bmp`
- Conditions:
  - `server/contents/image.bmp` が存在しない
- Response:
  - Status: `404 Not Found`
  - Content-Type: `text/plain; charset=utf-8`
  - Body: `/` と同じ画像未配置説明文

## Behavioral Rules

- サーバはリクエストごとに `server/contents/image.bmp` の最新状態を参照する。
- 今回の固定 route は `/` と `/image.bmp` のみである。
- 追加 route や query parameter は今回の契約に含めない。
- `/` と `/image.bmp` 以外の route 挙動はこの feature の保証対象外とする。

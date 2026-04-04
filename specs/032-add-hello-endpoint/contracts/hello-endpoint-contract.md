# Contract: `GET /hello` 疎通確認契約

## Purpose

利用者が画像ファイルや画像変換状態に依存せず、HTTP サーバ自体が起動して応答可能かを 1 request で確認するための契約を定義する。

## Endpoint Contract

- Method:
  - `GET`
- Path:
  - `/hello`
- Authentication:
  - なし

## Request Contract

- Headers:
  - 特別な必須 header はない
- Body:
  - なし
- Behavior:
  - server は request body、画像ファイル、変換済み出力の有無に依存せず成功応答を返す

## Success Response Contract

- Status:
  - `200 OK`
- Content-Type:
  - `text/plain; charset=utf-8`
- Body expectations:
  - 本文は `hello`
  - 画像状態や内部 profile 情報のような追加診断を含めない

## Error Handling Contract

- `GET /hello` 自体は画像未配置や画像 decode 失敗を理由に失敗してはならない
- `/hello` 以外の未定義 path は引き続き not found を返す

## Compatibility Contract

- `GET /`
- `GET /image.bmp`
- `GET /image.bin`
- `POST /upload`

上記既存 route は `/hello` 追加後も path、役割、主要な成功 / 失敗条件を変更しない。

## Logging Contract

- `/hello` への request も既存 access log 導線で記録する
- 少なくとも `method`、`path`、`status`、`remote`、成功 outcome を追えること

# Data Model: Ping 動作確認エンドポイント

## 1. PingProbeRequest

- Purpose:
  - `/ping` で server 到達性を確認する request。
- Fields:
  - `method`: `GET`
  - `path`: `/ping`
- Validation:
  - `method` は `GET` のみ

## 2. PingProbeResponse

- Purpose:
  - `/ping` の成功 response を表す。
- Fields:
  - `status`: `200 OK`
  - `body_length`: `0`
- Validation:
  - body は空であること

## 3. RouteContractSet

- Purpose:
  - `/ping` と既存 route の共存条件を表す。
- Fields:
  - `new_route`: `/ping`
  - `existing_routes`: `/hello` `/` `/image.bmp` `/image.bin` `/upload`
  - `fallback_policy`: `404 route not found`
- Validation:
  - `new_route` 追加で既存 route 契約を変更しないこと

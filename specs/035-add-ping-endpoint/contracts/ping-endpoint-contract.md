# Contract: Ping 動作確認エンドポイント

## Purpose

server 到達性だけを確認するための最小 endpoint `/ping` の契約を定義する。

## Request Contract

- Method:
  - `GET`
- Path:
  - `/ping`

## Response Contract

- Success status:
  - `200 OK`
- Body:
  - 空

## Logging Contract

- 既存の access log 経路を再利用する
- `path` は `/ping`
- `status` は `200`

## Scope Guard Contract

- `/hello` と既存画像 route は維持する
- 未定義 path の fallback 契約は維持する

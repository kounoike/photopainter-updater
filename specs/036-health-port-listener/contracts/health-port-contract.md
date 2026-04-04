# Contract: Health Port Listener

## Purpose

`PORT` の main listener を維持しながら、必要に応じて `PORT_HEALTH` で `/ping` だけを返す listener を追加する契約を定義する。

## Configuration Contract

- `PORT`:
  - 必須
- `PORT_HEALTH`:
  - 任意

## Listener Contract

- `PORT_HEALTH` 未指定:
  - health-only listener を起動しない
- `PORT_HEALTH == PORT`:
  - main listener の `/ping` を使い、追加 bind をしない
- `PORT_HEALTH != PORT`:
  - `/ping` だけを返す dedicated listener を追加で起動する

## Route Contract

- Main listener:
  - 既存 route 群と `/ping`
- Health listener:
  - `/ping` のみ
- Guard:
  - health listener は `/ping` 以外を公開しない

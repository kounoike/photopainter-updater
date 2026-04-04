# Research: Health Port Listener

## Decision 1: `PORT_HEALTH` は任意設定にする

- Decision:
  - `PORT_HEALTH` は未指定なら無効とする。
- Rationale:
  - 既存運用を壊さず追加設定として導入できる。
- Alternatives considered:
  - 必須設定にする: 既存起動方法を壊す。

## Decision 2: 同一 port 時は追加 listener を起動しない

- Decision:
  - `PORT_HEALTH == PORT` の場合は main listener 上の `/ping` を使う。
- Rationale:
  - 二重 bind を避けながら user 要求を満たせる。
- Alternatives considered:
  - 無条件に追加 bind を試す: 起動失敗になる。

## Decision 3: health-only router は `/ping` だけを公開する

- Decision:
  - health-only listener は `/ping` と fallback のみ持つ。
- Rationale:
  - health check 専用導線を明確に保てる。
- Alternatives considered:
  - main router 全体を再利用する: `/ping` 以外も露出してしまう。

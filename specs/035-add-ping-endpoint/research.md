# Research: Ping 動作確認エンドポイント

## Decision 1: `/ping` は空 body の `200 OK` に固定する

- Decision:
  - `GET /ping` は本文なしで `200 OK` を返す。
- Rationale:
  - ユーザー要件が「コンテンツは無くて良い」で明確で、最小の疎通確認に一致する。
  - `/hello` と役割を分けやすい。
- Alternatives considered:
  - `pong` などの本文を返す: 今回の要求より広い。

## Decision 2: 実装は既存 `routes.rs` に閉じる

- Decision:
  - route / handler / route test は `server/src/routes.rs` に追加する。
- Rationale:
  - 既存の `/hello` と同じ責務境界で扱えるため最小変更で済む。
- Alternatives considered:
  - 新規モジュール分割: 今回スコープでは過剰。

## Decision 3: `/hello` は維持し `/ping` と共存させる

- Decision:
  - `/hello` を置き換えず、`/ping` を追加で持つ。
- Rationale:
  - 既存契約を壊さず、`/ping` は空 body のさらに軽い check として使える。
- Alternatives considered:
  - `/hello` を `/ping` に置き換える: 既存導線を壊す。

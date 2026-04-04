# Research: RunPod Authenticated Fetch

## Decision 1: `bearer_token` は任意 non-empty string として追加する

- Decision:
  - `config.txt` に任意 string の `bearer_token` を追加し、存在する場合は non-empty を必須とする。
- Rationale:
  - 外部更新元の実用要件を満たしつつ、認証不要な既存更新元の後方互換を維持できる。
- Alternatives considered:
  - `bearer_token` を必須にする: 既存 HTTP / 認証不要 HTTPS 運用を壊す。
  - 空文字を許可する: 設定ミスと未設定の区別が曖昧になる。

## Decision 2: Bearer トークンは `Authorization: Bearer <token>` だけで送る

- Decision:
  - `bearer_token` が設定されている場合だけ `Authorization: Bearer <token>` を付与する。
- Rationale:
  - 利用者要求に直接対応し、認証方式の解釈を増やさず最小構成に保てる。
- Alternatives considered:
  - `X-API-Key` など複数方式を同時対応する: スコープが拡大する。
  - クエリ文字列へトークンを埋め込む: 契約が不明確になり安全性も下がる。

## Decision 3: `insecure` は Bearer 認証と独立した HTTPS 検証例外にする

- Decision:
  - `insecure` は Bearer トークン有無にかかわらず HTTPS のサーバ証明書検証無効化だけを制御する。
- Rationale:
  - 認証と TLS 検証の責務を分離でき、影響範囲を最小化できる。
- Alternatives considered:
  - Bearer トークン利用時は常に `insecure` を有効化する: 安全側既定値に反する。
  - 認証失敗時も `insecure` で緩和する: failure category が崩れる。

## Decision 4: 通常の HTTPS は certificate bundle で検証する

- Decision:
  - `https://` かつ `insecure` 未設定または `false` の場合は certificate bundle を使って証明書検証する。
- Rationale:
  - ESP-IDF 側の既存設定を活かしつつ、安全側既定値を維持できる。
- Alternatives considered:
  - 常に未検証 HTTPS にする: 運用品質を下げる。
  - 個別 CA 設定を `config.txt` に追加する: 今回のスコープを超える。

## Decision 5: 認証設定不備は通信開始前に config error とする

- Decision:
  - `bearer_token` 型不正、空文字、`insecure` 型不正はネットワーク接続前に設定不備として失敗させる。
- Rationale:
  - 設定ミス、認証拒否、通信障害を切り分けやすくなる。
- Alternatives considered:
  - 空文字を未設定扱いにする: 利用者の入力ミスが隠れる。
  - 型不正を暗黙変換する: 予期しない挙動になる。

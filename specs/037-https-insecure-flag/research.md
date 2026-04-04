# Research: Config Insecure HTTPS

## Decision 1: `insecure` は任意 boolean として追加する

- Decision:
  - `config.txt` に任意 boolean の `insecure` を追加し、未設定時は `false` として扱う。
- Rationale:
  - 既存設定との後方互換を維持しながら、利用者が明示的に危険な設定を有効化した場合だけ例外経路へ入れられる。
- Alternatives considered:
  - `insecure` を必須にする: 既存 `config.txt` を壊す。
  - `"true"` / `"false"` の文字列も許可する: 設定ミスの検出力が落ちる。

## Decision 2: `image_url` は `http://` と `https://` を受け付ける

- Decision:
  - `image_url` の許容 scheme を `http://` と `https://` に拡張する。
- Rationale:
  - 今回の機能価値は HTTPS 更新を可能にすることにあり、設定読込段階で `https://` を拒否すると feature が成立しない。
- Alternatives considered:
  - `http://` のまま据え置く: 仕様要求を満たせない。
  - `https://` だけ許可する: 既存 HTTP 運用の後方互換を壊す。

## Decision 3: 通常の HTTPS は certificate bundle で検証する

- Decision:
  - `insecure` 未設定または `false` の HTTPS では、ESP-IDF の certificate bundle を使ってサーバ証明書検証を行う。
- Rationale:
  - 現在の firmware には TLS 機能と certificate bundle 設定が既にあり、安全側の既定値を維持したまま HTTPS を有効化できる。
- Alternatives considered:
  - 常に証明書検証を無効化する: 安全側既定値に反する。
  - 個別 PEM ファイルを `config.txt` に持たせる: 今回のスコープを超える。

## Decision 4: `insecure: true` は HTTPS の証明書検証だけを省略する

- Decision:
  - `insecure: true` の効果は `https://` 利用時のサーバ証明書検証無効化に限定し、HTTP や他の通信失敗処理には影響させない。
- Rationale:
  - 利用者要求を満たしつつ、影響範囲を最小に保てる。HTTP はそもそも証明書検証を行わないため、意味のない分岐を増やさずに済む。
- Alternatives considered:
  - `insecure` を HTTP にも適用する: 挙動が不明確で不要。
  - 通信失敗全般を緩和する: failure category の意味が崩れる。

## Decision 5: `insecure` 型不正は config error として扱う

- Decision:
  - `insecure` が存在しても boolean でない場合は、ネットワーク接続前に設定不備として失敗させる。
- Rationale:
  - 設定ミスを通信障害と区別でき、現地切り分けが容易になる。
- Alternatives considered:
  - truthy / falsy で暗黙変換する: 予期しない危険設定が有効化され得る。
  - 型不正を無視して `false` 扱いする: 設定ミスが隠れる。

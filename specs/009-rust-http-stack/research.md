# Research: Rust HTTPスタック再評価

## Baseline Comparison Conditions

- 比較対象は `axum`、`actix-web`、`warp` の 3 候補とする。
- 比較軸は 008 の前提を維持し、画像前処理適合性、telemetry API 適合性、保守性、依存の重さ、配布容易性、開発体験とする。
- 画像前処理要件は [convert.py](/workspaces/photopainter-updater/ref/convert.py) を参照し、フルカラー画像入力、回転、スケーリング、ディザリング、6 色インデックス化を含む。
- telemetry 要件はデバイスからの `HTTP POST` による `battery_level` 受信を初期想定とし、Grafana などの監視基盤へ接続しやすい構成を評価する。
- 今回は Rust 採用自体を再比較しない。008 の「Rust を第一候補にする」判断は維持し、その中で HTTP framework を再評価する。

## Decision 1: 最終候補は `axum` を維持する

- Decision: 後続の Rust サーバ実装 feature に渡す最終候補は `axum` とする。
- Rationale: `axum` は ergonomics と modularity を前面に出し、Tokio と Hyper の上に素直に乗る構成で、extractor、response、error handling、middleware、state 共有が整理されている。加えて Tower ecosystem をそのまま使えるため、tracing、timeout、compression、authorization などの HTTP 横断関心事を後付けしやすい。今回のサーバは画像前処理が本体責務になり得るため、HTTP framework 側では handler と middleware を過度に独自化せず、Tokio/Hyper/Tower の標準的な非同期 stack に乗れることが有利である。telemetry POST も `Json` extractor と state 注入で素直に扱える。  
  Sources: [axum docs](https://docs.rs/axum/latest/axum/), [Tokio stack](https://tokio.rs/)
- Alternatives considered:
  - `actix-web`
    - かなり有力だが、今回の比較では Tower 系資産との接続性と、後続 feature で複雑化しにくい構成の点で `axum` を上回らなかった。
  - `warp`
    - composable filter は魅力だが、今回の責務では filter 中心モデルが長期保守上の優位になりにくい。

## Decision 2: 第一対抗候補は `actix-web` とする

- Decision: `actix-web` は第一対抗候補として残す。
- Rationale: 公式サイトと crate docs は `actix-web` を powerful、pragmatic、extremely fast と位置づけており、HTTP/1.x/2、streaming、compression、WebSocket、middleware、extractor を広く備える。telemetry API のような典型的な JSON POST と、将来の静的配信や追加 endpoint をまとめるには十分強い。画像前処理そのものは framework 差よりライブラリ差の比率が大きいため、HTTP framework 単体の能力では `actix-web` でも問題ない。ただし今回欲しいのは HTTP framework 自体の多機能さより、Tokio/Tower 系の標準的 stack との自然な接続と、後続実装での説明しやすさである。その点で `axum` を逆転させる決定打にはならなかった。  
  Sources: [Actix Web site](https://actix.rs/), [actix-web docs](https://docs.rs/actix-web/latest/actix_web/)
- Alternatives considered:
  - `actix-web` を最終候補に上げる
    - 見送り理由: 必要機能は十分だが、今回の比較軸では `axum` を覆すほどの優位が明確でない。

## Decision 3: `warp` は参考候補に留める

- Decision: `warp` は参考候補として記録するが、最終候補や第一対抗候補にはしない。
- Rationale: `warp` は filter system による composable な route 定義と test module を備え、Hyper 上で動くため HTTP/1/2 と非同期基盤の点でも十分有力である。一方で、今回の将来責務は単純な route composition よりも、画像前処理 job、telemetry 受信、周辺 middleware、将来の server-side pre-render 契約へ自然に拡張できることにある。filter ベースの組み方は小さな API では魅力だが、責務が増えた時に `axum` の handler + extractor + Tower middleware モデルより優位とまでは言いにくい。加えて 008 からの再比較コストを正当化するには、`warp` が勝ち筋を明確に示す必要があるが、今回はそこまでの差は確認できなかった。  
  Sources: [warp docs](https://docs.rs/warp/latest/warp/)
- Alternatives considered:
  - `warp` を第一対抗候補に上げる
    - 見送り理由: composability は強いが、今回の責務と比較軸では `actix-web` より上に置く根拠が弱い。

## Decision 4: 画像前処理 workload の優位は HTTP framework 自体より ecosystem との噛み合わせで決まる

- Decision: 画像前処理適合性の差は、HTTP framework 単体よりも Tokio/Hyper/Tower 系との接続の素直さと、Rust 側の画像処理ライブラリ選定を進めやすい構成かで評価する。
- Rationale: [convert.py](/workspaces/photopainter-updater/ref/convert.py) が示す処理は、HTTP route の巧拙よりも、CPU 寄りの画像変換 pipeline を server-side job としてどう整理するかが本質である。そのため `axum`、`actix-web`、`warp` のいずれでも実現自体は可能だが、`axum` は Tower middleware と state 管理を含めて job orchestration を整理しやすい。`actix-web` も十分可能で、`warp` も実現可能だが、今回は ecosystem 接続性と後続 feature での説明しやすさを優先する。
- Alternatives considered:
  - 画像処理向け補助 crate の豊富さだけで framework を決める
    - 却下理由: 画像処理 crate は framework 横断で使えるため、HTTP framework 決定因にはなりにくい。

## Decision 5: telemetry API ではどの候補でも実現可能だが、`axum` と `actix-web` が長期保守しやすい

- Decision: telemetry POST と監視連携自体は 3 候補とも可能だが、長期保守性では `axum` と `actix-web` が `warp` より有利とみなす。
- Rationale: telemetry は JSON body の受信、validation、状態注入、metrics export 連携が主要関心であり、`axum` は extractor と response モデル、`actix-web` は responder と extractor、middleware 周辺がそれぞれ素直である。`warp` も JSON body や filter composition は強いが、今回の将来責務では API が増えるたびに filter 中心の組み立てを追う負荷が相対的に高くなる。Grafana 監視や通知へ進む場合も、structured logging や middleware 連携を整理しやすい方が有利である。
- Alternatives considered:
  - telemetry だけを基準に `warp` を高く評価する
    - 却下理由: telemetry 単体では十分でも、画像前処理と合わせた全体責務では優位が薄い。

## Decision 6: 008 の結論は維持しつつ、Rust 内の順序を `axum` > `actix-web` > `warp` に固定する

- Decision: 008 の「Rust を第一候補にする」結論は維持し、その内訳は `axum` を最終候補、`actix-web` を第一対抗候補、`warp` を参考候補とする。
- Rationale: これで 008 の比較結果と矛盾せず、後続 feature は Python/Go に戻らず Rust 実装に進みつつ、Rust 内でも `axum` 決め打ちに見える不安を解消できる。必要十分な比較を残したうえで、再比較コストをここで止められる。
- Alternatives considered:
  - 008 の `axum` を再検討せず据え置く
    - 見送り理由: Rust 実装に入る前に、Rust 内比較が省略されたままだと再度同じ疑問が発生するため。


# Research: POST画像保存

## Decision 1: `POST /upload` は raw body と multipart/form-data の両方を受け付ける

- Decision: 更新 endpoint は `POST /upload` に固定し、`Content-Type` を見て raw body と multipart/form-data の処理を分ける。multipart の受理には `axum` の `Multipart` extractor を使う。
- Rationale: spec で両形式の受理が確定している。`axum` の公式 docs.rs では `Multipart` は `multipart` feature 前提の extractor とされており、追加依存なしで HTTP 層に統合しやすい。path を既存の GET route から分離すると、取得契約との衝突も避けられる。
- Alternatives considered:
  - raw body のみ受け付ける: 実装は単純だが、clarify で確定した multipart 要件を満たせない。
  - multipart/form-data のみ受け付ける: curl や簡易クライアントからの送信が冗長になる。
  - 別 crate で typed multipart を導入する: 現状の `axum` だけで必要要件を満たせるため過剰。

## Decision 2: 画像形式の判定と PNG 正規化は `image` crate に統一し、受理対象は 5 形式へ固定する

- Decision: 受信したバイト列は `image` crate で decode し、受理対象を PNG、JPG/JPEG、GIF、BMP、WebP に固定する。保存時は `write_to(..., ImageFormat::Png)` で `image.png` に正規化する。
- Rationale: 既存 server crate はすでに `image` を使って入力画像 decode と変換を行っているため、同じ依存で upload 側も閉じられる。`image` の docs.rs には `ImageReader::new(...).with_guessed_format()` や `DynamicImage::write_to` があり、拡張子依存ではなく内容ベースで decode と再 encode を完結できる。受理形式を 5 つに明示すると、必要な codec feature とテストデータを実装前に固定できる。
- Alternatives considered:
  - PNG だけ受け付ける: ユーザーが明示的に指定した 5 形式受理に反する。
  - 画像形式ごとに個別 crate を追加する: 依存が増え、既存パイプラインとの一貫性も落ちる。
  - 生バイトをそのまま保存して後段で decode する: 保存結果を常に `image.png` とする spec に反する。

## Decision 2.5: upload 失敗時の status code を入力種別ごとに固定する

- Decision: 空 body、multipart 構造不正、multipart 内の画像不足は `400 Bad Request`、PNG/JPG/JPEG/GIF/BMP/WebP 以外の対応外形式または decode 不可は `415 Unsupported Media Type`、保存失敗は `500 Internal Server Error` とする。
- Rationale: contract test と HTTP handler の期待値を先に固定しないと、同じ失敗でも実装者ごとに status code がぶれる。入力不備と media/decode 問題、保存失敗を分けると、応答本文とログの両方で原因を切り分けやすい。
- Alternatives considered:
  - 失敗をすべて `400` に寄せる: decode 不可や対応外形式との区別が消える。
  - `400` と `415` を実装者判断に任せる: contract が固定されず、テストが不安定になる。
  - `422` を使う: 現行 contract と tasks の期待値を増やし、必要以上に分岐が増える。

## Decision 3: 480x800 正規化はアスペクト比維持の中央クロップ規則を固定する

- Decision: decode 後の画像は 480x800 と異なる場合、アスペクト比を維持して target を満たすまで拡大縮小し、中央クロップで 480x800 に整える。
- Rationale: clarify でこの規則が確定している。`image` の docs.rs には `DynamicImage::resize_to_fill` があり、中央クロップ前提の「埋める」正規化を素直に表現できる。余白付与や引き伸ばしよりも、現在の配信入力画像として一貫した見た目を保ちやすい。
- Alternatives considered:
  - 余白追加で全体を収める: 画面全体を埋めず、既存配信前提とずれる。
  - 変形して 480x800 に引き伸ばす: 画像の見た目が歪み、受け入れテストも不自然になる。
  - 上寄せクロップや自動注目領域検出: 規則が増えてテスト期待値がぶれる。

## Decision 4: 現在画像の更新は一時ファイル経由で原子的に置き換える

- Decision: 正規化が成功した画像は `image.png.tmp` のような一時ファイルへ書き出し、書き切れた後で `image.png` を置換する。
- Rationale: spec では失敗時に既存 `image.png` を壊さないことが必須である。直接上書きでは途中失敗で現在画像を破損させる可能性があるため、一時ファイル経由の置換が最小で安全な手法になる。
- Alternatives considered:
  - 既存ファイルへ直接上書きする: 途中失敗で部分書き込みが残りうる。
  - 履歴ファイルを複数保持する: Forbidden Scope の履歴保存に踏み込む。
  - 外部ストレージや DB を使う: ローカル優先と最小構成に反する。

## Decision 5: `POST /upload` も既存アクセスログ導線へ統合する

- Decision: upload 成功・入力不正・保存失敗・内部失敗を `logging.rs` の request log に統合し、`POST /upload` も既存 GET route と同じ request counter と remote 記録を使う。
- Rationale: spec は応答とログの両方で成否判定できることを求めている。既存の `record(...)` 導線へ統合すれば、新しい監視基盤を増やさずに観測性を揃えられる。
- Alternatives considered:
  - upload だけ別ログにする: 運用者の確認導線が増える。
  - 応答本文だけで判定させる: サーバ側記録が残らず、失敗時の追跡が弱い。
  - 詳細監視基盤を追加する: Forbidden Scope に抵触する。

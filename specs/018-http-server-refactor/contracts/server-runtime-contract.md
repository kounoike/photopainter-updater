# Contract: HTTPサーバ runtime 維持契約

## Purpose

HTTP サーバの内部構成を整理しても、利用者と運用者が依存している起動方法、主要 route、失敗時の案内、ログ確認導線を壊さないための契約を定義する。

## Runtime Entry Contract

- 起動導線:
  - `server/run.sh`
  - `cargo run --release` を使う既存導線を維持する
- 入力:
  - 環境変数 `PORT`
  - 環境変数 `CONTENT_DIR`
  - 既存のディザ関連環境変数群
- Behavior:
  - 未指定の設定は既定値で補完される
  - 不正値は起動失敗として判別できる案内を返す
  - 起動成功時は待受先、入力元、主要変換設定、ログ確認方法を同じ確認導線で把握できる
  - `server/run.sh` の事前案内とアプリ本体の `tracing` startup log が矛盾しない

## HTTP Route Contract

- `GET /`
  - firmware 向け binary 応答を返す既存契約を維持する
- `GET /image.bmp`
  - BMP 応答を返す既存契約を維持する
- `GET /image.bin`
  - firmware 向け binary 応答を返す既存契約を維持する
- その他の route:
  - 従来どおり not found 扱いとする

## Response Behavior Contract

- 正常系:
  - 既存 route ごとの `Content-Type` と body 形式を変更しない
- 入力画像未配置:
  - 利用者が入力不足と判別できる失敗応答を返す
- 入力画像 decode 失敗:
  - 利用者が入力画像起因の失敗と判別できる応答を返す
- 変換または内部失敗:
  - 成功と誤認しない失敗応答を返す

## Logging Contract

- 起動時:
  - 待受 URL、入力元、主要設定、停止方法を `tracing` の startup log で確認できる
- リクエスト時:
  - path、status、成功/失敗分類、可能なら remote を `tracing` の 1 リクエスト 1 行で確認できる
- 失敗時:
  - 正常系と同じ導線で原因分類を把握できる

## Refactoring Constraints

- route 名、起動引数の意味、既定の入力元を変更してはならない
- 画像変換アルゴリズムの利用者視点の結果を意図的に変えてはならない
- 内部モジュール分割のために `firmware/` や `xiaozhi-esp32/` を変更してはならない

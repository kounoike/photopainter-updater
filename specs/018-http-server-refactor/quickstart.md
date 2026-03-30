# Quickstart: HTTPサーバ構成整理

## 1. 既定値起動確認

1. `server/run.sh` を既定値のまま起動する。
2. `server/run.sh` の事前案内に待受先と route 一覧が出ることを確認する。
3. アプリ本体の `tracing` startup log に待受先、入力元、binary/BMP の取得先、主要変換設定、停止方法が出ることを確認する。
4. 既定の入力元が `server/contents` と整合することを確認する。

## 2. 設定上書き確認

1. `PORT` を明示して `server/run.sh` を起動する。
2. 起動ログの待受先が指定ポートへ変わることを確認する。
3. `CONTENT_DIR` もしくは `server/run.sh [CONTENT_DIR]` で入力元を変更し、起動ログへ反映されることを確認する。

## 3. 不正設定確認

1. `PORT` に数値以外を与えて起動する。
2. 起動が成功扱いにならず、不正設定であることが分かる案内が出ることを確認する。
3. 必要に応じてディザ関連の不正値も与え、同様に原因が判別できることを確認する。

## 4. HTTP route 回帰確認

1. サーバ起動後に `GET /`、`GET /image.bmp`、`GET /image.bin` へアクセスする。
2. `/` と `/image.bin` が従来どおり binary 応答を返すことを確認する。
3. `/image.bmp` が従来どおり BMP 応答を返すことを確認する。
4. 存在しない path が not found 応答になることを確認する。

## 5. ログ導線確認

1. 正常なリクエストを 1 回送る。
2. `tracing` のアクセスログが 1 リクエストにつき 1 行出ることを確認する。
3. 入力画像未配置または decode 不能な画像で失敗系を発生させる。
4. 正常系と失敗系が同じ導線で区別できることを確認する。

## 6. 自動テスト確認

1. `cd server && cargo test` を実行する。
2. 既存 route 応答の回帰、設定読込、起動メッセージ、失敗系が通ることを確認する。
3. 必要なら `cargo fmt --check` も実行し、分割後の構成が整形規約を満たすことを確認する。

## 7. 実施済み確認

1. `cd server && cargo test` を実行し、23 tests passed を確認した。
2. `PORT=8123 ./run.sh` で起動し、wrapper の案内と `tracing` startup log の両方を確認した。
3. `GET /`、`GET /image.bmp`、`GET /image.bin`、`GET /unknown` を実行し、順に `200 binary`、`200 bmp`、`200 binary`、`404 text/plain` を確認した。
4. `tracing` のアクセスログで `request_id`、`path`、`status`、`outcome` が 1 リクエスト 1 行で出ることを確認した。

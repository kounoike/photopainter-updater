# Quickstart: Ping 動作確認エンドポイント

## 前提

1. server が起動している。
2. `curl` などで HTTP request を送れる。

## 1. `/ping` を確認する

手順:

1. `GET /ping` を実行する

期待結果:

- `200 OK` が返る
- response body は空である
- 画像未配置でも成功する

## 2. `/hello` や画像 route と切り分ける

確認:

- `/ping` は server 到達性だけを確認する
- `/hello` は本文付きの疎通確認に使える
- `/image.bmp` `/image.bin` は画像処理確認に使う
- 未定義 path は引き続き `404` を返す

期待結果:

- server 起動問題と画像問題を切り分けやすい

## 3. 回帰確認

確認:

- `/hello` は引き続き成功する
- `/` `/image.bmp` `/image.bin` `/upload` の既存動作を維持する
- 未定義 path は `404` と既存本文を維持する

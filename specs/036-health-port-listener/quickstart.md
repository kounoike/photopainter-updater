# Quickstart: Health Port Listener

## 1. `PORT_HEALTH` 未指定

期待結果:

- `PORT` の main listener だけが起動する
- `/ping` は main listener で使える

## 2. `PORT_HEALTH` が `PORT` と異なる

期待結果:

- `PORT` で既存 route 群を返す
- `PORT_HEALTH` で `/ping` だけを返す
- health port の未定義 path は `404` になる

## 3. `PORT_HEALTH` が `PORT` と同じ

期待結果:

- 起動失敗しない
- 追加 listener を起動せず main listener 上の `/ping` を使う

## 4. 回帰確認

確認:

- `/hello`、`/`、`/image.bmp`、`/image.bin`、`/upload` を維持する
- `/ping` の response 契約は維持する

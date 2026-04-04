# Quickstart: Hello 動作確認エンドポイント

## 前提

1. リポジトリ root で作業する。
2. `.env.example` をもとに `.env` を用意する。
3. Docker Compose で `server` サービスを起動できる。

## 1. サーバを起動する

```bash
cd /workspaces/photopainter-updater
docker compose up -d server
docker compose logs --tail=200 server
```

期待結果:

- `server` サービスが起動する
- 既定では `http://127.0.0.1:8000` で待ち受ける

## 2. `/hello` で疎通確認する

```bash
curl -i http://127.0.0.1:${SERVER_EXPOSE_PORT:-8000}/hello
```

確認:

- 成功 status が返る
- body が `hello` である
- この確認は `image.png` の有無に依存しない
- access log では `path=/hello` と成功 outcome を確認できる

## 3. 画像未配置でも `/hello` が使えることを確認する

```bash
curl -i http://127.0.0.1:${SERVER_EXPOSE_PORT:-8000}/hello
```

確認:

- 画像取得 endpoint が失敗し得る状態でも `/hello` は成功する
- 疎通確認と画像処理確認を別々に切り分けられる

## 4. 既存 endpoint の回帰を確認する

```bash
curl -I http://127.0.0.1:${SERVER_EXPOSE_PORT:-8000}/image.bmp
curl -I http://127.0.0.1:${SERVER_EXPOSE_PORT:-8000}/image.bin
```

確認:

- `/hello` 追加後も既存 endpoint の確認手順を継続できる
- 既存 endpoint の成功・失敗は従来どおり画像状態に依存する
- `/upload` の利用方法や期待挙動は変更しない

## 5. 未定義 path の挙動を確認する

```bash
curl -i http://127.0.0.1:${SERVER_EXPOSE_PORT:-8000}/not-found
```

確認:

- 未定義 path は引き続き not found になる
- 本文は `route not found` のままである
- access log では `LogOutcome::NotFound` 相当の not found 記録を維持する
- `/hello` 追加が fallback 契約を壊していない

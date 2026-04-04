# HTTP BMP Server

このディレクトリには PhotoPainter 向けの Rust 製 HTTP サーバ実装があります。

## 起動方法

Docker Compose を使います。

```bash
cp .env.example .env
docker compose up -d server
docker compose logs --tail=200 server
```

既定では host 側 `127.0.0.1:8000` で待ち受けます。container 内の listen port は常に
`8000` 固定で、host 側公開ポートは `.env` の `SERVER_EXPOSE_PORT` で変更します。
配信元ディレクトリは `.env` の `SERVER_CONTENT_DIR` で変更できます。

## 設定項目

- `SERVER_EXPOSE_PORT`: host 側へ公開する HTTP ポート。既定値は `8000`
- `PORT_HEALTH`: health-only listener 用 port。未指定時は無効、`PORT` と同じ値なら main listener の `/ping` を使う
- `SERVER_CONTENT_DIR`: 入力画像 `image.png` を読む host 側ディレクトリ。既定値は `./server/contents`
- `IMAGE_PROFILE`: `baseline` / `no-sat-boost` / `color-priority` / `hue-guard` / `color-priority-hue-guard`
- `COMPARE_WITH_BASELINE`: `0/1` または `true/false`
- `COMPARE_PROFILE`: 比較相手の profile。指定時は `baseline` 以外とも比較できる
- `COMPARE_SPLIT`: `vertical` または `horizontal`
- `DITHER_USE_LAB`: `0/1` または `true/false`
- `DITHER_USE_ATKINSON`: `0/1` または `true/false`
- `DITHER_DIFFUSION_RATE`: 数値。実装では `0.0..=1.0` に正規化
- `DITHER_ZIGZAG`: `0/1` または `true/false`

## 画像更新 API

- Endpoint: `POST /upload`
- 認証: なし
- 受理形式:
  - raw body
  - `multipart/form-data`
- 受理する画像形式:
  - `PNG`
  - `JPG` / `JPEG`
  - `GIF`
  - `BMP`
  - `WebP`

保存時の挙動:

- 受理した画像は `image.png` へ正規化して保存する
- 解像度が `480x800` でない場合は、アスペクト比を維持した拡大縮小と中央クロップで `480x800` に合わせる
- 保存成功後は、既存の `GET /`、`GET /image.bmp`、`GET /image.bin` が新しい `image.png` を入力として返す

失敗時の status:

- `400 Bad Request`: 空 body、multipart 構造不正、multipart 内の画像不足
- `415 Unsupported Media Type`: 対応外形式、decode 不能な画像データ
- `500 Internal Server Error`: 保存失敗

raw body の例:

```bash
curl -i \
  -X POST \
  -H 'Content-Type: image/png' \
  --data-binary @./server/contents/image.png \
  http://127.0.0.1:${SERVER_EXPOSE_PORT:-8000}/upload
```

multipart の例:

```bash
curl -i \
  -X POST \
  -F 'file=@./server/contents/image.png' \
  http://127.0.0.1:${SERVER_EXPOSE_PORT:-8000}/upload
```

## 起動確認

```bash
curl -i http://127.0.0.1:${SERVER_EXPOSE_PORT:-8000}/ping
curl -i http://127.0.0.1:${SERVER_EXPOSE_PORT:-8000}/hello
curl -I http://127.0.0.1:${SERVER_EXPOSE_PORT:-8000}/
curl -I http://127.0.0.1:${SERVER_EXPOSE_PORT:-8000}/image.bmp
curl -I http://127.0.0.1:${SERVER_EXPOSE_PORT:-8000}/image.bin
```

- まず `GET /ping` が `200 OK` と空 body を返すことを確認する
- まず `GET /hello` が `200 OK` と本文 `hello` を返すことを確認する
- `/ping` は最小の到達性確認、`/hello` は本文付きの疎通確認として使い分けられる
- `PORT_HEALTH` を `PORT` と異なる値で指定した場合、その port では `/ping` だけを返す health-only listener が起動する
- `PORT_HEALTH` を未指定または `PORT` と同じ値にした場合、health check は main listener の `/ping` を使う
- `image.png` が未配置でも `/hello` は疎通確認に使える
- その後に `/image.bmp` や `/image.bin` を確認すると、server 疎通と画像処理の問題を切り分けやすい
- 未定義 path は引き続き `404` と本文 `route not found` を返し、access log でも `not-found` として記録される

この repository 作業環境では `docker` コマンドが使えない場合がある。その場合、Compose
起動確認は Docker 利用可能な実行環境で実施する。

## 責務分割後の変更対象

- `server/src/main.rs`: 最小の起動エントリ
- `server/src/app.rs`: `AppState` と起動配線、起動メッセージ
- `server/src/config.rs`: 環境変数読込、既定値、入力検証
- `server/src/routes.rs`: router、handler、HTTP レベルの回帰テスト
- `server/src/logging.rs`: `tracing` 初期化、アクセスログ形式
- `server/src/response.rs`: HTTP response helper
- `server/src/image_pipeline/`: 画像読込、upload 正規化と保存、ディザ、BMP/Binary 生成

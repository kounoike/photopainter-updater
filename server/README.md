# HTTP BMP Server

このディレクトリには PhotoPainter 向けの Rust 製 HTTP サーバ実装があります。

## 起動方法

`server/run.sh [CONTENT_DIR]` を使います。引数を省略すると `server/contents` を使います。

## 設定項目

- `PORT`: 待受ポート。既定値は `8000`
- `CONTENT_DIR`: 入力画像 `image.png` を読むディレクトリ。既定値は `server/contents`
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
  --data-binary @./contents/image.png \
  http://127.0.0.1:8000/upload
```

multipart の例:

```bash
curl -i \
  -X POST \
  -F 'file=@./contents/image.png' \
  http://127.0.0.1:8000/upload
```

## 比較実験の例

baseline をそのまま表示:

```bash
./run.sh
```

`color-priority` を baseline と左右比較:

```bash
IMAGE_PROFILE=color-priority \
COMPARE_WITH_BASELINE=1 \
COMPARE_SPLIT=vertical \
./run.sh
```

`hue-guard` と `color-priority` を上下比較:

```bash
IMAGE_PROFILE=hue-guard \
COMPARE_PROFILE=color-priority \
COMPARE_SPLIT=horizontal \
./run.sh
```


## 責務分割後の変更対象

- `server/src/main.rs`: 最小の起動エントリ
- `server/src/app.rs`: `AppState` と起動配線、起動メッセージ
- `server/src/config.rs`: 環境変数読込、既定値、入力検証
- `server/src/routes.rs`: router、handler、HTTP レベルの回帰テスト
- `server/src/logging.rs`: `tracing` 初期化、アクセスログ形式
- `server/src/response.rs`: HTTP response helper
- `server/src/image_pipeline/`: 画像読込、upload 正規化と保存、ディザ、BMP/Binary 生成

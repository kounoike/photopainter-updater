# HTTP BMP Server

このディレクトリには PhotoPainter 向けの Rust 製 HTTP サーバ実装があります。

## 起動方法

`server/run.sh [CONTENT_DIR]` を使います。引数を省略すると `server/contents` を使います。

## 設定項目

- `PORT`: 待受ポート。既定値は `8000`
- `CONTENT_DIR`: 入力画像 `image.png` を読むディレクトリ。既定値は `server/contents`
- `DITHER_USE_LAB`: `0/1` または `true/false`
- `DITHER_USE_ATKINSON`: `0/1` または `true/false`
- `DITHER_DIFFUSION_RATE`: 数値。実装では `0.0..=1.0` に正規化
- `DITHER_ZIGZAG`: `0/1` または `true/false`

## 責務分割後の変更対象

- `server/src/main.rs`: 最小の起動エントリ
- `server/src/app.rs`: `AppState` と起動配線、起動メッセージ
- `server/src/config.rs`: 環境変数読込、既定値、入力検証
- `server/src/routes.rs`: router、handler、HTTP レベルの回帰テスト
- `server/src/logging.rs`: `tracing` 初期化、アクセスログ形式
- `server/src/response.rs`: HTTP response helper
- `server/src/image_pipeline/`: 画像読込、ディザ、BMP/Binary 生成

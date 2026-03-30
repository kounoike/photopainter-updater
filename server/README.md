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
- `server/src/image_pipeline/`: 画像読込、ディザ、BMP/Binary 生成

# Contract: HTTP画像の描画経路

## Contract Goal

HTTP 取得画像を e-paper に反映する経路について、現行方式と採用可否判断を固定する。

## Current Render Path

1. `config.txt` から `image_url` を読む
2. HTTP 応答本文を `/sdcard/download.bmp` に保存する
3. 保存済み BMP のヘッダを検証する
4. `GUI_ReadBmp_RGB_6Color("/sdcard/download.bmp", 0, 0)` で描画バッファへ展開する
5. `epaper_port_display()` で表示する

## Accepted Contract for This Feature

- この feature では `direct_stream_render` は採用しない
- `sd_cached_render` を現行契約として維持する
- `/sdcard/download.bmp` は描画前提の一時キャッシュとして引き続き利用する

## Rejection Conditions for Direct Stream Render

- path ベース BMP デコード API を置き換える必要がある
- 既存 failure category を同等に維持できない
- 24-bit BMP のヘッダ検証、行パディング、bottom-up 変換を安全に再実装する必要がある
- 変更規模が今回の feature 価値に対して大きすぎる

## Documentation Contract

- 利用者向け文書は、更新時に `/sdcard/download.bmp` を一時的に利用する現行挙動を説明する
- 開発文書は、なぜ direct stream render を採用しなかったかを説明する

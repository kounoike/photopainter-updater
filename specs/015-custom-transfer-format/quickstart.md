# Quickstart: 独自画像転送形式追加

## 1. 既存 BMP 経路の維持確認

1. `server/run.sh` でサーバを起動する。
2. `GET /` と `GET /image.bmp` にアクセスする。
3. どちらも従来どおり BMP 応答であることを確認する。

## 2. `/image.bin` 応答確認

1. `GET /image.bin` にアクセスする。
2. BMP ではない binary 応答が返ることを確認する。
3. `Content-Type: application/vnd.photopainter-frame` と `Content-Length` が付くことを確認する。
4. 応答先頭に `PPBF` ヘッダがあり、幅 `800`、高さ `480`、payload 長 `192000` が入ることを確認する。

## 3. firmware の保存なし更新確認

1. firmware の `config.txt` で `image_url` を `.bin` で終わる URL に設定する。
2. 起動時更新または BOOT ボタン更新を実行する。
3. SD カード上の中間 BMP を作らずに表示更新が完了することを確認する。
4. 既存の `/sdcard/download.bmp` が新規作成も更新もされないことを確認する。

## 4. firmware の経路選択確認

1. firmware の `config.txt` で `image_url` を `.bin` 以外で終わる URL に設定する。
2. 更新を実行し、既存 BMP 経路が使われることを確認する。
3. firmware の `config.txt` で `image_url` を `.bin` で終わる URL に戻す。
4. 更新を実行し、独自形式経路が使われることを確認する。
5. serial log 上で `route=bmp` と `route=binary` が切り替わることを確認する。

## 5. 互換表示確認

1. 同じ入力画像で BMP 経路と独自形式経路をそれぞれ使って更新する。
2. 利用者視点で同等の最終表示結果になることを確認する。

## 6. 失敗系確認

1. 入力画像未配置、形式不整合、途中中断を個別に発生させる。
2. 独自形式経路が成功扱いにならず、通信、入力画像、形式不整合のいずれかへ切り分けられることを確認する。
3. `.bin` 経路では checksum mismatch や不正ヘッダ時に `kImageError`、通信失敗時に `kHttpError` が記録されることを確認する。

## 実施済み確認

- `cargo test` により `/`、`/image.bmp`、`/image.bin` の server 応答と binary contract 検証が通ることを確認済み
- `./scripts/build-merged-image.sh` により `esp32s3` 向け firmware build と merged image 生成が通ることを確認済み
- 実機で `App version: 2ce504e-dirty` を確認済み
- 実機で `image_url` を `.bin` にした更新により `route=binary`、`Displaying binary frame (192000 bytes)`、`Update finished successfully` を確認済み
- 実機で `image_url` を `.bmp` にした更新により `/sdcard/download.bmp` 保存後の `Displaying BMP frame` を確認済み

# Quickstart: 画像ディザリング回転配信

## 1. 変換済み BMP の取得確認

1. `server/contents/image.png` を配置する。
2. `server/run.sh` を起動する。
3. `GET /` にアクセスする。
4. `GET /image.bmp` にもアクセスする。
5. どちらも `200 OK` と `image/bmp` が返り、24bit BMP として取得できることを確認する。
6. `curl -sS http://127.0.0.1:8000/ -o /tmp/out-root.bmp` と `curl -sS http://127.0.0.1:8000/image.bmp -o /tmp/out-image.bmp` の出力が一致することを確認する。

## 2. 参照変換との比較確認

1. `server/testdata/image-dither-rotate/pre.png` と `server/testdata/image-dither-rotate/post.png` を使って彩度単体テストを実行する。
2. 代表座標 `(4,4)` `(12,4)` `(4,12)` `(20,12)` `(12,20)` `(4,28)` `(12,28)` `(20,28)` の RGB が各チャネル差 `±6` 以内で一致することを確認する。
3. 同じ fixture から生成した最終 BMP について、参照 palette 内の色だけが使われることを確認する。
4. 通し比較では、右 90 度回転後のサイズが一致し、palette 分布差が小さいことを確認する。

## 3. 入力画像差し替え確認

1. `server/contents/image.png` を配置してサーバを起動する。
2. `GET /` または `GET /image.bmp` で最初の変換結果を取得する。
3. サーバを止めずに `image.png` を別画像へ差し替える。
4. 再度アクセスし、差し替え後の変換結果が返ることを確認する。

## 4. 失敗応答確認

1. `image.png` を置かない状態でサーバを起動する。
2. `GET /` または `GET /image.bmp` にアクセスする。
3. `404 Not Found` と `text/plain; charset=utf-8` が返り、入力画像未配置と判別できる文言が含まれることを確認する。
4. 破損した `image.png` を置いた場合は `422 Unprocessable Entity` が返り、変換不能と判別できる文言が含まれることを確認する。

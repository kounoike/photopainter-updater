# Quickstart: POST画像保存

## 前提

1. リポジトリ root で作業する。
2. `server/` の依存解決が済んでいる。
3. 既存の起動方法どおり `server/run.sh` でサーバを起動できる。

## 1. サーバ起動

```bash
cd /workspaces/photopainter-updater/server
./run.sh
```

期待結果:

- サーバが `0.0.0.0:8000` で待ち受ける
- 既存の `GET /`、`GET /image.bmp`、`GET /image.bin` が利用可能

## 2. raw body で画像を更新する

```bash
curl -i \
  -X POST \
  -H 'Content-Type: image/jpeg' \
  --data-binary @./testdata/upload-sample.jpg \
  http://127.0.0.1:8000/upload
```

確認:

- `200 OK` が返る
- 成功文言が含まれる
- `server/contents/image.png` が更新される

## 3. multipart/form-data で画像を更新する

```bash
curl -i \
  -X POST \
  -F 'file=@./testdata/upload-sample.png' \
  http://127.0.0.1:8000/upload
```

確認:

- `200 OK` が返る
- `image.png` が更新される
- 直後の GET で更新後画像が反映される

## 4. 正規化結果を確認する

```bash
file ./contents/image.png
```

確認:

- 保存結果が PNG である
- 実装テストまたは補助コマンドで 480x800 になっていることを確認できる
- 画像が 480x800 以外でも、中央クロップ規則で正規化される

## 5. 既存 GET route への反映を確認する

```bash
curl -I http://127.0.0.1:8000/image.bmp
curl -I http://127.0.0.1:8000/image.bin
```

確認:

- 既存 route が成功する
- upload 後の現在画像を入力として変換結果を返す

## 6. 失敗系を確認する

```bash
printf 'not-an-image' | curl -i \
  -X POST \
  -H 'Content-Type: application/octet-stream' \
  --data-binary @- \
  http://127.0.0.1:8000/upload
```

確認:

- `400` または `415` の失敗応答が返る
- 既存の `image.png` が保持される
- アクセスログで `POST /upload` の失敗が判別できる

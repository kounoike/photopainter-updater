# Quickstart: BMP配信HTTPサーバ

## 1. 画像ありの確認

1. `server/contents/image.bmp` を配置する。
2. サーバを起動する。
3. `GET /` にアクセスする。
4. `GET /image.bmp` にもアクセスする。
5. 両方とも `200 OK` と `image/bmp` が返り、レスポンス内容が配置した `image.bmp` と一致することを確認する。

## 2. 画像未配置の確認

1. `server/contents/image.bmp` を置かない状態にする。
2. サーバを起動する。
3. `GET /` にアクセスする。
4. `GET /image.bmp` にもアクセスする。
5. どちらも成功レスポンスではなく、画像未配置と判別できる失敗応答が返ることを確認する。

## 3. 差し替え反映の確認

1. `server/contents/image.bmp` を配置してサーバを起動する。
2. `GET /` にアクセスして最初の画像を確認する。
3. `GET /image.bmp` にアクセスして同じ内容であることを確認する。
4. サーバを止めずに `server/contents/image.bmp` を別の BMP に差し替える。
5. 再度 `GET /` と `GET /image.bmp` にアクセスし、両方で差し替え後の内容が返ることを確認する。

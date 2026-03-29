# Data Model: BMP配信HTTPサーバ

## Entity: 配信画像

- Purpose: `GET /` で返す単一の BMP ファイルを表す。
- Fields:
  - `path`: `server/contents/image.bmp`
  - `exists`: ファイルが存在するか
  - `size_bytes`: ファイルサイズ
  - `content_type`: `image/bmp`
  - `last_modified`: 差し替え判定に使える更新時刻

## Entity: 画像取得応答

- Purpose: `GET /` または `GET /image.bmp` に対するレスポンス結果を表す。
- Fields:
  - `status`: 成功または失敗の HTTP ステータス
  - `content_type`: 成功時は `image/bmp`、失敗時は説明用の text
  - `body_source`: 成功時は配信画像、失敗時は未配置説明
  - `cache_behavior`: リクエストごとに最新ファイルを読む前提

## Relationships

- `画像取得応答` は `配信画像` の存在有無に依存する。
- `配信画像.exists = true` のとき、`画像取得応答` は成功応答になる。
- `配信画像.exists = false` のとき、`画像取得応答` は未配置を示す失敗応答になる。

## Validation Rules

- 成功応答は必ず `image/bmp` を返す。
- 失敗応答は画像未配置と判別できる内容を返す。
- サーバは `image.bmp` を固定パスから読み、差し替え後の次回アクセスに反映しなければならない。

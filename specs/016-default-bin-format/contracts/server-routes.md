# サーバー API コントラクト: 画像配信エンドポイント

**Branch**: `016-default-bin-format` | **Date**: 2026-03-30

## エンドポイント一覧（変更後）

### `GET /`

**説明:** デフォルト画像エンドポイント。変更後は `.bin` フォーマットを返す。

**レスポンス（成功時）:**
- Status: `200 OK`
- Content-Type: `application/vnd.photopainter-frame`
- Body: バイナリフレームデータ（`PPBF` マジック付き、20 バイトヘッダー + ペイロード）

**レスポンス（エラー時）:**
- Status: `404 Not Found` — 画像ファイルが存在しない場合
- Status: `500 Internal Server Error` — 変換処理エラー

**変更内容:** BMP → バイナリフレーム（`serve_binary_image` と同等）

---

### `GET /image.bin`

**説明:** バイナリフレームエンドポイント（変更なし）。`/` と同一の動作。

**レスポンス:** `GET /` と同一

---

### `GET /image.bmp`

**説明:** BMP フォールバックエンドポイント（変更なし）。

**レスポンス（成功時）:**
- Status: `200 OK`
- Content-Type: `image/bmp`
- Body: 24bit BMP 画像データ（800×480）

---

## 動作変更サマリー

| エンドポイント | 変更前           | 変更後            |
|--------------|------------------|------------------|
| `GET /`      | BMP (`image/bmp`) | バイナリフレーム (`application/vnd.photopainter-frame`) |
| `GET /image.bin` | バイナリフレーム | バイナリフレーム（変更なし）|
| `GET /image.bmp` | BMP             | BMP（変更なし）   |

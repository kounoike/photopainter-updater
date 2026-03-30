# クイックスタート: デフォルト画像フォーマットを .bin に変更

**Branch**: `016-default-bin-format` | **Date**: 2026-03-30

## 変更概要

本機能は 2 か所の最小変更で実現する:

1. **サーバー** (`server/src/main.rs:173`): `/` ルートのハンドラを差し替え
2. **ファームウェア** (`firmware/main/config.cc:35-106`): フォーマット判定ロジックを反転

---

## 変更 1: サーバー側ルート変更

**ファイル:** `server/src/main.rs`

```rust
// 変更前
.route("/", get(serve_image))

// 変更後
.route("/", get(serve_binary_image))
```

**確認方法:**
```bash
cd server
cargo run &
curl -I http://localhost:<port>/
# Content-Type: application/vnd.photopainter-frame が返ることを確認
```

---

## 変更 2: ファームウェア側フォーマット判定変更

**ファイル:** `firmware/main/config.cc`

```c
// 追加: BMP サフィックス判定関数
bool HasBmpSuffix(const char* value) {
    if (value == nullptr) {
        return false;
    }
    size_t length = strlen(value);
    return length >= 4 && strcmp(value + length - 4, ".bmp") == 0;
}

// 変更: デフォルトをバイナリに変更（.bmp の場合のみ BMP パス）
bool IsBinaryImageUrl(const char* image_url) {
    return !HasBmpSuffix(image_url);
}
```

**確認方法:**
- `image_url = "http://server/"` → `IsBinaryImageUrl` が `true` を返す
- `image_url = "http://server/image.bmp"` → `IsBinaryImageUrl` が `false` を返す
- `image_url = "http://server/image.bin"` → `IsBinaryImageUrl` が `true` を返す（後方互換）

---

## エンドツーエンド動作フロー（変更後）

```
ファームウェア起動
    ↓
config.txt から image_url 読み込み（例: http://server/）
    ↓
IsBinaryImageUrl("http://server/") → true（.bmp でない）
    ↓
DownloadBinaryFrameToDisplay() でバイナリストリーム
    ↓
PPBF ヘッダー検証 → ペイロード受信 → チェックサム検証
    ↓
ePaper ディスプレイ描画
```

```
サーバー GET /
    ↓
serve_binary_image() → ResponseFormat::Binary
    ↓
render_binary_frame_response() → encode_binary_frame()
    ↓
PPBF ヘッダー + ニブル形式ペイロード
```

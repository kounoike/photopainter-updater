# リサーチ: デフォルト画像フォーマットを .bin に変更

**Branch**: `016-default-bin-format` | **Date**: 2026-03-30

## 調査対象と結論

### 1. サーバー側ルーティング現状

**調査結果:**
- ファイル: `server/src/main.rs:173-175`
- 現在の構成:
  - `/` → `serve_image()` → `ResponseFormat::Bmp`（BMP返却）
  - `/image.bmp` → `serve_image()` → `ResponseFormat::Bmp`
  - `/image.bin` → `serve_binary_image()` → `ResponseFormat::Binary`

**決定事項:** `/` ルートを `serve_image` から `serve_binary_image` に切り替える。既存の `serve_binary_image` 実装をそのまま再利用できるため、新規コードは不要。

**代替案と却下理由:**
- 新たなハンドラを作成する案 → 既存 `serve_binary_image` で十分。不要な重複を避ける。
- `/image.bmp` に BMP を残す案 → 仕様通り。BMP フォールバック用途で有効。

---

### 2. バイナリフレームフォーマット仕様

**調査結果:**
- ファイル: `server/src/main.rs:346-361`
- フォーマット:
  - マジック: `PPBF`（4 bytes）
  - バージョン: `1`（1 byte）
  - フラグ: `0`（1 byte）
  - ヘッダー長: `20`（2 bytes, little-endian）
  - 幅: `800`（2 bytes, little-endian）
  - 高さ: `480`（2 bytes, little-endian）
  - ペイロード長: 4 bytes
  - チェックサム: 4 bytes（wrapping add）
  - ペイロード: ニブル形式 4bit/ピクセル

**決定事項:** フォーマット変更なし。既存 `.bin` フォーマットをそのまま使用。

---

### 3. ファームウェア側フォーマット判定現状

**調査結果:**
- ファイル: `firmware/main/config.cc:35-41, 104-106`
- 現在のロジック: URL が `.bin` で終わる場合にバイナリパスを選択
  ```c
  bool HasBinSuffix(const char* value) { ... strcmp(value + length - 4, ".bin") == 0 }
  bool IsBinaryImageUrl(const char* image_url) { return HasBinSuffix(image_url); }
  ```
- 呼び出し元: `firmware/main/update_job.cc:262`

**決定事項:** ロジックを反転し、URL が `.bmp` で終わる場合のみ BMP パスを選択（デフォルトをバイナリに変更）。`HasBmpSuffix` を追加し、`IsBinaryImageUrl` を `!HasBmpSuffix(image_url)` に変更する。

**代替案と却下理由:**
- SD カード上の `.bmp` ファイル存在確認案 → ファームウェアはダウンロード前にファイルが存在しないため不適切。URL ベースの判定が適切。
- `image_url` デフォルト値を `.bin` に変更する案 → 設定ファイルとの互換性リスクあり。URL 判定ロジックの変更で十分。

---

### 4. 変更範囲の確認

**影響ファイル（最小構成）:**
1. `server/src/main.rs:173` — ルート変更（1 行）
2. `firmware/main/config.cc:35-41, 104-106` — 判定ロジック変更（数行）

**影響なし:**
- `/image.bin` ルート → 変更不要（Forbidden Scope）
- `/image.bmp` ルート → BMP フォールバック用途で維持
- バイナリフォーマット仕様 → 変更なし
- 認証・その他ルート → 変更なし

## NEEDS CLARIFICATION の解消

調査の結果、未解決項目はなし。すべての変更は既存コードの最小修正で実現可能。

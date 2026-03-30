# データモデル: デフォルト画像フォーマットを .bin に変更

**Branch**: `016-default-bin-format` | **Date**: 2026-03-30

## 概要

本機能は既存データ構造の変更を伴わない。変更はルーティングロジックおよびフォーマット判定ロジックのみ。

---

## サーバー側: ルートマッピング（変更後）

| ルート       | ハンドラ             | フォーマット              | 変更 |
|-------------|---------------------|--------------------------|------|
| `/`         | `serve_binary_image` | `ResponseFormat::Binary` | ✅ 変更 |
| `/image.bmp` | `serve_image`       | `ResponseFormat::Bmp`    | 変更なし |
| `/image.bin` | `serve_binary_image` | `ResponseFormat::Binary` | 変更なし |

**変更前:**
```
/ → serve_image (BMP)
```

**変更後:**
```
/ → serve_binary_image (Binary)
```

---

## ファームウェア側: フォーマット判定ロジック（変更後）

### `IsBinaryImageUrl` の判定フロー

```
image_url
    ↓
HasBmpSuffix(image_url) ?
    ├── YES (.bmp で終わる) → BMP パス（RenderBmpFromSdCard）
    └── NO  (それ以外)      → バイナリパス（DownloadBinaryFrameToDisplay）← デフォルト
```

**変更前:**
```
HasBinSuffix → .bin で終わる場合のみバイナリ
それ以外は BMP（デフォルト BMP）
```

**変更後:**
```
HasBmpSuffix → .bmp で終わる場合のみ BMP
それ以外はバイナリ（デフォルト Binary）
```

### 状態遷移: image_url の値とパス選択

| image_url の例                        | `HasBmpSuffix` | `IsBinaryImageUrl` | 選択パス |
|--------------------------------------|----------------|---------------------|---------|
| `http://server/image.bin`            | false          | true                | バイナリ |
| `http://server/`                     | false          | true                | バイナリ |
| `http://server/image.bmp`            | true           | false               | BMP     |
| `http://server/image`（拡張子なし）   | false          | true                | バイナリ |

---

## バイナリフレームフォーマット（変更なし）

既存フォーマットを変更しない。参照のみ。

```
Offset  Size  内容
0       4     マジック: "PPBF"
4       1     バージョン: 1
5       1     フラグ: 0
6       2     ヘッダー長: 20 (little-endian)
8       2     幅: 800 (little-endian)
10      2     高さ: 480 (little-endian)
12      4     ペイロード長 (little-endian)
16      4     チェックサム (little-endian, wrapping add)
20      N     ペイロード (ニブル形式, 4bit/ピクセル)
```

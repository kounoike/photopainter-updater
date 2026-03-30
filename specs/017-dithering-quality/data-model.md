# データモデル: ディザリングアルゴリズムの改善

**Branch**: `017-dithering-quality` | **Date**: 2026-03-30

## 概要

本機能はデータ構造の追加を伴わない。変更はサーバーの画像処理ロジックのみ。

---

## アルゴリズム設定（実行時パラメータ）

サーバー起動時に環境変数から読み込む 2 つのフラグ。

| 環境変数 | 型 | デフォルト | 意味 |
|---------|-----|-----------|------|
| `DITHER_USE_LAB` | bool（"1" で有効） | false | A案: 色距離計算を CIE Lab 空間で行う |
| `DITHER_USE_ATKINSON` | bool（"1" で有効） | false | B案: Atkinson アルゴリズムを使用 |

### 有効な組み合わせ

| DITHER_USE_LAB | DITHER_USE_ATKINSON | モード |
|----------------|---------------------|--------|
| false | false | デフォルト（Floyd-Steinberg + RGB距離） |
| false | true | B案（Atkinson + RGB距離） |
| true | false | A案（Floyd-Steinberg + Lab距離） |
| true | true | A+B（Atkinson + Lab距離） |

---

## 処理フロー（変更後）

```
PNG 画像読み込み
    ↓
彩度ブースト（変更なし）
    ↓
ディザリング処理（変更対象）
    ├── 色距離計算: RGB または CIE Lab（DITHER_USE_LAB）
    └── 誤差拡散: Floyd-Steinberg または Atkinson（DITHER_USE_ATKINSON）
    ↓
回転（変更なし）
    ↓
バイナリフレームエンコード（変更なし）
```

---

## 変更対象関数（server/src/main.rs）

| 関数 | 変更内容 |
|------|---------|
| `apply_reference_dither` | フラグに応じて誤差拡散係数を切り替え |
| `squared_distance` / `nearest_palette_color` | フラグに応じて Lab 距離または RGB 距離を使用 |
| `main` または初期化処理 | 環境変数読み込みを追加 |

新規追加関数（評価期間中のみ）:
- `rgb_to_lab([u8; 3]) -> [f32; 3]` — sRGB → CIE Lab 変換
- `lab_distance([f32; 3], [u8; 3]) -> f32` — Lab 色空間での色距離計算

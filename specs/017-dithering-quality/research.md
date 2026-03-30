# リサーチ: ディザリングアルゴリズムの改善

**Branch**: `017-dithering-quality` | **Date**: 2026-03-30

## 1. Atkinson ディザリング係数

**決定事項:** Atkinson アルゴリズムを採用。

誤差拡散パターン（現在ピクセルを `*` として）:

```
. * 1 1
1 1 1 .
. 1 . .
```

各係数は **1/8**。合計 6/8（75%）のみ拡散し、残り 25% は意図的に捨てる。

| 隣接ピクセル | 係数 |
|-------------|------|
| (x+1, y)    | 1/8  |
| (x+2, y)    | 1/8  |
| (x-1, y+1)  | 1/8  |
| (x,   y+1)  | 1/8  |
| (x+1, y+1)  | 1/8  |
| (x,   y+2)  | 1/8  |

**Floyd-Steinberg との比較:**

| 特性 | Floyd-Steinberg | Atkinson |
|------|----------------|----------|
| 誤差保存率 | 100% | 75% |
| 明るい領域 | 階調を忠実に再現 | 白に寄る |
| 暗い領域 | 階調を忠実に再現 | 黒に寄る |
| 粒状感 | 規則的なパターンが出やすい | すっきりした印象 |

ePaper の 6 色パレットでは中間色が少ないため、Atkinson の「エッジ寄り」特性は粒状感軽減に有効と判断。

**代替案と却下理由:**
- Sierra / Stucki → 係数が複雑な割に ePaper 向けの優位性が不明確。まず Atkinson を評価する。

---

## 2. CIE Lab 色空間変換

**決定事項:** 外部クレート不使用、純 Rust でインライン実装する。

現在の依存は `axum`, `image`, `tokio` のみ。`palette` クレートは高機能だが本機能に必要なのは sRGB→Lab の距離計算のみであり、追加依存は不要。

**変換式 (sRGB → CIE Lab, D65 光源):**

```
Step 1: sRGB → 線形 RGB（ガンマ補正除去）
  c_linear = c/255
  if c_linear <= 0.04045: c_linear / 12.92
  else: ((c_linear + 0.055) / 1.055)^2.4

Step 2: 線形 RGB → XYZ（D65 変換行列）
  X = 0.4124564*R + 0.3575761*G + 0.1804375*B
  Y = 0.2126729*R + 0.7151522*G + 0.0721750*B
  Z = 0.0193339*R + 0.1191920*G + 0.9503041*B

Step 3: XYZ → Lab（D65 白色点: Xn=0.95047, Yn=1.0, Zn=1.08883）
  f(t) = t^(1/3)        if t > 0.008856
       = 7.787*t + 16/116  otherwise
  L = 116*f(Y/Yn) - 16
  a = 500*(f(X/Xn) - f(Y/Yn))
  b = 200*(f(Y/Yn) - f(Z/Zn))
```

**色距離計算:**
現行の RGB ユークリッド距離 `ΔE = √(ΔR²+ΔG²+ΔB²)` を Lab 空間の `ΔE = √(ΔL²+Δa²+Δb²)`（CIE76）に変更する。

**代替案と却下理由:**
- `palette` クレート → 機能過多、依存追加のコストに対して得られる価値が小さい。
- CIE94 / CIEDE2000 → より正確だが計算が複雑。CIE76 で ePaper の 6 色パレット向けには十分。

---

## 3. アルゴリズム切り替え機構

**決定事項:** 環境変数 2 つで独立制御する。

```
DITHER_USE_LAB=1      # A案: Lab 色空間を使用（未設定時は RGB）
DITHER_USE_ATKINSON=1 # B案: Atkinson アルゴリズムを使用（未設定時は Floyd-Steinberg）
```

4 パターンの起動例:
```bash
# デフォルト（Floyd-Steinberg + RGB）
cargo run --release

# B案のみ（Atkinson + RGB）
DITHER_USE_ATKINSON=1 cargo run --release

# A案のみ（Floyd-Steinberg + Lab）
DITHER_USE_LAB=1 cargo run --release

# A+B（Atkinson + Lab）
DITHER_USE_LAB=1 DITHER_USE_ATKINSON=1 cargo run --release
```

`run.sh` 経由でも `DITHER_USE_LAB=1 server/run.sh` で渡せる。

**代替案と却下理由:**
- コマンドライン引数 → `run.sh` の引数設計の変更が必要でコストが高い。
- 単一の `DITHER_ALGO=floyd-rgb|floyd-lab|atkinson-rgb|atkinson-lab` → 組み合わせが増えると列挙が煩雑。2 フラグの方が直感的。
- フィーチャーフラグ（`--features`）→ 再コンパイルが必要で評価サイクルが遅くなる。

---

## 4. クリーンアップ戦略

**決定事項:** 評価完了後、切り替え分岐と不採用アルゴリズムのコードをまとめて削除し、環境変数読み込みも除去する。

- 採用アルゴリズムのみを残す
- `apply_reference_dither` と `squared_distance` / `nearest_palette_color` の関数名・シグネチャはそのまま維持してよい（内部実装のみ変更）
- テストコードも採用アルゴリズムに合わせて更新する

## NEEDS CLARIFICATION の解消

なし。全項目を調査・決定済み。

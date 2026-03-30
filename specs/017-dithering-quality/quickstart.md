# クイックスタート: ディザリングアルゴリズムの評価手順

**Branch**: `017-dithering-quality` | **Date**: 2026-03-30

## 評価手順

### ステップ 1: B案のみ（Atkinson + RGB）

```bash
cd server
DITHER_USE_ATKINSON=1 cargo run --release
# または
DITHER_USE_ATKINSON=1 ./run.sh
```

実機で画像を表示して現行と比較する。

---

### ステップ 2: A案のみ（Floyd-Steinberg + Lab）

```bash
DITHER_USE_LAB=1 cargo run --release
```

実機で画像を表示して現行・B案と比較する。

---

### ステップ 3: A+B組み合わせ（Atkinson + Lab）

```bash
DITHER_USE_LAB=1 DITHER_USE_ATKINSON=1 cargo run --release
```

実機で画像を表示して全パターンと比較する。

---

### ステップ 4: デフォルト（現行確認用）

```bash
cargo run --release
# 環境変数なし = Floyd-Steinberg + RGB（変更前と同じ）
```

---

## クリーンアップ（採用決定後）

採用アルゴリズムを決定したら `/speckit.implement` を再実行してクリーンアップを行う。

クリーンアップ後の確認:
```bash
# 環境変数なしで起動し、採用アルゴリズムが動作することを確認
cargo run --release
```

コードに `DITHER_USE_LAB` / `DITHER_USE_ATKINSON` の参照が残っていないことを確認:
```bash
grep -r "DITHER_USE" server/src/
# 出力なしであればOK
```

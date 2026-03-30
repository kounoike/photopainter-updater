# 実装計画: ディザリングアルゴリズムの改善

**Branch**: `017-dithering-quality` | **Date**: 2026-03-30 | **Spec**: [spec.md](spec.md)
**Input**: `/specs/017-dithering-quality/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

ePaper ディスプレイへの画像表示における粒状感を改善するため、サーバー側のディザリング処理を拡張する。A案（CIE Lab 色空間での色距離計算）と B案（Atkinson 誤差拡散アルゴリズム）を環境変数で独立に切り替えられる形で実装し、B案のみ → A案のみ → A+B の順で実機評価する。評価完了後に採用アルゴリズムのみ残してクリーンアップする。外部クレートは追加しない。

## Technical Context

**Language/Version**: Rust（サーバーのみ）
**Primary Dependencies**: 既存の `image` クレートのみ（新規依存なし）
**Storage**: N/A（画像変換はオンザフライ）
**Testing**: `cargo test`（既存テストの維持）、実機目視確認
**Target Platform**: Linux サーバー
**Project Type**: サーバー単体変更
**Performance Goals**: 処理時間の大幅増加なし（Lab 変換コストは許容範囲）
**Constraints**: 外部クレート追加なし、ファームウェア変更なし、HTTP インターフェース変更なし
**Scale/Scope**: シングルデバイス・家庭内 LAN 運用

## Constitution Check

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を守っている
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし（外部クレート不追加）、複雑化は正当化されている

**Phase 1 design 後再確認:**
- [x] 切り替え機構は環境変数のみ（最小構成）
- [x] クリーンアップで評価用コードを完全除去することが計画されている
- [x] HTTP インターフェース・バイナリフォーマット・ファームウェアへの影響なし

## Project Structure

### Documentation (this feature)

```text
specs/017-dithering-quality/
├── plan.md              # このファイル
├── research.md          # Phase 0 成果物 ✅
├── data-model.md        # Phase 1 成果物 ✅
├── quickstart.md        # Phase 1 成果物 ✅
├── contracts/
│   └── server-config.md # Phase 1 成果物 ✅
└── tasks.md             # Phase 2 成果物 (/speckit.tasks)
```

### Source Code（変更対象）

```text
server/
└── src/
    └── main.rs    # ディザリング処理・色距離計算・環境変数読み込み
```

**Structure Decision**: 変更は `server/src/main.rs` 1 ファイルのみ。新規ファイル不要。

## 実装詳細

### A案: CIE Lab 色距離計算

`nearest_palette_color` の距離計算を差し替える。`DITHER_USE_LAB=1` のとき `rgb_to_lab` で変換後に Lab 空間でユークリッド距離を計算する。

**追加関数:**
- `rgb_to_lab(pixel: [u8; 3]) -> [f32; 3]` — sRGB → CIE Lab（D65、CIE76）
- `lab_squared_distance(pixel: [f32; 3], candidate: [u8; 3]) -> f32`

### B案: Atkinson 誤差拡散

`apply_reference_dither` の `diffuse_error` 呼び出し部分を差し替える。`DITHER_USE_ATKINSON=1` のとき 6 隣接ピクセルに各 1/8 を拡散する。

**Atkinson 拡散先:**

| オフセット | 係数 |
|-----------|------|
| (+1, 0)   | 1/8  |
| (+2, 0)   | 1/8  |
| (-1, +1)  | 1/8  |
| (0,  +1)  | 1/8  |
| (+1, +1)  | 1/8  |
| (0,  +2)  | 1/8  |

### 環境変数読み込み

`main()` または `AppState` 初期化時に読み込み、`AppState` に `use_lab: bool` と `use_atkinson: bool` を追加して各処理に渡す。

### クリーンアップ

評価完了後:
1. 不採用アルゴリズムのコードブロックを削除
2. `use_lab` / `use_atkinson` フラグと環境変数読み込みを削除
3. 採用アルゴリズムをハードコードされた唯一の実装として整理

## 検証方針

| 対象 | 検証方法 | 期待結果 |
|------|---------|---------|
| B案: cargo test | `cargo test` | 既存テスト全通過、ディザリング出力がパレット色のみ |
| B案: 実機 | `DITHER_USE_ATKINSON=1` で起動、ePaper 表示 | 粒状感の変化を目視確認 |
| A案: cargo test | `cargo test` | 既存テスト全通過 |
| A案: 実機 | `DITHER_USE_LAB=1` で起動、ePaper 表示 | 色選択の自然さを目視確認 |
| A+B: 実機 | 両フラグ有効で起動、ePaper 表示 | 組み合わせ効果を目視確認 |
| クリーンアップ後 | `cargo test` + 実機 | テスト通過・表示品質維持 |

## Complexity Tracking

*憲章チェック違反なし。評価用フラグは一時的な複雑化であり、クリーンアップで除去されることを計画に明記している。*

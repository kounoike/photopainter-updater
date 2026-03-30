# 実装計画: ディザリング向け画像改善アイデア整理

**Branch**: `019-dither-image-ideas` | **Date**: 2026-03-30 | **Spec**: [spec.md](spec.md)  
**Input**: `/specs/019-dither-image-ideas/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

ePaper の表示特性、既存パレット、既存ディザリング前提を踏まえ、サーバー側の画像加工パイプラインに比較実験しやすい改善プロファイルを追加する。改善対象は前処理とディザリング処理の両方を含むが、HTTP エンドポイント、転送フォーマット、ファームウェアは変更しない。固定画像セットを基準にしつつ必要時のみ追加画像で補足し、実機 ePaper 表示を主判定として比較結果を記録できるようにする。

## Technical Context

<!--
  ACTION REQUIRED:
  この節は実装前提を具体値で置き換える。
  不明点は推測せず NEEDS CLARIFICATION または TODO: を使う。
-->

**Language/Version**: Rust stable（edition 2024）、Bash（既存起動補助）  
**Primary Dependencies**: 既存 `axum`, `envconfig`, `image`, `tokio`, `tracing`  
**Storage**: ローカルファイル（`server/contents/`、`server/testdata/`、`specs/019-dither-image-ideas/` の実験記録）  
**Testing**: `cargo test`、固定画像セットによる手動比較、実機 ePaper 表示確認  
**Target Platform**: Linux 上のローカル HTTP サーバー、LAN 内の既存 ePaper 端末  
**Project Type**: Rust サーバー + ドキュメント主導の比較実験ワークフロー  
**Performance Goals**: 既存の画像更新フローを維持し、比較用プロファイル追加後もローカル更新操作で待ち時間が実用範囲に収まること  
**Constraints**: LAN 内完結、既存 HTTP ルート不変、既存転送フォーマット不変、ファームウェア不変、実機主判定  
**Scale/Scope**: 単一サーバーから単一または少数の ePaper 端末へ同一画像を配信する評価用途

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されている

**Phase 1 design 後再確認:**
- [x] 比較実験は既存サーバー設定拡張のみで表現し、外部サービスや追加配信経路を導入しない
- [x] ファームウェア、HTTP エンドポイント、転送フォーマットは変更対象から除外されている
- [x] 固定画像セット、実験プロファイル、比較記録により各 story の検証経路を定義している

## Project Structure

### Documentation (this feature)

```text
specs/019-dither-image-ideas/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── experiment-config.md
└── tasks.md
```

### Source Code (repository root)
<!--
  ACTION REQUIRED:
  実際の構成に置き換える。未使用の選択肢は削除し、実パスのみ残す。
-->

```text
server/
├── run.sh
├── README.md
├── src/
│   ├── app.rs
│   ├── config.rs
│   ├── routes.rs
│   └── image_pipeline/
│       ├── mod.rs
│       ├── dither.rs
│       ├── bmp.rs
│       ├── binary.rs
│       └── load.rs
└── testdata/
    └── image-dither-rotate/

firmware/
└── main/
    ├── update_job.cc
    └── display_update.cc
```

**Structure Decision**: 実装変更は `server/` 配下に限定する。比較実験の設定追加は `server/src/config.rs` と `server/src/image_pipeline/` に寄せ、既存配信経路は `routes.rs` と `app.rs` の現行配線を維持する。比較に使う固定画像セットは既存 `server/testdata/` を拡張し、成果物としての比較観点と手順は `specs/019-dither-image-ideas/` に保持する。

## 設計方針

### 1. 改善候補の表現

改善候補は「個別フラグの寄せ集め」ではなく、前処理とディザリング設定を束ねた named profile として扱う。これにより、実験実行、結果記録、比較表の対応関係を崩さずに管理できる。

### 2. 比較実験の再現性

固定画像セットを比較の基準にし、必要な場合のみ追加画像を補足として使う。固定セットは少なくとも無彩色階調、低彩度写真寄り、高彩度領域、輪郭重視画像を含め、比較結果は profile ごとに同じ順序で確認する。

### 3. 既存インターフェース維持

比較対象の切り替えは既存の環境変数読込と `run.sh` 運用を拡張して表現する。HTTP エンドポイント、レスポンス形式、firmware 側取得方法は変更しない。

### 4. 実験結果の扱い

結果は実機 ePaper 表示を主判定として記録し、PC 上の BMP/Binary 出力やテスト結果は補助情報として使う。比較結果は profile、画像セット、観点、判断、次アクションの形で残す。

## 実装対象メモ

- `server/src/config.rs`: 改善 profile と実験用入力セット指定の設定追加
- `server/src/image_pipeline/mod.rs`: パイプライン段階の整理と profile 適用入口
- `server/src/image_pipeline/dither.rs`: 前処理とディザリング改善候補の適用点
- `server/src/app.rs`: 起動時の実験モード表示整理
- `server/README.md`: 実験モードの起動方法追記
- `server/testdata/`: 固定画像セットの拡充
- `specs/019-dither-image-ideas/`: 比較観点、実施手順、記録様式の文書化

## Complexity Tracking

憲章チェック違反なし。named profile と固定画像セットは実験再現性のための最小限の構造化であり、外部サービスや新規配信経路を導入する案より単純である。

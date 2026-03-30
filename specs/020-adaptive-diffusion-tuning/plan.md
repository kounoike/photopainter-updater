# 実装計画: 写真調ディザリングの追加改善

**Branch**: `020-adaptive-diffusion-tuning` | **Date**: 2026-03-30 | **Spec**: [spec.md](spec.md)  
**Input**: `/specs/020-adaptive-diffusion-tuning/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

`019` で暫定上位候補になった `color-priority + DITHER_DIFFUSION_RATE=0.8` を比較基準として維持しつつ、写真調画像で残った 3 つの弱点である「青空など青系の広い面の保持」「明るい低彩度面の濁り抑制」「肌の中間調保持」を追加改善候補として検討した。検討の結果、試作アルゴリズムは runtime には残さず、`server/` 側の現行実装は据え置いたうえで、比較用 fixture、評価手順、再現用アルゴリズム文書を残す。HTTP エンドポイント、転送フォーマット、firmware は変更しない。

## Technical Context

**Language/Version**: Rust stable（edition 2024）、Bash（既存起動補助）  
**Primary Dependencies**: 既存 `axum`, `envconfig`, `image`, `tokio`, `tracing`  
**Storage**: ローカルファイル（`server/contents/`、`server/testdata/`、`specs/020-adaptive-diffusion-tuning/`）  
**Testing**: `cargo test`、fixture ベースの画像パイプライン検証、手動比較、必要時の実機 ePaper 確認  
**Target Platform**: Linux 上のローカル HTTP サーバー、LAN 内の既存 ePaper 端末  
**Project Type**: Rust サーバー + ドキュメント主導の比較実験ワークフロー  
**Performance Goals**: 既存の画像更新フローを維持し、追加改善 profile を有効にしてもローカル変換待ち時間が体感で悪化しないこと  
**Constraints**: LAN 内完結、既存 HTTP ルート不変、既存転送フォーマット不変、`firmware/` 不変、`xiaozhi-esp32/` 不変、実機主判定を維持  
**Scale/Scope**: 単一サーバーから単一または少数の ePaper 端末へ同一画像を配信する評価用途

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されている

**Phase 1 design 後再確認:**
- [x] 追加改善は既存の `server/` パイプライン拡張で完結し、新規外部依存や外部サービスを導入しない
- [x] HTTP エンドポイント、転送フォーマット、firmware、`xiaozhi-esp32/` は変更対象から除外されている
- [x] fixture、比較結果、ローカル検証手順により各 story の検証経路を定義している

## Project Structure

### Documentation (this feature)

```text
specs/020-adaptive-diffusion-tuning/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── adaptive-profile-config.md
└── tasks.md
```

### Source Code (repository root)

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
    ├── dither-result-check/
    └── image-dither-rotate/

specs/
├── 019-dither-image-ideas/
└── 020-adaptive-diffusion-tuning/
```

**Structure Decision**: 実装変更は `server/` 配下に限定する。設定追加は `server/src/config.rs`、改善ロジックは `server/src/image_pipeline/dither.rs`、比較出力の配線は `server/src/image_pipeline/mod.rs` と `server/src/app.rs` に寄せる。評価入力画像は `server/testdata/dither-result-check/` に集約し、比較手順と判断基準は `specs/020-adaptive-diffusion-tuning/` に保持する。

## 設計方針

### 1. 追加改善は再現用アルゴリズムとして残す

`019` で導入した profile ベース運用は維持するが、今回の追加改善は常設 profile としては採用しない。代わりに、再試行可能なアルゴリズム要約を文書へ残し、`color-priority + DITHER_DIFFUSION_RATE=0.8` を基準に差分を切り分けた記録だけを保持する。

### 2. 写真調の弱点に対して局所制御を入れる

固定の `DITHER_DIFFUSION_RATE` だけでは、青空のような広い青面、明るい低彩度面、肌の中間調に対して同じ効き方になりすぎる。今回の追加改善は、画素ごとの色特性に応じて候補色選択と誤差拡散の強さを調整する方向で設計する。

### 3. 既存外部インターフェースは維持する

比較対象の切り替えは既存の `IMAGE_PROFILE` と `DITHER_*` を拡張して表現し、HTTP エンドポイント、レスポンス形式、firmware 側の取得方法は変更しない。

### 4. 評価画像と記録を強化する

`image7` で不足していた青保持評価を補うため、青系の広い面を含む代表画像を追加し、写真調 2 系統以上で比較結果を残せるようにする。比較結果は profile、画像、観察点、判定、次アクションの形で文書化する。

### 5. ローカル回帰を先に固める

実機主判定は維持するが、追加改善の基礎検証はローカルで再現できるようにする。fixture ベースのテストでは、青寄り、高明度低彩度、肌寄りの色域で既存 profile と新 profile の差分を定量的に確認する。

## 実装対象メモ

- `server/src/config.rs`: 現行 profile を維持する
- `server/src/image_pipeline/dither.rs`: 追加実装は残さず、現行ロジックを維持する
- `server/README.md`: 現行 profile の説明を維持する
- `server/testdata/dither-result-check/`: 青保持を評価できる代表画像の整備
- `specs/020-adaptive-diffusion-tuning/`: 判断基準、比較結果、再現用アルゴリズム、手順を文書化

## Complexity Tracking

憲章チェック違反なし。profile ベースの拡張と fixture 追加は `019` の実験基盤を再利用する最小変更であり、新規配信経路や自動評価基盤を追加する案より単純である。

## 現時点の判断

- runtime に残す具体化対象はなし
- 比較の結果、現行 `color-priority + DITHER_DIFFUSION_RATE=0.8` を置き換える根拠は得られなかった
- 次に進める場合は、新たに判明した cyan/teal 系の服色の崩れを問題設定へ加えて再設計する

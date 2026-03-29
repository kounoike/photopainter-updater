# 実装計画: Rust HTTPスタック再評価

**Branch**: `009-rust-http-stack` | **Date**: 2026-03-29 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/009-rust-http-stack/spec.md)  
**Input**: `/specs/009-rust-http-stack/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

008 では Rust 採用を前提に `axum` を暫定第一候補として整理したが、Rust 実装着手前に Rust 内の HTTP framework 候補比較を明示的にやり直す。比較対象は `axum`、`actix-web`、`warp` とし、`ref/convert.py` が示す画像前処理要件、デバイスからの telemetry POST、ローカル運用、保守性、依存の重さ、配布容易性、開発体験を同一軸で評価する。今回の計画では、現時点の最終候補を `axum`、第一対抗候補を `actix-web`、参考候補を `warp` として設計成果物に固定する。

## Technical Context

**Language/Version**: Rust stable 系候補の比較調査  
**Primary Dependencies**: `axum`、`actix-web`、`warp`、Tokio stack、`ref/convert.py` を参照した画像前処理要件  
**Storage**: N/A（文書調査のみ）  
**Testing**: 文書レビュー、比較表確認、後続 feature での参照可能性確認  
**Target Platform**: devcontainer 上のローカル開発環境、LAN 内 companion HTTP サーバ想定  
**Project Type**: Rust 内 HTTP framework selection record for future local server implementation  
**Performance Goals**: 画像前処理と telemetry API を同一サーバ責務として扱っても、後続 feature で HTTP framework の再比較を不要にすること  
**Constraints**: 今回は実装しない、Rust 候補比較に限定する、ローカル優先、`ref/convert.py` が示す回転・スケーリング・ディザリング・6 色インデックス化前提を維持する  
**Scale/Scope**: 単一家庭内または少数端末向けの LAN 内サーバ想定。将来責務は画像変換と telemetry 収集を含む

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は比較理由として明示されている

## Project Structure

### Documentation (this feature)

```text
specs/009-rust-http-stack/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── rust-http-selection-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
ref/
└── convert.py

server/
├── contents/
└── run.sh

specs/
├── 008-http-server-stack/
└── 009-rust-http-stack/
```

**Structure Decision**: 今回は Rust 実装コードを追加せず、`specs/009-rust-http-stack/` に Rust 内候補比較の結果を閉じて記録する。`ref/convert.py` は画像前処理要件の参照根拠として扱い、後続の Rust サーバ実装 feature はこの比較結果を前提に framework を固定する。

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

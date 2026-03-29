# 実装計画: HTTP アクセスログ追加

**Branch**: `014-access-logs` | **Date**: 2026-03-29 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/014-access-logs/spec.md)  
**Input**: `/specs/014-access-logs/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

既存の Rust HTTP サーバへアクセスログを追加し、`/`、`/image.bmp`、失敗系、存在しない path へのアクセスを 1 リクエスト 1 行で追えるようにする。ログは既存の標準出力導線に載せ、少なくとも時刻、アクセス元、対象 path、応答ステータスを含める。既存の BMP 配信内容や route 契約は変更せず、運用時の切り分けしやすさだけを高める。

## Technical Context

**Language/Version**: Rust stable  
**Primary Dependencies**: `axum`、Tokio、Rust 標準ライブラリの時刻/ソケット/標準出力  
**Storage**: N/A（永続保存なし、標準出力ログのみ）  
**Testing**: Rust 自動テスト、HTTP ハンドラ単体テスト、手動でのログ確認  
**Target Platform**: devcontainer またはローカル開発環境、LAN 内のローカル HTTP サーバ  
**Project Type**: ローカル常駐 HTTP サーバの運用性改善  
**Performance Goals**: 各リクエストに対して 1 行のログを出し、既存レスポンス契約を維持すること  
**Constraints**: 既存 route `/` と `/image.bmp` を維持する、ログは標準出力で確認できること、外部ログ基盤や永続ログ保管は追加しない  
**Scale/Scope**: 単一サーバ、少数クライアント向けのローカル運用。アクセスログは基本的な切り分け情報に限定する

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されていない

## Project Structure

### Documentation (this feature)

```text
specs/014-access-logs/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── access-log-output-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
server/
├── Cargo.toml
├── Cargo.lock
├── run.sh
└── src/
    └── main.rs

specs/
├── 013-image-dither-rotate/
└── 014-access-logs/
```

**Structure Decision**: 既存サーバの責務を保つため、変更は `server/src/main.rs` と必要最小限の運用文書に閉じる。アクセスログはミドルウェア追加よりも簡潔なハンドラ内/共通ヘルパー出力で実現し、既存 route 契約を壊さずに運用観測性を強化する。

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

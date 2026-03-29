# 実装計画: run.sh 配信設定改善

**Branch**: `012-fix-run-access-path` | **Date**: 2026-03-29 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/012-fix-run-access-path/spec.md)  
**Input**: `/specs/012-fix-run-access-path/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

`run.sh` の起動導線を、LAN 内の別端末から利用できる運用に合わせて見直す。具体的には、待受先が外部到達可能であることを利用者へ正しく案内し、配信元ディレクトリを起動時に切り替えられるようにする。既定値は `server/contents/` のまま維持しつつ、カレントディレクトリに依存しない解決方式へ統一し、既存の最小構成を崩さずに運用柔軟性を追加する。

## Technical Context

**Language/Version**: Bash (POSIX shell) + Rust stable  
**Primary Dependencies**: `server/run.sh`、`cargo run --release`、既存 Rust HTTP サーバ (`axum` / Tokio)  
**Storage**: ローカルファイルシステム上の任意ディレクトリ。既定値は `server/contents/`  
**Testing**: Bash 起動確認、Rust 自動テスト、LAN/localhost からの手動 HTTP 確認  
**Target Platform**: devcontainer またはローカル開発環境、LAN 内の BMP 配信サーバ利用環境  
**Project Type**: ローカル常駐 HTTP サーバの起動スクリプト改善  
**Performance Goals**: 従来どおり即時起動でき、利用者が 1 回の起動で別端末から配信確認できること  
**Constraints**: ローカル優先、既定利用者に追加必須設定を増やさない、配信 route や画像処理機能は拡張しない、起動元カレントディレクトリに依存しない  
**Scale/Scope**: 単一開発者、単一ホスト、少数 LAN 端末向けの起動導線改善

## Constitution Check

*GATE: Phase 0 research 前に確認し、Phase 1 design 後に再確認済み。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化していない

## Project Structure

### Documentation (this feature)

```text
specs/012-fix-run-access-path/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── run-script-invocation-contract.md
└── tasks.md
```

### Source Code (repository root)
```text
server/
├── contents/
│   └── .gitignore
├── Cargo.toml
├── Cargo.lock
├── run.sh
└── src/
    └── main.rs

specs/
├── 002-add-run-script/
├── 010-http-bmp-server/
└── 012-fix-run-access-path/
```

**Structure Decision**: 実装は `server/` 配下に限定する。起動入口は `server/run.sh`、HTTP 待受やファイル配信の本体は `server/src/main.rs` に既に存在するため、feature では起動パラメータ受け渡し、既定値解決、利用者向け案内の整合を中心に設計する。設計成果物は `specs/012-fix-run-access-path/` に集約する。

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

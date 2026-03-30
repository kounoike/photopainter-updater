# 実装計画: HTTPサーバ構成整理

**Branch**: `018-http-server-refactor` | **Date**: 2026-03-30 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/018-http-server-refactor/spec.md)  
**Input**: `/specs/018-http-server-refactor/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

既存の Rust 製 HTTP サーバは `server/src/main.rs` 1 ファイルに、起動設定、route 定義、画像変換、バイナリ/BMP 応答生成、アクセスログ、起動メッセージ、テスト補助が集中している。今回の feature では既存の `/`、`/image.bmp`、`/image.bin` の配信契約を維持したまま、設定読込と検証を `envconfig` ベースで整理し、起動時およびアクセス時の運用ログを `tracing` 系に寄せ、責務単位のファイル分割で保守性と検証性を高める。

## Technical Context

**Language/Version**: Rust stable（edition 2024）  
**Primary Dependencies**: `axum`、Tokio、`image`、`envconfig`、`tracing`、`tracing-subscriber`  
**Storage**: ローカルファイル（既定は `server/contents/image.png`）、永続 DB なし  
**Testing**: `cargo test`、HTTP 応答の自動テスト、設定読込の単体テスト、起動ログ/失敗系の手動確認  
**Target Platform**: ローカル LAN 上で動く開発用 HTTP サーバ、Linux 系 devcontainer / ローカル実行環境  
**Project Type**: 単一 Rust サーバの内部リファクタリング  
**Performance Goals**: 既存の `cargo test` で確認できる主要 route 応答と変換回帰を維持し、起動時には待受先と設定解決結果を 1 回の起動ログ確認で判断できること  
**Constraints**: 既存 route と response contract 維持、画像変換アルゴリズム自体は変えない、外部基盤を追加しない、`server/` 配下の変更に限定する  
**Scale/Scope**: 単一プロセス・単一入力画像・少数クライアント向けのローカル運用。対象は server crate の構造整理と回帰防止

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されている

## Project Structure

### Documentation (this feature)

```text
specs/018-http-server-refactor/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── server-runtime-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
server/
├── Cargo.toml
├── run.sh
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── app.rs
│   ├── logging.rs
│   ├── routes.rs
│   ├── response.rs
│   └── image_pipeline/
│       ├── mod.rs
│       ├── load.rs
│       ├── dither.rs
│       ├── bmp.rs
│       └── binary.rs
└── testdata/
    └── image-dither-rotate/
```

**Structure Decision**: 単一 crate は維持しつつ、起動設定、router/handler、ログ、応答生成、画像変換をモジュール分離する。`main.rs` は起動配線に限定し、既存テストは責務ごとのモジュールへ再配置または共通 helper 化して、回帰確認の粒度を上げる。

## Phase 0: Research Summary

- `envconfig` を導入し、環境変数と既定値を 1 箇所へ集約する方針を採る
- `tracing` + `tracing-subscriber` を導入し、起動ログとアクセスログを同一導線へ統一する
- モジュール分割は「設定」「起動配線」「route/handler」「ログ」「画像変換」「HTTP 応答」の責務境界で行う
- 既存 route 契約は contract とテストで固定し、内部整理と外部挙動変更を分離する

## Phase 1: Design & Contracts

### Data Model Output

- `ServerConfig`: 環境変数から解決される起動設定
- `AppState`: request handler が共有する実行時状態
- `AccessLogEvent`: リクエスト単位の構造化ログ要素
- `ImagePipelineRequest` / `ImagePipelineResult`: 変換要求と生成結果

### Contract Output

- `contracts/server-runtime-contract.md`: 起動設定、起動ログ、HTTP route 維持条件、失敗時の案内規則

### Quickstart Output

- 既定値起動確認
- 設定値上書き確認
- `/`、`/image.bmp`、`/image.bin` の回帰確認
- ログ出力と失敗系の確認

## Post-Design Constitution Check

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を維持している
- [x] Allowed Scope / Forbidden Scope を超える変更を計画していない
- [x] 文書成果物は日本語で記述している
- [x] 各主要要求に対する検証方法を plan / quickstart に反映した
- [x] 単一 crate 維持のまま最小限の依存追加に留め、ローカル優先を維持している

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

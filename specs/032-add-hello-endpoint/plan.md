# 実装計画: Hello 動作確認エンドポイント

**Branch**: `032-add-hello-endpoint` | **Date**: 2026-04-04 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/032-add-hello-endpoint/spec.md)  
**Input**: `/specs/032-add-hello-endpoint/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

既存の Rust 製 HTTP サーバは画像取得用の `/`、`/image.bmp`、`/image.bin` と画像更新用の `POST /upload` を提供している。今回の feature では、画像ファイルの有無や変換状態に左右されない疎通確認用 endpoint として `GET /hello` を追加し、利用者が `text/plain` の `hello` を受け取るだけでサーバ稼働を判定できるようにする。実装は既存 router と logging の流れに沿って最小変更で行い、not found 方針と既存 endpoint の契約は維持したまま、README と quickstart に `/hello` ベースの確認手順を追記する。

## Technical Context

**Language/Version**: Rust stable（edition 2024）  
**Primary Dependencies**: `axum` 0.8、Tokio、`tracing`、`tracing-subscriber`、既存 `http-bmp-server` crate 内の `routes.rs` / `response.rs` / `logging.rs`  
**Storage**: N/A（`/hello` 自体は永続化やファイル入出力を持たない）  
**Testing**: `cargo test`、HTTP route の自動テスト、README に沿った `curl` 手動確認  
**Target Platform**: Docker Compose 経由またはローカルで動作する開発用 HTTP サーバ、Linux 系 devcontainer / ローカル実行環境  
**Project Type**: 単一 Rust サーバへの小規模 endpoint 追加  
**Performance Goals**: `/hello` は画像処理を介さず即時に応答し、利用者が 1 リクエストで server 稼働を判定できること  
**Constraints**: 既存の `/`、`/image.bmp`、`/image.bin`、`/upload` の契約を変更しない、`/hello` は画像状態に依存しない、既存 logging / fallback 方針を維持する、`firmware/` と `xiaozhi-esp32/` は変更しない  
**Scale/Scope**: 単一プロセス・少数クライアント向けのローカル運用。対象は `server/` 配下の route 追加、回帰テスト、利用文書更新に限定する

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
specs/032-add-hello-endpoint/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── hello-endpoint-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
server/
├── Cargo.toml
├── README.md
└── src/
    ├── app.rs
    ├── config.rs
    ├── logging.rs
    ├── response.rs
    └── routes.rs
```

**Structure Decision**: 単一 crate 構成を維持し、HTTP 契約追加は `server/src/routes.rs` を中心に行う。疎通確認応答の生成は既存 `response.rs` helper を再利用し、アクセスログは既存 `logging.rs` / `record` 経路へ載せる。利用手順は `server/README.md` と quickstart に集約し、新規の永続データや補助モジュールは追加しない。

## Phase 0: Research Summary

- `GET /hello` は画像変換パイプラインを通さない専用 handler とし、画像状態に左右されない疎通確認専用 route とする
- 応答形式は既存 helper と同じ `text/plain` に揃え、本文は固定文字列 `hello` として利用者とテストが 1 回の request で成功判定できるようにする
- 既存 fallback は維持し、`/hello` だけを明示的 route として追加することで未定義 path の `404` 契約を壊さない
- `GET` route と同じ logging 経路に載せ、`method`、`path`、`status`、`outcome` の記録方針を維持する
- README と quickstart の起動確認は `/hello` を先頭導線に更新し、画像未配置時でも使える確認手順へ寄せる

## Phase 1: Design & Contracts

### Data Model Output

- `HelloProbeRequest`: 利用者または運用者が `/hello` へ送る疎通確認 request
- `HelloProbeResponse`: 稼働確認に使う成功レスポンス。画像状態に依存せず `hello` を返す
- `AccessLogEvent`: 既存の request log 1 件。`/hello` 追加後も同じ構造で記録する
- `RouteContractSet`: `/hello` と既存 route 群の共存条件。新 route が既存契約を変えないことを明示する

### Contract Output

- `contracts/hello-endpoint-contract.md`: `GET /hello` の request / response 契約、既存 route との共存条件、ログ期待値

### Quickstart Output

- Docker Compose 起動後に `/hello` で疎通確認する手順
- 画像未配置でも `/hello` が成功することの確認
- `/hello` 追加後も既存 `/image.bmp` と `/image.bin` が従来どおり確認できることの確認

## Post-Design Constitution Check

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を維持している
- [x] Allowed Scope / Forbidden Scope を超える変更を計画していない
- [x] 文書成果物は日本語で記述している
- [x] 各主要要求に対する検証方法を plan / quickstart / contract に反映した
- [x] 単一 Rust サーバと既存運用導線を維持し、外部基盤を増やさない最小構成を保っている

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

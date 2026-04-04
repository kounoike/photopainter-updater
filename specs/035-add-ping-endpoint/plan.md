# 実装計画: Ping 動作確認エンドポイント

**Branch**: `035-add-ping-endpoint` | **Date**: 2026-04-04 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/035-add-ping-endpoint/spec.md)  
**Input**: `/specs/035-add-ping-endpoint/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

既存の Rust 製 HTTP サーバには `/hello` を含む疎通確認導線があるが、今回はさらに本文なしで `200 OK` だけを返す最小 endpoint として `GET /ping` を追加する。実装は `server/src/routes.rs` に専用 handler を追加し、既存の logging 経路へ載せる最小変更とする。`/ping` は画像状態に依存しない route とし、README と quickstart へ使い分けを追記する。既存 `/hello`、画像取得系 endpoint、fallback、upload の契約は維持する。

## Technical Context

**Language/Version**: Rust stable（edition 2024）  
**Primary Dependencies**: `axum` 0.8、Tokio、既存 `routes.rs` / `response.rs` / `logging.rs`  
**Storage**: N/A（`/ping` 自体は永続化やファイル入出力を持たない）  
**Testing**: `cargo test`、route 単体テスト、README の `curl` 手順確認  
**Target Platform**: Docker Compose 経由またはローカルで動作する開発用 HTTP サーバ  
**Project Type**: 単一 Rust サーバへの小規模 endpoint 追加  
**Performance Goals**: `/ping` は画像処理を介さず即時に `200 OK` を返し、1 request で server 到達性を判定できること  
**Constraints**: `/ping` の本文は空、画像状態に依存しない、既存 `/hello` と他 endpoint を壊さない、`firmware/` と `xiaozhi-esp32/` は変更しない  
**Scale/Scope**: `server/` 配下の route 追加、回帰テスト、最小の文書更新に限定する

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
specs/035-add-ping-endpoint/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── ping-endpoint-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
server/
├── README.md
└── src/
    └── routes.rs
```

**Structure Decision**: 単一 crate 構成を維持し、`/ping` の route / handler / route test は `server/src/routes.rs` に追加する。成功 response は既存 helper を再利用し、access log は既存 `record` 経路へ載せる。利用文書は `server/README.md` と quickstart に限定し、新規モジュールや永続データは追加しない。

## Phase 0: Research Summary

- `GET /ping` は画像変換パイプラインを通さない専用 handler とする
- response は `200 OK` と空 body に固定し、追加情報は載せない
- 既存 `/hello` は維持し、`/ping` はさらに軽い到達性確認用として共存させる
- 未定義 path の `404` 契約と既存 logging 方針は維持する
- README には `/ping` と `/hello` の使い分けを最小限追記する

## Phase 1: Design & Contracts

### Data Model Output

- `PingProbeRequest`: `GET /ping` による到達性確認 request
- `PingProbeResponse`: `200 OK` と空 body の成功 response
- `RouteContractSet`: `/ping` と既存 route 群の共存条件

### Contract Output

- `contracts/ping-endpoint-contract.md`: `GET /ping` の request / response 契約、既存 route との共存条件、ログ期待値

### Quickstart Output

- server 起動後に `/ping` で到達性確認する手順
- `/ping` の後に `/hello` や画像 route を確認して切り分ける手順

## Post-Design Constitution Check

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を維持している
- [x] Allowed Scope / Forbidden Scope を超える変更を計画していない
- [x] 文書成果物は日本語で記述している
- [x] 各主要要求に対する検証方法を plan / quickstart / contract に反映した
- [x] 単一 Rust サーバと既存運用導線を維持し、外部基盤を増やさない最小構成を保っている

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

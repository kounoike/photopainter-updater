# 実装計画: HTTPサーバ Compose 統合

**Branch**: `029-compose-http-server` | **Date**: 2026-04-03 | **Spec**: [spec.md](./spec.md)  
**Input**: `/specs/029-compose-http-server/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

既存 Rust HTTP サーバを Docker Compose 管理下へ移し、`server/run.sh` を廃止する。repo ルート `compose.yml` に `server` サービスを追加し、server バイナリは container 内で起動、配信データは既存 `server/contents/` を bind mount で渡す。ComfyUI / Ollama / AI Toolkit とは同一 compose プロジェクト内で共存させ、README と server 関連手順を compose 起動前提へ一本化する。

## Technical Context

**Language/Version**: Docker Compose v2 YAML、Dockerfile syntax、Rust stable（既存 server）  
**Primary Dependencies**: 既存 `compose.yml`、既存 `server/` Rust server（`axum` / Tokio）、新規 `server/Dockerfile`  
**Storage**: bind mount（`${SERVER_CONTENT_DIR:-./server/contents}`、必要に応じて build cache）  
**Testing**: `docker compose config`、HTTP サーバ単体起動の compose 手動確認、既存 endpoint 疎通確認、README / quickstart 整合確認  
**Target Platform**: Docker Engine + Docker Compose v2 が使えるローカル開発環境  
**Project Type**: Compose 設定更新 + server container 化 + 運用ドキュメント  
**Performance Goals**: 既存 endpoint の体感応答を悪化させず、compose 起動だけで server 利用開始できる  
**Constraints**: 既存 API 契約は変更しない、ComfyUI/Ollama/AI Toolkit 共存を壊さない、配信データは既存 `server/contents/` を継続利用する、`server/run.sh` は廃止する  
**Scale/Scope**: 単一ホスト・単一 compose ファイル・単一 server service

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は compose + 単一 Dockerfile の範囲に留めている

**Phase 1 再確認（Design 後）**:
- [x] server container 追加以外に新規サービスや分散要素を導入していない
- [x] 既存 server 実装は極力維持し、起動導線中心の変更に留めている
- [x] 検証手順は server 単体、既存 compose 共存、文書一本化の 3 観点を持つ

## Project Structure

### Documentation (this feature)

```text
specs/029-compose-http-server/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── compose-http-server-runtime-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
compose.yml
.env.example
README.md
.env.example
server/
├── Dockerfile
├── README.md
├── contents/
├── Cargo.toml
└── src/
```

**Structure Decision**: compose との統合点は root `compose.yml` に置き、HTTP サーバの container 化責務は `server/Dockerfile` に閉じ込める。既存配信データは `SERVER_CONTENT_DIR` で切り替え可能にしつつ既定値を `server/contents/` に保ち、`server/run.sh` は廃止対象とする。

## Phase 0: Research 成果物

→ [research.md](./research.md) 参照

## Phase 1: Design

### Runtime 設計

- `compose.yml` に `server` サービスを追加する
- `server/Dockerfile` で Rust server を build / run できるようにする
- 配信データは `${SERVER_CONTENT_DIR:-./server/contents}` を bind mount する
- `.env.example` に `SERVER_PORT` と `SERVER_CONTENT_DIR` を追加し、compose から server process へ `PORT` と `CONTENT_DIR` を渡す
- 既存 endpoint `/`、`/image.bmp`、`/image.bin`、`/upload` は変更しない

### Validation 設計

- `docker compose config` で service 定義が解決される
- server 単体起動相当で `/image.bmp` と `/upload` が利用できる
- 既存 ComfyUI / Ollama / AI Toolkit 設定が残る
- README / server README / feature quickstart が compose 前提へ統一される

## Phase 1: Contracts

→ [contracts/compose-http-server-runtime-contract.md](./contracts/compose-http-server-runtime-contract.md) 参照

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| なし | なし | なし |

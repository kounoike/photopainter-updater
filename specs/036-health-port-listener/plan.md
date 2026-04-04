# 実装計画: Health Port Listener

**Branch**: `036-health-port-listener` | **Date**: 2026-04-04 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/036-health-port-listener/spec.md)  
**Input**: `/specs/036-health-port-listener/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

この feature では、既存の main listener (`PORT`) を維持したまま、任意の `PORT_HEALTH` で `/ping` だけを返す health-only listener を追加できるようにする。`PORT_HEALTH` 未指定時は従来どおり main listener だけを起動し、`PORT_HEALTH == PORT` のときは二重 bind を避けて main listener 上の `/ping` をそのまま使う。実装は `server/src/config.rs` で設定を拡張し、`server/src/routes.rs` に health-only router を追加し、`server/src/app.rs` で listener 起動条件と startup message を制御する最小変更とする。

## Technical Context

**Language/Version**: Rust stable（edition 2024）  
**Primary Dependencies**: `axum` 0.8、Tokio、既存 `app.rs` / `config.rs` / `routes.rs` / `logging.rs`  
**Storage**: N/A  
**Testing**: `cargo test`、config unit test、route test、app startup message test  
**Target Platform**: Docker Compose 経由またはローカルで動作する開発用 HTTP サーバ  
**Project Type**: 単一 Rust サーバの listener 構成拡張  
**Performance Goals**: health check は軽量に `/ping` だけへ到達でき、main listener の既存動作を劣化させないこと  
**Constraints**: `PORT` の既存意味を変えない、`PORT_HEALTH` は任意、同一 port 時は起動失敗しない、health-only listener は `/ping` 以外を公開しない  
**Scale/Scope**: `server/` 配下の設定、起動、router、README 更新に限定する

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は listener 条件分岐に限定して正当化されている

## Project Structure

### Documentation (this feature)

```text
specs/036-health-port-listener/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── health-port-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
server/
├── README.md
└── src/
    ├── app.rs
    ├── config.rs
    └── routes.rs
```

**Structure Decision**: port 設定の読み込みは `config.rs`、listener 起動と startup message は `app.rs`、route 提供範囲は `routes.rs` に集約する。health-only listener のために新規 crate や別 binary は追加せず、同じ `AppState` を共有する。テストは既存の config / route / app unit test に寄せる。

## Phase 0: Research Summary

- `PORT_HEALTH` は `Option<u16>` として扱い、未指定時は health listener を起動しない
- health-only listener は `/ping` のみ route 登録し、それ以外は fallback で `404` を返す
- `PORT_HEALTH == PORT` のときは追加 bind を行わず、main listener の `/ping` を health check として使う
- startup message には main port と health port の関係を明示し、未指定 / 同一 / 別 port の 3 パターンを区別する
- main listener と health listener の共存は `tokio::try_join!` で同時 serve し、どちらかの異常終了を surface する

## Phase 1: Design & Contracts

### Data Model Output

- `PortConfiguration`: `port` と `health_port` の組み合わせ
- `HealthListenerMode`: `Disabled` / `SharedWithMain` / `Dedicated`
- `ListenerBindingPlan`: 起動時に bind する listen 先の一覧

### Contract Output

- `contracts/health-port-contract.md`: `PORT_HEALTH` の有効条件、`/ping` の公開範囲、同一 port 時の扱い

### Quickstart Output

- `PORT_HEALTH` 未指定 / 同一 / 別 port の確認手順
- health-only listener では `/ping` 以外が公開されないことの確認

## Post-Design Constitution Check

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を維持している
- [x] Allowed Scope / Forbidden Scope を超える変更を計画していない
- [x] 文書成果物は日本語で記述している
- [x] 各主要要求に対する検証方法を plan / quickstart / contract に反映した
- [x] 単一プロセス・単一 crate の最小構成を維持している

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

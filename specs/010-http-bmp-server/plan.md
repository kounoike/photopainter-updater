# 実装計画: BMP配信HTTPサーバ

**Branch**: `010-http-bmp-server` | **Date**: 2026-03-29 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/010-http-bmp-server/spec.md)  
**Input**: `/specs/010-http-bmp-server/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

PhotoPainter 向けの最小 HTTP サーバとして、`GET /` と `GET /image.bmp` の両方で同一の `image.bmp` を返す機能を実装する。技術選定は 008 と 009 の結論に従い Rust + `axum` とし、今回は画像変換や telemetry を含めず、ローカル LAN 内で単一画像を返すことだけに集中する。`image.bmp` は利用者が後から配置または差し替える前提とし、未配置時は「サーバ故障」ではなく「画像未配置」と判別できる失敗応答を返す。

## Technical Context

**Language/Version**: Rust stable  
**Primary Dependencies**: `axum`、Tokio、Rust 標準ライブラリのファイルアクセス  
**Storage**: ローカルファイル (`server/contents/image.bmp`)  
**Testing**: Rust の自動テスト、HTTP 手動確認、quickstart による運用確認  
**Target Platform**: devcontainer 上のローカル開発環境、LAN 内 companion HTTP サーバ  
**Project Type**: ローカル常駐の単一バイナリ HTTP サーバ  
**Performance Goals**: 単一 BMP ファイルをローカル LAN 内で安定して返し、PhotoPainter が `/` または `/image.bmp` から取得できること  
**Constraints**: 初期スコープは `/` と `/image.bmp` の固定 2 route のみ、画像生成や前処理は行わない、`image.bmp` は利用者が別途用意する、ローカル優先の最小構成を維持する  
**Scale/Scope**: 単一家庭内または少数端末向け。単一画像配信のみを扱う最小サーバ

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は Rust + `axum` 採用理由として限定的に正当化されている

## Project Structure

### Documentation (this feature)

```text
specs/010-http-bmp-server/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── root-bmp-response-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
server/
├── contents/
│   └── .gitignore
├── Cargo.toml
├── run.sh
└── src/
    └── main.rs

docs/
├── firmware.md
└── firmware-http-epaper.md
```

**Structure Decision**: サーバ実装は `server/` 配下に閉じる。`server/src/main.rs` に最小の `axum` アプリケーションを置き、`server/contents/image.bmp` を配信元とする。既存の `server/run.sh` は Rust サーバの起動導線として維持または差し替える。

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

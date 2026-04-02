# 実装計画: POST画像保存

**Branch**: `026-post-image-upload` | **Date**: 2026-04-02 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/026-post-image-upload/spec.md)  
**Input**: `/specs/026-post-image-upload/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

既存の Rust 製 HTTP サーバは `server/contents/image.png` を入力として `GET /`、`GET /image.bmp`、`GET /image.bin` を返している。今回の feature ではこの取得契約を維持したまま、新たに `POST /upload` を追加し、raw body と multipart/form-data の両方で受け取った画像を現在画像として更新できるようにする。受理した画像は保存前に形式判定して PNG へ正規化し、必要に応じて 480x800 へアスペクト比維持の拡大縮小と中央クロップを適用してから `image.png` に置き換える。無効な入力や途中失敗時は既存の `image.png` を保護し、運用者が 1 回の応答とログ確認で成否を判別できる構成を採る。

## Technical Context

**Language/Version**: Rust stable（edition 2024）  
**Primary Dependencies**: `axum` 0.8（`multipart` feature を追加）、Tokio、`image` 0.25、`envconfig`、`tracing`、`tracing-subscriber`  
**Storage**: ローカルファイル（既定は `server/contents/image.png`）、永続 DB なし  
**Testing**: `cargo test`、HTTP route の自動テスト、画像正規化の単体テスト、`curl` による手動アップロード確認  
**Target Platform**: ローカル LAN 上で動く開発用 HTTP サーバ、Linux 系 devcontainer / ローカル実行環境  
**Project Type**: 単一 Rust サーバへの機能追加  
**Performance Goals**: 正常な更新要求 1 回で現在画像を置き換え、直後の取得要求から更新結果を返せること。失敗時は追加調査なしで応答本文とアクセスログから原因分類を判断できること  
**Constraints**: 既存の `GET /`、`GET /image.bmp`、`GET /image.bin` の契約を維持する、更新 path は `POST /upload`、認証なし、外部ストレージや認証基盤を追加しない、保存結果は常に PNG かつ 480x800、`firmware/` と `xiaozhi-esp32/` は変更しない  
**Scale/Scope**: 単一プロセス・単一現在画像・少数クライアント向けのローカル運用。対象は `server/` 配下のアップロード受理、画像正規化、応答/ログ更新、関連文書更新に限定する

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
specs/026-post-image-upload/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── upload-endpoint-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
server/
├── Cargo.toml
├── run.sh
├── README.md
├── contents/
└── src/
    ├── main.rs
    ├── app.rs
    ├── config.rs
    ├── logging.rs
    ├── response.rs
    ├── routes.rs
    └── image_pipeline/
        ├── mod.rs
        ├── load.rs
        ├── dither.rs
        ├── bmp.rs
        └── binary.rs
```

**Structure Decision**: 単一 crate 構成は維持し、HTTP 境界の変更は `routes.rs` と `response.rs` に集約する。画像アップロードの decode・正規化・保存は新規の upload 専用モジュール、または `image_pipeline/` 配下の責務として切り出し、既存の GET 変換パイプラインと POST 保存パイプラインを混線させない。設定値と定数は `config.rs`、運用ログは `logging.rs` を継続利用し、`server/README.md` と quickstart にアップロード導線を追記する。

## Phase 0: Research Summary

- `POST /upload` は `Content-Type` に応じて raw body と multipart/form-data の処理を分ける
- multipart 受理には `axum` の `multipart` feature を追加し、単一画像ファイルだけを保存候補として扱う
- 画像形式の自動判定と PNG 正規化は `image` crate の decode / `guess_format` / PNG encode に統一する
- 480x800 正規化はアスペクト比維持の拡大縮小と中央クロップ規則を採用する
- 保存時は一時ファイル経由の置換で現在画像を保護し、失敗時は既存 `image.png` を残す
- `POST /upload` も既存アクセスログ導線へ統合し、成否と失敗分類を同じ確認経路で追えるようにする

## Phase 1: Design & Contracts

### Data Model Output

- `UploadRequest`: raw body / multipart のどちらから来たかを含むアップロード要求の正準表現
- `UploadCandidate`: decode 済みで保存前の画像とメタ情報
- `NormalizedImage`: PNG かつ 480x800 に整形済みの保存候補
- `UploadResult`: 成功、入力不正、保存失敗、内部失敗の結果分類
- `CurrentImageFile`: 現在の `image.png` と一時置換のライフサイクル

### Contract Output

- `contracts/upload-endpoint-contract.md`: `POST /upload` の request 形式、成功応答、失敗応答、既存 GET route との整合条件

### Quickstart Output

- raw body でのアップロード確認
- multipart/form-data でのアップロード確認
- 正規化後の保存結果と GET route 反映確認
- 不正入力時に既存画像が保持されることの確認

## Post-Design Constitution Check

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を維持している
- [x] Allowed Scope / Forbidden Scope を超える変更を計画していない
- [x] 文書成果物は日本語で記述している
- [x] 各主要要求に対する検証方法を plan / quickstart / contract に反映した
- [x] 単一 Rust サーバとローカルファイル保存を維持し、外部基盤を増やさない最小構成を保っている

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

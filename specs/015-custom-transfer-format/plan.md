# 実装計画: 独自画像転送形式追加

**Branch**: `015-custom-transfer-format` | **Date**: 2026-03-29 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/015-custom-transfer-format/spec.md)  
**Input**: `/specs/015-custom-transfer-format/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

既存の Rust HTTP サーバは `/` と `/image.bmp` で変換済み BMP を返し、firmware はそれを `download.bmp` として SD カードへ保存してから `GUI_ReadBmp_RGB_6Color` で描画している。今回の feature では BMP 互換経路を維持したまま `/image.bin` を追加し、server が e-paper 向けの 4bit packed frame buffer と最小ヘッダを返し、firmware はそれを SD カードへ保存せずに受信後そのまま表示バッファへ反映できる構成を採る。

## Technical Context

**Language/Version**: Rust stable、C/C++ on ESP-IDF v5.5.x  
**Primary Dependencies**: `axum`、Tokio、Rust 標準ライブラリの HTTP/byte 処理、ESP-IDF `esp_http_client`、`Paint_*` / `epaper_port_display`  
**Storage**: server 側はローカルファイル (`image.png` 入力)、firmware 側は設定用 SD カードを維持。ただし画像更新時の中間 BMP 保存は不要にする  
**Testing**: Rust 自動テスト、server の HTTP 応答検証、firmware の手動更新確認、失敗系の手動確認  
**Target Platform**: ローカル LAN 上の HTTP サーバ、PhotoPainter firmware を動かす ESP32 + e-paper  
**Project Type**: server + firmware の転送契約追加  
**Performance Goals**: 既存 BMP 経路より不要なファイル I/O を減らし、1 回の更新で保存なしに表示更新を完了できること  
**Constraints**: `/` と `/image.bmp` の既存互換維持、`firmware/` 配下のみ変更、LAN/ローカル優先、表示サイズは `800x480`、e-paper 色値は既存 6 色 index に合わせる  
**Scale/Scope**: 単一サーバと少数デバイスのローカル運用。対象は単一画像の HTTP 配信と 1 台の e-paper 描画経路

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
specs/015-custom-transfer-format/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── image-bin-transfer-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
server/
├── run.sh
└── src/
    └── main.rs

firmware/
└── main/
    ├── config.cc
    ├── display_update.cc
    ├── display_update.h
    └── update_job.cc
```

**Structure Decision**: 変更対象は server の新 route と payload 生成、firmware の HTTP 受信と直接描画経路に限定する。既存 BMP 経路は比較基準として残し、独自形式の契約は `contracts/` へ明示する。

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

# 実装計画: HTTP画像の直接表示検討

**Branch**: `007-stream-http-render` | **Date**: 2026-03-29 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/007-stream-http-render/spec.md)  
**Input**: `/specs/007-stream-http-render/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

`firmware/` の現行更新経路は、HTTP で取得した BMP を `/sdcard/download.bmp` に保存し、その後 `GUI_ReadBmp_RGB_6Color()` へファイルパスを渡して e-paper へ反映している。今回の planning では、HTTP 応答を SD カードへ保存せずに直接表示できるかを評価し、既存の GUI/BMP 読み込み API とメモリ制約を踏まえて採用可否を判断する。調査結果として、現行 `epaper_src` はファイルパスとフル画像バッファを前提にしており、直接表示のためには BMP デコーダ置き換え級の変更が必要で、要求に対して複雑化が大きい。したがって本 feature では現行の SD 一時保存方式を維持し、採用見送り理由と運用影響を文書化する。

## Technical Context

**Language/Version**: C/C++ on ESP-IDF v5.5.x  
**Primary Dependencies**: `firmware/main/display_update.*`、`firmware/main/update_job.cc`、`xiaozhi-esp32/components/epaper_src/GUI_BMPfile.c`、`esp_http_client`、`sdcard_bsp`  
**Storage**: SD card (`/sdcard/config.txt`, `/sdcard/download.bmp`) と NVS の既存 failure/developer mode 領域  
**Testing**: 既存 `idf.py -C firmware build`、仕様文書確認、既存実機手順に沿った手動確認  
**Target Platform**: Waveshare ESP32-S3-PhotoPainter  
**Project Type**: firmware + hardware integration  
**Performance Goals**: 起動時更新と BOOT ボタン更新で既存どおり 60 秒以内に表示更新できることを維持し、保存方式見直しのためにそれ以上の表示遅延を持ち込まない  
**Constraints**: `firmware/` のみ変更、`xiaozhi-esp32/` は参照専用、24-bit BMP 前提を維持、失敗分類と deep sleep 挙動を壊さない、LAN 内ローカル構成を維持、追加 RAM 消費や新規大規模デコーダ導入は避ける  
**Scale/Scope**: 単一デバイスの HTTP→表示経路の評価と最小限の文書反映のみ

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は見送り判断として明示的に抑制している

## Project Structure

### Documentation (this feature)

```text
specs/007-stream-http-render/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── render-path-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
firmware/
├── main/
│   ├── display_update.*
│   ├── update_job.cc
│   └── failure_state.*
└── build/

docs/
├── firmware.md
└── firmware-http-epaper.md

xiaozhi-esp32/
└── components/
    └── epaper_src/
        ├── GUI_BMPfile.c
        └── GUI_BMPfile.h
```

**Structure Decision**: 実装コードは `firmware/` に限定し、BMP デコードの実現可能性判断には `xiaozhi-esp32/components/epaper_src` を参照する。今回の結論は「直接表示方式を採用しない」なので、主な成果物は設計文書、契約文書、利用者向け文書更新になる。

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

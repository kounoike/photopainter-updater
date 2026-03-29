# 実装計画: ACT LED アクティビティ表示

**Branch**: `006-activity-led-indicator` | **Date**: 2026-03-29 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/006-activity-led-indicator/spec.md)  
**Input**: `/specs/006-activity-led-indicator/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

`firmware/` の起動時更新と BOOT ボタン更新に対して、進行中だけ活動表示 LED を点滅させる。既存の `xiaozhi-esp32` 参照実装に含まれる `led_bsp` を第一候補として流用しつつ、実機で利用可能な LED/GPIO を確認した上で、更新ジョブの開始と終了に同期した単純な点滅制御を `firmware/` 側へ統合する。正常完了と失敗終了のどちらでも点滅は停止し、待機状態で誤って進行中に見えないことを重視する。

## Technical Context

**Language/Version**: C/C++ on ESP-IDF v5.5.x  
**Primary Dependencies**: `firmware/` 配下の更新ジョブ実装、`xiaozhi-esp32/components/led_bsp`、既存 `button_bsp`、`sdcard_bsp`、`epaper_port`、実機 LED/GPIO 確認結果  
**Storage**: N/A  
**Testing**: 実機手動確認、既存更新フロー上での LED 観察  
**Target Platform**: Waveshare ESP32-S3-PhotoPainter  
**Project Type**: firmware + hardware integration  
**Performance Goals**: 更新ジョブ開始後すぐに LED 点滅へ入り、進行中は継続し、終了後は速やかに停止する  
**Constraints**: `firmware/` 側だけを変更する、`xiaozhi-esp32/` は参照専用、既存の更新・sleep・設定仕様は変更しない、LAN 内の単機能運用を維持する、LED 点滅は単一パターンで十分に視認可能であること  
**Scale/Scope**: 単一デバイス上の ACT LED 状態表示追加のみ

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
specs/006-activity-led-indicator/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── activity-led-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
firmware/
├── CMakeLists.txt
├── sdkconfig.defaults
└── main/
    ├── CMakeLists.txt
    ├── main.cc
    ├── update_job.*
    ├── display_update.*
    └── failure_state.*

xiaozhi-esp32/
└── components/
    └── led_bsp/
```

**Structure Decision**: 実装コードは既存どおり `firmware/` に閉じ、LED 制御部品は `xiaozhi-esp32/components/led_bsp` を参照 component として追加する。仕様成果物は feature 専用ディレクトリ配下にまとめる。

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

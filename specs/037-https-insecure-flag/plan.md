# 実装計画: Config Insecure HTTPS

**Branch**: `037-https-insecure-flag` | **Date**: 2026-04-04 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/037-https-insecure-flag/spec.md)  
**Input**: `/specs/037-https-insecure-flag/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

この feature では、`firmware/` の `config.txt` 仕様を拡張し、任意 boolean の `insecure` を追加して `https://` の `image_url` を扱えるようにする。`insecure: true` のときだけ HTTPS 証明書未検証通信を許可し、未設定または `false` では通常の証明書検証を維持する。実装は `firmware/main/config.*` の設定読込契約拡張、`firmware/main/display_update.*` の HTTP client 設定分岐、関連ドキュメント更新に限定し、既存の HTTP / BMP / binary 更新フローと失敗分類を壊さない最小変更とする。

## Technical Context

**Language/Version**: C/C++ on ESP-IDF v5.5.x  
**Primary Dependencies**: `esp_http_client`、ESP-TLS / mbedTLS、既存 `config.cc` / `display_update.cc` / `update_job.cc`、既存 `sdkconfig` の certificate bundle 設定  
**Storage**: SD card 上の `/sdcard/config.txt`、既存 NVS failure/developer mode 領域  
**Testing**: `idf.py build`、手動設定検証（HTTP、検証付き HTTPS、未検証 HTTPS、設定不備）  
**Target Platform**: ESP32-S3 ベースの PhotoPainter firmware、ローカル LAN / WiFi 環境  
**Project Type**: 単一 firmware の設定契約・通信挙動拡張  
**Performance Goals**: HTTPS 追加後も既存の起動時更新 / BOOT ボタン更新フローを維持し、証明書検証分岐以外で余計な再試行や追加待機を発生させないこと  
**Constraints**: `xiaozhi-esp32/` は変更しない、`config.txt` の既存必須項目は維持、`insecure` は任意 boolean のみ、HTTP 挙動は不変、`insecure: true` でも証明書検証以外の失敗は従来どおり失敗扱い  
**Scale/Scope**: `firmware/main/`、`docs/firmware-http-epaper.md`、feature 成果物に限定。単一デバイス設定とローカル運用が前提

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は firmware 既存通信経路の条件分岐追加に限定して正当化されている

## Project Structure

### Documentation (this feature)

```text
specs/037-https-insecure-flag/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── insecure-https-config-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
firmware/
├── sdkconfig
└── main/
    ├── config.cc
    ├── config.h
    ├── display_update.cc
    ├── display_update.h
    └── update_job.cc

docs/
└── firmware-http-epaper.md
```

**Structure Decision**: `config.txt` の schema と入力検証は `firmware/main/config.*` に集約し、HTTPS 通信時の検証ポリシー切替は `firmware/main/display_update.*` に閉じ込める。`update_job.cc` は既存の更新シーケンス維持を優先し、必要最小限の設定受け渡しに留める。文書変更は既存 firmware 運用説明のある `docs/firmware-http-epaper.md` を中心に行う。

## Phase 0: Research Summary

- `config.txt` の `insecure` は任意 boolean とし、未設定時は `false` に正規化する
- `image_url` は `http://` と `https://` の両方を受け付けるが、`insecure` の効果は `https://` のときだけ持つ
- HTTPS の通常経路は ESP-IDF の certificate bundle を使ったサーバ証明書検証を有効にする
- `insecure: true` の HTTPS だけ、証明書検証を省略する client 設定へ切り替える
- `insecure` が不正型のときは通信開始前に config error として失敗させる

## Phase 1: Design & Contracts

### Data Model Output

- `FirmwareConfig`: 既存 WiFi / URL 設定に `insecure` を追加した設定モデル
- `ImageTransportPolicy`: `image_url` の scheme と `insecure` の組み合わせで決まる検証方針
- `ConfigValidationResult`: 設定読込時に成功、型不正、scheme 不正を切り分ける結果

### Contract Output

- `contracts/insecure-https-config-contract.md`: `config.txt` の追加フィールド、URL scheme 許容範囲、HTTPS 検証有無、失敗分類を定義する

### Quickstart Output

- `http://` の既存更新回帰確認
- `https://` + `insecure: false` での検証付き更新確認
- `https://` + `insecure: true` での未検証更新確認
- `insecure` 型不正時の設定不備確認

## Post-Design Constitution Check

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を維持している
- [x] Allowed Scope / Forbidden Scope を超える変更を計画していない
- [x] 文書成果物は日本語で記述している
- [x] 各主要要求に対する検証方法を plan / quickstart / contract に反映した
- [x] firmware 既存構成を維持し、外部依存の追加やアーキテクチャ拡張を伴わない最小構成になっている

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

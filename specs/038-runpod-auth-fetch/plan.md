# 実装計画: RunPod Authenticated Fetch

**Branch**: `038-runpod-auth-fetch` | **Date**: 2026-04-04 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/038-runpod-auth-fetch/spec.md)  
**Input**: `/specs/038-runpod-auth-fetch/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

この feature では、`firmware/` の画像取得設定を外部 HTTPS サーバー運用向けに拡張し、`config.txt` に任意 boolean の `insecure` と任意 string の `bearer_token` を追加する。`image_url` は `http://` と `https://` を受け付け、`bearer_token` がある場合だけ `Authorization: Bearer <token>` を送る。`insecure: true` のときだけ HTTPS 証明書未検証通信を許可し、それ以外は通常の証明書検証を維持する。実装は `firmware/main/config.*` の設定契約拡張、`firmware/main/display_update.*` の HTTP client 設定とヘッダ付与、`firmware/main/update_job.cc` の呼び出し更新、関連ドキュメント更新に限定し、既存の HTTP / BMP / binary 更新フローを維持する最小変更とする。

## Technical Context

**Language/Version**: C/C++ on ESP-IDF v5.5.x  
**Primary Dependencies**: `esp_http_client`、ESP-TLS / mbedTLS、既存 `config.cc` / `display_update.cc` / `update_job.cc`、既存 `sdkconfig` の certificate bundle 設定  
**Storage**: SD card 上の `/sdcard/config.txt`、既存 NVS failure/developer mode 領域  
**Testing**: `idf.py build`、手動設定検証（認証付き HTTPS、認証付き未検証 HTTPS、認証失敗、設定不備、HTTP 回帰）  
**Target Platform**: ESP32-S3 ベースの PhotoPainter firmware、外部 HTTPS 更新元を含む WiFi 環境  
**Project Type**: 単一 firmware の設定契約・認証付き通信挙動拡張  
**Performance Goals**: Bearer 認証と HTTPS 追加後も既存の起動時更新 / BOOT ボタン更新フローを維持し、追加処理はヘッダ付与と検証分岐に限定すること  
**Constraints**: `xiaozhi-esp32/` は変更しない、`config.txt` の既存必須項目は維持、`bearer_token` は任意 non-empty string のみ、`insecure` は任意 boolean のみ、HTTP 挙動は不変、秘密情報暗号化保存は今回扱わない  
**Scale/Scope**: `firmware/main/`、`docs/firmware-http-epaper.md`、feature 成果物に限定。単一デバイス設定と外部更新元 1 件の運用が前提

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
specs/038-runpod-auth-fetch/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── authenticated-fetch-contract.md
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

**Structure Decision**: `config.txt` の schema と入力検証は `firmware/main/config.*` に集約し、認証ヘッダ付与と HTTPS 検証ポリシー切替は `firmware/main/display_update.*` に閉じ込める。`update_job.cc` は既存の更新シーケンス維持を優先し、設定受け渡しだけを追加する。文書変更は既存 firmware 運用説明のある `docs/firmware-http-epaper.md` を中心に行う。

## Phase 0: Research Summary

- `config.txt` に `bearer_token` を任意 non-empty string として追加し、未設定時は認証ヘッダを送らない
- `config.txt` に `insecure` を任意 boolean として追加し、未設定時は `false` に正規化する
- `image_url` は `http://` と `https://` の両方を受け付ける
- Bearer 認証は `Authorization: Bearer <token>` で付与し、他の認証方式へ自動変換しない
- HTTPS の通常経路は certificate bundle を使ったサーバ証明書検証を維持し、`insecure: true` の HTTPS だけ未検証接続へ切り替える
- `bearer_token` 型不正、空文字、`insecure` 型不正は通信開始前に config error として失敗させる

## Phase 1: Design & Contracts

### Data Model Output

- `FirmwareConfig`: 既存 WiFi / URL 設定に `insecure` と `bearer_token` を追加した設定モデル
- `ImageRequestAuth`: 認証ヘッダ送信の有無と Bearer トークン値を表す認証条件
- `ImageTransportPolicy`: `image_url` の scheme と `insecure` の組み合わせで決まる検証方針
- `ConfigValidationResult`: 設定読込時に成功、型不正、空文字、不正 scheme を切り分ける結果

### Contract Output

- `contracts/authenticated-fetch-contract.md`: `config.txt` の追加フィールド、Authorization ヘッダ条件、HTTPS 検証条件、失敗分類を定義する

### Quickstart Output

- Bearer 認証付き HTTPS 更新の確認
- `insecure: true` を併用した未検証 HTTPS 更新の確認
- Bearer トークン未設定または無効時の失敗確認
- `http://` 更新回帰の確認

## Post-Design Constitution Check

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を維持している
- [x] Allowed Scope / Forbidden Scope を超える変更を計画していない
- [x] 文書成果物は日本語で記述している
- [x] 各主要要求に対する検証方法を plan / quickstart / contract に反映した
- [x] firmware 既存構成を維持し、外部依存の追加やアーキテクチャ拡張を伴わない最小構成になっている

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

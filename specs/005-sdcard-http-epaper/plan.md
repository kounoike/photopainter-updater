# 実装計画: SDカード設定 HTTP e-paper 更新ファーム

**Branch**: `005-sdcard-http-epaper` | **Date**: 2026-03-29 | **Spec**: [/home/kounoike/ghq/github.com/kounoike/photopainter-updater/specs/005-sdcard-http-epaper/spec.md](/home/kounoike/ghq/github.com/kounoike/photopainter-updater/specs/005-sdcard-http-epaper/spec.md)  
**Input**: `/specs/005-sdcard-http-epaper/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

`xiaozhi-esp32` を参照元として活用しつつ、`firmware/` 配下に SDカードルートの `config.json` を読んで WiFi 接続し、起動時と BOOT ボタン押下時に単一の画像 URL から HTTP 取得を行い、e-paper を更新する専用ファームウェアを作る。失敗時はリトライ継続や対話機能へフォールバックせず、原因を判断できる状態を残して更新処理を終了し、そのままシャットダウンする。

## Technical Context

**Language/Version**: C/C++（既存 `xiaozhi-esp32` / ESP-IDF ベース）、Markdown（設計文書）  
**Primary Dependencies**: `firmware/` 配下の新規実装、`xiaozhi-esp32/main/`、`components/sdcard_bsp`、`components/button_bsp`、`components/epaper_port`、既存ネットワーク/HTTP 関連部品  
**Storage**: SDカード上の `config.json`、必要に応じた既存 NVS 設定領域  
**Testing**: 手動実機確認、設定ファイル差し替え確認、起動時更新確認、BOOT ボタン更新確認、失敗時シャットダウン確認  
**Target Platform**: SDカード、WiFi、BOOT ボタン、e-paper を備えた ESP32 系デバイス  
**Project Type**: 既存ファームウェア派生の単機能専用ファームウェア  
**Performance Goals**: 正常系では起動後 60 秒以内に画像更新を完了し、BOOT ボタン更新でも 60 秒以内に表示更新結果を確認できること  
**Constraints**: `config.json` は SDカードルート固定、画像取得先は単一 URL、失敗時は更新処理終了後にシャットダウン、実装コードは `firmware/` 配下に置き、`xiaozhi-esp32/` は参照専用で書き換えない、HTTP 以外の取得方式や複数画像管理には拡張しない  
**Scale/Scope**: 単一デバイス向けの起動時更新と手動再更新を初期スコープとし、定期更新・クラウド対話・複数画像配信は扱わない

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

### Phase 0 Gate

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されていない

### Phase 1 Re-Check

- [x] 設計は `xiaozhi-esp32` の既存 SDカード、ボタン、e-paper、HTTP 周辺部品を参照しつつ、実装は `firmware/` に分離している
- [x] スコープは単一 URL 取得と BOOT ボタン更新に限定され、会話機能や複数画像機能へ拡張していない
- [x] 正常系と失敗系の手動検証方針が定義されている
- [x] ローカル設定ファイル駆動で動作し、外部インターネット依存を前提にしていない

## Project Structure

### Documentation (this feature)

```text
docs/
└── firmware-http-epaper.md

specs/005-sdcard-http-epaper/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── config-and-update-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
firmware/
├── CMakeLists.txt              # ファームウェア用プロジェクト入口
├── sdkconfig.defaults          # 最低限の既定設定
└── main/
    ├── CMakeLists.txt          # main component 定義
    ├── main.cc                 # 起動入口
    ├── config.*                # config.json 読込と必須項目検証
    ├── update_job.*            # 起動時/BOOT 更新の直列制御
    ├── display_update.*        # HTTP 取得と e-paper 更新
    └── failure_state.*         # 失敗分類とシャットダウン制御

xiaozhi-esp32/
├── main/
│   ├── main.cc
│   ├── application.cc
│   ├── settings.cc
│   └── boards/
├── components/
│   ├── sdcard_bsp/
│   ├── button_bsp/
│   ├── epaper_port/
│   ├── http_client_bsp/
│   ├── json_bsp/
│   └── user_app_bsp/
└── README.md

specs/005-sdcard-http-epaper/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
```

**Structure Decision**: 実装本体は `firmware/` 配下に新設し、`xiaozhi-esp32/` は参照専用の同梱コードとして扱う。最低限の初期構成として `firmware/CMakeLists.txt`、`firmware/sdkconfig.defaults`、`firmware/main/CMakeLists.txt`、`firmware/main/main.cc` と、設定読込・更新ジョブ・表示更新・失敗制御の責務単位ファイルを置ける状態を先に作る。とくに、`xiaozhi-esp32` の起動モード分岐、SDカードアクセス、ボタン入力、e-paper 表示、HTTP 取得周辺を設計参考とし、派生実装は `firmware/` 側で独立して組み立てる。

## Phase 0: Research Outcome

Phase 0 では、`config.json` を SDカードルートの単一設定ファイルとすること、画像取得は単一 URL に限定すること、失敗時は更新処理終了後にシャットダウンすること、実装は `firmware/` 配下に置いて `xiaozhi-esp32/` は書き換えないこと、既存の `sdcard_bsp` / `button_bsp` / `epaper_port` を参照候補とすることを確定した。`Technical Context` に `NEEDS CLARIFICATION` は残っていない。

## Phase 1: Design Outcome

- `data-model.md` で、設定ファイル、更新ジョブ、表示画像、失敗状態の関係を定義する
- `contracts/config-and-update-contract.md` で、`config.json` の必須項目と更新ジョブの振る舞い契約を定義する
- `quickstart.md` で、正常系と失敗系の手動検証手順を定義する
- 実装では、`firmware/` 側に起動時更新、BOOT ボタン更新、失敗時シャットダウンの一貫した制御フローを組み立てる

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

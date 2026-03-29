# AGENTS.md

## Purpose

This repository uses Kanban as an orchestration layer and Spec Kit as the execution workflow.

All agents must treat Kanban cards as executable units and follow Spec Kit workflow strictly.

---

## Core Principles

* When working on tasks described in a Kanban card, if the work is a feature addition, the Spec Kit workflow must be followed strictly.

---

## Required Execution Spec Kit Workflow

For every implementation task, follow this order:

1. specify
2. clarify (only for local ambiguities)
3. plan
4. tasks
5. analyze
6. implement
7. commit
8. merge

Rules:

* Do NOT skip phases
* Do NOT jump directly to implementation
* Each phase must produce artifacts (spec/plan/tasks/etc.)
* commit & merge only after successful implementation and testing

Phase transition commit policy:

* commit 前提は「前の step の最後」ではなく「次の step の開始条件」として扱うこと
* `clarify` と `analyze` は commit 前提の対象外とし、未 commit 状態でも実行してよい
* `plan` を始める前に、`specify` の成果物が commit 済みであること
* `tasks` を始める前に、`plan` の成果物が commit 済みであること
* `implement` を始める前に、`tasks` までの成果物が commit 済みであること
* 上記の commit 前提がある phase (`plan` `tasks` `implement`) の開始時に、そこまでの成果物が未 commit の場合は、次の phase へ進む前に必ずユーザーへ続行可否を確認すること
* ユーザーが明示的に続行を許可しない限り、未 commit 状態で次 phase へ進んではならない
* commit を行う場合は、その phase の成果物を優先してまとめ、無関係な変更を巻き込まないこと

---

# Codex notes

Use skill commands:

$speckit-specify
$speckit-clarify
$speckit-plan
$speckit-tasks
$speckit-analyze
$speckit-implement


---

# Claude code notes

Use slash commands:

* /speckit.specify
* /speckit.clarify
* /speckit.plan
* /speckit.tasks
* /speckit.analyze
* /speckit.implement

---

## Firmware Boundary

- `005-sdcard-http-epaper` 以降の本命ファームウェア実装は `firmware/` 配下に置く。
- `xiaozhi-esp32/` は参照専用の同梱コードとして扱い、直接書き換えない。
- `xiaozhi-esp32/` から設計や実装方針を取り込む場合も、変更は `firmware/` 側へ反映する。

---

## Active Technologies
- Bash (POSIX shell) / Python 3 (実行環境) + Python 標準の HTTP 配信機能 (002-add-run-script)
- files (`server/contents/`) (002-add-run-script)
- Markdown（成果物）、既存参照実装は C/C++ on ESP-IDF + 既存 `xiaozhi-esp32` ソースツリー、既存 README 群、Spec Kit 成果物 (004-document-xiaozhi-arch)
- リポジトリ内ファイル（`docs/` と `specs/004-document-xiaozhi-arch/`） (004-document-xiaozhi-arch)
- C/C++（既存 `xiaozhi-esp32` / ESP-IDF ベース）、Markdown（設計文書） + `xiaozhi-esp32/main/`、`components/sdcard_bsp`、`components/button_bsp`、`components/epaper_port`、既存ネットワーク/HTTP 関連部品 (005-sdcard-http-epaper)
- SDカード上の `config.json`、必要に応じた既存 NVS 設定領域 (005-sdcard-http-epaper)
- C/C++ on ESP-IDF v5.5.x + `firmware/` 配下の更新ジョブ実装、`xiaozhi-esp32/components/led_bsp`、既存 `button_bsp`、`sdcard_bsp`、`epaper_port` (006-activity-led-indicator)
- C/C++ on ESP-IDF v5.5.x + `firmware/main/display_update.*`、`firmware/main/update_job.cc`、`xiaozhi-esp32/components/epaper_src/GUI_BMPfile.c`、`esp_http_client`、`sdcard_bsp` (007-stream-http-render)
- SD card (`/sdcard/config.txt`, `/sdcard/download.bmp`) と NVS の既存 failure/developer mode 領域 (007-stream-http-render)
- Python 3 系、Rust stable、Go stable の比較調査 + 既存 `server/run.sh`、Python 標準 `http.server`、FastAPI、axum、Go 標準 `net/http` (008-http-server-stack)
- files (`server/contents/`) を中心としたローカルファイル運用 (008-http-server-stack)
- Python 3 系、Rust stable、Go stable の比較調査 + FastAPI、axum、Go 標準 `net/http`、`ref/convert.py`、将来候補としての画像処理ライブラリ連携、監視基盤連携、コンテナ配布前提 (008-http-server-stack)
- Rust stable 系候補の比較調査 + `axum`、`actix-web`、`warp`、Tokio stack、`ref/convert.py` を参照した画像前処理要件 (009-rust-http-stack)
- N/A（文書調査のみ） (009-rust-http-stack)
- Rust stable + `axum`、Tokio、Rust 標準ライブラリのファイルアクセス (010-http-bmp-server)
- ローカルファイル (`server/contents/image.bmp`) (010-http-bmp-server)
- Dockerfile syntax、devcontainer.json、Bash、Node.js LTS ベースの開発 CLI + Dev Containers 構成、ESP-IDF v5.5.1、GitHub CLI、`@openai/codex`、`@anthropic-ai/claude-code` (011-improve-devcontainer)
- devcontainer 設定ファイル、Git 管理外の永続認証キャッシュ領域、`.devcontainer/.env` (011-improve-devcontainer)
- Bash (POSIX shell) + Rust stable + `server/run.sh`、`cargo run --release`、既存 Rust HTTP サーバ (`axum` / Tokio) (012-fix-run-access-path)
- ローカルファイルシステム上の任意ディレクトリ。既定値は `server/contents/` (012-fix-run-access-path)
- Rust stable + 参照用 Python 3 スクリプト + `axum`、Tokio、Rust 標準ライブラリのファイルアクセス、画像変換ライブラリ、`ref/convert.py` を参照したディザリング方針 (013-image-dither-rotate)
- ローカルファイル (`server/contents/image.png` 入力、配信時に生成される 24bit BMP 出力) (013-image-dither-rotate)

## Recent Changes
- 002-add-run-script: Added Bash (POSIX shell) / Python 3 (実行環境) + Python 標準の HTTP 配信機能

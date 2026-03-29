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

## Recent Changes
- 002-add-run-script: Added Bash (POSIX shell) / Python 3 (実行環境) + Python 標準の HTTP 配信機能

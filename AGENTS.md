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
* `specify` `clarify` `plan` `tasks` `implement` の各 phase が完了したら、特段の問題がない限りその phase の成果物を自動で commit すること
* `tasks.md` の対象タスクがすべて完了し、`implement` が成功して必要なテストも通った場合は、特段の問題がない限り追加のユーザー指示を待たずに feature branch の成果物を自動で commit し、そのまま `main` へ merge して終了してよい
* `analyze` は指摘だけなら commit しない。analyze の指摘に基づく remediation を反映して成果物を更新した場合のみ、その更新内容を自動で commit してよい
* `analyze` で HIGH または MEDIUM の指摘が出た場合は、直せる範囲のものを自動で remediation すること
* `analyze` の remediation では、spec / plan / tasks など成果物の整合修正を優先し、分からないことや判断が分岐することだけをユーザーへ確認すること
* `analyze` の指摘が LOW のみで、成果物更新が不要なら remediation は必須ではない
* 各 phase 完了後、次の phase に進むための前提が満たされており、ユーザー確認が必須の事項や明示的な停止条件がない場合は、ユーザーの追加指示を待たずに次の phase へ自動で進んでよい
* 自動進行を止めるのは、未 commit 前提、判断分岐、BLOCKER、スコープ外変更、テスト失敗、git 異常、またはユーザーが明示的に停止した場合だけとする
* `plan` を始める前に、`specify` の成果物が commit 済みであること
* `tasks` を始める前に、`plan` の成果物が commit 済みであること
* `implement` を始める前に、`tasks` までの成果物が commit 済みであること
* 上記の commit 前提がある phase (`plan` `tasks` `implement`) の開始時に、そこまでの成果物が未 commit の場合は、次の phase へ進む前に必ずユーザーへ続行可否を確認すること
* ユーザーが明示的に続行を許可しない限り、未 commit 状態で次 phase へ進んではならない
* ただし、自動 commit の前に無関係な変更が混在している、commit 対象が曖昧、必要なテストが未実施または失敗、git lock / conflict などの異常がある場合は、先にユーザーへ確認すること
* commit を行う場合は、その phase の成果物を優先してまとめ、無関係な変更を巻き込まないこと
* 自動 merge を行う場合も、merge 対象はその feature branch の成果物に限定し、競合、未解決差分、無関係な変更混在、またはユーザー確認が必要な異常がある場合は停止して確認すること

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
- Rust stable + `axum`、Tokio、Rust 標準ライブラリの時刻/ソケット/標準出力 (014-access-logs)
- N/A（永続保存なし、標準出力ログのみ） (014-access-logs)
- Rust stable、C/C++ on ESP-IDF v5.5.x + `axum`、Tokio、Rust 標準ライブラリの HTTP/byte 処理、ESP-IDF `esp_http_client`、`Paint_*` / `epaper_port_display` (015-custom-transfer-format)
- server 側はローカルファイル (`image.png` 入力)、firmware 側は設定用 SD カードを維持。ただし画像更新時の中間 BMP 保存は不要にする (015-custom-transfer-format)
- Rust stable（edition 2024） + `axum`、Tokio、`image`、`envconfig`、`tracing`、`tracing-subscriber` (018-http-server-refactor)
- ローカルファイル（既定は `server/contents/image.png`）、永続 DB なし (018-http-server-refactor)
- Rust stable（edition 2024）、Bash（既存起動補助） + 既存 `axum`, `envconfig`, `image`, `tokio`, `tracing` (019-dither-image-ideas)
- ローカルファイル（`server/contents/`、`server/testdata/`、`specs/019-dither-image-ideas/` の実験記録） (019-dither-image-ideas)
- ローカルファイル（`server/contents/`、`server/testdata/`、`specs/020-adaptive-diffusion-tuning/`） (020-adaptive-diffusion-tuning)
- Docker Compose v2、YAML、Markdown + Docker Engine / Docker Compose v2、公式イメージ `ollama/ollama`、既存 `yanwk/comfyui-boot:cu128-slim` (023-add-ollama-compose)
- ホスト bind mount ディレクトリ（`./comfyui-data`、新規 `./ollama-data`） (023-add-ollama-compose)
- Python 3.10-3.13、Bash (POSIX shell)、Markdown + `SimpleTuner`、`bitsandbytes` または同等の量子化バックエンド、Hugging Face Hub、既存 ComfyUI / `Z-Image` 推論環境 (024-zimage-character-lora)
- ローカルファイル（参照画像セット、学習設定、キャッシュ、LoRA 成果物、validation 画像） (024-zimage-character-lora)
- Docker Compose v2、YAML、Markdown、既存コンテナイメージ設定 + Docker Engine / Docker Compose v2、既存 `yanwk/comfyui-boot:cu128-slim`、既存 `ollama/ollama`、既存 `.env.example` と `README.md` (025-ai-toolkit-env)
- ホスト bind mount ディレクトリ（`./comfyui-data`、`./ollama-data`）、feature 配下の Markdown 成果物 (025-ai-toolkit-env)
- Docker Compose v2、YAML、Markdown + Docker Engine / Docker Compose v2、`ostris/aitoolkit:latest`、既存 `yanwk/comfyui-boot:cu128-slim`、既存 `ollama/ollama` (025-ai-toolkit-env)
- ホスト bind mount または bind mount 相当のローカルパス（AI Toolkit 用 config / datasets / output / DB / Hugging Face cache）、feature 配下の Markdown 成果物 (025-ai-toolkit-env)
- Rust stable（edition 2024） + `axum` 0.8（`multipart` feature を追加）、Tokio、`image` 0.25、`envconfig`、`tracing`、`tracing-subscriber` (026-post-image-upload)
- Python 3.x（ComfyUI ランタイム同梱版） + ComfyUI custom node backend API（`NODE_CLASS_MAPPINGS` / `OUTPUT_NODE`）、Python 標準ライブラリ `urllib` / `io`、既存ランタイムに含まれる `Pillow`、`numpy` (027-comfyui-post-node)
- ローカルファイル（repo 内 `comfyui/custom_node/comfyui-photopainter-custom/`、実行時 `comfyui-data/custom_nodes/`） (027-comfyui-post-node)
- Docker Compose v2 YAML、Markdown + 既存 `compose.yml`、既存 `yanwk/comfyui-boot:cu128-slim`、既存 repo 配下 `comfyui/custom_node/comfyui-photopainter-custom` (028-auto-mount-comfyui-post-node)
- bind mount（repo 内 custom node ディレクトリ、`${COMFYUI_DATA_DIR:-./comfyui-data}/custom_nodes`） (028-auto-mount-comfyui-post-node)
- Docker Compose v2 YAML、Dockerfile syntax、Rust stable（既存 server） + 既存 `compose.yml`、既存 `server/` Rust server（`axum` / Tokio）、新規 `server/Dockerfile` (029-compose-http-server)
- bind mount（`server/contents/`） (029-compose-http-server)
- Docker Compose v2 YAML、Dockerfile syntax、Bash（補助手順）、既存 ComfyUI runtime + 既存 `compose.yml`、新規 `comfyui/Dockerfile`、既存 `comfyui/custom_node/comfyui-photopainter-custom`、Docker BuildKit、NVIDIA Container Toolkit (030-build-comfyui-image)
- bind mount（`${COMFYUI_DATA_DIR:-./comfyui-data}` 配下）、repo 内 `comfyui/` build context、`.env.example` (030-build-comfyui-image)
- Docker Compose v2 YAML、Dockerfile syntax、Bash、Python 3.13、既存 ComfyUI runtime + 既存 `compose.yml`、既存 `comfyui/Dockerfile`、既存 `comfyui/entrypoint.sh`、`uv`、ComfyUI upstream manual install 手順、repo 管理 `comfyui/custom_node/comfyui-photopainter-custom` (031-bake-custom-node)
- bind mount（`${COMFYUI_DATA_DIR:-./comfyui-data}` 配下の models / output / input / user 設定）、image に焼き込む repo 管理 custom node、`.env.example` (031-bake-custom-node)
- Rust stable（edition 2024） + `axum` 0.8、Tokio、`tracing`、`tracing-subscriber`、既存 `http-bmp-server` crate 内の `routes.rs` / `response.rs` / `logging.rs` (032-add-hello-endpoint)
- N/A（`/hello` 自体は永続化やファイル入出力を持たない） (032-add-hello-endpoint)
- YAML（GitHub Actions workflow / release drafter 設定）、Markdown + GitHub Actions、Release Drafter、既存 GitHub repository 運用、pull request labels (033-release-drafter)
- GitHub repository 内の workflow / 設定ファイル、永続 DB なし (033-release-drafter)
- YAML（GitHub Actions workflow）、Markdown、既存 Dockerfile syntax + GitHub Actions、GitHub Releases event、GHCR、Docker Buildx、`docker/login-action`、`docker/metadata-action`、`docker/build-push-action` (034-ghcr-release-publish)
- GitHub repository 内 workflow / publish target 定義 / 文書、GHCR 上の container image (034-ghcr-release-publish)

## Recent Changes
- 002-add-run-script: Added Bash (POSIX shell) / Python 3 (実行環境) + Python 標準の HTTP 配信機能

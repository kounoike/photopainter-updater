# 実装計画: AI Toolkit 試用環境

**Branch**: `025-ai-toolkit-env` | **Date**: 2026-04-01 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/025-ai-toolkit-env/spec.md)  
**Input**: `/specs/025-ai-toolkit-env/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

既存の `compose.yml` で運用している ComfyUI と Ollama を AI Toolkit 試用環境の土台として位置づけ、利用者が「どの前提で起動し、何を確認すれば試せたと判断できるか」を迷わない導線へ整理する。実装は新しいアプリ本体や複雑な自動化ではなく、Compose ベースの起動契約、前提条件、代表操作、既存導線との境界、失敗時の復帰方法を `README.md` と feature 成果物へ明文化する最小構成を採る。

## Technical Context

<!--
  ACTION REQUIRED:
  この節は実装前提を具体値で置き換える。
  不明点は推測せず NEEDS CLARIFICATION または TODO: を使う。
-->

**Language/Version**: Docker Compose v2、YAML、Markdown、既存コンテナイメージ設定  
**Primary Dependencies**: Docker Engine / Docker Compose v2、既存 `yanwk/comfyui-boot:cu128-slim`、既存 `ollama/ollama`、既存 `.env.example` と `README.md`  
**Storage**: ホスト bind mount ディレクトリ（`./comfyui-data`、`./ollama-data`）、feature 配下の Markdown 成果物  
**Testing**: `docker compose config`、`docker compose up -d`、`docker compose ps`、`docker compose exec ollama ollama list`、`docker compose exec comfyui curl -fsS http://ollama:11434/api/version`、手順書追従確認  
**Target Platform**: NVIDIA GPU を使うローカル Linux 開発環境、単一ホストの Docker Compose 運用  
**Project Type**: Docker Compose ベースのローカル AI 試用環境 + ドキュメント整備  
**Performance Goals**: 新規利用者が 15 分以内に主要サービス起動と代表操作 1 件の成功可否を判断できること、既存利用者が既存導線を維持したまま AI Toolkit 入口を理解できること  
**Constraints**: ローカル優先、既存 ComfyUI/Ollama を置き換えない、外部公開前提を増やさない、外部サービス固有の認証差異は対象外、`firmware/` と `server/` 本体へ手を入れない、最小限の追加ファイルに留める
**Scale/Scope**: 単一 compose プロジェクト、主要サービス 2 本を中心にした試用導線、ローカル利用者 1-2 名の再現可能な開始手順

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されている

Phase 1 再確認結果:

- [x] 追加設計は `compose.yml`、`.env.example`、`README.md`、`specs/025-ai-toolkit-env/` に限定されている
- [x] AI Toolkit を既存 ComfyUI/Ollama 上の試用導線として定義し、新しい常駐基盤や外部依存を増やしていない
- [x] 検証手順は起動、前提条件、代表操作、失敗時復帰、既存導線非破壊の 5 観点を満たしている

## Project Structure

### Documentation (this feature)

```text
specs/025-ai-toolkit-env/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── ai-toolkit-compose-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
.
├── compose.yml
├── .env.example
├── README.md
├── comfyui-data/
├── ollama-data/                # 実行時に生成される想定
├── docs/
├── firmware/
├── server/
└── specs/
    ├── 022-add-comfyui-compose/
    ├── 023-add-ollama-compose/
    ├── 024-zimage-character-lora/
    └── 025-ai-toolkit-env/
```

**Structure Decision**: 既存の単一 `compose.yml` と `.env.example` を AI Toolkit 試用環境の実体として維持し、feature 実装ではアプリケーションコードではなく導線の明文化と契約整理に集中する。利用者向け入口はルート `README.md`、詳細手順は feature 配下 `quickstart.md`、変更境界と受け入れ条件は `contracts/` と `data-model.md` に分担する。

## Phase 0: Research Summary

詳細は [research.md](/workspaces/photopainter-updater/specs/025-ai-toolkit-env/research.md) を参照。

- AI Toolkit は新規ランタイムではなく、既存 ComfyUI と Ollama を束ねた Compose ベース試用環境として定義する
- 利用者の試用成功判定は「主要サービス起動 + `docker compose exec comfyui curl -fsS http://ollama:11434/api/version` の成功」に固定する
- 導線は二層構成とし、ルート `README.md` では入口だけを示し、詳細は feature 配下 `quickstart.md` に逃がす
- 失敗時の復帰は、Compose 状態確認、環境変数確認、既存データディレクトリ確認の 3 系統で整理する

## Phase 1: Design Artifacts

### Data Model

詳細は [data-model.md](/workspaces/photopainter-updater/specs/025-ai-toolkit-env/data-model.md) を参照。

- `AiToolkitEnvironment`
- `BaseServiceSet`
- `TrialEntryPoint`
- `RepresentativeOperation`
- `RecoveryHint`

### Contracts

詳細は [ai-toolkit-compose-contract.md](/workspaces/photopainter-updater/specs/025-ai-toolkit-env/contracts/ai-toolkit-compose-contract.md) を参照。

- `compose.yml` / `.env.example` / `README.md` が維持すべき利用者向けインターフェースを固定する
- 既存 ComfyUI / Ollama 単体導線を壊さず、AI Toolkit 入口を追加する条件を互換契約として扱う

### Quickstart

詳細は [quickstart.md](/workspaces/photopainter-updater/specs/025-ai-toolkit-env/quickstart.md) を参照。

- `.env` の準備
- 基本サービスの起動
- `comfyui` から `ollama` への代表操作確認
- 停止、再開、失敗時の確認ポイント

## Implementation Strategy

1. 既存 Compose 運用を AI Toolkit 試用環境としてどう見せるかを contract と data model で固定する
2. ルート `README.md` には AI Toolkit の入口だけを追加し、詳細手順は `quickstart.md` に委譲する
3. `compose.yml` と `.env.example` は必要な補助説明や命名整理に限定して変更する
4. 代表操作を `comfyui` から `ollama` への API 疎通確認に固定し、失敗時復帰条件とあわせて利用開始判定を人が再現できる状態にする
5. 既存 ComfyUI / Ollama の単独利用導線を回帰させないことを実装・検証の前提にする

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| なし | - | - |

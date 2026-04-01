# 実装計画: AI Toolkit 試用環境

**Branch**: `025-ai-toolkit-env` | **Date**: 2026-04-01 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/025-ai-toolkit-env/spec.md)  
**Input**: `/specs/025-ai-toolkit-env/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

`ostris/ai-toolkit` をこのリポジトリの既存 `compose.yml` へ `ai-toolkit` サービスとして追加し、利用者が `.env` を準備して `docker compose up -d ai-toolkit` を実行すれば Web UI へ到達できる状態を整える。既存の ComfyUI / Ollama 導線は維持しつつ、AI Toolkit 用の永続化先、環境変数、認証入口、起動確認、再起動手順を `README.md` と feature 成果物へ同期する。

## Technical Context

**Language/Version**: Docker Compose v2、YAML、Markdown
**Primary Dependencies**: Docker Engine / Docker Compose v2、`ostris/aitoolkit:latest`、既存 `yanwk/comfyui-boot:cu128-slim`、既存 `ollama/ollama`
**Storage**: ホスト bind mount または bind mount 相当のローカルパス（AI Toolkit 用 config / datasets / output / DB / Hugging Face cache）、feature 配下の Markdown 成果物
**Testing**: `docker compose config`、`docker compose up -d ai-toolkit`、`docker compose ps ai-toolkit`、ブラウザまたは同等手段での Web UI 到達確認、再起動後の保存先確認、手順書追従確認
**Target Platform**: NVIDIA GPU を使うローカル Linux 開発環境、単一ホストの Docker Compose 運用  
**Project Type**: Docker Compose ベースのローカル AI 学習ツール試用環境 + ドキュメント整備
**Performance Goals**: 新規利用者が 15 分以内に AI Toolkit 起動と Web UI 到達可否を判断できること、再起動後も同じ保存先で試用を再開できること
**Constraints**: ローカル優先、既存 ComfyUI/Ollama を置き換えない、AI Toolkit 自体のソースは改変しない、最小限の追加ファイルと設定変更に留める
**Scale/Scope**: 単一 compose プロジェクトへの 1 サービス追加、ローカル利用者 1 名、まずは起動して試せる環境整備まで

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されている

Phase 1 再確認結果:

- [x] 追加設計は `compose.yml`、`.env.example`、`README.md`、`specs/025-ai-toolkit-env/` に限定されている
- [x] AI Toolkit は既存 Compose へ 1 サービスを追加するだけで、新しい分散基盤や外部オーケストレーションを導入していない
- [x] 検証手順は起動、Web UI 到達、永続化、既存導線非破壊の 4 観点を満たしている

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
├── ollama-data/
├── docs/
├── firmware/
├── server/
└── specs/
    ├── 022-add-comfyui-compose/
    ├── 023-add-ollama-compose/
    ├── 024-zimage-character-lora/
    └── 025-ai-toolkit-env/
```

**Structure Decision**: 既存の単一 `compose.yml` を維持し、その中へ `ai-toolkit` サービスを追加する。AI Toolkit 用の設定値は `.env.example` に集約し、利用者向け入口はルート `README.md`、詳細手順は feature 配下 `quickstart.md`、実装境界と互換条件は `contracts/` と `data-model.md` に分担する。

## Phase 0: Research Summary

詳細は [research.md](/workspaces/photopainter-updater/specs/025-ai-toolkit-env/research.md) を参照。

- AI Toolkit は `ostris/ai-toolkit` の公式コンテナイメージ前提で compose へ追加する
- サービス名は `ai-toolkit`、利用確認は Web UI 到達で判定する
- 保存先は config / datasets / output / DB / cache をホスト側へ保持する
- 認証は `AI_TOOLKIT_AUTH` のような入口を `.env.example` で案内する

## Phase 1: Design Artifacts

### Data Model

詳細は [data-model.md](/workspaces/photopainter-updater/specs/025-ai-toolkit-env/data-model.md) を参照。

- `AiToolkitService`
- `AiToolkitStorage`
- `AiToolkitEnvConfig`
- `AiToolkitAccessPath`
- `RecoveryHint`

### Contracts

詳細は [ai-toolkit-compose-contract.md](/workspaces/photopainter-updater/specs/025-ai-toolkit-env/contracts/ai-toolkit-compose-contract.md) を参照。

- `compose.yml` / `.env.example` / `README.md` が維持すべき AI Toolkit 利用者向けインターフェースを固定する
- 既存 ComfyUI / Ollama 単体導線を壊さないことを互換契約として扱う

### Quickstart

詳細は [quickstart.md](/workspaces/photopainter-updater/specs/025-ai-toolkit-env/quickstart.md) を参照。

- `.env` の準備
- AI Toolkit 保存先の確認
- `docker compose up -d ai-toolkit`
- Web UI 到達確認
- 停止、再起動、失敗時確認

## Implementation Strategy

1. upstream `docker-compose.yml` を参照しつつ、この repo の `compose.yml` に `ai-toolkit` サービスを追加する
2. `.env.example` に AI Toolkit 用の主要環境変数を追加する
3. `README.md` へ AI Toolkit の入口を追加し、詳細は `quickstart.md` に委譲する
4. 保存先と再起動前提を明文化し、Web UI 到達確認を最小の受け入れ条件として固定する
5. 既存 ComfyUI / Ollama の導線と設定は保持し、追加サービスとして共存させる

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| なし | - | - |

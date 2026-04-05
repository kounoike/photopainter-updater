# 実装計画: ComfyUI 長文回答 continuation

**Branch**: `042-continue-long-answers` | **Date**: 2026-04-05 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/042-continue-long-answers/spec.md)  
**Input**: `/specs/042-continue-long-answers/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

回答本文が長くて 1 回の generation では `max_tokens` または実効予算に収まらない場合、local LLM node が continuation を行い、完結した最終回答を返せるようにする。主対象は単純な長文回答であり、`think_mode=off` の reasoning trace 救済や JSON mode の曖昧 rescue は行わない。debug 出力には continuation 回数と停止理由を追加し、無限継続を避ける上限を明示する。

## Technical Context

**Language/Version**: Python 3.12（ComfyUI custom node runtime）  
**Primary Dependencies**: ComfyUI custom node backend API、`transformers`、`llama-cpp-python`、`lm-format-enforcer`、`jsonschema`  
**Storage**: N/A（runtime 内メモリのみ。既存 local model cache は継続利用）  
**Testing**: `python -m py_compile comfyui/custom_node/comfyui-photopainter-custom/__init__.py`、`python -m unittest discover -s comfyui/custom_node/comfyui-photopainter-custom/tests -v`  
**Target Platform**: Docker Compose 上の ComfyUI container  
**Project Type**: ComfyUI workflow integration / custom node library  
**Performance Goals**: 単純な長文回答は切れずに最後まで返す。不要な continuation は行わず、無限ループや進展のない再生成を避ける。  
**Constraints**: `think_mode=off` 契約を壊さない、JSON/schema 契約を壊さない、外部常駐サービスを追加しない、backend ごとの対応差を明示する  
**Scale/Scope**: custom node 1 パッケージ内の continuation 実装、tests / README / feature 文書更新

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されている

## Project Structure

### Documentation (this feature)

```text
specs/042-continue-long-answers/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── continuation-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
comfyui/
└── custom_node/
    └── comfyui-photopainter-custom/
        ├── __init__.py
        ├── README.md
        └── tests/
            ├── test_contract.py
            └── test_node_logic.py

specs/
└── 042-continue-long-answers/
```

**Structure Decision**: continuation は既存 node package 内の generation helper に局所追加する。node class の UI 追加は最小限に留め、断片連結、停止判定、debug 反映を共有 helper として実装する。

## Phase 0: Outline & Research

### Research Goals

- continuation が必要なケースと不要なケースの判定信号を決める
- backend ごとの対応範囲を決める
- `think_mode=off` と JSON mode を continuation 対象に含めるかを決める
- 無限ループ防止と停止理由の debug 表現を決める

### Research Output

- [research.md](/workspaces/photopainter-updater/specs/042-continue-long-answers/research.md)

## Phase 1: Design & Contracts

### Data Model

- [data-model.md](/workspaces/photopainter-updater/specs/042-continue-long-answers/data-model.md)
- `ContinuationPlan`
- `ContinuationState`
- `ContinuationDebugInfo`

### Contracts

- [contracts/continuation-contract.md](/workspaces/photopainter-updater/specs/042-continue-long-answers/contracts/continuation-contract.md)

### Quickstart

- [quickstart.md](/workspaces/photopainter-updater/specs/042-continue-long-answers/quickstart.md)

### Agent Context

- `.specify/scripts/bash/update-agent-context.sh codex` を実行し、feature 文脈を agent context に反映する

## Post-Design Constitution Check

- [x] 変更範囲は custom node 本体、README、tests、feature 文書に限定されている
- [x] continuation の検証手順を quickstart と tasks で再現できる
- [x] 新しい外部サービスや分散基盤を追加していない
- [x] `think_mode=off` と JSON mode の既存厳格契約を緩めていない
- [x] 文書は日本語で統一されている

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | - | - |

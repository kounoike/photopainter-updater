# 実装計画: ComfyUI think off 強制

**Branch**: `041-enforce-think-off` | **Date**: 2026-04-05 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/041-enforce-think-off/spec.md)  
**Input**: `/specs/041-enforce-think-off/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

`PhotoPainter Transformers LLM Generate` ノードの `think_mode=off` を best-effort prompt ではなく、documented disable を実際に適用できる場合だけ成功する厳格契約へ変更する。Qwen 系の chat template disable が使える経路は成功のまま維持し、未対応 family や runtime fallback は unsupported failure に変える。併せて、`off` 実行で reasoning trace が出た場合は sanitize で救済せず失敗させ、debug 出力と README に `off` 保証の成否を明示する。

## Technical Context

**Language/Version**: Python 3.12（ComfyUI custom node runtime）  
**Primary Dependencies**: ComfyUI custom node backend API、`transformers`、`bitsandbytes`、`accelerate`、`lm-format-enforcer`、`jsonschema`  
**Storage**: N/A（runtime 内メモリと既存 local model cache のみ。新規永続化なし）  
**Testing**: `python -m py_compile comfyui/custom_node/comfyui-photopainter-custom/__init__.py`、`python -m unittest discover -s comfyui/custom_node/comfyui-photopainter-custom/tests -v`  
**Target Platform**: Docker Compose 上の ComfyUI GPU container  
**Project Type**: ComfyUI workflow integration / custom node library  
**Performance Goals**: `think_mode=off` で hidden reasoning による余分な token 消費と待ち時間を成功扱いにしない。成功時は documented disable が効く経路だけを許可し、unsupported 経路は早期 failure で切り分けられるようにする。  
**Constraints**: LAN/ローカル優先、外部サービス追加禁止、`llama-cpp` ノード非対象、既存 `off` 以外の mode は壊さない、JSON retry 契約は維持  
**Scale/Scope**: 単一 custom node package 内の `think_mode=off` 厳格化、tests / README / feature 文書の更新

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
specs/041-enforce-think-off/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── think-off-contract.md
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
└── 041-enforce-think-off/
```

**Structure Decision**: 実装は既存 `comfyui/custom_node/comfyui-photopainter-custom` へ局所適用する。node class の追加や package 分割は行わず、`think_mode=off` 判定、validation、debug 契約だけを更新して影響範囲を最小化する。

## Phase 0: Outline & Research

### Research Goals

- `think_mode=off` を成功させてよい条件を明文化する
- tokenizer/chat template fallback を unsupported とみなす基準を固定する
- `off` で reasoning trace が出た場合の失敗方針を決める
- debug / README に何を追加すれば利用者が「効いたか」を判断できるかを定義する

### Research Output

- [research.md](/workspaces/photopainter-updater/specs/041-enforce-think-off/research.md)

## Phase 1: Design & Contracts

### Data Model

- [data-model.md](/workspaces/photopainter-updater/specs/041-enforce-think-off/data-model.md)
- `ThinkOffEnforcementPlan`
- `ThinkOffDebugStatus`
- `TransformersGenerationResult`

### Contracts

- [contracts/think-off-contract.md](/workspaces/photopainter-updater/specs/041-enforce-think-off/contracts/think-off-contract.md)

### Quickstart

- [quickstart.md](/workspaces/photopainter-updater/specs/041-enforce-think-off/quickstart.md)

### Agent Context

- `.specify/scripts/bash/update-agent-context.sh codex` を実行し、feature 文脈を agent context に反映する

## Post-Design Constitution Check

- [x] 変更範囲は custom node 本体、README、tests、feature 文書に限定されている
- [x] `off` の検証手順を quickstart と tasks で再現できる
- [x] 新しい外部依存や常駐サービスを追加していない
- [x] `llama-cpp` や server / firmware へスコープが漏れていない
- [x] 文書は日本語で統一されている

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | - | - |

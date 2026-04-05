# 実装計画: ComfyUI LLM ノード分離

**Branch**: `040-split-llm-nodes` | **Date**: 2026-04-05 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/040-split-llm-nodes/spec.md)  
**Input**: `/specs/040-split-llm-nodes/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

既存の単一 `PhotoPainter LLM Generate` ノードを削除し、`transformers` 専用ノードと `llama-cpp` 専用ノードへ分離する。`transformers` 側には `quantization_mode` と family ごとの think 制御を集中させ、`llama-cpp` 側には GGUF / `model_file` / context window validation を集中させる。共通の JSON/schema 検証、debug 出力、メモリ解放 helper は再利用しつつ、UI 契約と node 名を backend ごとに明確に分ける。

## Technical Context

**Language/Version**: Python 3.12（ComfyUI custom node runtime）  
**Primary Dependencies**: ComfyUI custom node backend API、`transformers`、`bitsandbytes`、`accelerate`、`llama-cpp-python`、`lm-format-enforcer`、`jsonschema`  
**Storage**: ローカルファイル（ComfyUI container 内 model cache、必要に応じた bind mount 永続ディレクトリ）  
**Testing**: `python -m py_compile`、`python -m unittest discover -s comfyui/custom_node/comfyui-photopainter-custom/tests -v`、ComfyUI 手動 workflow 確認  
**Target Platform**: Docker Compose 上の ComfyUI GPU container  
**Project Type**: ComfyUI workflow integration / custom node library  
**Performance Goals**: backend 固有 UI により設定ミスを減らし、`transformers` 本命経路では 12GB 級 VRAM でも `bnb_4bit` を使った Qwen3.5 9B 試行を可能にする。画像生成本体の VRAM を優先するため、生成後は backend メモリを解放する。  
**Constraints**: LAN/ローカル優先、外部常駐推論サービス禁止、backend ごとの think 制御差、GGUF と Hugging Face 通常重みの運用差、ComfyUI image への依存追加は最小限  
**Scale/Scope**: 単一リポジトリ内の custom node ライブラリ更新、ComfyUI workflow 利用者向けの 2 ノード分離、README・workflow・tests 更新

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
specs/040-split-llm-nodes/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── comfyui-node-contracts.md
└── tasks.md
```

### Source Code (repository root)

```text
comfyui/
├── Dockerfile
├── workflows/
└── custom_node/
    └── comfyui-photopainter-custom/
        ├── __init__.py
        ├── README.md
        └── tests/
            ├── test_contract.py
            └── test_node_logic.py

specs/
└── 040-split-llm-nodes/
```

**Structure Decision**: 既存 `comfyui/custom_node/comfyui-photopainter-custom` を維持し、その内部で共通 helper を再利用しながら node class を 2 つへ分離する。backend 差分の中心は custom node UI / contract なので、別 package 化ではなく既存ライブラリ内の再編に留める。

## Phase 0: Outline & Research

### Research Goals

- backend 分離時にどこまで共通 helper を残すかを決める
- `transformers` 側の `quantization_mode` を node 契約に固定する
- `llama-cpp` 側から `think_mode` を完全に外す
- 旧単一ノード削除後の移行手順を定義する

### Research Output

- [research.md](/workspaces/photopainter-updater/specs/040-split-llm-nodes/research.md)

## Phase 1: Design & Contracts

### Data Model

- [data-model.md](/workspaces/photopainter-updater/specs/040-split-llm-nodes/data-model.md)
- `TransformersLlmNodeInput`
- `LlamaCppLlmNodeInput`
- `SharedGenerationResult`

### Contracts

- [contracts/comfyui-node-contracts.md](/workspaces/photopainter-updater/specs/040-split-llm-nodes/contracts/comfyui-node-contracts.md)

### Quickstart

- [quickstart.md](/workspaces/photopainter-updater/specs/040-split-llm-nodes/quickstart.md)

### Agent Context

- `AGENTS.md` を `040` の技術前提に合わせて更新する

## Post-Design Constitution Check

- [x] backend 分離は Allowed Scope 内の custom node / README / tests / Dockerfile に限定している
- [x] 文書と contract は日本語で統一している
- [x] user story ごとの独立検証は node 契約と手動 workflow で再現できる
- [x] 外部常駐サービスを追加せず、既存 local runtime のまま整理している
- [x] 画像生成本体の VRAM 優先方針を維持している

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | - | - |

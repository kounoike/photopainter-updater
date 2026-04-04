# 実装計画: ComfyUI local LLM node

**Branch**: `039-comfyui-llm-node` | **Date**: 2026-04-04 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/039-comfyui-llm-node/spec.md)  
**Input**: `/specs/039-comfyui-llm-node/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

この feature では、既存 `comfyui/custom_node/comfyui-photopainter-custom` ライブラリへ、ローカル推論専用の薄い LLM node を追加する。node は `system_prompt`、`user_prompt`、backend、`model_id`、任意の `model_file`、`think_mode`、`json_output`、`json_schema`、retry 回数を受け取り、`transformers` または `llama-cpp` で単発推論を実行する。`think_mode` は単なる共通 prompt preset ではなく、model family ごとの documented な think 制御を優先して適用する。構造化出力が必要な場合は generation-time constraint を優先し、自由文 cleanup だけに依存しない。`json_output=true` の場合は constrained generation と strict validation を組み合わせ、parse 失敗または schema 不一致のみ限定 retry の対象とする。model 保存先は環境変数 `COMFYUI_LLM_MODEL_CACHE_DIR` で統一し、`.env` から `compose.yml` 経由で ComfyUI container に注入する。成功時の output は単一 `STRING` とし、`json_output=true` のときだけその文字列が valid JSON を表す。実装は custom node 本体、unit/contract test、README、必要最小限の compose / 環境変数文書更新に限定する。

## Technical Context

**Language/Version**: Python 3.13（既存 ComfyUI image）、devcontainer 上の検証用 Python 3.12  
**Primary Dependencies**: ComfyUI custom node backend API、`transformers`、`llama-cpp-python`、`jsonschema`、`lm-format-enforcer`、Python 標準ライブラリ  
**Storage**: ローカルファイル（ComfyUI container 内 model cache、必要に応じた bind mount 永続ディレクトリ）  
**Testing**: `python -m unittest discover`、`python -m py_compile`、ComfyUI 手動実行確認、devcontainer 内 GPU 検証  
**Target Platform**: Docker Compose で起動する ComfyUI GPU container、または同等のローカル ComfyUI 実行環境  
**Project Type**: ComfyUI custom node 拡張 + ドキュメント更新  
**Performance Goals**: 1 回の node 実行で単発のローカル推論を完了し、schema 不一致時の retry は少数回に限定して workflow の待ち時間を過度に悪化させない  
**Constraints**: node は薄く保つ、HTTP 待受や常駐サービスは追加しない、`think_mode` 初期対応は `off` / `generic` / `qwen` / `gemma` / `deepseek_r1` に限定、schema は `json_schema` 文字列入力のみ、retry 対象は parse 失敗または schema 不一致のみ  
**Scale/Scope**: 単一利用者の ComfyUI workflow、1 node 実行あたり 1 回のローカル LLM 推論、単一 prompt 応答の生成と検証

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
specs/039-comfyui-llm-node/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── comfyui-local-llm-node-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
comfyui/
├── Dockerfile
├── custom_node/
│   └── comfyui-photopainter-custom/
│       ├── __init__.py
│       ├── README.md
│       └── tests/
│           ├── test_contract.py
│           └── test_node_logic.py
└── entrypoint.sh

compose.yml
.env.example
```

**Structure Decision**: 実装は既存の `comfyui-photopainter-custom` モジュールに集約し、ComfyUI image build で同梱する現在の配布経路を維持する。backend adapter、documented think 制御、generation-time structured output、schema 検証、retry 制御は custom node 内の helper 関数に閉じ込め、ComfyUI 本体や server 側へ責務を広げない。環境変数の利用例は `.env.example` と node README に寄せ、runtime 注入は `compose.yml` で `COMFYUI_LLM_MODEL_CACHE_DIR` を container 環境変数として渡す。

## Phase 0: Research Summary

- `transformers` と `llama-cpp-python` の両方を lazy import し、node 入力の backend 切替で単一 node に収める
- `think_mode` は node 共通列挙値 `off` / `generic` / `qwen` / `gemma` / `deepseek_r1` とするが、family 固有 mode では documented な think 制御方法を優先して使う
- `generic` は family 固有制御の代替ではなく best-effort mode として扱う
- model 保存先は環境変数 `COMFYUI_LLM_MODEL_CACHE_DIR` を一次参照とし、未設定時は backend 既定保存先へ fallback する
- `model_id` は Hugging Face Hub の `user/repo` を前提とし、`llama-cpp` では任意入力 `model_file` で repo 内 GGUF を補助指定できる
- JSON/schema 検証は `jsonschema` を使い、構造化出力が要求された場合は `lm-format-enforcer` による generation-time constraint を優先する
- retry は parse 失敗または schema 不一致に限って最大少数回とし、backend や model 自体の失敗は即失敗とする
- node は文字列を後続へ渡す non-output node とし、成功時は単一 `STRING` を返し、失敗時は例外で workflow を止める

## Phase 1: Design & Contracts

### Data Model Output

- `LocalLlmNodeConfig`: node 入力と widget 値を表す設定モデル
- `ResolvedModelPathPolicy`: 環境変数と backend 既定保存先の優先解決結果
- `ThinkControlPlan`: `think_mode` と model family の対応、および documented な think 制御の適用計画
- `StructuredOutputContract`: `json_output`、`json_schema`、generation-time constraint、検証成否、failure kind を表す構造化契約
- `LlmGenerationResult`: 成功時の単一 `STRING` 出力 / UI summary と失敗分類を表す出力モデル

### Contract Output

- `contracts/comfyui-local-llm-node-contract.md`: node metadata、入力 widget、`model_id` / `model_file`、`think_mode`、JSON/schema/retry、failure kind の契約を定義する

### Quickstart Output

- ComfyUI image を再 build して node を読み込む手順
- `COMFYUI_LLM_MODEL_CACHE_DIR` を任意設定する手順
- `transformers + Qwen3.5 + think_mode=off` の smoke workflow
- JSON mode と schema 検証、retry、family ごとの think 切替の確認手順

## Post-Design Constitution Check

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を維持している
- [x] Allowed Scope / Forbidden Scope を超える変更を計画していない
- [x] 文書成果物は日本語で記述している
- [x] 各主要要求に対する検証方法を plan / quickstart / contract に反映した
- [x] ローカル実行の custom node 拡張に閉じ、外部サービスや常駐基盤を増やしていない

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

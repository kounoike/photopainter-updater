# 実装計画: Local RunPod image 統一

**Branch**: `044-local-runpod-image` | **Date**: 2026-04-06 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/044-local-runpod-image/spec.md)  
**Input**: `/specs/044-local-runpod-image/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

この feature では、ComfyUI の local runtime を RunPod serverless 向け `worker-comfyui` ベース image へ統一し、既存の local 専用 `comfyui/Dockerfile` と独立 `ollama` service を廃止する。`compose.yml` の既存 `comfyui` service は維持したまま、build 元を `comfyui/runpod/Dockerfile` へ切り替え、local でも `/runpod-volume` bind mount を必須にして RunPod と同じ model path 前提へ寄せる。RunPod と local の差分は起動方式ではなく mount の有無と運用環境だけに限定し、README / quickstart / RunPod README も「同じ image を local と RunPod が共有する」前提へ整理する。

## Technical Context

**Language/Version**: Dockerfile syntax、Docker Compose v2 YAML、Bash、Markdown、Python 3.x runtime（upstream `worker-comfyui` 同梱環境）  
**Primary Dependencies**: RunPod `worker-comfyui` base image / `/start.sh`、既存 `comfyui/runpod/Dockerfile` と `start-ollama-worker.sh`、Docker Compose v2、RunPod Network Volume 前提の `/runpod-volume` path、`comfyui-ollama` custom node  
**Storage**: host bind mount による `/runpod-volume`、その配下の `/runpod-volume/models` と `/runpod-volume/ollama/models`、repo 内の compose / Dockerfile / Markdown 成果物  
**Testing**: `docker compose config`、local Compose 起動の手動確認、`docker compose exec comfyui curl http://127.0.0.1:11434/api/version`、ComfyUI Web UI 到達確認、README / quickstart 手順整合確認  
**Target Platform**: Linux 系ローカル Docker 環境、および RunPod Serverless GPU worker container  
**Project Type**: Docker runtime 統一 + compose orchestration 更新 + 運用ドキュメント整理  
**Performance Goals**: local と RunPod の runtime 差分を image と model path の観点でなくし、追加の手動 Ollama 起動なしに local `comfyui` 起動だけで ComfyUI と Ollama の両方を利用可能にする  
**Constraints**: 既存 `compose.yml` の `comfyui` service 名は維持する、local 独立 `ollama` service は廃止する、local でも `/runpod-volume` bind mount を必須にする、Ollama API は引き続きコンテナ内 localhost 前提、custom node 機能追加はしない  
**Scale/Scope**: 単一 compose project 内の `comfyui` runtime 1 系統、RunPod / local 共通 image 1 つ、host bind mount による 1 個の `/runpod-volume`

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は既存の 2 系統 runtime を 1 系統へ畳むための更新に限定している

## Project Structure

### Documentation (this feature)

```text
specs/044-local-runpod-image/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── local-runpod-runtime-contract.md
└── tasks.md
```

### Source Code (repository root)
```text
README.md
compose.yml
.env.example
comfyui/
├── Dockerfile
├── entrypoint.sh
├── install-custom-nodes.sh
├── .dockerignore
├── custom_node/
│   └── comfyui-photopainter-custom/
└── runpod/
    ├── Dockerfile
    ├── README.md
    └── start-ollama-worker.sh

specs/044-local-runpod-image/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
    └── local-runpod-runtime-contract.md
```

**Structure Decision**: 実装対象は local Compose 導線と RunPod 用 runtime asset の統合に限定する。`compose.yml` の `comfyui` service を `comfyui/runpod/Dockerfile` ベースへ切り替え、必要な local 向け mount / env だけを compose 側へ残す。旧 local 専用 `comfyui/Dockerfile` / `entrypoint.sh` / `install-custom-nodes.sh` は廃止または非推奨化の対象として整理し、runtime 実体は `comfyui/runpod/` へ一本化する。

## Phase 0: Research 成果物

→ [research.md](/workspaces/photopainter-updater/specs/044-local-runpod-image/research.md) 参照

## Phase 1: Design

### Runtime 統合設計

- `compose.yml` の `comfyui` service は名前を維持したまま、build context を `./comfyui`、Dockerfile を `runpod/Dockerfile` へ切り替える
- local では `/runpod-volume` bind mount を必須にし、ComfyUI model path は `/runpod-volume/models`、Ollama model path は `/runpod-volume/ollama/models` に固定する
- local の独立 `ollama` service は削除し、Ollama は `comfyui` コンテナ内で `start-ollama-worker.sh` により起動させる
- local の Ollama 確認は host 公開ではなく `docker compose exec comfyui curl http://127.0.0.1:11434/api/version` を標準手順にする
- `OLLAMA_PULL_MODELS` など RunPod image の起動時 env は local compose からも渡せるようにし、local / RunPod で同じ runtime contract を共有する
- 旧 local 専用 runtime 資産は保守導線から外し、README / quickstart / RunPod README も共通 image 前提へ更新する

### Migration 設計

- `.env.example` は `COMFYUI_DATA_DIR` / `OLLAMA_DATA_DIR` ベースの旧 path 依存を整理し、`/runpod-volume` bind mount に必要な host path 設定へ寄せる
- README では「ComfyUI を起動する = RunPod と同じ image を compose で起動する」と読める構成に書き換える
- `comfyui/runpod/README.md` は serverless 専用説明から、local / RunPod 共通 runtime の詳細説明へ役割変更する
- quickstart は local compose 起動、Ollama 疎通、RunPod build / run の両導線を同一 runtime の利用例として並べる

### Validation 設計

- `docker compose config` が `comfyui` 単独 service と `/runpod-volume` bind mount 前提で解決できる
- local で `docker compose up -d comfyui` 後、`http://localhost:${COMFYUI_PORT:-18188}` の ComfyUI Web UI 到達を確認できる
- `docker compose exec comfyui curl -fsS http://127.0.0.1:11434/api/version` が成功する
- local でも `OLLAMA_PULL_MODELS` 指定時に `model_result` ログを追える
- README / quickstart / RunPod README の説明から、旧 local 導線や独立 `ollama` service 前提を読み取れない

## Phase 1: Contracts

→ [contracts/local-runpod-runtime-contract.md](/workspaces/photopainter-updater/specs/044-local-runpod-image/contracts/local-runpod-runtime-contract.md) 参照

## Post-Design Constitution Check

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を維持している
- [x] Allowed Scope / Forbidden Scope を超える変更を計画していない
- [x] 文書成果物は日本語で記述している
- [x] 各主要要求に対する検証方法を plan / quickstart / contract に反映した
- [x] local と RunPod を同一 runtime へ寄せ、保守対象を減らす方向で最小構成を満たしている

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| なし | なし | なし |

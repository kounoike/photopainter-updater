# 実装計画: RunPod Ollama sidecar

**Branch**: `043-runpod-ollama-sidecar` | **Date**: 2026-04-06 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/043-runpod-ollama-sidecar/spec.md)  
**Input**: `/specs/043-runpod-ollama-sidecar/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

この feature では、既存のローカル向け `comfyui/Dockerfile` と `compose.yml` は壊さずに、RunPod serverless 用の ComfyUI worker カスタマイズ資産を別導線で追加する。RunPod の `worker-comfyui` をベースにした custom image を用意し、wrapper start script で `ollama serve` を localhost 限定で前置起動し、API 疎通確認後に upstream `/start.sh` へ制御を渡す。モデル保存先は RunPod Network Volume が接続されている場合は `/runpod-volume` 配下を使い、未接続時は一時領域へフォールバックする。起動時の事前取得モデルは単一設定値のカンマ区切り一覧で指定し、pull 失敗は warning として記録して worker 起動は継続する。さらに `worker-comfyui` の development 手順を踏まえたローカル擬似検証手順を整備し、`/runpod-volume` bind mount あり・なしの両系統を事前確認できるようにする。

## Technical Context

**Language/Version**: Dockerfile syntax、Bash、Python 3.x runtime（upstream `worker-comfyui` 同梱環境）  
**Primary Dependencies**: RunPod `worker-comfyui` upstream image / `/start.sh`、RunPod customization / network volume / development docs、Ollama Linux installer / `ollama serve` / `ollama pull`、既存 repo の `comfyui/Dockerfile` と `compose.yml`  
**Storage**: RunPod Network Volume (`/runpod-volume`)、Ollama model directory（`OLLAMA_MODELS` で指定）、Network Volume 未接続時のコンテナ内一時領域、repo 内の RunPod 用 Docker build assets と文書  
**Testing**: ローカル Docker build、`docker run` による worker 起動、`/runpod-volume` bind mount あり・なしでの手動確認、container 内 `curl` による Ollama API 疎通確認、RunPod local API 相当への test payload 投入、文書整合確認  
**Target Platform**: RunPod Serverless の GPU worker container、および Linux 系ローカル Docker 環境  
**Project Type**: serverless worker image customization + startup script orchestration + 運用ドキュメント更新  
**Performance Goals**: コンテナ起動後に追加の手動プロセス起動なしで Ollama API が利用可能であること、永続領域がある場合は再 pull なしでモデル再利用できること、ローカル擬似検証で本番前に起動経路を再現できること  
**Constraints**: 既存 `compose.yml` とローカル ComfyUI self-build 導線は壊さない、Ollama API は localhost 限定、`KEEP_ALIVE` はノード側制御のまま、pull 対象は単一設定値のカンマ区切り、pull 失敗は warning で継続、RunPod Network Volume 未接続時は一時領域フォールバックを許容する  
**Scale/Scope**: 単一 RunPod worker container、単一 Ollama sidecar process、0 個以上の事前 pull model、RunPod 本番とローカル擬似検証の 2 導線

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は既存ローカル ComfyUI 導線を維持したまま RunPod 用 asset を別導線で追加する範囲に留めている

## Project Structure

### Documentation (this feature)

```text
specs/043-runpod-ollama-sidecar/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── runpod-ollama-runtime-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
README.md
comfyui/
├── Dockerfile
├── entrypoint.sh
├── install-custom-nodes.sh
└── runpod/
    ├── Dockerfile
    ├── start-ollama-worker.sh
    └── README.md

compose.yml
```

**Structure Decision**: 既存の `comfyui/Dockerfile` と `compose.yml` はローカル Compose 導線として維持し、RunPod serverless 用 asset は `comfyui/runpod/` に分離する。RunPod 用 Dockerfile は upstream `runpod/worker-comfyui:<version>-base` を継承し、wrapper start script を追加して `ollama serve`、API wait、model pull、`/start.sh` への委譲を担当させる。これにより、ローカル向け runtime と RunPod 向け runtime の責務が混ざらず、既存導線の回帰を避けながら serverless 専用要件を追加できる。

## Phase 0: Research 成果物

→ [research.md](/workspaces/photopainter-updater/specs/043-runpod-ollama-sidecar/research.md) 参照

## Phase 1: Design

### Runtime 設計

- RunPod 用 image は `runpod/worker-comfyui:<version>-base` を継承し、upstream worker の `/start.sh` と handler 導線を再利用する
- RunPod 用 wrapper script は `ollama serve` を background 起動し、`http://127.0.0.1:11434/api/version` で疎通確認後に upstream `/start.sh` を `exec` する
- Ollama API は localhost 限定とし、外部 bind は行わない
- モデル保存先は `/runpod-volume` が存在し書き込み可能なら永続領域、そうでなければ一時領域へ切り替える
- モデル事前取得設定は `OLLAMA_PULL_MODELS` のような単一 env 値で受け取り、trim 後のカンマ区切り一覧として順次 `ollama pull` する
- model pull 失敗は warning を残して起動継続し、失敗モデル名をログで判別できるようにする
- `KEEP_ALIVE` は Dockerfile や wrapper script に固定せず、ComfyUI 側 node 入力で `0` を指定する既存運用へ委ねる
- 既存 repo の `comfyui/Dockerfile`、`comfyui/entrypoint.sh`、`compose.yml` は RunPod feature の副作用で壊さない

### Validation 設計

- ローカル Docker build で RunPod 用 image を作成できる
- `docker run` で worker を起動し、container 内 `curl http://127.0.0.1:11434/api/version` が成功する
- `/runpod-volume` bind mount ありのローカル起動で、永続保存先が選ばれ、model pull 後の再起動でもモデル再利用前提を確認できる
- `/runpod-volume` bind mount なしのローカル起動で、一時領域フォールバックと warning / mode 表示を確認できる
- `worker-comfyui` development 導線に沿って test payload を送ったとき、ComfyUI worker と同居 Ollama の起動経路が壊れていない
- 文書から RunPod 本番設定、Network Volume 前提、localhost 制約、ローカル擬似検証手順を追える

## Phase 1: Contracts

→ [contracts/runpod-ollama-runtime-contract.md](/workspaces/photopainter-updater/specs/043-runpod-ollama-sidecar/contracts/runpod-ollama-runtime-contract.md) 参照

## Post-Design Constitution Check

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を維持している
- [x] Allowed Scope / Forbidden Scope を超える変更を計画していない
- [x] 文書成果物は日本語で記述している
- [x] 各主要要求に対する検証方法を plan / quickstart / contract に反映した
- [x] 既存のローカル Compose / ComfyUI 導線を温存し、RunPod 用 asset 分離で最小構成を維持している

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| なし | なし | なし |

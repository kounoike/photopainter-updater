# 実装計画: ComfyUI custom node 同梱コンテナ

**Branch**: `031-bake-custom-node` | **Date**: 2026-04-04 | **Spec**: [spec.md](./spec.md)  
**Input**: `/specs/031-bake-custom-node/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

既存の ComfyUI self-build runtime は repo 管理 custom node を bind mount で見せているが、この方式では runtime 依存が残り、container を作る時点で「custom node 入りの状態」を固定できない。今回の feature では `comfyui/custom_node/` 配下の repo 管理 node に加え、選定済み third-party custom node 4 件を ComfyUI image へ焼き込み、`docker compose build comfyui` 時点で同梱された runtime を生成する。追加 custom node の永続維持は今回不要なので、`${COMFYUI_DATA_DIR}/custom_nodes` の互換は求めず、repo 管理 node と pinned ref の third-party node だけを baked-in 対象として扱う。これにより、`docker compose up -d comfyui`、`restart`、`down && up` の全てで、同じ custom node 構成へ image 由来で復帰できる。

## Technical Context

**Language/Version**: Docker Compose v2 YAML、Dockerfile syntax、Bash、Python 3.13、既存 ComfyUI runtime  
**Primary Dependencies**: 既存 `compose.yml`、既存 `comfyui/Dockerfile`、既存 `comfyui/entrypoint.sh`、`uv`、ComfyUI upstream manual install 手順、repo 管理 `comfyui/custom_node/comfyui-photopainter-custom`、`ComfyUI-Manager`、`ComfyUI-Easy-Use`、`comfyui-ollama`、`ComfyUI-Xz3r0-Nodes`  
**Storage**: bind mount（`${COMFYUI_DATA_DIR:-./comfyui-data}` 配下の models / output / input / user 設定）、image に焼き込む repo 管理 custom node と pinned third-party custom node、`.env.example`  
**Testing**: `docker compose config`、`docker compose build comfyui`、`docker compose up -d comfyui`、`docker compose restart comfyui`、`docker compose down && docker compose up -d comfyui`、node discovery と README / quickstart 整合確認、build ログで third-party 依存導入確認  
**Target Platform**: Docker Engine + Docker Compose v2 + NVIDIA GPU が使えるローカル Linux 系環境  
**Project Type**: Compose 設定更新 + ComfyUI image build 資産更新 + 運用ドキュメント更新  
**Performance Goals**: repo 管理 custom node を runtime mount なしで利用可能にしつつ、既存の利用開始導線と再現性を維持する  
**Constraints**: `comfyui` service 名と公開 URL は維持する、repo 管理 custom node と選定済み third-party custom node は image 同梱とする、追加 custom node の永続維持は対象外とする、追加 service は導入しない、NVIDIA/CUDA 前提を崩さない、tag がある repo は tag 固定を優先する  
**Scale/Scope**: 単一ホスト・単一 GPU・単一 `comfyui` service・repo 管理 custom node 1 系統と third-party custom node 4 系統の同梱

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は既存 ComfyUI runtime と entrypoint の責務整理に留めている

**Phase 1 再確認（Design 後）**:
- [x] 追加 service を導入せず、既存 `comfyui` service の build/runtime 調整に限定している
- [x] repo 管理 custom node と追加 custom node の責務境界を明示している
- [x] 検証手順は build、起動、再起動、再作成、node 共存確認の 5 観点を持つ

## Project Structure

### Documentation (this feature)

```text
specs/031-bake-custom-node/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── comfyui-baked-custom-node-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
compose.yml
.env.example
README.md
comfyui/
├── Dockerfile
├── entrypoint.sh
└── custom_node/
    └── comfyui-photopainter-custom/
```

**Structure Decision**: repo 管理 custom node は `comfyui/custom_node/` 配下に残し、`comfyui/Dockerfile` で image へ copy する。third-party custom node 4 件は Dockerfile 内で pinned ref を clone し、各 `requirements.txt` を `uv pip install --system` で導入する。`ComfyUI-Xz3r0-Nodes` 向けに `ffmpeg` を image へ追加する。`compose.yml` では repo 管理 node 用 bind mount と `${COMFYUI_DATA_DIR}/custom_nodes` mount を外し、`comfyui/entrypoint.sh` は baked-in node を前提とした最小処理へ保つ。README と quickstart は「repo 管理 node 更新時は rebuild、追加 custom node は維持対象外、third-party node は pinned ref で同梱」という運用へ統一する。

## Phase 0: Research 成果物

→ [research.md](./research.md) 参照

## Phase 1: Design

### Runtime 設計

- repo 管理 custom node は `comfyui/Dockerfile` の build context に含め、ComfyUI image 内の `custom_nodes` 配下へ copy する
- `ComfyUI-Manager`、`ComfyUI-Easy-Use`、`comfyui-ollama`、`ComfyUI-Xz3r0-Nodes` は Dockerfile 内で pinned ref を clone し、ComfyUI image 内の `custom_nodes` 配下へ配置する
- third-party node のうち tag がある repo は stable tag 固定、tag がない `comfyui-ollama` は commit 固定にする
- third-party node の `requirements.txt` は build 時に導入し、`ComfyUI-Xz3r0-Nodes` 向けに `ffmpeg` を apt で追加する
- `compose.yml` から repo 管理 custom node の bind mount は外す
- `${COMFYUI_DATA_DIR}/custom_nodes` の bind mount は削除し、repo 管理 node だけを baked-in 対象とする
- `comfyui/entrypoint.sh` は repo 管理 baked-in node をそのまま使う前提で最小化する
- repo 管理 custom node 更新時は rebuild が必要であることを文書化する
- `README.md`、feature quickstart、custom node README は baked-in node と追加 custom node の扱いを同じ用語で説明する
- root README と feature quickstart は同梱 third-party node 一覧、pinned ref 方針、`comfyui-ollama` の Ollama 接続前提を説明する

### Validation 設計

- `docker compose config` で repo 管理 custom node の bind mount と `custom_nodes` 永続 mount が消えている
- `docker compose build comfyui` 後、container 作成直後から `PhotoPainter PNG POST` と同梱 third-party node が見える
- `docker compose restart comfyui` 後も baked-in node が見える
- `docker compose down && docker compose up -d comfyui` 後も同じ node 構成へ復帰できる
- README / quickstart / custom node README の全てで、repo 管理 node は rebuild、追加 custom node は維持対象外という運用が説明されている

## Phase 1: Contracts

→ [contracts/comfyui-baked-custom-node-contract.md](./contracts/comfyui-baked-custom-node-contract.md) 参照

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| なし | なし | なし |

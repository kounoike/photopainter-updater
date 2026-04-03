# 実装計画: ComfyUI 自前イメージ構築

**Branch**: `030-build-comfyui-image` | **Date**: 2026-04-04 | **Spec**: [spec.md](./spec.md)  
**Input**: `/specs/030-build-comfyui-image/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

現在の `comfyui` service は公開 image 前提で起動しており、container 内の状態変化や再作成時の差分が環境の不安定さにつながっている。これを repo 管理の Dockerfile から build する方式へ置き換え、ComfyUI 実行環境の基準を repo 側へ戻す。`docker compose up -d comfyui` という既存導線、既存の `COMFYUI_PORT`、`COMFYUI_DATA_DIR`、repo 管理 custom node mount、Ollama/HTTP サーバ/AI Toolkit との共存は維持しつつ、初回 build、再 build、再起動、再作成の手順を README と quickstart で再現可能にする。

## Technical Context

**Language/Version**: Docker Compose v2 YAML、Dockerfile syntax、Bash（補助手順）、既存 ComfyUI runtime  
**Primary Dependencies**: 既存 `compose.yml`、新規 `comfyui/Dockerfile`、既存 `comfyui/custom_node/comfyui-photopainter-custom`、Docker BuildKit、NVIDIA Container Toolkit  
**Storage**: bind mount（`${COMFYUI_DATA_DIR:-./comfyui-data}` 配下）、repo 内 `comfyui/` build context、`.env.example`  
**Testing**: `docker compose config`、`docker compose build comfyui`、`docker compose up -d comfyui`、UI 到達確認、`docker compose restart comfyui` / `docker compose down && docker compose up -d comfyui` の手動確認、README / quickstart 整合確認  
**Target Platform**: Docker Engine + Docker Compose v2 + NVIDIA GPU が使えるローカル Linux 系環境  
**Project Type**: Compose 設定更新 + ComfyUI image build 資産追加 + 運用ドキュメント更新  
**Performance Goals**: 既存の ComfyUI 利用開始導線を維持しつつ、再起動・再作成後も追加の場当たり修正なしで利用再開できること  
**Constraints**: `comfyui` service 名と Web UI 到達方法は維持する、既存 `COMFYUI_DATA_DIR` の主要保存先を継続利用する、repo 管理 custom node 導線を壊さない、複雑な外部オーケストレーションは導入しない、runtime の可変状態を image build へ寄せる  
**Scale/Scope**: 単一ホスト・単一 GPU マシン・単一 `comfyui` service の運用改善

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は単一 Dockerfile と既存 compose 更新の範囲に留めている

**Phase 1 再確認（Design 後）**:
- [x] 追加サービスは導入せず、既存 `comfyui` service の build 方式変更に限定している
- [x] 既存データ保存先と既存 compose 導線を維持し、運用複雑化を抑えている
- [x] 検証手順は build、起動、再起動、再作成、既存導線維持の 5 観点を持つ

## Project Structure

### Documentation (this feature)

```text
specs/030-build-comfyui-image/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── comfyui-self-build-runtime-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
compose.yml
.env.example
README.md
comfyui/
├── Dockerfile
├── custom_node/
│   └── comfyui-photopainter-custom/
└── ...
specs/
└── 022-add-comfyui-compose/
```

**Structure Decision**: ComfyUI 用 image build 資産は `comfyui/` 配下へ集約し、compose の build context もそこへ寄せる。repo ルートには既存どおり `compose.yml` と README を置き、利用者の起動入口は `docker compose` に統一する。既存の `COMFYUI_DATA_DIR` はそのまま親ディレクトリとして使い、`models`、`custom_nodes`、`output`、`user`、`input`、`dot-cache`、`dot-local` を継続利用対象として扱う。

## Phase 0: Research 成果物

→ [research.md](./research.md) 参照

## Phase 1: Design

### Runtime 設計

- `compose.yml` の `comfyui` service は `image:` 直指定ではなく repo 管理 Dockerfile の `build:` を使う
- build 対象 image は再現性のある upstream base を土台にしつつ、repo 管理の初期構成を Dockerfile へ閉じ込める
- `docker compose up -d comfyui` で build 済み image を起動でき、必要に応じて `docker compose build comfyui` で明示再 build できるようにする
- 既存の `COMFYUI_PORT`、`COMFYUI_DATA_DIR`、GPU 設定、healthcheck、`photopainter` network、`depends_on: ollama` は互換条件として扱う
- repo 管理 custom node は引き続き runtime から見えるようにし、既存の利用者 custom node 全体保存先も保持する
- README / quickstart は pull 前提ではなく build、再 build、再起動、再作成の導線へ更新する

### Validation 設計

- `docker compose config` で `comfyui` service 定義が解決される
- `docker compose build comfyui` が通り、repo 管理 image が生成される
- `docker compose up -d comfyui` 後に既存 URL で UI 到達可否を判断できる
- `docker compose restart comfyui` と `docker compose down && docker compose up -d comfyui` 後も同じ起動導線で復帰できる
- repo 管理 custom node と既存保存先の導線が維持される
- README / quickstart / feature 成果物が同じ運用導線を説明している

## Phase 1: Contracts

→ [contracts/comfyui-self-build-runtime-contract.md](./contracts/comfyui-self-build-runtime-contract.md) 参照

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| なし | なし | なし |

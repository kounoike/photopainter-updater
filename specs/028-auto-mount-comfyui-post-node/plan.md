# 実装計画: ComfyUI custom node 自動登録

**Branch**: `028-auto-mount-comfyui-post-node` | **Date**: 2026-04-03 | **Spec**: [spec.md](./spec.md)  
**Input**: `/specs/028-auto-mount-comfyui-post-node/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

既存の ComfyUI compose 構成を保ったまま、repo 管理の `comfyui/custom_node/comfyui-photopainter-custom` を `compose.yml` の volume として ComfyUI container の `custom_nodes` 探索先へ自動 mount する。これにより、`docker compose up -d comfyui` だけで `PhotoPainter PNG POST` node が利用可能になる。既存の `${COMFYUI_DATA_DIR}/custom_nodes` 全体 mount と ComfyUI Manager の運用は維持し、README と quickstart の manual copy 導線を自動登録導線へ置き換える。

## Technical Context

**Language/Version**: Docker Compose v2 YAML、Markdown  
**Primary Dependencies**: 既存 `compose.yml`、既存 `yanwk/comfyui-boot:cu128-slim`、既存 repo 配下 `comfyui/custom_node/comfyui-photopainter-custom`  
**Storage**: bind mount（repo 内 custom node ディレクトリ、`${COMFYUI_DATA_DIR:-./comfyui-data}/custom_nodes`）  
**Testing**: `docker compose config`、ComfyUI 起動後の node discovery 手動確認、README / quickstart 整合確認  
**Target Platform**: Docker Compose で起動するローカル ComfyUI 環境  
**Project Type**: Compose 設定更新 + 導入手順ドキュメント  
**Performance Goals**: 既存 ComfyUI 起動導線に追加の手動導入ステップを増やさない  
**Constraints**: 既存 `custom_nodes` 全体 mount を維持する、ComfyUI Manager を壊さない、新規 service を追加しない、repo 側 node 更新は再起動で反映できること  
**Scale/Scope**: 単一 compose ファイル、単一 custom node の自動登録

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されていない

**Phase 1 再確認（Design 後）**:
- [x] compose 既存 service を維持し、追加 service は導入しない
- [x] custom node 自体の機能には手を入れない
- [x] 成功導線、既存 custom_nodes 併存、文書導線の 3 観点で検証手順を持つ

## Project Structure

### Documentation (this feature)

```text
specs/028-auto-mount-comfyui-post-node/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── comfyui-custom-node-mount-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
compose.yml
README.md
comfyui/
└── custom_node/
    └── comfyui-photopainter-custom/

specs/027-comfyui-post-node/quickstart.md
```

**Structure Decision**: 自動登録は `compose.yml` の volume 追加だけで実現し、repo 内 custom node ソースは引き続き `comfyui/custom_node/comfyui-photopainter-custom` に置く。導線の置換は root README と 027 の quickstart へ反映する。

## Phase 0: Research 成果物

→ [research.md](./research.md) 参照

## Phase 1: Design

### Mount 設計

- 既存 volume:
  - `${COMFYUI_DATA_DIR:-./comfyui-data}/custom_nodes:/root/ComfyUI/custom_nodes`
- 追加 volume:
  - `./comfyui/custom_node/comfyui-photopainter-custom:/root/ComfyUI/custom_nodes/comfyui-photopainter-custom:ro`

この構成により、既存 `custom_nodes` ディレクトリ全体はそのまま維持しつつ、PhotoPainter node だけを子 path に追加 bind mount する。

### Validation 設計

- `docker compose config` で volume 定義が正しく解決される
- ComfyUI 起動後に `PhotoPainter PNG POST` が Add Node で見える
- `comfyui-data/custom_nodes` 直下の既存 node が消えない
- 文書から manual copy 手順を除去し、自動登録導線へ統一する

## Phase 1: Contracts

→ [contracts/comfyui-custom-node-mount-contract.md](./contracts/comfyui-custom-node-mount-contract.md) 参照

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| なし | なし | なし |

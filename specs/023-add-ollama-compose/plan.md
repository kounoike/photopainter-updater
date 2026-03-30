# 実装計画: Ollama Docker Compose 追加

**Branch**: `023-add-ollama-compose` | **Date**: 2026-03-30 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/023-add-ollama-compose/spec.md)  
**Input**: `/specs/023-add-ollama-compose/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

既存の `compose.yml` に `ollama/ollama` サービスを追加し、ComfyUI と同じ `photopainter` ネットワーク上で管理する。Ollama はホストへポート公開せず、Compose 内ネットワーク専用 API として扱う。モデル保存領域は既存 ComfyUI と同様に bind mount ベースでホスト側へ永続化し、`.env.example` と `README.md`、feature 配下の手順書を同期して、起動・疎通・再利用手順を再現可能にする。

## Technical Context

**Language/Version**: Docker Compose v2、YAML、Markdown  
**Primary Dependencies**: Docker Engine / Docker Compose v2、公式イメージ `ollama/ollama`、既存 `yanwk/comfyui-boot:cu128-slim`  
**Storage**: ホスト bind mount ディレクトリ（`./comfyui-data`、新規 `./ollama-data`）  
**Testing**: `docker compose config`、`docker compose up -d ollama`、`docker compose exec ollama ollama list`、`docker compose up -d`、`docker compose exec comfyui curl -fsS http://ollama:11434/api/version` による手動確認  
**Target Platform**: ローカル Linux 開発環境、単一ホスト上の Docker Compose 運用  
**Project Type**: Docker Compose ベースのローカル運用構成 + ドキュメント更新  
**Performance Goals**: 5 分以内に起動確認できること、コンテナ再作成後もモデル再取得が不要であること  
**Constraints**: Ollama は外部公開しない、既存 ComfyUI の利用手順を壊さない、複雑な追加サービスやオーケストレーションは導入しない  
**Scale/Scope**: 単一 compose ファイル、サービス 2 本（ComfyUI + Ollama）、ローカル利用者 1 名を想定

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されている

Phase 1 再確認結果:

- [x] 追加設計は `compose.yml`、`.env.example`、`README.md`、feature 成果物に限定されている
- [x] Ollama を内部ネットワーク専用にすることでローカル優先・運用単純性を維持している
- [x] 検証手順は起動、内部疎通、永続化、既存 ComfyUI 維持の 4 観点を満たしている

## Project Structure

### Documentation (this feature)

```text
specs/023-add-ollama-compose/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── ollama-compose-runtime-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
.
├── compose.yml
├── .env.example
├── README.md
├── server/
├── firmware/
└── specs/
    ├── 022-add-comfyui-compose/
    └── 023-add-ollama-compose/
```

**Structure Decision**: 既存の単一 `compose.yml` を維持し、その中に Ollama サービスを追加する。設定変更は `.env.example` の追加変数で吸収し、運用説明はルート `README.md` と feature 配下の `quickstart.md` に分離する。アプリ本体や firmware には触れない。

## Phase 0: Research Summary

詳細は [research.md](/workspaces/photopainter-updater/specs/023-add-ollama-compose/research.md) を参照。

- 公式イメージは `ollama/ollama` を採用する
- 永続化は既存 Compose 方針に合わせて host bind mount を採用する
- 公開方式はポート公開なし、`photopainter` ネットワーク内通信に限定する
- GPU は初期実装の必須要件にせず、CPU ベースでも起動可能な最小構成を優先する

## Phase 1: Design Artifacts

### Data Model

詳細は [data-model.md](/workspaces/photopainter-updater/specs/023-add-ollama-compose/data-model.md) を参照。

- `.env` 追加変数
  - `OLLAMA_DATA_DIR`
- Compose service
  - `ollama`
  - `container_name`
  - bind mount to `/root/.ollama`
  - `photopainter` network 参加
  - ホストへの `ports` は設定しない

### Contracts

詳細は [ollama-compose-runtime-contract.md](/workspaces/photopainter-updater/specs/023-add-ollama-compose/contracts/ollama-compose-runtime-contract.md) を参照。

- `compose.yml` 上で保証するサービス名、永続化先、ネットワーク可視性、検証手順を固定する
- 既存 ComfyUI の `ports` / GPU 設定 / healthcheck を壊さないことを互換条件として扱う

### Quickstart

詳細は [quickstart.md](/workspaces/photopainter-updater/specs/023-add-ollama-compose/quickstart.md) を参照。

- `.env` 準備
- Ollama データディレクトリ準備
- `docker compose up -d ollama`
- `docker compose exec ollama ollama list`
- モデル pull と再作成後の保持確認
- ComfyUI からの `curl http://ollama:11434/api/version` は共存確認で使う

## Implementation Strategy

1. `compose.yml` に `ollama` サービスを追加する
2. `.env.example` に `OLLAMA_DATA_DIR` を追加する
3. `README.md` に Ollama 利用導線を追記する
4. 実運用・検証手順を `quickstart.md` に記録する
5. 実装後は `docker compose config` と起動確認で回帰を潰す

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| なし | - | - |

# 実装計画: ComfyUI Docker Compose 統合

**Branch**: `022-add-comfyui-compose` | **Date**: 2026-03-30 | **Spec**: [spec.md](./spec.md)  
**Input**: `/specs/022-add-comfyui-compose/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

`yanwk/comfyui-boot:cu128-slim` を使用した ComfyUI Docker Compose 環境を構築する。リポジトリルートに `compose.yml` を配置し、NVIDIA GPU 必須構成でモデル・カスタムノード・pip 依存ライブラリ・出力を bind mount で永続化する。ポートはデフォルト `18188`（ホスト側）、ホストのボリュームパスは `.env` で設定可能。将来の HTTP サーバ統合を見越した名前付きネットワーク定義を含める。Docker Compose ベストプラクティス調査結果は `research.md` にまとめ、設計根拠として参照する。

## Technical Context

**Language/Version**: YAML (Docker Compose v2 / `compose.yml` 形式)  
**Primary Dependencies**: `yanwk/comfyui-boot:cu128-slim`、Docker Compose v2、NVIDIA Container Toolkit  
**Storage**: bind mount（ホスト側任意ディレクトリ、`.env` で指定）  
**Testing**: 手動検証（`docker compose up` → ブラウザで `http://localhost:18188` アクセス確認）  
**Target Platform**: Linux + NVIDIA GPU（NVIDIA Container Toolkit インストール済み）  
**Project Type**: Docker Compose 設定ファイル + 運用ドキュメント  
**Performance Goals**: イメージ pull 済み環境で起動後 30 秒以内に Web UI アクセス可能  
**Constraints**: NVIDIA GPU 必須・CPU フォールバックなし・LAN 内運用  
**Scale/Scope**: 単一ホスト・単一 ComfyUI インスタンス（複数インスタンスはポート変更で対応）

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
  - Allowed: `compose.yml`、`.env.example`、`.gitignore` 更新、`specs/022-*` 成果物
  - Forbidden: HTTP サーバ統合（将来フィーチャー）、既存ファームウェア・Rust サーバへの変更
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されている

**Phase 1 再確認（Design 後）**:
- [x] 追加の外部依存なし（公開イメージを使用、独自ビルドなし）
- [x] 設定は `.env` ファイル 1 枚で完結、複雑な設定管理ツールは不使用
- [x] ネットワーク定義は将来のサービス追加に対して最小変更で対応可能

## Project Structure

### Documentation (this feature)

```text
specs/022-add-comfyui-compose/
├── plan.md              # このファイル
├── research.md          # Phase 0 成果物（ベストプラクティス調査）
├── data-model.md        # Phase 1 成果物（ボリューム・環境変数・ネットワーク設計）
├── quickstart.md        # Phase 1 成果物（起動手順）
└── tasks.md             # Phase 2 成果物（/speckit.tasks）
```

### Source Code (repository root)

```text
compose.yml              # ComfyUI サービス定義（新規作成）
.env.example             # 環境変数テンプレート（新規作成）
.gitignore               # .env と comfyui-data/ を追加
```

**Structure Decision**: Docker Compose 設定ファイルはリポジトリルートに配置する（clarify で確定）。データディレクトリ（`comfyui-data/` 等）はホスト側任意ディレクトリで `.env` で指定するため、リポジトリ内に含まない。

## Phase 0: Research 成果物

→ [research.md](./research.md) 参照

**主要決定事項**:
| 決定事項 | 採用内容 | 根拠 |
|---------|---------|------|
| イメージ | `yanwk/comfyui-boot:cu128-slim` | ローカル開発向け設計・dockerful design・毎日ビルド |
| GPU 設定 | `deploy.resources.reservations.devices` | Docker Compose v2 の推奨方式 |
| 必須ボリューム | models / custom_nodes / .local / output | カスタムノード依存含む永続化 |
| 推奨ボリューム | user / input / .cache | 設定・キャッシュ永続化 |
| ネットワーク | 名前付き bridge `photopainter` | 将来のサービス追加に最小変更で対応 |
| ポート | `0.0.0.0:${COMFYUI_PORT:-18188}:8188` | clarify 確定値 |
| restart | `unless-stopped` | 開発・実験用途に適切 |
| healthcheck | `/system_stats` エンドポイント利用 | ComfyUI 標準 API を活用 |

## Phase 1: Design

### compose.yml 設計

```yaml
services:
  comfyui:
    image: yanwk/comfyui-boot:cu128-slim
    container_name: photopainter-comfyui
    ports:
      - "0.0.0.0:${COMFYUI_PORT:-18188}:8188"
    volumes:
      # 必須: モデル・カスタムノード・依存・出力
      - ${COMFYUI_DATA_DIR:-./comfyui-data}/models:/root/ComfyUI/models
      - ${COMFYUI_DATA_DIR:-./comfyui-data}/custom_nodes:/root/ComfyUI/custom_nodes
      - ${COMFYUI_DATA_DIR:-./comfyui-data}/dot-local:/root/.local
      - ${COMFYUI_DATA_DIR:-./comfyui-data}/output:/root/ComfyUI/output
      # 推奨: ユーザー設定・入力・キャッシュ
      - ${COMFYUI_DATA_DIR:-./comfyui-data}/user:/root/ComfyUI/user
      - ${COMFYUI_DATA_DIR:-./comfyui-data}/input:/root/ComfyUI/input
      - ${COMFYUI_DATA_DIR:-./comfyui-data}/dot-cache:/root/.cache
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: all
              capabilities: [gpu]
    environment:
      - NVIDIA_VISIBLE_DEVICES=all
      - NVIDIA_DRIVER_CAPABILITIES=compute,utility
      - CLI_ARGS=${COMFYUI_CLI_ARGS:---fast}
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8188/system_stats"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 60s
    networks:
      - photopainter

networks:
  photopainter:
    driver: bridge
```

### .env.example 設計

```dotenv
# ComfyUI ポート（ホスト側、デフォルト 18188）
COMFYUI_PORT=18188

# データディレクトリ（モデル・カスタムノード・出力の親ディレクトリ）
# 絶対パスまたは compose.yml 相対パスで指定可能
# 例: COMFYUI_DATA_DIR=/mnt/data/comfyui
COMFYUI_DATA_DIR=./comfyui-data

# ComfyUI 起動フラグ（省略時: --fast）
# --lowvram: VRAM 不足時に分割処理
# --cpu: CPU のみ使用（非常に遅い）
# COMFYUI_CLI_ARGS=--fast
```

### ディレクトリ構成（デフォルト）

```text
./comfyui-data/          ← COMFYUI_DATA_DIR のデフォルト値（.gitignore 対象）
├── models/              ← /root/ComfyUI/models
├── custom_nodes/        ← /root/ComfyUI/custom_nodes
├── dot-local/           ← /root/.local（pip --user ライブラリ）
├── output/              ← /root/ComfyUI/output
├── user/                ← /root/ComfyUI/user
├── input/               ← /root/ComfyUI/input
└── dot-cache/           ← /root/.cache（pip/HF/torch キャッシュ）
```

### .gitignore 追加項目

```gitignore
# ComfyUI Docker データ
.env
comfyui-data/
```

## Phase 1: Contracts

本フィーチャーの成果物は Docker Compose 設定ファイルであり、外部 API や HTTP エンドポイントを新たに定義するものではない。ComfyUI 自体が提供する Web UI（ポート 18188）は上流イメージの仕様に従う。

将来の HTTP サーバとの統合インターフェースは次フィーチャーで定義するため、本フィーチャーでは contracts/ ディレクトリを作成しない。

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| 外部公開イメージへの依存 | ComfyUI 本体の Docker 化は複雑すぎて独自維持不可 | 独自 Dockerfile は PyTorch/CUDA 依存管理が困難で保守コスト高 |


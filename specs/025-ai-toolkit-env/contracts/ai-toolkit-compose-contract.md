# 実行コントラクト: AI Toolkit Compose 試用環境

**Branch**: `025-ai-toolkit-env` | **Date**: 2026-04-01

## 目的

既存 `compose.yml` 上の ComfyUI と Ollama を AI Toolkit 試用環境として見せる際、利用者が期待してよい入口、確認方法、既存導線との互換条件を固定する。

## 維持する既存インターフェース

| 項目 | 契約 |
|------|------|
| `comfyui` service 名 | 変更しない |
| `ollama` service 名 | 変更しない |
| `COMFYUI_PORT` | 既存どおり ComfyUI Web UI のホスト公開に使う |
| `COMFYUI_DATA_DIR` / `OLLAMA_DATA_DIR` | 既存どおりホスト側永続化先として扱う |
| `README.md` の ComfyUI / Ollama 個別導線 | 破壊しない |
| `photopainter` network | 既存どおり共有 bridge network として維持する |

## AI Toolkit として追加で見せるインターフェース

### 利用者入口

| 項目 | 契約 |
|------|------|
| 入口文書 | ルート `README.md` に AI Toolkit 試用環境の入口を追加する |
| 詳細手順 | `specs/025-ai-toolkit-env/quickstart.md` に置く |
| 正式導線 | Docker Compose 起動を中心とする |
| 土台サービス | ComfyUI と Ollama を明示する |

### 試用成功判定

| 項目 | 契約 |
|------|------|
| 最小成功条件 | 主要サービス起動後、代表操作 1 件を成功できること |
| 代表操作 | `docker compose exec comfyui curl -fsS http://ollama:11434/api/version` を使い、README と quickstart で同じ説明を使う |
| 復帰方針 | `compose-state`、`env-config`、`persistent-data` の 3 系統で案内する |

## 利用契約

- 利用者は `.env.example` を `.env` へコピーして試用を開始できること
- 利用者は `docker compose up -d` または明示された同等コマンドで主要サービスを起動できること
- 利用者は README から quickstart へ移動し、代表操作 1 件の成功可否を自力で判断できること
- AI Toolkit を使わない利用者も、従来どおり ComfyUI または Ollama の単独導線を参照できること

## 検証契約

最低限、以下の確認が再現できること。

1. `docker compose config` が成功する
2. 主要サービスが Compose 上で起動できる
3. 利用者が状態確認コマンドまたは同等の観測方法で起動状態を把握できる
4. `docker compose exec comfyui curl -fsS http://ollama:11434/api/version` の成功可否を README / quickstart の手順どおりに判断できる
5. 失敗時に `compose-state`、`env-config`、`persistent-data` のいずれかから次の確認先を特定できる

## 非目標

- 新しい本番運用向け AI サービス基盤の追加
- `firmware/` や `server/` 本体への AI Toolkit 統合実装
- 既存 ComfyUI / Ollama 導線の置き換え
- クラウド依存の新規追加や複雑なオーケストレーション導入

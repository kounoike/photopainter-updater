# 実行コントラクト: AI Toolkit Compose 試用環境

**Branch**: `025-ai-toolkit-env` | **Date**: 2026-04-01

## 目的

`ostris/ai-toolkit` をこのリポジトリの `compose.yml` へ追加する際、利用者が期待してよい service 名、起動方法、Web UI 到達条件、既存導線との互換性を固定する。

## 維持する既存インターフェース

| 項目 | 契約 |
|------|------|
| `comfyui` service 名 | 変更しない |
| `ollama` service 名 | 変更しない |
| `COMFYUI_PORT` | 既存どおり ComfyUI Web UI 公開に使う |
| `README.md` の ComfyUI / Ollama 個別導線 | 破壊しない |
| `photopainter` network | 既存どおり共有 bridge network として維持する |

## AI Toolkit として追加するインターフェース

### `ai-toolkit` service

| 項目 | 契約 |
|------|------|
| service 名 | `ai-toolkit` |
| image | `ostris/aitoolkit:latest` |
| ports | Web UI 到達用ポートを公開する |
| volumes | config / datasets / output / DB / cache の保存先を持つ |
| restart | `unless-stopped` 相当を維持する |
| GPU 前提 | 既存 compose 方針に沿って明示する |

### `.env` 入口

| 項目 | 契約 |
|------|------|
| UI ポート | `.env.example` に入口を持つ |
| 認証設定 | `AI_TOOLKIT_AUTH` を案内する |
| 保存先 | AI Toolkit 用保存先の説明を持つ |

## 利用契約

- 利用者は `.env.example` を `.env` へコピーして AI Toolkit を起動できること
- 利用者は `docker compose up -d ai-toolkit` で AI Toolkit を個別起動できること
- 利用者は README から quickstart へ移動し、Web UI 到達可否を自力で判断できること
- AI Toolkit を使わない利用者も、従来どおり ComfyUI または Ollama の単独導線を参照できること

## 検証契約

最低限、以下の確認が再現できること。

1. `docker compose config` が成功する
2. `docker compose up -d ai-toolkit` でサービス起動できる
3. `docker compose ps ai-toolkit` または同等手段で起動状態を把握できる
4. 利用者が指定 URL へアクセスして Web UI 到達可否を判断できる
5. 再起動後も保存先の参照位置が維持される

## 非目標

- AI Toolkit 自体のソース改変
- AI Toolkit の学習ジョブ設計やモデル選定の最適化
- `firmware/` や `server/` 本体との統合
- 既存 ComfyUI / Ollama 導線の置換

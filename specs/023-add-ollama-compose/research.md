# 調査メモ: Ollama Docker Compose 追加

**Phase 0 成果物** | Branch: `023-add-ollama-compose` | Date: 2026-03-30

## 1. 公式イメージと保存先

**Decision**: 公式 Docker イメージ `ollama/ollama` を使い、モデル保存領域は `/root/.ollama` をホスト bind mount で永続化する。  
**Rationale**: Ollama 公式の Docker 案内では `ollama/ollama` と `/root/.ollama` への保存が基本形として示されている。既存 ComfyUI も bind mount で運用しているため、同じ管理方針に揃えた方が利用者の理解コストが低い。  
**Alternatives considered**:
- Docker named volume: Compose 記述は短いが、既存の `COMFYUI_DATA_DIR` 系と揃わず、保存先の見通しが悪くなるため不採用
- 独自 Dockerfile: 今回は公式イメージで要件を満たせるため過剰

Source:
- https://ollama.com/blog/ollama-is-now-available-as-an-official-docker-image

## 2. 接続公開範囲

**Decision**: Ollama は `photopainter` ネットワーク内専用とし、ホストへの `ports` 公開は行わない。  
**Rationale**: clarify で外部公開方法を決めない方針が確定している。内部サービスとして扱えば、不要な API 露出を避けつつ、将来別サービスから `http://ollama:11434` で参照できる。  
**Alternatives considered**:
- `localhost:11434` へ公開: 利便性は上がるが、今回の明示判断に反する
- LAN 公開: 現スコープでは不要で、セキュリティ面の前提も増えるため不採用

## 3. GPU 方針

**Decision**: 初期実装では Ollama の GPU 利用を必須要件にしない。まずは CPU でも起動可能な最小構成を定義し、GPU 利用は将来の拡張余地として文書で触れるに留める。  
**Rationale**: spec は GPU 利用の有無よりも Compose 統合、永続化、ComfyUI 共存を重視している。ComfyUI は NVIDIA 前提だが、Ollama まで同じ要件を初手で強制すると利用環境を狭める。  
**Alternatives considered**:
- Ollama も常に NVIDIA GPU を要求: ComfyUI と GPU 競合しやすく、最小構成から外れる
- GPU/CPU を `.env` で切替可能にする: 将来価値はあるが、今回の feature には設定分岐が多い

## 4. 疎通確認方法

**Decision**: 疎通確認は 2 段階に分ける。サービス単体確認には `docker compose exec ollama ollama list` を使い、Compose 内ネットワーク確認には `docker compose exec comfyui curl -fsS http://ollama:11434/api/version` を使う。  
**Rationale**: spec 上の必須は Compose 内ネットワークでの到達確認である。既存 ComfyUI イメージは `curl` を健康診断に使っているため、そのまま内部 API の確認にも利用できる。  
**Alternatives considered**:
- Ollama サービス単体の `curl localhost:11434`: 内部ネットワーク疎通の確認にならない
- 別の検証用コンテナを compose に追加: 過剰

## 5. ドキュメント同期方針

**Decision**: ルート `README.md` には短い導線のみを追記し、詳細手順は feature 配下の `quickstart.md` に集約する。  
**Rationale**: ComfyUI feature でも同じ構成を採っており、README を肥大化させずに済む。  
**Alternatives considered**:
- README にすべて記載: 詳細化しやすいが、保守対象が散らばる
- feature 文書だけ更新: ルート README から Ollama の存在が見えなくなるため不採用

# リサーチ: AI Toolkit 試用環境

## Decision 1: AI Toolkit は `ostris/ai-toolkit` の公式コンテナイメージ前提で compose へ追加する

- Decision: この feature では `ostris/ai-toolkit` を `ai-toolkit` サービスとして既存 `compose.yml` へ追加する。
- Rationale: upstream リポジトリには `ostris/aitoolkit:latest` を使う `docker-compose.yml` があり、UI のポート公開、主要な volume、環境変数入口が示されている。既存イメージ前提にする方が、ソース改変なしで最短に試用環境を整えられる。
- Alternatives considered:
  - `ostris/ai-toolkit` を git clone してローカル build する: セットアップ負荷が増え、今回の「試せる環境」の最短経路から外れる。
  - AI Toolkit を既存 ComfyUI/Ollama 導線名として扱う: 実体を誤解しており要件に合わない。

## Decision 2: 最小成功条件は `ai-toolkit` 起動と Web UI 到達

- Decision: 受け入れ条件の最小線は `docker compose up -d ai-toolkit` の成功と、公開ポート経由で Web UI へ到達できることとする。
- Rationale: upstream README では AI Toolkit UI を Web インターフェースとして使う前提が明示されている。今回の主目的は「起動して試せること」であり、学習ジョブ実行やモデル設定の詳細までは広げない方がスコープに合う。
- Alternatives considered:
  - 学習ジョブ起動まで必須にする: GPU 要件やデータ準備まで要求し、初期 feature として重い。
  - コンテナ起動のみで成功扱いにする: 利用者価値に近い UI 到達確認が欠ける。

## Decision 3: 保存先は upstream compose の責務分解を踏襲する

- Decision: AI Toolkit 用には少なくとも config、datasets、output、DB、必要なキャッシュの保存先をホスト側へ持たせる。
- Rationale: upstream `docker-compose.yml` は `aitk_db.db`、`datasets`、`output`、`config`、Hugging Face cache を分離しており、再起動後の継続利用に必要な最小単位が分かる。これをそのまま利用者向け説明へ落とすのが自然。
- Alternatives considered:
  - 保存先を一切永続化しない: US2 を満たせない。
  - 保存先を 1 ディレクトリへ雑にまとめる: 何が保持されるか利用者が理解しづらい。

## Decision 4: 認証は `AI_TOOLKIT_AUTH` の入口を `.env.example` で案内する

- Decision: 認証付きで UI を使いたい利用者向けに、`AI_TOOLKIT_AUTH` を `.env.example` の主要設定として案内する。
- Rationale: upstream README は UI 公開時の保護方法として `AI_TOOLKIT_AUTH` を案内している。今回の feature は本番セキュリティ設計ではないが、最低限の入口は文書化しておくべき。
- Alternatives considered:
  - 認証設定を完全に省く: UI 公開時の基本的な注意点が不足する。
  - 認証を必須化する: ローカル試用の初期ハードルを不必要に上げる。

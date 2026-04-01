# リサーチ: AI Toolkit 試用環境

## Decision 1: AI Toolkit は新規製品名ではなく既存 Compose 資産の試用導線として扱う

- Decision: この feature では AI Toolkit を、既存 `compose.yml` 上の ComfyUI と Ollama を束ねて試すための利用者向け導線名として扱う。
- Rationale: リポジトリにはすでに ComfyUI と Ollama の Compose 運用が存在し、clarify でもそれらを土台に不足分だけ追加する方針を確定した。新規サービス群を設計し直すより、既存資産の入口と完了条件を整理する方がスコープに合う。
- Alternatives considered:
  - 新規の独立 AI 基盤を追加する: Allowed Scope を超えやすく、既存導線を壊すリスクが高い。
  - Ollama のみを AI Toolkit とみなす: 既存 ComfyUI 資産を土台にする clarify 結果と矛盾する。

## Decision 2: 利用者向けの成功判定は「主要サービス起動 + ComfyUI から Ollama への API 疎通成功」

- Decision: 試用成功の最小条件は、Compose 上で主要サービスが起動し、`docker compose exec comfyui curl -fsS http://ollama:11434/api/version` を成功させることとする。
- Rationale: 既存の ComfyUI/Ollama 共存確認手順をそのまま AI Toolkit の代表操作へ昇格できる。追加実装なしで両サービスの同時起動、ネットワーク疎通、利用者の自己確認を 1 回で検証できる。
- Alternatives considered:
  - サービス起動のみで成功扱い: 実際に試せたか判断できない。
  - `ollama list` を代表操作にする: Ollama 単体確認に寄り、AI Toolkit としての統合導線を示しにくい。
  - 複数の代表操作を必須化する: 初期 feature としては検証負荷が大きい。

## Decision 3: 導線は README の入口と feature 配下 quickstart の詳細に分離する

- Decision: ルート `README.md` では AI Toolkit 試用環境の存在、前提、入口コマンドだけを案内し、詳細手順、代表操作、復帰方法は `specs/025-ai-toolkit-env/quickstart.md` へ集約する。
- Rationale: 既存 README は ComfyUI と Ollama の個別導線をすでに持っている。そこへ長い運用説明を直接足すと既存利用者の視認性が下がるため、入口と詳細を分離した方が回帰しにくい。
- Alternatives considered:
  - README に全手順を埋め込む: 既存導線が冗長になる。
  - feature 配下だけで案内する: 新規利用者が入口を見つけにくい。

## Decision 4: 復帰方法は Compose 状態、環境変数、永続ディレクトリの三層で整理する

- Decision: 失敗時の案内は `docker compose ps` / `logs` 等の状態確認、`.env` の設定確認、`comfyui-data` / `ollama-data` の確認の 3 観点で統一する。
- Rationale: 現在の Compose 構成では、利用開始不能の多くがサービス未起動、設定未調整、永続ディレクトリの認識違いに集約される。利用者が自己解決しやすい最小の切り分け軸になる。
- Alternatives considered:
  - サービスごとに完全に別の troubleshooting を持つ: 文書量が増え、AI Toolkit という共通導線の価値が薄れる。
  - 復帰方法を plan で決めず implement へ送る: tasks の粒度と検証観点がぶれる。

# Research: Local RunPod image 統一

## Decision 1: local の `comfyui` service を `comfyui/runpod/Dockerfile` ベースへ置き換える

- Decision: `compose.yml` の既存 `comfyui` service は維持しつつ、build 対象を local 専用 `comfyui/Dockerfile` から `comfyui/runpod/Dockerfile` へ切り替える。
- Rationale: 利用者入口を増やさずに runtime を 1 系統へ畳める。service 名を残すことで既存の `docker compose up -d comfyui` 導線も維持できる。
- Alternatives considered:
  - `comfyui-runpod` のような新 service を追加する: service 名と導線が増え、統一の価値が落ちるため不採用。
  - local は `docker run` 専用にする: 既存 compose 利用者の入口が変わり、他 service との併用も悪化するため不採用。

## Decision 2: local の独立 `ollama` service を廃止し、同一コンテナへ統合する

- Decision: local でも Ollama は `comfyui` コンテナ内で `start-ollama-worker.sh` により起動し、独立 `ollama` service は削除する。
- Rationale: RunPod と local で Ollama の配置と起動シーケンスが一致する。runtime 差分の主因だった「別コンテナの Ollama 接続」を消せる。
- Alternatives considered:
  - local だけ独立 `ollama` service を残す: `http://ollama:11434` 前提が残り、RunPod との差分が維持されるため不採用。
  - 両方残して選択式にする: 現行導線が複数残り、保守対象削減に逆行するため不採用。

## Decision 3: local でも `/runpod-volume` bind mount を必須前提にする

- Decision: local compose でも `/runpod-volume` bind mount を必須とし、ComfyUI model path は `/runpod-volume/models`、Ollama model path は `/runpod-volume/ollama/models` に揃える。
- Rationale: model path の考え方を local / RunPod で一致させると、workflow・troubleshooting・README の説明が一本化できる。
- Alternatives considered:
  - local だけ `./comfyui-data` / `./ollama-data` を維持する: path 差分が残り、統一価値が半減するため不採用。
  - 任意 path を受けて内部で吸収する: 実装は可能だが、説明と検証が複雑化するため不採用。

## Decision 4: local の Ollama 疎通確認は container 内 localhost で行う

- Decision: local の標準確認手順は `docker compose exec comfyui curl -fsS http://127.0.0.1:11434/api/version` とする。
- Rationale: feature 043 で確立した「Ollama API は localhost 限定」の contract をそのまま維持できる。余分な host 公開ポートも増えない。
- Alternatives considered:
  - `11434:11434` を host 公開する: local だけ外部公開経路が増え、RunPod と契約がずれるため不採用。
  - ComfyUI node だけで確認する: 起動確認と node 動作確認が混ざり、切り分けが悪くなるため不採用。

## Decision 5: `comfyui/runpod/README.md` を共通 runtime 文書へ昇格させる

- Decision: `comfyui/runpod/README.md` は serverless 専用説明から、local / RunPod 共通 runtime の詳細説明へ役割変更する。
- Rationale: runtime 実装の実体が `comfyui/runpod/` にあるため、その場所を一次情報にするのが自然。root README は要約と入口に絞れる。
- Alternatives considered:
  - root README に詳細を全部寄せる: 実装 asset の近くに一次情報が残らず、保守しにくいため不採用。
  - 新しい共通 README を別パスに追加する: 文書が分散し、どれを参照すべきか曖昧になるため不採用。

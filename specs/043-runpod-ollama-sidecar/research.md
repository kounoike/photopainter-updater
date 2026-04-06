# 調査メモ: RunPod Ollama sidecar

## Decision 1: RunPod 用 runtime は既存ローカル ComfyUI runtime と分離する

**Decision**: RunPod serverless 用の Docker build asset は `comfyui/runpod/` に分離し、既存の `comfyui/Dockerfile` と `compose.yml` はローカル Compose 導線として維持する。  
**Rationale**: 現在の repo にはローカル向け ComfyUI self-build 導線と独立 `ollama` service が既にある。ここへ RunPod worker 固有の handler / `/start.sh` 前提を直接混ぜると、既存ローカル運用を壊しやすい。RunPod 側は upstream `worker-comfyui` のカスタマイズ導線が別に存在するため、asset 分離の方が責務が明確になる。  
**Alternatives considered**:
- 既存 `comfyui/Dockerfile` をそのまま RunPod worker 化する: ローカル Compose と RunPod worker の責務が衝突する
- 既存 `compose.yml` に RunPod 用 service を足す: serverless worker 導線としては不要で、ローカル構成を複雑化する

## Decision 2: Ollama は wrapper start script から前置起動し、upstream `/start.sh` へ委譲する

**Decision**: RunPod 用 image では wrapper script を entrypoint/CMD として追加し、`ollama serve` を background 起動して API readiness を待った後、upstream `/start.sh` を `exec` する。  
**Rationale**: `worker-comfyui` upstream は `/start.sh` 内で ComfyUI 本体と handler を起動している。upstream flow を丸ごと置き換えるより、Ollama sidecar の責務だけ wrapper に閉じ込めた方が保守しやすい。ユーザー要求の「`start.sh` の前に `ollama serve &`」もこの形で満たせる。  
**Alternatives considered**:
- upstream `src/start.sh` を全面 copy して改造する: upstream 更新追従が重い
- ComfyUI node 側で都度 `ollama serve` する: 起動競合と遅延が大きい

## Decision 3: Ollama API は localhost 限定にする

**Decision**: Ollama は `127.0.0.1:11434` で listen する前提とし、外部 bind や公開経路は追加しない。  
**Rationale**: Ollama 公式 FAQ では既定 bind が `127.0.0.1:11434` とされ、公開が必要な場合だけ `OLLAMA_HOST` を変更する前提である。今回の利用者は同一コンテナ内の ComfyUI node だけなので、localhost 限定が最も単純で安全である。  
**Alternatives considered**:
- `0.0.0.0:11434` に bind する: 不要な露出面が増える
- 外部リバースプロキシ前提にする: 今回の scope を超える

## Decision 4: モデル保存先は RunPod Network Volume 優先、未接続時は一時領域へフォールバックする

**Decision**: `/runpod-volume` が存在し利用可能なら、Ollama の model directory はその配下を使う。未接続時はコンテナ内一時領域へ切り替えて起動継続する。  
**Rationale**: `worker-comfyui` の network volume docs は serverless endpoint で Network Volume を選ぶと `/runpod-volume` として見える前提であり、Dockerfile 側で自由に `volumes` を宣言する方式ではない。一方で clarification で「未接続でも一時領域で起動継続」と決めたため、永続・一時の二段階解決が必要になる。Ollama FAQ では `OLLAMA_MODELS` で保存先を変更できる。  
**Alternatives considered**:
- Network Volume 未接続を起動失敗にする: clarification で不採用
- 常にデフォルト保存先へ書く: 永続利用か一時利用かを切り分けにくい

## Decision 5: 事前 pull モデルは単一 env 値のカンマ区切り一覧で扱う

**Decision**: 事前取得モデルは単一設定値で受け取り、trim 済みのカンマ区切り一覧として順次 `ollama pull` する。設定が空なら pull を行わない。  
**Rationale**: clarification で指定形式が確定しており、serverless endpoint の環境変数設定とも相性が良い。複数の設定値や固定ファイルより、image を変えずに運用設定だけで切り替えやすい。  
**Alternatives considered**:
- モデルごとに個別 env を増やす: 設定数が増え、手順書が煩雑になる
- image build 時にモデルを焼き込む: spec の永続領域再利用方針と合わない

## Decision 6: model pull 失敗は warning として記録し、worker 起動は継続する

**Decision**: model pull の一部または全部が失敗しても、失敗モデル名を warning として残した上で worker 起動は続行する。  
**Rationale**: clarification でこの運用方針が確定している。serverless では endpoint 全体を落とさず、利用可能なモデルだけ先に使える状態を残す方が運用柔軟性が高い。  
**Alternatives considered**:
- 1 件でも pull 失敗したら起動失敗にする: clarification で不採用
- 失敗モデルを黙殺する: 障害切り分けができない

## Decision 7: `KEEP_ALIVE` は Docker 側で固定せず、ComfyUI node 側制御のままにする

**Decision**: wrapper script と Dockerfile では `keep_alive` を固定せず、ComfyUI-ollama node など既存クライアントが `0` を指定する運用を前提にする。  
**Rationale**: user 指示で明示されており、Ollama API でも `keep_alive` は request 単位で指定できる。runtime 全体で固定すると node ごとの実験余地を減らす。  
**Alternatives considered**:
- `OLLAMA_KEEP_ALIVE=0` を image へ固定する: クライアント側運用と衝突する
- keep_alive を wrapper script 引数で持つ: 今回の scope を超える

## Decision 8: ローカル擬似検証は `worker-comfyui` development 導線に合わせて `docker run` と test payload で行う

**Decision**: ローカル検証手順は `worker-comfyui` の development docs に合わせ、Docker で worker を起動し、必要に応じて `/runpod-volume` 相当の bind mount を付け、test payload を送る形で整備する。  
**Rationale**: upstream development docs はローカル API と `test_input.json` ベースの確認導線を示している。RunPod endpoint 設定自体はローカルで再現できないが、起動シーケンス、localhost Ollama、永続領域切替は十分に確認できる。  
**Alternatives considered**:
- RunPod 本番 endpoint だけで確認する: 起動失敗時の切り分けコストが高い
- ローカル Compose だけで RunPod worker 互換を担保する: upstream handler の挙動確認が不足する

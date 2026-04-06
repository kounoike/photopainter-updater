# Quickstart: RunPod Ollama sidecar

## 1. 前提条件

- RunPod serverless 用 ComfyUI worker image をカスタマイズする前提であること
- Docker でローカル build / run ができること
- RunPod 本番では Network Volume を endpoint 設定で接続すると `/runpod-volume` に見えること
- Ollama API は同一コンテナ内 localhost 利用前提であり、外部公開しないこと

## 2. ローカルで image を build する

```bash
docker build -t photopainter-runpod-comfyui-ollama -f comfyui/runpod/Dockerfile .
```

RunPod 用 image build は既存の `compose.yml` とは独立している。既存ローカル ComfyUI 導線はこの build の対象外とする。

## 3. 永続領域ありの擬似検証を行う

```bash
mkdir -p ./.tmp-runpod-volume

docker run --rm --gpus all \
  -p 3000:3000 \
  -v "$PWD/.tmp-runpod-volume:/runpod-volume" \
  -e OLLAMA_PULL_MODELS="qwen3.5:4b" \
  photopainter-runpod-comfyui-ollama
```

別ターミナルで container 内または worker API 経由の確認を行う。

- container 内 `curl http://127.0.0.1:11434/api/version` が成功すること
- `/runpod-volume` 配下が model 保存先として選ばれていること
- 指定 model の pull 結果がログで確認できること

## 4. 永続領域なしの擬似検証を行う

```bash
docker run --rm --gpus all \
  -p 3000:3000 \
  -e OLLAMA_PULL_MODELS="qwen3.5:4b" \
  photopainter-runpod-comfyui-ollama
```

この場合は一時領域フォールバックになる。起動が継続しつつ、再利用不能モードであることをログから判断する。

## 5. Worker API へ test payload を送る

upstream `worker-comfyui` の development 手順に沿って、ローカル worker API へ workflow payload を送る。

```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d @test_input.json \
  http://localhost:3000/run
```

必要に応じて `test_input.json` をこの feature 用 workflow に合わせて調整する。

## 6. RunPod 本番で確認する

- Endpoint の Advanced 設定で Network Volume を選択する
- model 一覧は単一 env 値のカンマ区切りで設定する
- 起動後ログから `persistent` / `ephemeral` のどちらで起動したか確認する
- ComfyUI 側の Ollama node は `http://127.0.0.1:11434` 前提で設定する
- `keep_alive` は node 側で `0` を指定する

## 7. 困ったときの確認先

- Ollama が起動しない: wrapper start script のログと `api/version` 疎通
- model pull が失敗する: warning ログ内の model 名と pull 結果
- 永続領域へ保存されない: `/runpod-volume` 接続有無と書き込み可否
- RunPod 本番だけ失敗する: Endpoint の Network Volume 設定と env 値の綴り
- ローカル worker API が応答しない: upstream `worker-comfyui` development 手順と port 設定

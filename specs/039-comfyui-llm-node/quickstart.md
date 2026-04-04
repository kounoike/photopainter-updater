# Quickstart: ComfyUI local LLM node

## 前提

- リポジトリルートで `docker compose build comfyui && docker compose up -d comfyui` を実行できる
- ComfyUI custom node は repo 内 `comfyui/custom_node/comfyui-photopainter-custom/` から image に同梱される
- 利用する model は `transformers` または `llama-cpp` でロード可能である
- 構造化出力は generation-time constraint を使う前提で確認する

## 1. 必要なら model 保存先を設定する

既定保存先を使わない場合だけ `.env` に model root を追加する。

```bash
cp .env.example .env
printf '\nCOMFYUI_LLM_MODEL_CACHE_DIR=./comfyui-data/llm-models\n' >> .env
```

`compose.yml` がこの値を ComfyUI container へ渡す。未設定でも node は backend 既定保存先で動作できる。

## 2. ComfyUI image を再 build して起動する

```bash
docker compose build comfyui
docker compose up -d comfyui
```

## 3. node が読み込まれることを確認する

```bash
docker compose logs --tail=200 comfyui
```

`PhotoPainter LLM Generate` が Add Node に現れること、custom node 読み込み失敗が出ていないことを確認する。

## 4. smoke check を確認する

最初は `transformers + Qwen3.5 + think_mode=off + text mode` のみを見る。

1. `PhotoPainter LLM Generate` を workflow に追加する
2. `backend` に `transformers` を設定する
3. `model_id` に `Qwen/Qwen3.5-4B` を入力する
4. `think_mode` を `off` にする
5. `json_output` を `false` にする
6. `system_prompt` に `You are a concise assistant.` を入れる
7. `user_prompt` に `Say hello in one short sentence.` を入れる
8. 実行する

期待結果:

- 単一 `STRING` 出力に短い応答が入る
- 実行失敗時は node が例外を返し、workflow 全体が失敗扱いになる

## 5. JSON mode を確認する

`json_output` を `true` にし、たとえば次のような schema を `json_schema` に貼る。

```json
{
  "type": "object",
  "required": ["positive_prompt", "negative_prompt"],
  "properties": {
    "positive_prompt": { "type": "string" },
    "negative_prompt": { "type": "string" }
  },
  "additionalProperties": false
}
```

期待結果:

- generation-time constraint を有効化した状態で結果が生成される
- 成功時は単一 `STRING` 出力に schema を満たす JSON 文字列が入る

## 6. retry と failure を確認する

1. `json_output=true` のまま、schema を満たしにくい prompt を与える
2. `max_retries` を 2 以上に設定する
3. 実行結果を見る

期待結果:

- parse 失敗または schema 不一致のときだけ retry する
- retry 上限後も不一致なら成功扱いにせず node error になる
- backend 失敗や model load 失敗は retry せず即失敗する

## 7. `think_mode` の family 切替を確認する

同じ workflow で `think_mode` を `qwen`、`gemma`、`deepseek_r1` に切り替え、family ごとの documented control が利用されることを確認する。`generic` は best-effort mode であり、特定 model での thinking 挙動を保証しないことに注意する。

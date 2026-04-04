# Quickstart: ComfyUI local LLM node

## 前提

- リポジトリルートで `docker compose build comfyui && docker compose up -d comfyui` を実行できる
- ComfyUI custom node は repo 内 `comfyui/custom_node/comfyui-photopainter-custom/` から image に同梱される
- 利用する model は `transformers` または `llama-cpp` でロード可能である

## 1. 必要なら model 保存先を設定する

既定保存先を使わない場合だけ `.env` に model root を追加する。

```bash
cp .env.example .env
printf '\nPHOTOPAINTER_LLM_MODEL_ROOT=./comfyui-data/llm-models\n' >> .env
```

未設定でも node は backend 既定保存先で動作できる。

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

## 4. text mode の最小動作を確認する

1. `PhotoPainter LLM Generate` を workflow に追加する
2. `backend` に `transformers` または `llama-cpp` を設定する
3. `model_id` を入力する
4. `think_mode` を `off` にする
5. `json_output` を `false` にする
6. `system_prompt` と `user_prompt` を入力して実行する

期待結果:

- `text` 出力に生成結果が入る
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

- 成功時は `json_text` に schema を満たす JSON 文字列が入る
- `text` にも最終出力を保持する

## 6. retry と failure を確認する

1. `json_output=true` のまま、schema を満たしにくい prompt を与える
2. `max_retries` を 2 以上に設定する
3. 実行結果を見る

期待結果:

- parse 失敗または schema 不一致のときだけ retry する
- retry 上限後も不一致なら成功扱いにせず node error になる
- backend 失敗や model load 失敗は retry せず即失敗する

## 7. `think_mode` の family 切替を確認する

同じ workflow で `think_mode` を `qwen`、`gemma`、`deepseek_r1` に切り替え、利用 model / backend と整合する組み合わせだけが成功することを確認する。未対応の組み合わせは暗黙変換されず失敗することを確認する。

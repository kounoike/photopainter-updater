# quickstart.md

## 目的

単一 `PhotoPainter LLM Generate` ノードを削除し、backend ごとの専用ノードへ置き換える。

## Transformers ノードへ移行

1. 旧ノードを削除する
2. `PhotoPainter LLM Generate (Transformers)` を配置する
3. 旧 `backend=transformers` の設定を移す
4. `model_file` は捨てる
5. 必要なら `quantization_mode=bnb_4bit` または `bnb_8bit` を選ぶ
6. `debug_json` と `raw_text` で think 制御と structured output を確認する

## llama-cpp ノードへ移行

1. 旧ノードを削除する
2. `PhotoPainter LLM Generate (llama-cpp)` を配置する
3. GGUF repo の `model_id` と `model_file` を設定する
4. `think_mode` は存在しないので削除する
5. `debug_json` と `raw_text` で GGUF 実行結果を確認する

## 検証観点

- `transformers` ノードに `model_file` が無いこと
- `llama-cpp` ノードに `quantization_mode` と `think_mode` が無いこと
- どちらも `output_text`, `debug_json`, `raw_text` の 3 出力を持つこと

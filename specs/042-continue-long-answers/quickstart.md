# quickstart.md

## 目的

長文回答が 1 回で収まらない場合でも、node が continuation により最後まで本文を返せることを確認する。

## 手順 1: text mode の continuation 成功確認

1. `PhotoPainter LLM Generate (Transformers)` を配置する
2. 長文回答を返しやすい prompt を設定する
3. `response_budget=manual` と小さめの `max_tokens` を設定し、1 回では切れる条件を作る
4. 実行後、`output_text` が途中で切れず完結していることを確認する
5. `debug_json` で以下を確認する
   - `continuation_used` が `true`
   - `continuation_count` が 1 以上
   - `continuation_stop_reason` が完結理由になっている

## 手順 2: continuation 不要ケースの確認

1. 短い回答を返す prompt に切り替える
2. node を実行する
3. `debug_json.continuation_used=false` で 1 回の generation だけで終わることを確認する

## 手順 3: continuation 非対象ケースの確認

1. `think_mode=off` または `json_output=true` の条件で途中終了しやすい prompt を試す
2. continuation が既存契約を壊して走らないこと、または非対象として判別できることを確認する

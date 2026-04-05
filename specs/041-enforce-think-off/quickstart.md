# quickstart.md

## 目的

`PhotoPainter LLM Generate (Transformers)` の `think_mode=off` が、本当に hidden reasoning を止められる経路だけ成功することを確認する。

## 手順 1: documented disable が使えるモデルで成功確認

1. `PhotoPainter LLM Generate (Transformers)` を配置する
2. `model_id` に Qwen 系 model を設定する
3. `think_mode=off` を選ぶ
4. 短い `system_prompt` と `user_prompt` で実行する
5. `debug_json` で以下を確認する
   - `think_mode` が `off`
   - `off_enforcement_supported` が `true`
   - `off_enforcement_guaranteed` が `true`
   - `off_failure_reason` が `null`
   - `requested_enable_thinking` が `false`
6. `raw_text` に `<think>` 相当が無いことを確認する

## 手順 2: unsupported モデルで failure 確認

1. `think_mode=off` のまま、documented disable を持たない model family へ切り替える
2. ノードを実行する
3. 実行が成功扱いにならず、unsupported な `off` 制御として失敗することを確認する
4. エラーが `think_mode_error` であり、debug 情報またはメッセージから unsupported 理由を確認する

## 手順 3: trace violation の検証

1. unit test または mock を使い、`think_mode=off` で `<think>...</think>` を含む `raw_text` を返すケースを再現する
2. 実装が sanitize 成功にせず failure を返すことを確認する
3. エラーが `backend_error` であり、reasoning trace が `off` 契約違反として扱われることを確認する
4. `think_mode=qwen` など `off` 以外では既存挙動が維持されることを確認する

## 検証観点

- `think_mode=off` 成功は guarantee 可能な経路だけであること
- fallback / unsupported / trace violation が success に混ざらないこと
- `debug_json` だけで `off` の成否を判別できること

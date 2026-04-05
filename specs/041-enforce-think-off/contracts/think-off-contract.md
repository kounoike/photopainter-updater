# contracts/think-off-contract.md

## Target Node

- Internal: `PhotopainterTransformersLlmGenerate`
- Display: `PhotoPainter LLM Generate (Transformers)`

## Scope

- `think_mode=off` の成功条件、failure 条件、debug 契約を定義する
- `llama-cpp` ノードは対象外

## Input Contract

- `think_mode=off` を指定した場合、node は generation 前に documented disable の適用可能性を判定する
- documented disable を持たない family または runtime fallback は unsupported でなければならない

## Success Contract

- `think_mode=off` 成功時、`debug_json` は少なくとも以下を含む
  - `think_mode="off"`
  - `documented_control_available=true`
  - `off_enforcement_supported=true`
  - `off_enforcement_guaranteed=true`
  - `off_failure_reason=null`
- `control_kind` は実際に使った disable control を表す
- `requested_enable_thinking` は disable 要求を反映した値を保持する

## Failure Contract

- 次の場合、node は `think_mode=off` を成功扱いにしてはならない
  - family が documented disable 非対応
  - tokenizer/chat template が disable 引数を受け付けず fallback した
  - generation 結果に reasoning trace が含まれた
- これらの failure は JSON retry 対象外である
- unsupported / guarantee 不能は `think_mode_error`、reasoning trace は `backend_error` として区別できなければならない
- failure 時は message または debug から unsupported / trace violation を判別できなければならない

## Raw Output Contract

- `think_mode=off` 成功時、`raw_text` は reasoning trace を含んではならない
- `think_mode=off` で reasoning trace が観測された場合は `output_text` を sanitize して返すのではなく失敗しなければならない

## Backward Compatibility Notes

- `think_mode=off` の意味は「なるべく抑える」から「保証できる場合だけ成功する」に変わる
- `think_mode=generic`、`qwen`、`gemma`、`deepseek_r1` の既存契約は今回の対象外

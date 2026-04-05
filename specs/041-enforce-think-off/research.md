# research.md

## Decision 1: `think_mode=off` は guarantee 可能な経路だけ成功させる
- Decision: `think_mode=off` は final-only prompt を足すだけの best-effort mode ではなく、documented disable control を generation 前に適用できる経路だけ成功させる。
- Rationale: 利用者が欲しいのは見た目上 `<think>` が消えることではなく、処理時間差から分かる hidden reasoning 自体を抑止できることだから。保証不能な経路を成功扱いにすると、`off` が契約として成立しない。
- Alternatives considered:
  - 既存の final-only prompt を維持する: 成功と unsupported の境界が曖昧で、処理時間差の要因が切り分けられないため不採用。
  - warning のみ出して成功する: workflow 上では成功扱いのままなので、`off` の実効性を担保できず不採用。

## Decision 2: Qwen chat template の documented disable だけを現行の保証経路とみなす
- Decision: 現在の `transformers` 実装では、Qwen 系の `apply_chat_template(..., chat_template_kwargs={enable_thinking: False})` が通る経路を `off` 成功の主経路とする。
- Rationale: 現行コードで documented な disable を明示的に扱っているのはこの経路だけであり、ここを基準にすると実装差分を最小化しつつ guarantee 条件を明快にできる。
- Alternatives considered:
  - Gemma の `<|think|>` 非付与を disable とみなす: documented disable ではなく、thinking token を足していないだけなので `off` 保証の根拠にならず不採用。
  - family 非依存の prompt suffix だけで `off` を通す: モデル依存で hidden reasoning が残るため不採用。

## Decision 3: documented disable を渡せない fallback は unsupported failure にする
- Decision: tokenizer が `chat_template_kwargs` を受け付けず fallback 実行になった場合や family 自体が disable 非対応の場合は、`think_mode=off` を unsupported failure にする。
- Rationale: ここを黙って fallback 成功にすると、まさに「think: off なのに think している」状態を温存してしまう。runtime capability の欠如は成功ではなく failure として見せるべきである。
- Alternatives considered:
  - fallback して raw 出力だけ sanitize する: hidden reasoning の token/時間消費を止められないため不採用。
  - fallback で generic prompt を強化する: best-effort を濃くするだけで guarantee にならないため不採用。

## Decision 4: `off` で reasoning trace が出たら sanitize 成功を禁止する
- Decision: `think_mode=off` で `<think>...</think>` など reasoning trace が観測された場合は、sanitize 後の最終文が妥当でも契約違反として失敗させる。
- Rationale: 今回の主眼は hidden reasoning の抑止であり、後処理で見た目を整えることではない。trace が出た時点で `off` の実効性は失われている。
- Alternatives considered:
  - `output_text` だけ sanitize して `raw_text` へ痕跡を残す: 見た目は成功してしまい、workflow 利用者が失敗を見落とすため不採用。
  - `debug_json` にフラグだけ追加して成功継続する: 契約違反を利用者の目視確認へ委ねるため不採用。

## Decision 5: debug には guarantee 成否を直接出す
- Decision: `debug_json` に requested disable の有無だけでなく、`off` が guarantee されたか、unsupported だったか、trace violation だったかを直接表すフィールドを追加する。
- Rationale: 利用者は処理時間差と併せて `debug_json` を見て判断したい。既存の `requested_enable_thinking=false` だけでは、disable 要求を出したことしか分からず、成功保証までは読み取れない。
- Alternatives considered:
  - エラーメッセージだけに頼る: 成功時の guarantee 状態が読めず不採用。
  - `control_kind` だけで推測させる: fallback 経路や runtime 非対応を判断しきれず不採用。

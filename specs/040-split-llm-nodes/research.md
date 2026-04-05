# research.md

## Decision 1: backend ごとに別ノードへ分離する
- Decision: 既存の単一 `PhotoPainter LLM Generate` ノードを削除し、`PhotoPainter LLM Generate (Transformers)` と `PhotoPainter LLM Generate (llama-cpp)` の 2 ノードへ分離する。
- Rationale: 現在の単一ノードは `quantization_mode`、`model_file`、`think_mode` の適用範囲が backend ごとに大きく異なり、UI と失敗要因が混線している。backend 固有の入力を専用ノードへ閉じ込めた方が、利用者の設定ミスと debug の複雑さを減らせる。
- Alternatives considered:
  - 単一ノードを維持し続ける: backend 固有 UI が残り続け、責務分離の価値が出ないため不採用。
  - 既存単一ノードを互換ラッパーとして残す: workflow 互換性は上がるが、backend 差の切り分けが曖昧なまま残るため不採用。

## Decision 2: `transformers` ノードは `model_file` を持たない
- Decision: `transformers` 専用ノードから `model_file` を完全に削除し、`model_id` と `quantization_mode` を主入力にする。
- Rationale: `model_file` は GGUF / `llama-cpp` 運用の概念であり、`transformers` 利用者に見せると「Hugging Face repo と GGUF file のどちらを前提とするか」が不明瞭になる。`transformers` では repo id と load-time quantization に責務を集中させる方が自然である。
- Alternatives considered:
  - `model_file` を残して無視する: UI にノイズが残るため不採用。
  - 将来用の予約入力として残す: 現状の目的が責務分離である以上、予約入力は逆効果のため不採用。

## Decision 3: `llama-cpp` ノードは `think_mode` を持たない
- Decision: `llama-cpp` 専用ノードでは `think_mode` 入力自体を持たず、GGUF / `model_file` / structured output / debug に責務を限定する。
- Rationale: Qwen/Gemma 系の documented think control は `transformers` 側で扱うのが筋であり、`llama-cpp` では upstream 依存や GGUF 実装差が大きい。backend 専用ノードにしたうえで `think_mode` を外すと、利用者に「official control が効くはず」という誤解を与えずに済む。
- Alternatives considered:
  - `off/generic` のみ残す: まだ曖昧さが残り、node 役割を完全には切れないため不採用。
  - 全 think_mode を残す: 現状の不確実性を温存するため不採用。

## Decision 4: 共通ロジックは private helper に残し、UI 層のみノード別に分ける
- Decision: JSON/schema 検証、debug 生成、memory release、backend 実行 helper は共有し、ノードクラスだけを分離する。
- Rationale: backend ごとの UI と validation は分けたいが、structured output、error 分類、debug 形式を丸ごと複製すると差分管理が難しくなる。共通コア + backend 別 node class が最小変更である。
- Alternatives considered:
  - 実行ロジックごと別ファイルへ全面分離: 将来的にはあり得るが、今回の主眼は UI / 契約分離であり、即時の大規模再編は不要。
  - 共有ロジックなしで完全複製: テストと保守が重複するため不採用。

## Decision 5: `transformers` ノードは `quantization_mode` を維持する
- Decision: `transformers` 専用ノードには `quantization_mode = none | bnb_8bit | bnb_4bit` を維持する。
- Rationale: 12GB 級 VRAM で Qwen3.5 9B を扱うには load-time quantization が現実的であり、`transformers` を本命経路とする以上、`bitsandbytes`/`accelerate` を自然なノード入力として露出する価値がある。
- Alternatives considered:
  - 量子化を Dockerfile の hidden behavior にする: 利用者が backend 選択時に制御できないため不採用。
  - 量子化を別 feature へ送る: 現在の UI 分離と密接に関わるため不採用。

## Decision 6: 旧単一ノードは削除前提で README に移行手順を書く
- Decision: 旧 `PhotoPainter LLM Generate` は削除し、README と quickstart に新ノード名、主な入力差、置換手順を書く。
- Rationale: 単一ノードを残すと分離の効果が半減する。削除する代わりに移行手順を明記すれば、既存 workflow 修正コストを把握しやすい。
- Alternatives considered:
  - 無告知削除: workflow 破壊時の原因が追えないため不採用。
  - 長期 deprecation: 分離後の責務を曖昧にするため不採用。

# contracts/continuation-contract.md

## Target Nodes

- `PhotopainterTransformersLlmGenerate`
- `PhotopainterLlamaCppLlmGenerate`（対応可否は backend 実装に依存）

## Success Contract

- text mode の長文回答が途中終了した場合、node は continuation により完結した本文を返せる
- 成功時の `output_text` は最終的に連結済みの単一文字列である
- `debug_json` は continuation の有無と回数を示す

## Non-Applicability Contract

- `think_mode=off` の reasoning trace 救済には continuation を使わない
- `json_output=true` の strict JSON/schema 契約を壊す continuation は行わない

## Failure / Stop Contract

- continuation 非対応 backend は silent fallback せず、非対応または未使用理由を debug で判別できる
- continuation 上限到達、空文字、進展なしのときは無限継続せず停止する
- 停止理由は `debug_json` から判別できる

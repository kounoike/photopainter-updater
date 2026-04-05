# data-model.md

## Entities

### ContinuationPlan
- `enabled`: BOOLEAN
- `backend`: `transformers` / `llama-cpp`
- `supported`: BOOLEAN
- `max_continuations`: INT
- `allow_for_think_off`: BOOLEAN
- `allow_for_json_output`: BOOLEAN
- `unsupported_reason`: STRING または `None`

Validation rules:
- `think_mode=off` では `allow_for_think_off=false`
- `json_output=true` では `allow_for_json_output=false` を既定とする
- 未対応 backend では `supported=false` と理由が必須

### ContinuationState
- `base_output`: 1 回目 generation の本文
- `fragments`: generation 断片の配列
- `continuation_count`: INT
- `combined_output`: 現在までの連結本文
- `made_progress`: BOOLEAN
- `stop_reason`: STRING

Validation rules:
- `continuation_count` は `max_continuations` を超えてはならない
- 進展がない場合は continuation を継続してはならない

### ContinuationDebugInfo
- `continuation_supported`: BOOLEAN または `None`
- `continuation_count`: INT
- `continuation_stop_reason`: STRING または `None`
- `continuation_used`: BOOLEAN

Validation rules:
- continuation 未使用時は `continuation_count=0`
- continuation 使用時は `continuation_used=true`

## Relationships

- `ContinuationPlan` は generation 前の continuation 可否判定を表す
- `ContinuationState` は continuation 実行中の断片連結状態を表す
- `ContinuationDebugInfo` は `GenerationDebugInfo` へ埋め込まれて利用者へ返る

## State / Lifecycle Notes

- 1 回目 generation 後に continuation 必要性を判定する
- continuation が不要なら即時終了する
- continuation が必要で対応可能な場合だけ追加 generation を行う
- 上限到達、進展なし、または完結判定で停止する

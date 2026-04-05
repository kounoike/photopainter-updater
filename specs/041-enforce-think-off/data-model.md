# data-model.md

## Entities

### ThinkOffEnforcementPlan
- `think_mode`: enum。今回対象は `off`
- `family`: `qwen` またはその他 family / `None`
- `documented_control_available`: BOOLEAN
- `control_kind`: STRING または `None`
- `off_enforcement_supported`: BOOLEAN
- `off_enforcement_guaranteed`: BOOLEAN
- `off_failure_reason`: STRING または `None`

Validation rules:
- `think_mode != off` の場合、この entity は説明上の対象外
- `off_enforcement_guaranteed=true` は `documented_control_available=true` を前提とする
- unsupported family や runtime fallback では `off_failure_reason` が必須

### ThinkOffDebugStatus
- `requested_enable_thinking`: BOOLEAN または `None`
- `control_kind`: STRING または `None`
- `documented_control_available`: BOOLEAN
- `raw_had_think_block`: BOOLEAN
- `sanitized_output`: BOOLEAN
- `off_enforcement_supported`: BOOLEAN
- `off_enforcement_guaranteed`: BOOLEAN
- `off_failure_reason`: STRING または `None`

Validation rules:
- `think_mode=off` 成功時は `off_enforcement_guaranteed=true`
- `think_mode=off` 失敗時は `off_failure_reason` が理由を説明する
- `think_mode=off` で `raw_had_think_block=true` の場合、成功にしてはならない

### TransformersGenerationResult
- `output_text`: STRING
- `raw_text`: STRING
- `debug_json`: STRING

Validation rules:
- `think_mode=off` 成功時は `output_text == raw_text` か、少なくとも sanitize 依存成功ではない
- `debug_json` だけで `off` 保証の成否を判別できる

## Relationships

- `ThinkOffEnforcementPlan` は generation 前の capability 判定を表す
- `ThinkOffDebugStatus` は generation 後の debug 表現を表す
- `TransformersGenerationResult` は `ThinkOffEnforcementPlan` と `ThinkOffDebugStatus` の結果を利用者へ返す

## State / Lifecycle Notes

- `think_mode=off` 実行開始前に capability を判定する
- unsupported と判定した場合は generation を始める前、または documented control fallback 検出時点で失敗にする
- generation 後に reasoning trace が見つかった場合は sanitize せず失敗へ遷移する

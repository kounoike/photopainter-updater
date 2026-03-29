# Data Model: ACT LED アクティビティ表示

## Entity: 更新ジョブ活動状態

- Purpose: 起動時更新または BOOT ボタン更新が進行中かどうかを表す。
- Fields:
  - `trigger`: `startup` または `boot_button`
  - `phase`: `idle`, `running`, `finished_success`, `finished_failure`
  - `result`: `none`, `success`, `config_error`, `wifi_error`, `http_error`, `image_error`
- Validation Rules:
  - `idle` のとき `result` は `none`
  - `running` 中は 1 ジョブのみ存在する
  - `finished_*` は必ず `running` を経由して遷移する
- State Transitions:
  - `idle -> running`: 更新ジョブ開始
  - `running -> finished_success`: 正常完了
  - `running -> finished_failure`: 失敗終了
  - `finished_* -> idle`: 次回更新開始前の初期状態

## Entity: ACT LED 表示状態

- Purpose: 利用者へ「更新中かどうか」を伝える LED の見え方を表す。
- Fields:
  - `mode`: `off` または `blinking`
  - `pattern`: `none` または既定点滅パターン
  - `owner`: `none` または現在の更新ジョブ
- Validation Rules:
  - `mode=off` のとき `pattern` は `none`
  - `mode=blinking` のとき `owner` は存在する
  - 複数ジョブが同時に `owner` になることはない
- State Transitions:
  - `off -> blinking`: 更新開始
  - `blinking -> off`: 正常完了または失敗終了

## Relationships

- 更新ジョブ活動状態 `drives` ACT LED 表示状態
- 更新ジョブ活動状態 `reuses` 既存失敗分類結果
- ACT LED 表示状態は更新ジョブの直列実行制御に `constrained by` される

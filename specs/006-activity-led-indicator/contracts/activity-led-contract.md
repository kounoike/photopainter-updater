# Contract: ACT LED 活動表示

## Contract Goal

更新ジョブ進行中だけ ACT LED を点滅させ、終了時には必ず停止する振る舞いを固定する。

## Activity LED Contract

### Triggers

- 起動時更新開始
- BOOT ボタン更新開始

### Active State

- 更新ジョブが `running` に入った時点で ACT LED は点滅を開始する
- SD カード読込、Wi-Fi 接続、HTTP 取得、表示更新など、更新ジョブに含まれる待機時間中も点滅を継続する
- 同時に複数の点滅状態を作らない

### Completion State

- 正常完了時は ACT LED の点滅を停止する
- 失敗終了時も ACT LED の点滅を停止する
- 点滅停止後に待機または shutdown 動作へ移る

### Error Handling

- 点滅制御の開始または停止に失敗しても、更新ジョブ本体の成功/失敗判定を改変しない
- 利用者へ誤って進行中に見える状態を残さないことを優先する

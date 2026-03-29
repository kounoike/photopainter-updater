# Quickstart: ACT LED アクティビティ表示

## 目的

更新ジョブの進行中だけ活動表示 LED を点滅させ、正常完了時と失敗終了時には消灯へ戻ることを確認する。

## 実装手順

1. `spec.md`、`plan.md`、`research.md`、`data-model.md`、`contracts/activity-led-contract.md` を確認する。
2. 活動表示 LED は `xiaozhi-esp32/components/led_bsp/led_bsp.h` の `LED_PIN_Green (GPIO42)` を使う前提で確認する。
3. `firmware/` に `led_bsp` を参照 component として追加する。
4. 更新ジョブ開始時に活動表示 LED の 500ms 点滅パターンを開始し、終了時に停止する共通制御を `firmware/` に実装する。
5. 起動時更新と BOOT ボタン更新の両方へ同じ活動表示を統合する。
6. `docs/firmware-http-epaper.md` に ACT LED の挙動を反映する。

## 検証手順

1. 正常系 1: 正しい `config.txt` を置いて起動し、Green LED (`GPIO42`) が更新中だけ約 500ms 間隔で点滅し、更新完了後に停止することを確認する。
2. 正常系 2: 起動後に BOOT ボタンを押し、再更新中だけ Green LED (`GPIO42`) が約 500ms 間隔で点滅し、更新完了後に停止することを確認する。
3. 失敗系 1: `config.txt` を欠落させ、失敗確定まで Green LED (`GPIO42`) が点滅した後に停止することを確認する。
4. 失敗系 2: Wi-Fi 接続失敗を発生させ、失敗確定まで Green LED (`GPIO42`) が点滅した後に停止することを確認する。
5. 失敗系 3: HTTP 取得失敗を発生させ、失敗確定まで Green LED (`GPIO42`) が点滅した後に停止することを確認する。
6. 失敗系 4: 不正画像を返し、失敗確定まで Green LED (`GPIO42`) が点滅した後に停止することを確認する。
7. 待機状態: deep sleep 復帰後に更新を開始していない待機時間では、Green LED (`GPIO42`) が点灯しっぱなしにも点滅しっぱなしにもならないことを確認する。

## 完了条件

- 起動時更新と BOOT ボタン更新の両方で、進行中だけ ACT LED が点滅する。
- 成功終了後と失敗終了後の両方で、活動表示 LED が活動中に見える状態を残さない。
- 待機状態では活動表示 LED が点灯しっぱなし、または点滅しっぱなしにならない。
- 直列実行制御と矛盾する多重点滅が発生しない。

# Research: ACT LED アクティビティ表示

## Decision 1: LED 制御は `xiaozhi-esp32/components/led_bsp` を第一候補にし、`LED_PIN_Green (GPIO42)` を活動表示に使う

- Decision: `firmware/` から `led_bsp` を参照 component として取り込み、`led_bsp.h` が定義する `LED_PIN_Green (GPIO42)` を ACT LED に採用する。`waveshare-s3-PhotoPainter/config.h` の `BUILTIN_LED_GPIO` は `GPIO_NUM_NC` のままなので、汎用 board macro は使わず `led_bsp` 側の既存定義をそのまま参照する。
- Rationale: `xiaozhi-esp32` では Green LED が通常の状態表示に使われており、PhotoPainter 参照実装とも整合する。`firmware/` からは参照 component として利用するだけなので、Forbidden Scope に触れずに同じハードウェア前提を共有できる。
- Alternatives considered:
  - `driver/gpio` で `firmware/` に独自 LED 制御を新設する
    - 却下理由: 既存部品との責務重複が増え、将来の board 差し替えでも整合を崩しやすい。

## Decision 2: 点滅の開始/停止は更新ジョブのライフサイクルに同期させる

- Decision: 起動時更新と BOOT ボタン更新の両方で、キュー投入後に実行された単一更新ジョブに対して点滅を開始し、正常完了または失敗確定時に停止する。
- Rationale: 進行中表示は更新ジョブの直列実行制御と一致している必要があり、SD 読込、Wi-Fi 接続、HTTP 取得、表示更新などの待ち時間を自然に含められる。
- Alternatives considered:
  - SD 読込、HTTP 取得、表示更新ごとに個別制御する
    - 却下理由: 状態遷移が増え、失敗経路で消灯漏れを起こしやすい。

## Decision 3: 点滅周期は 500ms 間隔の既定単一パターンを採用する

- Decision: 更新ジョブ中は Green LED を 500ms ごとに反転させる単一パターンを維持する。
- Rationale: 500ms の on/off は視認しやすく、Wi-Fi 接続や e-paper 更新のような待機時間でも「処理中」であることを十分に示せる。追加 PWM や複数パターンは不要。
- Alternatives considered:
  - 新しい PWM ベースの細かな点滅周期制御を導入する
    - 却下理由: 要件に対して過剰であり、ESP-IDF の追加制御面を増やすだけになる。

## Decision 4: 待機状態と終了状態では LED を消灯する

- Decision: 成功後と失敗後のどちらでも点滅を停止し、待機状態では活動中に見える LED 状態を残さない。
- Rationale: 進行中と停止済みを見分けることがこの機能の主目的であり、停止後に光り続けると意味が失われる。
- Alternatives considered:
  - 成功時だけ常時点灯に切り替える
    - 却下理由: 更新済み表示という別意味を持ち込み、仕様範囲を広げる。

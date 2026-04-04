# Data Model: Config Insecure HTTPS

## 1. FirmwareConfig

- Purpose:
  - firmware が `config.txt` から読み込む更新設定を表す。
- Fields:
  - `wifi_ssid`
  - `wifi_password`
  - `image_url`
  - `insecure`
- Validation:
  - `wifi_ssid` は必須 non-empty string
  - `wifi_password` は必須 string
  - `image_url` は必須 non-empty string
  - `image_url` は `http://` または `https://` で始まる
  - `insecure` は任意
  - `insecure` が存在する場合は boolean のみ許可する
- Normalization:
  - `insecure` 未設定時は `false`

## 2. ImageTransportPolicy

- Purpose:
  - `image_url` の scheme と `insecure` に応じて HTTP client の検証方針を表す。
- Variants:
  - `HttpPlain`
  - `HttpsVerified`
  - `HttpsInsecure`
- Rules:
  - `image_url` が `http://` なら `HttpPlain`
  - `image_url` が `https://` かつ `insecure == false` なら `HttpsVerified`
  - `image_url` が `https://` かつ `insecure == true` なら `HttpsInsecure`

## 3. ConfigValidationResult

- Purpose:
  - `config.txt` の読込結果を failure category へ接続しやすい形で整理する。
- States:
  - `Valid`
  - `MissingRequiredField`
  - `InvalidJson`
  - `InvalidUrlScheme`
  - `InvalidInsecureType`
- Rules:
  - `InvalidInsecureType` は通信開始前の設定不備である
  - `InvalidUrlScheme` は `http://` / `https://` 以外を拒否する

## 4. UpdateRequestContext

- Purpose:
  - 既存の更新ジョブが画像取得前に参照する最小実行文脈を表す。
- Fields:
  - `image_url`
  - `insecure`
  - `binary_route_selected`
- Validation:
  - `binary_route_selected` は URL suffix 判定から決まる
  - `insecure` は route 選択には影響しない
- Relationship:
  - `FirmwareConfig` から導出される
  - `ImageTransportPolicy` を使って実際の通信設定を決定する

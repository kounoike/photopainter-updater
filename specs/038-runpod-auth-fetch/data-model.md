# Data Model: RunPod Authenticated Fetch

## 1. FirmwareConfig

- Purpose:
  - firmware が `config.txt` から読み込む更新設定を表す。
- Fields:
  - `wifi_ssid`
  - `wifi_password`
  - `image_url`
  - `insecure`
  - `bearer_token`
- Validation:
  - `wifi_ssid` は必須 non-empty string
  - `wifi_password` は必須 string
  - `image_url` は必須 non-empty string
  - `image_url` は `http://` または `https://` で始まる
  - `insecure` は任意 boolean
  - `bearer_token` は任意 non-empty string
- Normalization:
  - `insecure` 未設定時は `false`
  - `bearer_token` 未設定時は認証ヘッダ送信なし

## 2. ImageRequestAuth

- Purpose:
  - 画像取得時の認証条件を表す。
- Variants:
  - `None`
  - `Bearer`
- Fields:
  - `scheme`
  - `token`
- Rules:
  - `bearer_token` 未設定なら `None`
  - `bearer_token` 設定済みなら `Bearer`
  - `Bearer` の `token` は空文字を許可しない

## 3. ImageTransportPolicy

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

## 4. ConfigValidationResult

- Purpose:
  - `config.txt` の読込結果を failure category へ接続しやすい形で整理する。
- States:
  - `Valid`
  - `MissingRequiredField`
  - `InvalidJson`
  - `InvalidUrlScheme`
  - `InvalidInsecureType`
  - `InvalidBearerTokenType`
  - `EmptyBearerToken`
- Rules:
  - `InvalidBearerTokenType` と `EmptyBearerToken` は通信開始前の設定不備である
  - `InvalidUrlScheme` は `http://` / `https://` 以外を拒否する

## 5. UpdateRequestContext

- Purpose:
  - 既存の更新ジョブが画像取得前に参照する最小実行文脈を表す。
- Fields:
  - `image_url`
  - `insecure`
  - `bearer_token`
  - `binary_route_selected`
- Validation:
  - `binary_route_selected` は URL suffix 判定から決まる
  - `bearer_token` は route 選択に影響しない
  - `insecure` は認証方式に影響しない

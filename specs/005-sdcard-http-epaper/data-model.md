# Data Model: SDカード設定 HTTP e-paper 更新ファーム

## Entity: 設定 JSON (`config.txt`)

- Purpose: 起動時および手動更新時に使う外部設定ファイル。
- Fields:
  - `wifi_ssid`: 接続先 SSID
  - `wifi_password`: 接続先パスワード
  - `image_url`: 単一の画像取得 URL
- Validation Rules:
  - SDカードルートに存在すること
  - 必須項目がすべて存在すること
  - `image_url` は単一値であること
  - JSON として解釈可能であること

## Entity: 更新ジョブ

- Purpose: 起動時または BOOT ボタン押下時に発生する、設定読込から表示更新までの一連処理。
- Fields:
  - `trigger`: `startup` または `boot_button`
  - `state`: `pending` / `reading_config` / `connecting_wifi` / `fetching_image` / `updating_display` / `failed` / `completed` / `shutdown`
  - `failure_reason`: 失敗時の理由
  - `started_at`: 開始時点
- Validation Rules:
  - 同時に複数走らないこと
  - 失敗時は `failed` を経て `shutdown` に遷移すること
  - 成功時は `completed` で終了すること

## Entity: 表示画像

- Purpose: HTTP 取得後に e-paper へ反映する画像データ。
- Fields:
  - `source_url`: 取得元 URL
  - `payload`: 取得した画像データ
  - `is_displayable`: e-paper 表示可否
- Validation Rules:
  - 表示可能と判断された場合のみ e-paper 更新に使う
  - 不正画像は失敗扱いとする

## Entity: 失敗状態

- Purpose: 更新処理を継続せずシャットダウンする前に、原因を切り分け可能にする状態。
- Fields:
  - `category`: `config_error` / `wifi_error` / `http_error` / `image_error`
  - `detail`: 補足情報
  - `trigger`: `startup` または `boot_button`
- Validation Rules:
  - 更新失敗時は必ず 1 つの category を持つ
  - 利用者または開発者が失敗種別を区別できること

## Relationships

- `config.txt` `drives` 更新ジョブ
- 更新ジョブ `fetches` 表示画像
- 更新ジョブ `produces` 失敗状態
- 表示画像が妥当な場合のみ 更新ジョブ `updates` e-paper 表示

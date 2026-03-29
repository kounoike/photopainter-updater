# Contract: `config.json` と更新ジョブ

## Contract Goal

専用ファームが満たすべき設定ファイル仕様と、起動時/BOOT ボタン更新の振る舞いを固定する。

## `config.json` Contract

### Location

- SDカードルートに固定する
- ファイル名は `config.json` に固定する

### Required Fields

- `wifi_ssid`
- `wifi_password`
- `image_url`

### Rules

- JSON として正しく解釈できること
- `image_url` は単一の URL 値であること
- 必須項目欠落時は設定不備として失敗扱いにすること

## Update Job Contract

### Triggers

- 起動時
- BOOT ボタン押下時

### Success Path

1. `config.json` 読込成功
2. WiFi 接続成功
3. `image_url` から HTTP 取得成功
4. e-paper 更新成功

### Failure Path

- 設定不備、WiFi 失敗、HTTP 失敗、画像不正のいずれでも更新処理を継続しない
- 失敗種別を判断できる状態を残して終了する
- 終了後はそのままシャットダウンする

### Concurrency Rule

- 更新ジョブは常に 1 件のみ実行される
- 起動時更新と BOOT ボタン更新が重複しないこと

# Contract: authenticated fetch from external HTTPS source

## Purpose

`config.txt` に `insecure` と `bearer_token` を追加したときの設定契約と、認証付き HTTP / HTTPS 更新ジョブの通信条件を固定する。

## `config.txt` Contract

### Location

- SDカードルートに固定する
- ファイル名は `config.txt` に固定する
- 中身は JSON として解釈する

### Required Fields

- `wifi_ssid`
- `wifi_password`
- `image_url`

### Optional Fields

- `insecure`
- `bearer_token`

### Rules

- JSON として正しく解釈できること
- `image_url` は単一の `http://` または `https://` URL 値であること
- `insecure` は省略可能で、存在する場合は boolean であること
- `insecure` 省略時は `false` として扱うこと
- `bearer_token` は省略可能で、存在する場合は non-empty string であること
- `bearer_token` が設定されている場合だけ `Authorization: Bearer <token>` を送ること
- `insecure: true` は `https://` のときだけ証明書未検証通信を許可すること
- 必須項目欠落、URL scheme 不正、`insecure` 型不正、`bearer_token` 型不正、空文字 token は設定不備として失敗扱いにすること

## Update Job Contract

### Triggers

- 起動時
- BOOT ボタン押下時

### Success Path

1. `config.txt` 読込成功
2. WiFi 接続成功
3. `image_url` と `insecure` から通信ポリシー確定
4. `bearer_token` の有無から認証ヘッダ送信条件確定
5. 画像取得成功
6. 取得画像の検証成功
7. e-paper 更新成功

### Request Contract

- `bearer_token` 未設定:
  - 認証ヘッダを送らない
- `bearer_token` 設定済み:
  - `Authorization: Bearer <token>` を送る

### Transport Policy Contract

- `image_url` が `http://`:
  - 既存 HTTP 経路を使う
- `image_url` が `https://` かつ `insecure` が未設定または `false`:
  - サーバ証明書検証付き HTTPS を使う
- `image_url` が `https://` かつ `insecure: true`:
  - サーバ証明書未検証 HTTPS を使う

### Failure Path

- `insecure` 型不正、`bearer_token` 型不正、空文字 token、URL scheme 不正は通信開始前に設定不備として終了する
- 認証拒否、HTTP status 異常、到達不能、payload 不正、画像不正は従来どおり更新失敗として終了する
- `insecure: true` でも証明書検証以外の失敗は緩和しない
- 失敗種別を判断できる状態を残して終了する

### Compatibility Rule

- `.bmp` suffix による BMP / binary の経路選択は維持する
- `bearer_token` と `insecure` は route 選択条件に影響しない
- 既存の HTTP-only `config.txt` は修正なしで継続利用できる

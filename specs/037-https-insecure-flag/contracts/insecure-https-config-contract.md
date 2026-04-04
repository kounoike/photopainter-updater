# Contract: `config.txt` insecure HTTPS

## Purpose

`config.txt` に `insecure` を追加したときの設定契約と、HTTP / HTTPS 更新ジョブの通信条件を固定する。

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

### Rules

- JSON として正しく解釈できること
- `image_url` は単一の `http://` または `https://` URL 値であること
- `insecure` は省略可能で、存在する場合は boolean であること
- `insecure` 省略時は `false` として扱うこと
- `insecure: true` は `https://` のときだけ証明書未検証通信を許可すること
- `http://` のときは `insecure` の値にかかわらず既存 HTTP 更新と同じ動作を維持すること
- 必須項目欠落、URL scheme 不正、`insecure` 型不正は設定不備として失敗扱いにすること

## Update Job Contract

### Triggers

- 起動時
- BOOT ボタン押下時

### Success Path

1. `config.txt` 読込成功
2. WiFi 接続成功
3. `image_url` の scheme と `insecure` から通信ポリシー確定
4. 画像取得成功
5. 取得画像の検証成功
6. e-paper 更新成功

### Transport Policy Contract

- `image_url` が `http://`:
  - 既存 HTTP 経路を使う
- `image_url` が `https://` かつ `insecure` が未設定または `false`:
  - サーバ証明書検証付き HTTPS を使う
- `image_url` が `https://` かつ `insecure: true`:
  - サーバ証明書未検証 HTTPS を使う

### Failure Path

- `insecure` 型不正や URL scheme 不正は通信開始前に設定不備として終了する
- HTTPS 証明書検証が必要な経路で検証に失敗した場合は HTTP 失敗として終了する
- `insecure: true` でも到達不能、HTTP status 異常、payload 不正、画像不正などは従来どおり失敗扱いにする
- 失敗種別を判断できる状態を残して終了する

### Compatibility Rule

- `.bmp` suffix による BMP / binary の経路選択は維持する
- `insecure` は route 選択条件に影響しない
- 既存の HTTP-only `config.txt` は修正なしで継続利用できる

# Quickstart: Config Insecure HTTPS

## 1. 既存 HTTP 更新の回帰確認

準備:

- `config.txt` の `image_url` に `http://...` を設定する
- `insecure` は省略するか `false` にする

期待結果:

- 起動時更新または BOOT ボタン更新が従来どおり成功する
- BMP / binary の経路選択は URL suffix に従って維持される
- failure category や表示完了の流れに新しい差分が出ない

## 2. 検証付き HTTPS 更新の確認

準備:

- `config.txt` の `image_url` に `https://...` を設定する
- サーバ証明書は通常検証で通過できる状態にする
- `insecure` は省略するか `false` にする

期待結果:

- 更新処理が成功する
- `insecure` を使わなくても HTTPS 更新できる
- 既存の成功時フローで e-paper 更新完了まで到達する

## 3. 未検証 HTTPS 更新の確認

準備:

- `config.txt` の `image_url` に `https://...` を設定する
- サーバ証明書は通常検証では通過できない状態にする
- `insecure: true` を設定する

期待結果:

- 更新処理が証明書未検証 HTTPS 経路で成功する
- 画像取得から表示更新完了まで到達する
- 到達不能や payload 不正など証明書以外の失敗は成功扱いにならない

## 4. 安全側既定値の確認

準備:

- `config.txt` の `image_url` に `https://...` を設定する
- サーバ証明書は通常検証では通過できない状態にする
- `insecure` は省略するか `false` にする

期待結果:

- 更新処理は成功扱いにならない
- 失敗は HTTP / 通信失敗として記録される
- `insecure` を明示しない限り未検証通信へ自動移行しない

## 5. 設定不備の確認

準備:

- `config.txt` の `insecure` に boolean 以外の値を設定する

期待結果:

- 通信開始前に設定不備として失敗する
- 画像取得失敗ではなく config error として切り分けられる

## 6. ビルド確認

確認:

- `idf.py build` が成功する
- firmware 文書に `insecure` の既定値、用途、注意点が追記されている

# Quickstart: RunPod Authenticated Fetch

## 1. Bearer 認証付き HTTPS 更新の確認

準備:

- `config.txt` の `image_url` に `https://...` を設定する
- 有効な `bearer_token` を設定する
- 更新元は Bearer 認証を要求し、通常の証明書検証を通過できる状態にする

期待結果:

- 起動時更新または BOOT ボタン更新が成功する
- `Authorization: Bearer <token>` を付けた画像取得で表示更新まで到達する
- BMP / binary の経路選択は既存の URL suffix ルールを維持する

## 2. Bearer 認証付き未検証 HTTPS 更新の確認

準備:

- `config.txt` の `image_url` に `https://...` を設定する
- 有効な `bearer_token` を設定する
- `insecure: true` を設定する
- 更新元は Bearer 認証を要求し、通常の証明書検証では通過できない状態にする

期待結果:

- 認証付き更新が成功する
- `insecure: true` を明示した場合だけ未検証 HTTPS を使う
- 証明書以外の失敗は成功扱いにならない

## 3. 安全側既定値の確認

準備:

- `config.txt` の `image_url` に `https://...` を設定する
- 有効な `bearer_token` を設定する
- `insecure` は省略するか `false` にする
- 更新元は通常の証明書検証では通過できない状態にする

期待結果:

- 更新処理は成功扱いにならない
- `insecure` を明示しない限り未検証 HTTPS へ自動移行しない

## 4. 認証失敗の確認

準備:

- `config.txt` の `image_url` に `https://...` を設定する
- `bearer_token` には無効な値を設定する

期待結果:

- 更新処理は成功扱いにならない
- 設定不備ではなく認証または HTTP 失敗として切り分けられる

## 5. 設定不備の確認

準備:

- `bearer_token` に空文字または string 以外の値を設定する
- または `insecure` に boolean 以外の値を設定する

期待結果:

- 通信開始前に設定不備として失敗する
- 認証失敗や通信失敗と区別できる

## 6. HTTP 回帰確認

準備:

- `config.txt` の `image_url` に `http://...` を設定する
- `bearer_token` は省略する
- `insecure` は省略するか `false` にする

期待結果:

- 既存の HTTP 更新フローが維持される
- firmware の build と表示更新の流れに回帰がない

## 7. ビルド確認

確認:

- `idf.py build` が成功する
- firmware 文書に `bearer_token` と `insecure` の既定値、用途、注意点が追記されている

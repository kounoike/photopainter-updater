# Firmware

`firmware/` は `xiaozhi-esp32/` を参照して作る専用ファームウェアです。`xiaozhi-esp32/` 自体は書き換えず、`firmware/` 側から必要な component を参照します。

## Merged Image の作成

このリポジトリでは devcontainer 環境を前提にします。devcontainer で起動したシェルから、次のスクリプトを実行してください。

```bash
./scripts/build-merged-image.sh
```

生成物は次に出力されます。

- `firmware/build/merged-flash.bin`

既存 build をそのまま使う場合は次も使えます。

```bash
./scripts/build-merged-image.sh --skip-build
```

## Devcontainer の認証キャッシュ

このリポジトリの devcontainer は `codex` と `claude` を image build 時点で含みます。初回接続時に CLI の追加インストール待ちは不要です。

- `codex` の認証保存先: `/home/vscode/.codex`
- `claude` の認証保存先: `/home/vscode/.claude`
- どちらも devcontainer named volume に mount されるので、rebuild/recreate 後も認証状態を再利用できます。

初回利用手順:

1. devcontainer を build/create する。
2. 接続後に `codex` と `claude` を起動する。
3. 必要なら通常手順でログインする。
4. 以後は devcontainer を rebuild/recreate しても同じ認証状態を使う。

意図的に認証キャッシュを初期化したい場合は、devcontainer を停止したうえで host 側から次を実行します。

```bash
docker volume rm photopainter-updater-codex-config photopainter-updater-claude-config
```

次回接続時は `codex` と `claude` の初回認証からやり直します。API key を `.devcontainer/.env` で渡す運用を使う場合は、volume を消しても環境変数ログインは引き続き有効です。

## 書き込み

書き込みには [ESP Launchpad](https://espressif.github.io/esp-launchpad/) を使います。ESP Launchpad の DIY モードでは、ローカルにある pre-built firmware image を選んで flash address を指定して書き込めます。

手順:

1. ボードの `PWR` ボタンを長押しして、いったん電源を切る。
2. `BOOT` ボタンを押しながら `PWR` ボタンを短く押して、転送モードで起動する。
3. ブラウザで [ESP Launchpad](https://espressif.github.io/esp-launchpad/) を開き、上部メニューの `Connect` ボタンでボードへ接続する。
4. 画面上で `DIY` を選び、ローカルの firmware image を自分で指定するモードへ切り替える。
5. local storage から `firmware/build/merged-flash.bin` を選ぶ。
6. flash address を `0x0` に設定して、書き込みボタンを押す。
7. 書き込み完了後、`PWR` ボタンを長押しして、いったん電源を切る。
8. `BOOT` ボタンは押さずに `PWR` ボタンを短く押して、通常起動する。

## Local HTTP Server

PhotoPainter の画像取得先として最小のローカル HTTP サーバを使う場合は、`server/contents/image.bmp` を配置してから `server/run.sh` を実行します。

```bash
cd server
./run.sh
```

既定では `http://127.0.0.1:8000/` と `http://127.0.0.1:8000/image.bmp` の両方で同じ `image.bmp` を返します。`image.bmp` が未配置のときは `404 Not Found` を返します。

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

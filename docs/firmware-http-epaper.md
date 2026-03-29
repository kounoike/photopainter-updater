# firmware HTTP e-paper

`firmware/` は `xiaozhi-esp32/` を参照して作る専用ファームウェアです。`xiaozhi-esp32/` 自体は書き換えず、`firmware/` 側から `sdcard_bsp`、`button_bsp`、`epaper_port`、`epaper_src` を component として参照します。

## config.txt

SD カードのルートに `config.txt` を配置します。

```json
{
  "wifi_ssid": "your-ssid",
  "wifi_password": "your-password",
  "image_url": "http://your-server/image.bmp"
}
```

制約:

- `config.txt` は SD カードルート固定です。
- 中身は JSON です。
- `image_url` は単一の `http://...` URL です。
- 取得画像は 24-bit BMP を前提にしています。
- 画像サイズは e-paper の表示範囲内である必要があります。
- `debug.txt` を同じ SD カードルートに空ファイルで置くと、開発用に deep sleep を抑止できます。
- BOOT 長押しで NVS 上の開発モードを切り替えできます。開発モード中は deep sleep と ACT LED をまとめて無効化します。

## 動作

1. 起動時に `/sdcard/config.txt` を読む
2. `wifi_ssid` / `wifi_password` で WiFi に接続する
3. `image_url` から BMP を `/sdcard/download.bmp` に取得する
4. 取得した BMP を e-paper に描画する
5. 起動後は BOOT ボタン単押しで同じ更新ジョブを再実行する
6. 更新ジョブ中は ACT LED として Green LED (`GPIO42`) が約 500ms 間隔で点滅し、完了または失敗後は消灯する

## 失敗時

- 設定不備、WiFi 失敗、HTTP 失敗、画像不正のいずれでも更新を継続しません。
- 失敗種別は NVS の `firmware` namespace に `last_failure` / `last_trigger` / `last_detail` として保存します。
- 失敗後は deep sleep に入ります。

## ビルド

このリポジトリの devcontainer は ESP-IDF v5.5.1 を `/opt/esp/idf` に入れる前提です。`codex` と `claude` は image build 時点で同梱され、認証状態は `~/.codex` と `~/.claude` の named volume に保持されます。devcontainer を再ビルドした後も、その認証状態を保ったまま `firmware/` をプロジェクトルートとして扱えます。

```bash
idf.py -C firmware set-target esp32s3
idf.py -C firmware build
idf.py -C firmware flash monitor
```

ターゲットは実機に合わせて変更してください。

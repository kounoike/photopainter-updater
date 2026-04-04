# firmware HTTP e-paper

`firmware/` は `xiaozhi-esp32/` を参照して作る専用ファームウェアです。`xiaozhi-esp32/` 自体は書き換えず、`firmware/` 側から `sdcard_bsp`、`button_bsp`、`epaper_port`、`epaper_src` を component として参照します。

このファームウェアの build target は `esp32s3` 固定です。`esp32` など他 target は対象外です。

## config.txt

SD カードのルートに `config.txt` を配置します。

```json
{
  "wifi_ssid": "your-ssid",
  "wifi_password": "your-password",
  "image_url": "https://your-server/image.bmp",
  "bearer_token": "your-bearer-token",
  "insecure": false
}
```

制約:

- `config.txt` は SD カードルート固定です。
- 中身は JSON です。
- `image_url` は単一の `http://...` または `https://...` URL です。
- `bearer_token` は任意です。設定した場合だけ `Authorization: Bearer <token>` を送ります。
- `bearer_token` を設定する場合、空文字は不可です。
- `insecure` は任意 boolean です。未設定時は `false` として扱います。
- `insecure: true` は `https://...` のときだけ意味を持ち、証明書未検証通信を許可します。
- 取得画像は 24-bit BMP を前提にしています。
- 画像サイズは e-paper の表示範囲内である必要があります。
- `debug.txt` を同じ SD カードルートに空ファイルで置くと、開発用に deep sleep を抑止できます。
- BOOT 長押しで NVS 上の開発モードを切り替えできます。開発モード中は deep sleep と ACT LED をまとめて無効化します。

## 動作

1. 起動時に `/sdcard/config.txt` を読む
2. `wifi_ssid` / `wifi_password` で WiFi に接続する
3. `bearer_token` があれば `Authorization: Bearer <token>` を付けて `image_url` から画像を取得する
4. `https://...` では既定で証明書検証を行い、`insecure: true` のときだけ未検証通信を使う
5. BMP 経路では `/sdcard/download.bmp` に取得し、binary 経路では直接描画バッファへ読み込む
6. 取得した画像を e-paper に描画する
7. 起動後は BOOT ボタン単押しで同じ更新ジョブを再実行する
8. 更新ジョブ中は ACT LED として Green LED (`GPIO42`) が約 500ms 間隔で点滅し、完了または失敗後は消灯する

## 失敗時

- 設定不備、WiFi 失敗、HTTP 失敗、画像不正のいずれでも更新を継続しません。
- `bearer_token` 型不正、空文字、`insecure` 型不正は設定不備として扱います。
- 認証拒否や証明書検証失敗は HTTP 失敗として扱います。
- 失敗種別は NVS の `firmware` namespace に `last_failure` / `last_trigger` / `last_detail` として保存します。
- 失敗後は deep sleep に入ります。

## ビルド

このリポジトリの devcontainer は ESP-IDF v5.5.1 を `/opt/esp/idf` に入れる前提です。`codex` と `claude` は image build 時点で同梱され、認証状態は `~/.codex` と `~/.claude` の named volume に保持されます。devcontainer を再ビルドした後も、その認証状態を保ったまま `firmware/` をプロジェクトルートとして扱えます。

```bash
idf.py -C firmware set-target esp32s3
idf.py -C firmware build
idf.py -C firmware flash monitor
```

通常は `./scripts/build-merged-image.sh` の利用を優先してください。誤って別 target の `sdkconfig` を作った疑いがある場合は、`idf.py -C firmware fullclean` の後に `set-target esp32s3` からやり直してください。

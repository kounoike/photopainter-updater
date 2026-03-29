# Quickstart: SDカード設定 HTTP e-paper 更新ファーム

## 目的

SDカードルートの `config.json` を読んで WiFi 接続し、起動時と BOOT ボタン押下時に画像取得して e-paper を更新する専用ファームを実装・検証する。

## 実装手順

1. `spec.md`、`plan.md`、`research.md`、`data-model.md`、`contracts/config-and-update-contract.md` を確認する。
2. `firmware/` を ESP-IDF プロジェクトルートとして初期化し、`xiaozhi-esp32/components/` 配下の `sdcard_bsp`、`button_bsp`、`epaper_port`、`epaper_src` を参照 component にする。
3. `firmware/main/config.*` に `/sdcard/config.json` 読込と `wifi_ssid` / `wifi_password` / `image_url` の必須項目検証を実装する。
4. `firmware/main/update_job.*` に起動時更新、BOOT ボタン更新、直列実行制御、WiFi 接続、失敗時 deep sleep を実装する。
5. `firmware/main/display_update.*` に `esp_http_client` を使った BMP ダウンロード、BMP 検証、`GUI_ReadBmp_RGB_6Color()` と `epaper_port_display()` を使った描画更新を実装する。
6. `firmware/main/failure_state.*` に失敗種別の記録と NVS への保持を実装する。
7. 利用者向け手順を `docs/firmware-http-epaper.md` にまとめる。

## 検証手順

1. 正常系 1: SDカードルートに正しい `config.json` を置いて起動し、60 秒以内に画像更新が完了することを確認する。
2. 正常系 2: 起動後に BOOT ボタンを押し、60 秒以内に画像再取得と表示更新が完了することを確認する。
3. 失敗系 1: `config.json` を欠落させ、更新処理が失敗理由を判断できる形で終了し、シャットダウンすることを確認する。
4. 失敗系 2: WiFi 接続失敗を発生させ、更新処理が終了しシャットダウンすることを確認する。
5. 失敗系 3: HTTP 取得失敗を発生させ、更新処理が終了しシャットダウンすることを確認する。
6. 失敗系 4: 24-bit BMP 以外、または表示範囲外の画像を返し、画像不正として終了しシャットダウンすることを確認する。

## 完了条件

- `config.json` の必須項目で起動時更新が動作する。
- BOOT ボタン押下で再更新が動作する。
- 更新ジョブは同時実行されない。
- 失敗時は失敗種別を NVS に残した状態で終了し、シャットダウンする。

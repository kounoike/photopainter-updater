# Quickstart: SDカード設定 HTTP e-paper 更新ファーム

## 目的

SDカードルートの `config.json` を読んで WiFi 接続し、起動時と BOOT ボタン押下時に画像取得して e-paper を更新する専用ファームを実装・検証する。

## 実装手順

1. `spec.md`、`plan.md`、`research.md`、`data-model.md`、`contracts/config-and-update-contract.md` を確認する。
2. `xiaozhi-esp32/main/main.cc` と既存モード分岐を確認し、起動時更新の入口を決める。
3. `components/sdcard_bsp` と `components/json_bsp` を確認し、`config.json` 読込方法を整理する。
4. `components/button_bsp` を確認し、BOOT ボタン押下で更新ジョブを開始する流れを整理する。
5. `components/http_client_bsp` または既存 HTTP 周辺部品を確認し、単一 URL 画像取得を組み込む。
6. `components/epaper_port` または既存 e-paper 描画経路を確認し、画像表示更新処理を実装する。
7. 失敗時は失敗種別を判断できる状態を残して更新処理を終了し、シャットダウンする。

## 検証手順

1. 正常系 1: SDカードルートに正しい `config.json` を置いて起動し、60 秒以内に画像更新が完了することを確認する。
2. 正常系 2: 起動後に BOOT ボタンを押し、60 秒以内に画像再取得と表示更新が完了することを確認する。
3. 失敗系 1: `config.json` を欠落させ、更新処理が失敗理由を判断できる形で終了し、シャットダウンすることを確認する。
4. 失敗系 2: WiFi 接続失敗を発生させ、更新処理が終了しシャットダウンすることを確認する。
5. 失敗系 3: HTTP 取得失敗を発生させ、更新処理が終了しシャットダウンすることを確認する。
6. 失敗系 4: 表示不能画像を返し、画像不正として終了しシャットダウンすることを確認する。

## 完了条件

- `config.json` の必須項目で起動時更新が動作する。
- BOOT ボタン押下で再更新が動作する。
- 更新ジョブは同時実行されない。
- 失敗時は失敗種別を区別できる状態で終了し、シャットダウンする。

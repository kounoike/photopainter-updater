# Quickstart: HTTP画像の直接表示検討

## 目的

HTTP 取得画像を SD カードへ保存せず直接表示できるかを評価し、今回の採用判断と運用影響を確認する。

## 調査手順

1. [spec.md](/workspaces/photopainter-updater/specs/007-stream-http-render/spec.md) と [research.md](/workspaces/photopainter-updater/specs/007-stream-http-render/research.md) を確認する。
2. `firmware/main/display_update.cc` で HTTP 応答が `/sdcard/download.bmp` へ書かれていることを確認する。
3. `xiaozhi-esp32/components/epaper_src/GUI_BMPfile.c` で `GUI_ReadBmp_RGB_6Color()` がファイルパスを受け取り、BMP 全体を読み込む実装であることを確認する。
4. 直接表示を採用するには新規ストリーム BMP デコーダ相当の実装が必要かを判断する。
5. 今回は採用しない結論とし、その理由を文書へ反映する。

## 検証手順

1. `idf.py -C firmware build` が継続して成功することを確認する。
2. `docs/firmware-http-epaper.md` で、現行方式が `/sdcard/download.bmp` を一時利用すると説明されていることを確認する。
3. `docs/firmware.md` の build / flash 手順とは独立に、更新方式の説明が矛盾していないことを確認する。
4. `research.md` と `contracts/render-path-contract.md` に、direct stream render を採用しない理由が明記されていることを確認する。

## 完了条件

- HTTP 取得後の描画が現行どおり `/sdcard/download.bmp` を経由することを文書で説明できる。
- direct stream render を採用しない理由が、API 形状、メモリ前提、検証コストの観点で説明されている。
- 利用者向け文書と開発文書の説明が一致している。

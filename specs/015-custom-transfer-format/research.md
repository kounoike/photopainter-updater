# Research: 独自画像転送形式追加

## Decision 1: `/image.bin` は e-paper 表示バッファ互換の packed 4bit frame buffer を返す

- Decision: `/image.bin` の payload は、e-paper の 6 色 index を 4bit nibble で 2 画素ずつ詰めた display-ready な frame buffer とし、firmware は検証後にそのまま表示バッファへコピーできる形にする。
- Rationale: 現行 firmware は BMP を SD カードへ保存し、再度ファイルとして読み直した上で `GUI_ReadBmp_RGB_6Color` が `Paint_SetPixel` を通じて同じ 4bit 表示バッファへ変換している。server 側で最終色 index とパッキングを済ませれば、firmware 側では BMP 解析もファイル I/O も不要になる。
- Alternatives considered:
  - 行単位の palette index 配列を送り、firmware が `Paint_SetPixel` で再構築する: BMP 解析は減るが、毎画素変換と描画ループは残り、転送後の CPU 負荷が高い。
  - RGB888 の独自形式を送る: 汎用形式依存は減るが、firmware 側に色変換処理が残り、中間最適化の価値が薄い。
  - BMP を chunked で直接読む: 24bit BMP 解析と row padding 処理が残り、独自形式導入の利点が弱い。

## Decision 2: 独自形式には固定長ヘッダを付け、magic/version/dimensions/payload length/checksum で完了判定する

- Decision: `/image.bin` の応答は「固定長ヘッダ + payload」とし、ヘッダには magic、version、width、height、payload length、payload checksum を含める。
- Rationale: firmware は受信途中で「期待サイズどおり終わったか」「別バージョンの形式ではないか」を判断する必要がある。BMP のような複雑なヘッダではなく、更新処理に必要な最小情報だけを持つ固定長ヘッダにすると、途中中断や形式不整合の切り分けが容易になる。
- Alternatives considered:
  - checksum なしで長さだけを見る: 部分破損や誤データ混入の検出力が弱い。
  - JSON ヘッダを前置する: 人には読みやすいが、firmware の解析処理が増え、固定長ヘッダより複雑になる。
  - ヘッダなしで raw bytes のみ送る: route の取り違えやバージョン不整合を安全に検出しにくい。

## Decision 3: `/` と `/image.bmp` は既存 BMP 応答を維持し、firmware は `image_url` 末尾が `.bin` かどうかで経路を選ぶ

- Decision: server は `/` と `/image.bmp` を変更せず、独自形式は `/image.bin` 追加のみとする。firmware は既存 `config.txt` の `image_url` 文字列末尾が `.bin` なら独自形式経路、それ以外なら BMP 経路を使う。
- Rationale: 現在の `config.txt` は `http://` の URL を 1 つ保持できるため、新しい設定キーを増やさなくても独自形式経路へ移行できる。末尾判定なら設定追加なしで経路選択が明確になり、BMP 経路を維持することで人手検証や既存クライアントの互換性も残せる。
- Alternatives considered:
  - `Accept` ヘッダで同一路由を分岐する: server/firmware 両方の条件分岐が増え、切り分けも難しくなる。
  - `/image.bmp` を独自形式に置き換える: 既存互換を壊すため不採用。
  - config に別キーを追加する: 将来はあり得るが、今回の最小変更方針には過剰。

## Decision 4: firmware は独自形式受信時に SD カード保存を行わず、検証済み payload を直接表示する

- Decision: firmware には `DownloadImageToSdCard` / `RenderBmpFromSdCard` と並ぶ独自形式用の更新経路を追加し、HTTP 受信データを一時ファイル化せずに RAM 上で検証し、そのまま `epaper_port_display` へ渡す。
- Rationale: 目的は「保存不要」であり、BMP 保存を残したまま route だけ増やしても価値がない。現在も描画用バッファ `s_epaper_image` は RAM に常駐するため、そのサイズに収まる display-ready payload なら中間保存を省ける。
- Alternatives considered:
  - 独自形式でも一度 SD カードへ保存する: 汎用形式依存は減るが、保存不要という主要求を満たさない。
  - 全受信完了後に別バッファへ変換してからコピーする: 安全だが、既存の表示バッファを直接使うよりメモリ負荷が高い。

## Decision 5: 失敗分類は通信失敗、入力画像失敗、形式失敗の 3 系統を明示する

- Decision: 独自形式経路の失敗は、HTTP 通信失敗、server 側の入力画像起因失敗、独自形式ヘッダ/サイズ/checksum 不整合の 3 系統に分けて扱う。
- Rationale: spec の SC-003 は 1 回の確認で切り分けられることを求めている。BMP 経路より専用性が高くなるぶん、「通信は成功したが形式が壊れている」ケースを独立させる必要がある。
- Alternatives considered:
  - すべて `http_error` に寄せる: 切り分け価値が低い。
  - server/firmware の詳細内部理由まで細分化する: 今回の最小運用要件に対しては過剰。

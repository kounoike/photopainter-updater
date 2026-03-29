# Research: HTTP画像の直接表示検討

## Decision 1: HTTP 応答をそのまま e-paper に流し込む直接表示方式は今回採用しない

- Decision: `firmware/` は現行どおり、HTTP 取得画像を `/sdcard/download.bmp` に保存してから描画する方式を維持する。
- Rationale: `display_update.cc` は `esp_http_client` の `HTTP_EVENT_ON_DATA` で受信データをそのまま `FILE*` へ書き出し、その後 `GUI_ReadBmp_RGB_6Color(path, ...)` にファイルパスを渡して描画している。参照実装の `GUI_ReadBmp_RGB_6Color()` は `fopen()` と `fread()` を前提に BMP 全体を読み、さらに `width * height * 3` バイト相当の PSRAM バッファを確保して色変換してから `Paint_SetPixel()` へ流し込む。したがって「HTTP 受信しながらそのまま画面更新する」ためには、少なくとも BMP デコード API を path ベースから stream ベースへ全面置換し、行パディング、bottom-up 配列、ヘッダ検証、エラー分類を全部持ち直す必要がある。今回の価値に対して変更規模が大きく、既存の 005/006 で安定化した更新フローを崩すリスクが高い。
- Alternatives considered:
  - `esp_http_client` の chunk を直接 `Paint_SetPixel()` へ流し込む
    - 却下理由: 24-bit BMP はヘッダ確定、行パディング処理、bottom-up 反転を要し、現行 `epaper_src` と責務が重複する。
  - `GUI_ReadBmp_RGB_6Color()` を `FILE*` や callback ベースに作り替える
    - 却下理由: `xiaozhi-esp32/` は直接変更禁止であり、`firmware/` 側へ互換デコーダを新設すると実質的に大規模 fork になる。

## Decision 2: 直接表示の不採用理由は「API 形状」だけでなく「メモリと検証コスト」を含めて記録する

- Decision: 採用見送り理由は、path ベース API 前提、フル画像バッファ前提、BMP 固有の順序制約、既存 failure semantics 維持コストの 4 点を明示する。
- Rationale: 単に「今の API が path を受けるから無理」とだけ書くと、将来の再調査で同じ検討が繰り返される。実際には描画前に PSRAM バッファを使う実装であり、SD 書き込みだけを除去してもメモリ使用量や色変換コストは減らないため、採用価値が限定的である。
- Alternatives considered:
  - 「現時点では未対応」とだけ短く記録する
    - 却下理由: 将来の判断材料として弱く、SC-004 の説明責任を満たしにくい。

## Decision 3: 実装スコープは現行方式維持を前提とした文書整合と観測ポイント整理に限定する

- Decision: `firmware/` の更新ロジックを大きく作り替えず、必要なら表示経路のログや文書だけを整える。利用者向け文書には「現行では `/sdcard/download.bmp` を使う」と明記する。
- Rationale: 既存機能は実機で正常に動いており、今回の調査価値は「実現可否の判断」と「採用しない場合の理由の明文化」にある。現行方式を説明できれば利用者と開発者の期待値が揃う。
- Alternatives considered:
  - ストリーム描画の PoC を `firmware/` に一時実装する
    - 却下理由: PoC がそのまま保守負債になりやすく、Forbidden Scope ぎりぎりの複雑化を招く。

## Deferred Follow-up: サーバ側前処理済みバイナリ配信は将来案として有望

- Decision: HTTP サーバ側でデバイス向けの描画済み形式へ前処理し、その専用バイナリを配信する方式は将来の別 feature 候補として記録するが、本 feature では扱わない。
- Rationale: もしサーバ側で BMP 解析、色変換、行順変換を済ませたうえで、デバイスがそのまま描画バッファへ積める形式で配信できれば、`GUI_ReadBmp_RGB_6Color()` の path 依存と BMP 固有処理を回避しやすい。これは「BMP を直接 stream 描画する」案より実現性が高い。一方で、HTTP サーバの基本機能整備、配信フォーマット contract、途中切断時の完全性保証、firmware/server の同時変更が必要になるため、今回の `007` に含めるとスコープが広がりすぎる。
- Alternatives considered:
  - 今回の feature で server-side pre-render 方式まで一気に設計・実装する
    - 却下理由: HTTP サーバ基盤の整備状況に依存し、単一 feature の調査・文書化スコープを超える。

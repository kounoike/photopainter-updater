# Research: Hello 動作確認エンドポイント

## Decision 1: `/hello` は画像処理と分離した専用 route にする

- Decision:
  - `GET /hello` は既存の画像読込・変換処理を呼ばない専用 handler として扱う。
- Rationale:
  - 疎通確認 endpoint が画像状態に依存すると、本来切り分けたい「サーバが生きているか」と「画像入力が正常か」が混ざるため。
  - 既存 spec の FR-002 と SC-002 を満たすには、ファイル I/O や画像 decode の影響を受けない経路が最も単純で確実である。
- Alternatives considered:
  - `/` をそのまま疎通確認用に流用する: `image.png` 未配置時に失敗するため不適。
  - 既存 image route に特別な bypass を加える: 既存契約を複雑化し、回帰リスクが上がるため不採用。

## Decision 2: 応答は固定の成功メッセージを返す

- Decision:
  - `/hello` は `text/plain` の固定本文 `hello` を返す。
- Rationale:
  - 動作確認 endpoint の目的は payload の多様性ではなく、到達性とサーバ稼働の即時判定である。
  - 本文を `hello` に固定すると、README の確認例、route test、手動 `curl` 確認の期待値をぶらさずに済む。
  - 既存 `text_response` helper を使う設計に寄せると、応答生成とテストが単純になる。
- Alternatives considered:
  - JSON 形式の診断情報を返す: 現行 server の他 endpoint と比べて過剰で、spec で要求されていない。
  - 空 body で `204` を返す: 人手の `curl` 確認で判別しづらく、成功文言要件に弱い。

## Decision 3: logging と fallback の既存方針を維持する

- Decision:
  - `/hello` も既存 `record` 経路で access log を残し、未定義 path は従来どおり fallback の `404` とする。
- Rationale:
  - 運用上は `GET /hello` の成功と他 path の `404` を同じログ導線で見られる方が切り分けしやすい。
  - 新 route の追加だけで期待動作を拡張し、既存の not found 契約を崩さない方が回帰しにくい。
- Alternatives considered:
  - `/hello` だけログを省略する: 運用時の確認導線が分断されるため不採用。
  - fallback の本文や status を調整する: feature scope 外であり不要。

## Decision 4: 文書の最初の確認手順を `/hello` へ寄せる

- Decision:
  - `server/README.md` と quickstart では、最初の起動確認を `/hello` に変更し、その後に `/image.bmp` など既存 endpoint の確認を続ける。
- Rationale:
  - 利用者が最初に確認すべきなのは server 自体の到達性であり、画像依存 route は二段目の確認に分ける方が意図に合う。
  - 文書導線を整理すると、障害切り分けも「疎通 OK / 画像処理 NG」の順で説明できる。
- Alternatives considered:
  - 文書を更新せず実装だけ追加する: 新 endpoint が discover しづらく、FR-005 を満たさない。
  - 既存 `/image.bmp` だけを起動確認例として残す: 画像未配置時に false negative を招くため不採用。

# Research: 画像ディザリング回転配信

## Decision 1: 既存の `image.bmp` 直配信を `image.png` 入力のオンデマンド変換へ置き換える

- Decision: 配信対象は保存済み `image.bmp` ではなく `image.png` を入力にして、取得要求時に変換済み 24bit BMP を返す。
- Rationale: spec の中心要求は「PNG を変換して BMP として配信する」ことであり、別途 BMP を事前生成して配置する運用では価値が不足する。既存 route を維持したまま変換処理を配信前段へ入れるのが最小変更である。
- Alternatives considered:
  - 事前バッチで BMP を生成してから配信する: 差し替え即反映と変換責務の一体化が弱くなるため不採用。
  - 新しい route を追加して変換結果だけ配信する: 既存クライアント互換性を崩すため不採用。

## Decision 2: `ref/convert.py` は黒重複を含む 7 エントリ palette と Floyd-Steinberg を品質基準として参照する

- Decision: 既存参照変換の品質基準は `ref/convert.py` の palette と dithering 方針とし、サーバ実装側も同等の見た目を目標にする。palette は黒重複を含む 7 エントリで、実色としては 6 色と扱う。サイズ調整は今回の必須参照範囲に含めない。
- Rationale: user story 2 と FR-002 は参照変換との同等性を求めている。特に palette と dithering 方針を揃えることが出力傾向の一致に直結する。サイズ調整はユーザー要求に含まれず、今回の比較軸を曖昧にするため対象外とする。
- Alternatives considered:
  - 完全に別の量子化方式を採用する: 参照との比較軸が失われるため不採用。
  - 回転や彩度補正だけを追加してディザリングを簡略化する: 表示品質の重要要件を落とすため不採用。

## Decision 2b: 彩度強調は `pre.png` / `post.png` fixture と代表画素の許容差で判定する

- Decision: 彩度強調のテスト基準は `server/testdata/image-dither-rotate/pre.png` と `server/testdata/image-dither-rotate/post.png` を使い、代表座標 `(4,4)` `(12,4)` `(4,12)` `(20,12)` `(12,20)` `(4,28)` `(12,28)` `(20,28)` の RGB を各チャネル差 `±3` で比較する。
- Rationale: 「PhotoShop でいう彩度 +70 相当」は数式基準より fixture 基準の方が安定する。人間確認済みの `post.png` を正として固定し、代表座標まで明示すれば、実装差よりも見た目の差を直接検知できる。
- Alternatives considered:
  - 数式で完全再現する: 実装コストと誤差説明が大きいため不採用。
  - 平均彩度のような全体指標だけで判定する: 画素単位の逸脱を見逃しやすいため不採用。

## Decision 3: 彩度強調は変換パイプラインの前段で適用し、その後にディザリングと回転を行う

- Decision: 変換順序は「入力画像読込 → 彩度強調 → 参照相当ディザリング → 右 90 度回転 → 24bit BMP 応答」とする。
- Rationale: spec が順序を明示しており、彩度強調後の色空間でディザリングすることで最終出力の色傾向が安定する。回転は最終的な配信向きを固定する目的なので、ディザリング結果に対して適用する。
- Alternatives considered:
  - 回転を先に行う: 参照比較の基準順序とずれるため不採用。
  - 彩度強調をディザリング後に行う: パレット変換後の色を再編集することになり、期待する色味から外れやすいため不採用。

## Decision 4: 失敗応答は「入力画像未配置」と「変換失敗」を切り分ける

- Decision: `image.png` 不在と変換不能を、利用者が入力画像起因と判断できる失敗応答として返す。
- Rationale: 既存の `image.bmp` 未配置応答をそのまま使うと、今回の責務である PNG 変換失敗と区別できない。運用切り分けを容易にする必要がある。
- Alternatives considered:
  - すべて内部エラーとしてまとめる: 切り分け不能になるため不採用。

## Decision 5: 出力は 24bit BMP として返すが、外部 API 契約は既存 route のままにする

- Decision: インターフェース契約は `GET /` と `GET /image.bmp` のまま維持し、レスポンス内容だけを変換済み 24bit BMP へ更新する。
- Rationale: 既存の取得先互換性を保ちながら機能追加できる。利用者は取得 URL を変更せず、新しい変換結果だけを受け取れる。
- Alternatives considered:
  - 変換メタデータ用 endpoint を追加する: 今回のスコープを超えるため不採用。

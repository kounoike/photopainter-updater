# Trait Separation

この feature では、学習・caption・将来統合の境界を次の 3 つに分ける。

## `character fixed traits`

キャラクターの恒常要素。LoRA で優先して保持したい。

- 顔立ち
- 髪型、髪色、メッシュ
- 目の色、オッドアイ
- 耳、しっぽ、角などの身体的特徴
- 体格、髪シルエット

## `outfit variable traits`

衣装側の可変要素。将来の `keep / adapt / replace` で切り替えたい。

- 通常衣装
- ケープ、ドレス、ブーツなどの衣装部位
- リボン、装飾、季節小物
- クリスマス、ハロウィンなどのイベント衣装

## `scene variable traits`

LoRA ではなく生成時 prompt / 制御で扱う。

- 日時
- 季節
- イベント
- 天気
- pose
- framing
- 背景

## Caption への反映

- 初回 trial では `character fixed traits` を優先する
- `outfit variable traits` は必要最小限に留める
- `scene variable traits` は caption に入れない

## 将来拡張

- 1 LoRA trial が成立したら、必要に応じて `character core` と `outfit` の分離を再検討する
- 今回は 1 LoRA trial を優先し、完全分離学習は scope 外とする

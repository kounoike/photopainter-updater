# Quickstart: 写真調ディザリングの追加改善

## 目的

今回の feature で追加する写真調向け改善 profile を、既存上位候補と同条件で比較し、ローカル回帰と手動評価を行う。

## 1. ローカル回帰確認

```bash
cd /workspaces/photopainter-updater/server
cargo test
```

確認事項:

- 既存ルートと画像パイプラインの回帰がないこと
- 新 profile の設定解決が壊れていないこと
- 色域別の追加テストで、青寄り、高明度低彩度、肌寄り暖色に対する期待差分を確認できること

## 2. 比較起動

baseline との左右比較:

```bash
cd /workspaces/photopainter-updater/server
IMAGE_PROFILE=<new-profile> \
COMPARE_WITH_BASELINE=1 \
COMPARE_SPLIT=vertical \
cargo run --release
```

既存上位候補との上下比較:

```bash
cd /workspaces/photopainter-updater/server
IMAGE_PROFILE=<new-profile> \
COMPARE_PROFILE=color-priority \
DITHER_DIFFUSION_RATE=0.8 \
COMPARE_SPLIT=horizontal \
cargo run --release
```

## 3. 入力画像の使い分け

- `server/testdata/dither-result-check/image7.png`
  - 明るい低彩度面、肌の中間調、背景ボケの保持を見る
- `server/testdata/dither-result-check/image8.png`
  - 青系の広い面、明るい空、淡色背景の保持を見る
- `server/testdata/dither-result-check/image6.png`
  - イラスト調での大きな回帰がないかをざっくり確認する

比較時は、確認したい画像を `server/contents/image.png` に手動配置してから起動する。

## 4. 観察ポイント

- 青空や青い広い面が白灰へ吸われていないか
- 明るい低彩度面がざらつきすぎたり、白面化しすぎたりしていないか
- 肌や暖色中間調が平板になりすぎていないか
- 改善と引き換えに黒粒や偽色が増えていないか

## 5. 記録項目

比較結果には少なくとも次を残す:

| 項目 | 記録内容 |
|------|----------|
| `profile` | 例: 新しい写真調 profile |
| `input_image` | 例: `image7` / `image8` |
| `compare_target` | `baseline` または `color-priority + 0.8` |
| `observations` | 青保持、低彩度面、肌、中間調、ノイズ |
| `decision` | `advance` / `hold` / `reject` |
| `next_action` | 次に試す追加調整や確認事項 |

## 6. 完了条件

- `cargo test` が通る
- 少なくとも 2 種類の写真調画像で比較結果が残る
- 既存上位候補との関係を `advance` / `hold` / `reject` で説明できる

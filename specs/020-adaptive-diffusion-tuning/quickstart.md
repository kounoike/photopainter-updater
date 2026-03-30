# Quickstart: 写真調ディザリングの追加改善

## 目的

今回の feature では、写真調向け追加改善を試作した結果を文書として残し、既存上位候補 `color-priority + DITHER_DIFFUSION_RATE=0.8` の妥当性と、新たに判明した問題点を手動評価で確認する。

## 1. ローカル回帰確認

```bash
cd /workspaces/photopainter-updater/server
cargo test
```

確認事項:

- 既存ルートと画像パイプラインの回帰がないこと
- 追加実験コードを戻した後も現行 profile の挙動が壊れていないこと

## 2. 比較起動

`color-priority + DITHER_DIFFUSION_RATE=0.8` を基準として確認する:

```bash
cd /workspaces/photopainter-updater/server
IMAGE_PROFILE=color-priority \
DITHER_DIFFUSION_RATE=0.8 \
cargo run --release
```

## 3. 入力画像の使い分け

- `server/testdata/dither-result-check/image7.png`
  - 明るい低彩度面、肌の中間調、背景ボケの保持を見る
- `server/testdata/dither-result-check/image8.png`
  - 青系の広い面、明るい空、淡色背景、cyan/teal 系の服色を見る
- `server/testdata/dither-result-check/image6.png`
  - イラスト調での大きな回帰がないかをざっくり確認する

比較時は、確認したい画像を `server/contents/image.png` に手動配置してから起動する。

## 4. 観察ポイント

- 青空や青い広い面が白灰へ吸われていないか
- 明るい低彩度面がざらつきすぎたり、白面化しすぎたりしていないか
- 肌や暖色中間調が平板になりすぎていないか
- cyan/teal 系の服色が普通の水色や青へ単純化されていないか
- 改善と引き換えに黒粒や偽色が増えていないか

## 5. 記録項目

比較結果には少なくとも次を残す:

| 項目 | 記録内容 |
|------|----------|
| `input_image` | 例: `image7` / `image8` |
| `compare_target` | `color-priority + 0.8` |
| `observations` | 青保持、低彩度面、肌、中間調、ノイズ、cyan/teal 系衣服の見え方 |
| `decision` | `hold` / `reject` |
| `next_action` | 次に試す追加調整や確認事項 |

## 6. 観察メモのチェックリスト

実機確認の前後で、少なくとも次をメモする。

### `image7`

- [ ] 肌の中間調が残って見えるか
- [ ] 明るい低彩度の背景やボケが白面化しすぎていないか
- [ ] 背景のざらつきや黒粒が目立ちすぎないか
- [ ] 色を残した代わりに不自然な偽色が増えていないか
- [ ] 総合評価は `hold` / `reject` のどちらか

### `image8`

- [ ] 青系の広い面が白灰へ逃げすぎていないか
- [ ] 明るい空や背景で粒状感が悪化していないか
- [ ] cyan/teal 系の服色が単純な水色や青へ潰れていないか
- [ ] 白や黒への吸い込みが強すぎないか
- [ ] 総合評価は `hold` / `reject` のどちらか

## 7. 観察メモのテンプレート

`research.md` へ転記する前の下書きとして、そのまま使える。

```md
### image7

- compare_target: `color-priority + DITHER_DIFFUSION_RATE=0.8`
- skin_midtones:
- bright_low_saturation:
- noise_or_black_speckles:
- false_color_risk:
- decision:
- next_action:

### image8

- compare_target: `color-priority + DITHER_DIFFUSION_RATE=0.8`
- blue_retention:
- bright_background_grain:
- cyan_teal_clothing:
- side_effects:
- neutral_absorption:
- decision:
- next_action:
```

## 8. 完了条件

- `cargo test` が通る
- 少なくとも 2 種類の写真調画像で実機比較結果が残る
- 現行 `color-priority + 0.8` の継続可否と、新たに見つかった問題点を説明できる

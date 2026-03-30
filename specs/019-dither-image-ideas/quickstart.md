# クイックスタート: ディザリング向け画像改善比較

**Branch**: `019-dither-image-ideas` | **Date**: 2026-03-30

## 目的

同じ入力画像を手動で差し替えながら、改善 profile を切り替えて server を起動し、実機 ePaper 表示で比較する。

## 事前準備

1. 比較したい入力画像を `server/contents/image.png` に手動で配置する
2. firmware 側は既存の `image.bin` または `image.bmp` 取得設定のままにする
3. 比較結果を記録するメモ先を `specs/019-dither-image-ideas/` 配下に用意する

## 初回比較で使う profile

- `baseline`
- `no-sat-boost`
- `color-priority`
- `hue-guard`

## 基準 profile の確認

```bash
cd /workspaces/photopainter-updater/server
IMAGE_PROFILE=baseline ./run.sh
```

- 手動で配置した入力画像の基準表示を確認する
- 実機表示の所見を比較表の基準行として記録する

## 改善 profile の比較

```bash
cd /workspaces/photopainter-updater/server
IMAGE_PROFILE=<profile-key> ./run.sh
```

- 同じ入力画像を使って profile ごとの差分を確認する
- 粒状感、輪郭保持、色の自然さ、破綻リスクを同じ順序で記録する

## split view 比較の起動例

```bash
cd /workspaces/photopainter-updater/server
IMAGE_PROFILE=color-priority \
COMPARE_WITH_BASELINE=1 \
COMPARE_SPLIT=vertical \
./run.sh
```

```bash
cd /workspaces/photopainter-updater/server
IMAGE_PROFILE=hue-guard \
COMPARE_PROFILE=color-priority \
COMPARE_SPLIT=horizontal \
./run.sh
```

## 分割比較の活用

- 実験初期は、baseline と比較対象 profile を 1 枚の画像内で左右または上下に分割して比較する方法を優先する
- split view は既定では baseline と評価対象 profile の適用・未適用を比べる
- `COMPARE_PROFILE` を使うと、任意の 2 profile を同じ入力画像上で比較できる
- 同一表示条件で差分を見やすくできる一方、分割境界が見え方に影響する可能性がある
- split view で有望だった案は、最終的に全画面表示でも再確認する

## 比較記録テンプレート

比較後は少なくとも次を残す。

```text
profile:
input_image:
compare_mode:
split_direction:
device_result:
decision:
next_action:
```

## 追加画像での追試

```bash
cp /path/to/another-image.png /workspaces/photopainter-updater/server/contents/image.png
cd /workspaces/photopainter-updater/server
IMAGE_PROFILE=<profile-key> ./run.sh
```

- 現在の入力画像で判断しづらい懸念がある場合のみ、別画像へ手動差し替えして補足する
- 差し替えた画像と使った理由を記録する

## ローカル確認

```bash
cd /workspaces/photopainter-updater/server
cargo test
```

- 既存ルートと画像パイプラインの回帰がないことを確認する
- 実機主判定に進む前の最低限のローカル検証として使う

## 現在の到達点

- ローカルでは `cargo test` が通っている
- split view の起動は `IMAGE_PROFILE=color-priority COMPARE_WITH_BASELINE=1 COMPARE_SPLIT=vertical cargo run` で確認済み
- `image6` のようなイラスト調入力では `color-priority + DITHER_DIFFUSION_RATE=0.8` が暫定上位候補
- `image7` のような写真調入力でも `color-priority + DITHER_DIFFUSION_RATE=0.8` が比較的安定している
- ただし `image7` は青空を含まないため、写真調での青保持評価は未完了
- 次タスクでは、青空を含む写真調画像を追加し、青保持と明るい低彩度面の保持を別途確認する

## 完了条件

- 少なくとも 2 件以上の改善 profile で実機比較結果が残っている
- 上位候補と保留候補の理由が区別できる
- 次に具体化または採用判断へ進める候補が 1 件以上ある

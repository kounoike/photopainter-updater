# クイックスタート: ディザリング向け画像改善比較

**Branch**: `019-dither-image-ideas` | **Date**: 2026-03-30

## 目的

同じ入力画像を手動で差し替えながら、改善 profile を切り替えて server を起動し、実機 ePaper 表示で比較する。

## 事前準備

1. 比較したい入力画像を `server/contents/image.png` に手動で配置する
2. firmware 側は既存の `image.bin` または `image.bmp` 取得設定のままにする
3. 比較結果を記録するメモ先を `specs/019-dither-image-ideas/` 配下に用意する

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

## 分割比較の活用

- 実験初期は、baseline と比較対象 profile を 1 枚の画像内で左右または上下に分割して比較する方法を優先する
- split view は baseline と評価対象 profile の適用・未適用を同じ入力画像上で比較するために使う
- 同一表示条件で差分を見やすくできる一方、分割境界が見え方に影響する可能性がある
- split view で有望だった案は、最終的に全画面表示でも再確認する

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

## 完了条件

- 少なくとも 2 件以上の改善 profile で実機比較結果が残っている
- 上位候補と保留候補の理由が区別できる
- 次に具体化または採用判断へ進める候補が 1 件以上ある

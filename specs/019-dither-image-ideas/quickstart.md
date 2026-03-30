# クイックスタート: ディザリング向け画像改善比較

**Branch**: `019-dither-image-ideas` | **Date**: 2026-03-30

## 目的

固定画像セットを基準に、改善 profile を切り替えながら server を起動し、実機 ePaper 表示で比較する。

## 事前準備

1. `server/testdata/` または比較用ディレクトリに、固定画像セットを配置する
2. firmware 側は既存の `image.bin` または `image.bmp` 取得設定のままにする
3. 比較結果を記録するメモ先を `specs/019-dither-image-ideas/` 配下に用意する

## 基準 profile の確認

```bash
cd /workspaces/photopainter-updater/server
IMAGE_PROFILE=baseline EXPERIMENT_IMAGE_SET=baseline ./run.sh
```

- 固定画像セットの基準表示を確認する
- 実機表示の所見を比較表の基準行として記録する

## 改善 profile の比較

```bash
cd /workspaces/photopainter-updater/server
IMAGE_PROFILE=<profile-key> EXPERIMENT_IMAGE_SET=baseline ./run.sh
```

- 同じ固定画像セットで profile ごとの差分を確認する
- 粒状感、輪郭保持、色の自然さ、破綻リスクを同じ順序で記録する

## 追加画像での追試

```bash
cd /workspaces/photopainter-updater/server
IMAGE_PROFILE=<profile-key> \
EXPERIMENT_IMAGE_SET=baseline \
EXTRA_CONTENT_DIR=/path/to/extra-images \
./run.sh
```

- 固定画像セットでは判断しづらい懸念がある場合のみ追加画像で補足する
- 追加画像を使った理由を記録する

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

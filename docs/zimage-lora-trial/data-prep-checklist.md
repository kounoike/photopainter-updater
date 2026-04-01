# Data Prep Checklist

## 画像選定

- [ ] 顔、髪、目が十分見える
- [ ] 耳、しっぽ、ケープなどの `character fixed traits` が読み取れる
- [ ] 顔崩れが目立つ画像を除外した
- [ ] 他キャラ混在画像を除外した
- [ ] 背景や字幕が主役の画像を除外した

## 軽い前処理

- [ ] `512x512` 正規化の方針を決めた
- [ ] 左右反転は使わない判断をした
- [ ] 必要なら crop / resize / padding のみを適用した
- [ ] 背景簡素化は本当に必要な画像だけに限定する

## caption

- [ ] `.txt` sidecar caption を各画像に配置した
- [ ] caption は短い自然言語または comma-separated phrases にした
- [ ] pose / background / season を caption へ入れていない
- [ ] `character fixed traits` と `outfit variable traits` を混ぜすぎていない

## データ量

- [ ] 最初の trial 用に 8〜20 枚程度をそろえた
- [ ] 枚数不足時だけ `repeats` を増やす方針にした
- [ ] 顔が小さすぎる画像だけで構成していない

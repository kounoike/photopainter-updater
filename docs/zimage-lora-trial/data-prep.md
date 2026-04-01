# Data Prep Guide

## Goal

少数画像 trial で、背景やポーズではなく `character fixed traits` を優先して学習させる。

## 推奨画像

- 顔と髪がはっきり見える
- 両目または少なくとも目の配色が分かる
- 耳、しっぽ、ケープなど恒常パーツが見える
- キャラがフレーム内で主役
- 背景や字幕が少ない

## 避ける画像

- 顔が崩れている
- キャラが小さすぎる
- 他キャラや UI が大きく重なっている
- pose や背景演出だけが強く、キャラ特徴が読み取りにくい
- 実写風、3D 風、通常絵柄から大きく外れた絵

## 軽い前処理

- 許容: crop、resize、padding、背景簡素化の最小限利用
- 非推奨: 左右反転、強い色変換、pose 変更、AI による描き直し
- まずは `512x512` に正規化し、必要な画像だけ crop を調整する

## Caption 方針

- 短い自然言語または短いフレーズ列
- `character fixed traits` を優先
- pose / framing / background / season は入れない
- 例:
  - `some_girl, blonde hair with blue streaks, pink eyes, animal ears, tail`
  - `some_girl, soft youthful face, blonde hair, pink eyes, animal ears`

## Dataset Layout

- 学習画像: `scripts/zimage-lora/workspace/datasets/<character_id>/train/`
- caption: 同 basename の `.txt`
- validation prompt: `scripts/zimage-lora/templates/validation-prompts.txt` を基に render

## Repeats

- 画像枚数が少なく trainer が dataset を拾わない場合だけ増やす
- 最初の目安は `ZLORA_DATASET_REPEATS=20`
- データ品質が悪いまま `repeats` だけ増やさない

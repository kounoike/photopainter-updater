# Quickstart: Z-Image キャラクター LoRA 試作基盤

## 1. 参照画像を準備する

1. 1 キャラクターにつき少数の高品質画像を用意する
2. 顔、髪、衣装、付属特徴が十分に見える画像を優先する
3. 背景やポーズの癖が強すぎる画像だけで構成しない
4. 必要なら crop や左右反転などの軽い前処理だけを許容する

## 2. trial 学習条件を用意する

1. `Z-Image Turbo` を対象にする
2. 12GB 前提では `NF4 + bf16` を第一候補にする
3. `train_batch_size=1`、`gradient_checkpointing=true`、group offload を有効候補にする
4. 初期 trial 解像度は 512 系を使う

## 3. SimpleTuner を準備する

1. Python 3.10-3.13 が使えることを確認する
2. `simpletuner[cuda]` を導入する
3. trial 用の `config.json` と dataloader 設定を作る
4. `model_family=z-image`、`model_flavour=turbo`、`pretrained_model_name_or_path=TONGYI-MAI/Z-Image-Turbo` を設定する

## 4. trial 学習を開始する

1. 少数画像セットを dataloader から参照できる位置へ置く
2. 画像枚数が少なく dataset が認識されない場合だけ `repeats` を増やす
3. trial 学習を開始し、キャッシュ生成と学習進行を確認する
4. OOM の場合は解像度、量子化、offload 条件を優先的に縮退する

## 5. 最小再利用確認を行う

1. 生成された LoRA 成果物を確認する
2. validation prompt を使って生成画像を出す
3. 同一キャラクターとして識別可能かを確認する
4. trial 継続可否をメモに残す

## 6. 将来統合へ向けて残すべき情報

1. どの trial 条件で成立したか
2. どの恒常特徴が保持されたか
3. どの drift が残ったか
4. scene 可変要素と character 恒常要素の責務分界

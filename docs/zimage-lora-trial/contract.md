# Trial Contract Summary

この文書は [zimage-trial-lora-contract.md](/home/kounoike/ghq/github.com/kounoike/photopainter-updater/specs/024-zimage-character-lora/contracts/zimage-trial-lora-contract.md) の実装向け要約である。

## 入力

- `ReferenceImageSet`
  - `character_id`
  - `source_dir`
  - `image_count`
  - `quality_notes`
- `TrialTrainingProfile`
  - `model_family=z_image`
  - `model_flavour=turbo`
  - `base_model_precision=nf4-bnb` を第一候補
  - `mixed_precision=bf16` を第一候補
  - `resolution=512`
  - `train_batch_size=1`
  - `gradient_checkpointing=true`
  - `offload_mode=group offload`

## 出力

- 学習済み LoRA ファイル
- validation 画像群
- trial 実行メモ

## 最小再利用確認

- canonical path は「既存のローカル `Z-Image` 推論環境へ artifact と validation prompt を渡す」形に固定する
- 今回はその推論環境自体の構築までは保証しない
- `validate-reuse.sh` は manifest 生成と、必要なら `VALIDATE_REUSE_COMMAND` の実行を担当する

## Deferred

- ComfyUI の本格統合
- `keep / adapt / replace` の本実装
- 手動 inpaint 前提の個別修正

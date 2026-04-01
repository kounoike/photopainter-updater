# Data Model: Z-Image キャラクター LoRA 試作基盤

## `ReferenceImageSet`

- Purpose: 少数画像 trial 学習に投入する参照画像集合を表す。
- Fields:
  - `character_id`: 対象キャラクターを識別する短い ID。
  - `image_count`: 学習に使う画像枚数。
  - `source_dir`: 画像を配置するローカルディレクトリ。
  - `quality_notes`: 画質、背景、構図の注意点。
  - `augmentation_policy`: crop、左右反転など許容する軽い前処理の方針。
- Validation Rules:
  - 顔、髪、衣装、付属特徴のいずれも読み取りにくい画像ばかりで構成してはならない。
  - 画像枚数が少なすぎる場合は `repeats` などの補助条件が必要である。

## `TrialTrainingProfile`

- Purpose: 12GB trial 学習を成立させるための実行条件を表す。
- Fields:
  - `model_family`: `z-image`
  - `model_flavour`: `turbo`
  - `base_precision`: `nf4-bnb` または `int8`
  - `mixed_precision`: `bf16` または `fp16`
  - `resolution`: trial で使う解像度。初期値は 512 系。
  - `train_batch_size`: 初期値 1。
  - `gradient_checkpointing`: 有効/無効。
  - `offload_mode`: group offload などの縮退手段。
  - `lora_rank`: LoRA の rank。
  - `validation_prompts`: trial 判定用の prompt 集合。
- Validation Rules:
  - 12GB 前提では高解像度・大 batch を標準条件にしない。
  - VRAM 不足時は解像度・量子化・offload 条件で先に縮退する。

## `CharacterLoraArtifact`

- Purpose: trial 学習の出力として得られるキャラクター表現を表す。
- Fields:
  - `artifact_path`: 学習済み LoRA ファイルの保存先。
  - `training_profile_id`: 生成元の `TrialTrainingProfile` 参照。
  - `character_id`: 対象キャラクター ID。
  - `created_at`: 生成日時。
  - `validation_snapshot_dir`: validation 画像の保存先。
  - `status`: `generated`, `validated`, `rejected` のいずれか。
- State Transitions:
  - `generated` → 学習完了直後
  - `generated` → `validated` 試作再利用確認が通った場合
  - `generated` → `rejected` 同一キャラクターとして識別困難で継続不可と判断した場合

## `ReuseValidationRun`

- Purpose: 学習成果物を推論へ適用した最小再利用確認を表す。
- Fields:
  - `artifact_path`: 適用した LoRA。
  - `prompt_set`: 検証に使った prompt 群。
  - `output_dir`: 生成画像の保存先。
  - `identity_result`: 同一キャラクターとして識別可能かの判定。
  - `notes`: 崩れ、衣装 drift、色 drift などの観察結果。
- Validation Rules:
  - 少なくとも 1 つ以上の validation 画像を残す。
  - 学習ジョブ成功だけで `validated` にしてはならない。

## `IntegrationBoundary`

- Purpose: 学習済みキャラクター表現と将来の scene 可変要素の責務分界を表す。
- Fields:
  - `character_fixed_traits`: 顔、髪、衣装基調、付属特徴などの恒常要素。
  - `scene_variable_traits`: 日時、季節、イベント、天気、構図、ポーズなどの可変要素。
  - `outfit_mode`: `keep`, `adapt`, `replace` を想定する将来拡張点。
- Validation Rules:
  - `character_fixed_traits` と `scene_variable_traits` を同一責務として混在させない。
  - 今回の feature では `outfit_mode` の本格実装を含めない。

# Z-Image Trial LoRA Contract

**Branch**: `024-zimage-character-lora` | **Date**: 2026-04-01

## Purpose

少数画像からの `Z-Image` trial 学習を、入力束・実行条件・出力物・最小再利用確認の単位で再現可能にする。

## Inputs

### Reference Image Set

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `character_id` | string | yes | 対象キャラクターを識別する短い名前 |
| `source_dir` | path | yes | 学習画像を配置するローカルディレクトリ |
| `image_count` | integer | yes | 画像枚数 |
| `quality_notes` | string | no | 背景や画質に関する注意点 |

Rules:

1. 少数画像でも、顔・髪・衣装・付属特徴が読み取れること
2. 背景やポーズの癖が強すぎる画像だけで構成しないこと

### Trial Training Profile

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model_family` | enum | yes | `z-image` |
| `model_flavour` | enum | yes | `turbo` |
| `base_precision` | enum | yes | `nf4-bnb` または `int8` |
| `mixed_precision` | enum | yes | `bf16` または `fp16` |
| `resolution` | integer | yes | 初期 trial は `512` を標準とする |
| `train_batch_size` | integer | yes | 初期値 `1` |
| `gradient_checkpointing` | boolean | yes | 12GB trial では有効を標準とする |
| `offload_mode` | string | no | group offload などの縮退条件 |
| `lora_rank` | integer | yes | 低 rank を許容する |

Rules:

1. 12GB trial の標準条件を逸脱する場合は、追加メモリ確保根拠を残すこと
2. 高解像度 trial を標準にしないこと

## Outputs

| Output | Required | Description |
|--------|----------|-------------|
| 学習済み LoRA ファイル | yes | trial 学習で得られる成果物 |
| validation 画像群 | yes | 最小再利用確認に使う生成結果 |
| trial 実行メモ | yes | 成立条件、失敗条件、観察結果の要約 |

## Minimal Reuse Validation

trial 完了は、学習済み LoRA を推論へ適用し、少なくとも 1 つ以上の validation 画像について「同一キャラクターとして識別可能か」を判断できる状態を満たすこと。

完了条件:

1. 学習成果物の保存先が特定できる
2. validation 画像の保存先が特定できる
3. trial 継続可否を判断するメモが残る

## Deferred Integration

以下は今回の contract では保証対象外とし、後続 feature で扱う。

1. ComfyUI 全自動生成フローへの本格統合
2. `keep / adapt / replace` の衣装モード分岐実装
3. 個別部位の手動修正や inpaint 前提の運用

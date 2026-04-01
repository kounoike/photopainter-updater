# Z-Image LoRA Trial

このディレクトリは `024-zimage-character-lora` の implement 成果物として、12GB 級 GPU で `Z-Image Turbo` 向けキャラクター LoRA を試すための手順と運用メモをまとめる。

## Scope

- 対象: Docker ベースの `SimpleTuner` 環境で試作学習を開始し、LoRA 成果物と validation 画像を確認する
- 対象: 少数画像セット、caption、validation prompt を再利用できる形にそろえる
- 対象外: ComfyUI の本格統合、衣装モードの本実装、手動 inpaint 前提の個別調整

## Directory Map

- `scripts/zimage-lora/configs/trial-12gb.env.example`
  Docker / bind mount / token / trial 名称の共通環境変数雛形
- `scripts/zimage-lora/configs/trial-12gb.json`
  `Z-Image Turbo` / 12GB trial 向け trainer 設定テンプレート
- `scripts/zimage-lora/configs/multidatabackend.trial.json`
  少数画像 dataset と text embed cache の dataloader 雛形
- `scripts/zimage-lora/train-trial.sh`
  config render、Docker image build、`simpletuner train` 実行の wrapper
- `scripts/zimage-lora/validate-trial-layout.sh`
  dataset layout、caption、prompt library の事前検証
- `scripts/zimage-lora/validate-reuse.sh`
  LoRA 成果物と validation prompt を既存のローカル `Z-Image` 推論環境へ渡す最小再利用確認 wrapper
- `scripts/zimage-lora/templates/*`
  caption / dataset layout / validation prompt の雛形

## Quick Start

1. `scripts/zimage-lora/configs/trial-12gb.env.example` を `scripts/zimage-lora/configs/trial-12gb.env` としてコピーし、`HF_TOKEN`、`CHARACTER_ID`、`TRAINING_NAME` を埋める
2. `scripts/zimage-lora/templates/dataset-layout.md` に従って `scripts/zimage-lora/workspace/datasets/<character_id>/train/` を用意する
3. `scripts/zimage-lora/validate-trial-layout.sh --env-file scripts/zimage-lora/configs/trial-12gb.env` を実行する
4. `scripts/zimage-lora/train-trial.sh --env-file scripts/zimage-lora/configs/trial-12gb.env` を実行する
5. 出力先 `scripts/zimage-lora/workspace/output/<training_name>/` と validation 画像を確認する
6. 必要なら `VALIDATE_REUSE_COMMAND` を設定し、`scripts/zimage-lora/validate-reuse.sh` で既存のローカル推論環境へ渡す

## Runtime Layout

`train-trial.sh` は以下のディレクトリを自動で利用する。

- `scripts/zimage-lora/workspace/datasets/`
- `scripts/zimage-lora/workspace/runtime/`
- `scripts/zimage-lora/workspace/cache/`
- `scripts/zimage-lora/workspace/output/`
- `scripts/zimage-lora/workspace/huggingface/`
- `scripts/zimage-lora/workspace/reuse/`

これらは `scripts/zimage-lora/.gitignore` で除外される。

## Outputs

- 学習済み LoRA: `scripts/zimage-lora/workspace/output/<training_name>/`
- validation 画像: `scripts/zimage-lora/workspace/output/<training_name>/validation/` または trainer 出力先
- rendered config: `scripts/zimage-lora/workspace/runtime/<training_name>/`
- reuse manifest: `scripts/zimage-lora/workspace/reuse/<training_name>/`

## Failure Handling

- build 失敗時は Docker / NVIDIA runtime / driver を優先確認する
- startup OOM 時は `ZLORA_FORCE_INT8=1`、`ZLORA_FORCE_FP16=1`、`ZLORA_DISABLE_GROUP_OFFLOAD=1` ではなく、まず `ZLORA_MAX_TRAIN_STEPS` を下げずに量子化条件と offload 条件を見直す
- dataset 不備時は `validate-trial-layout.sh` を再実行し、画像枚数と `.txt` sidecar caption を修正する

## Notes

- `validate-reuse.sh` は既存のローカル `Z-Image` 推論環境を前提にし、今回その推論環境自体は構築しない
- `character fixed traits` と `outfit variable traits` の分離指針は [trait-separation.md](/home/kounoike/ghq/github.com/kounoike/photopainter-updater/docs/zimage-lora-trial/trait-separation.md) を参照する
- `Qwen Image Edit` ベース workflow から LoRA 方針へ切り替えた背景は [specs/024-zimage-character-lora/artifacts/comfyui/README.md](/home/kounoike/ghq/github.com/kounoike/photopainter-updater/specs/024-zimage-character-lora/artifacts/comfyui/README.md) を参照する

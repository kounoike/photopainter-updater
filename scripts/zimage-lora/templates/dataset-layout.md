# Dataset Layout Template

```text
scripts/zimage-lora/workspace/datasets/<character_id>/
└── train/
    ├── 001.png
    ├── 001.txt
    ├── 002.png
    ├── 002.txt
    └── ...
```

## Rules

- 学習画像と caption は同 basename にする
- caption は `.txt` sidecar で置く
- `train/` 直下に混在させてよい
- validation prompt は `scripts/zimage-lora/templates/validation-prompts.txt` から render する

## Notes

- 最初の trial は 1 LoRA 前提なので `face/body/outfit` に分離しない
- ただし caption は `character fixed traits` 優先にする
- `outfit variable traits` を強く学ばせたい場合は後続 feature で別 dataset を検討する

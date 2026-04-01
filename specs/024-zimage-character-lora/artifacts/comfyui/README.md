# ComfyUI Artifacts

このディレクトリは、`Qwen3-VL -> Qwen3.5 -> Qwen Image -> Qwen Image Edit` を使った参照画像差し替え方式の検討 artifact を保持する。

## Files

- `extract-image-feature.json`
  参照画像からキャラクター特徴を抽出するための実験 workflow
- `photopainter-image-generate.json`
  季節イベント prompt 生成と `Qwen Image Edit` によるキャラ差し替えを試した workflow
- `run.png`
  `Image 1` / `Image 2` / `Image Edit` 出力の比較スクリーンショット

## Why Kept

- scene 画像を先に作ってからキャラ差し替えする方式では、pose / scene と identity の綱引きが起きた
- `Image 1` を強くすると pose は保てるが character 置換が弱くなる
- `Image 2` を強くすると pose が参照画像へ寄る
- 全自動運用を前提にすると、個別 inpaint や手動補正を前提にしにくい

このため、今回の feature では `Qwen Image Edit` workflow の改善ではなく、`character fixed traits` を事前学習した LoRA 方針へ寄せた。

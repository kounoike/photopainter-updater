# Research: Z-Image キャラクター LoRA 試作基盤

## Decision 1: trial 学習のベースは `Z-Image Turbo` + `SimpleTuner` を採用する

- Decision: 少数画像からのキャラクター LoRA trial 学習は `SimpleTuner` の `Z-Image [base / turbo] Quickstart` を基準に、`Z-Image Turbo` を対象として進める。
- Rationale: `Z-Image` は 6B flow-matching transformer として案内されており、`Qwen Image` より軽く、12GB 級 GPU でも量子化・offload 前提の trial 条件が明記されている。`SimpleTuner` は `model_family=z-image`、`model_flavour=turbo`、`pretrained_model_name_or_path=TONGYI-MAI/Z-Image-Turbo` などの設定例を持ち、trial 学習の出発点として最も具体的である。
- Alternatives considered:
  - `Qwen Image` を採用する: 既存 ComfyUI workflow との親和性は高いが、ローカル 12GB 前提では trial 学習の成立性確認に不利。
  - 参照画像差し替え workflow を継続する: LoRA 学習準備は不要だが、全自動運用前提ではポーズ保持とキャラクター保持の綱引きが解けない。

## Decision 2: 12GB trial の標準条件は `NF4 + bf16`、`batch_size=1`、`gradient_checkpointing`、group offload とする

- Decision: 12GB 級 GPU の trial 条件は、`NF4 + bf16` を第一候補にし、`batch_size=1`、`gradient_checkpointing=true`、group offload を標準条件とする。必要に応じて `int8` と `--quantize_via=cpu` を fallback にする。
- Rationale: `SimpleTuner` の `Z-Image` quickstart では、`~10–12G VRAM when quantising to NF4 + bf16 base/LoRA weights` とされ、さらに lowest VRAM config として `nf4-bnb` or `int8`、`Resolution: 512px`、`Batch size: 1`、`Enable --gradient_checkpointing`、`Enable Ramtorch or group offload` が明示されている。trial の主目的は成立性確認なので、1024px や高 rank より VRAM 余裕を優先する。
- Alternatives considered:
  - 非量子化または int8 のみで進める: 12GB 条件では余裕が少なく、試作開始自体が不安定になる。
  - 1024px を標準解像度にする: `Z-Image` の本来解像度に近いが、trial 開始条件としては重く、まずは 512px で成立性確認した方がよい。

## Decision 3: 少数画像 trial では高品質少数画像 + 短い恒常特徴 caption を優先する

- Decision: データセットは「高品質な少数画像」を優先し、顔、髪、衣装、付属特徴が大きく見える画像を採用する。caption はキャラクター恒常特徴中心の短い表現に寄せ、背景やポーズを主情報にしない。画像枚数が少なすぎて dataset が検出されない場合のみ `repeats` を増やす。
- Rationale: `SimpleTuner` は `Image quality for training is critical; Z-Image will absorb artifacts early` とし、dataset considerations で `Increase repeats if you see no images detected in dataset` を案内している。trial の目的は背景やポーズ学習ではなくキャラクター恒常特徴の保持なので、caption も恒常特徴中心に制約した方が downstream の自然言語生成フローと責務分界しやすい。
- Alternatives considered:
  - 大量の低品質画像を集める: 収集は楽でも、artifact を早く吸い込み、trial の失敗原因がデータ品質か学習条件か切り分けにくい。
  - 長文で scene/pose まで caption する: trial 学習の責務が膨らみ、将来の scene 可変要素と衝突しやすい。

## Decision 4: trial 成果物の完了判定は「LoRA を再利用して同一キャラクターとして識別できる validation 画像を得られるか」とする

- Decision: 本 feature の最小再利用確認は、学習成果物を推論に適用し、同一キャラクターとして識別可能か判断できる validation 画像を 1 つ以上得ることとする。ComfyUI への本格統合は後続 feature に分離する。
- Rationale: spec のスコープは学習試作基盤と最小限の再利用確認までに限定されており、全自動生成フローへの本格統合は forbidden である。`SimpleTuner` は validation prompt と user prompt library を使った継続評価を案内しているため、trial では validation 画像を完了判定の中心にするのが自然である。
- Alternatives considered:
  - ComfyUI での end-to-end 自動生成確認まで今回に含める: scope が膨らみ、trial 学習基盤の成立性確認より workflow 実装に引っ張られる。
  - 学習ジョブ完了だけで成功扱いにする: downstream 利用可能性が見えず、LoRA trial として価値が弱い。

## Decision 5: 既存 ComfyUI workflow JSON は検討 artifact として保持し、trial 学習成果物とは責務を分ける

- Decision: `extract-image-feature.json`、`photopainter-image-generate.json`、`run.png` は `specs/024-zimage-character-lora/artifacts/comfyui/` に保持し、「参照画像差し替え方式で確認した制約」の根拠資料として扱う。
- Rationale: 既存 workflow は `Qwen3-VL -> Qwen3.5 -> Qwen Image -> Qwen Image Edit` の検討結果であり、なぜ LoRA 化へ寄せたかを説明する artifact として再利用価値がある。一方で、本 feature の deliverable は workflow 改良ではなく trial 学習基盤なので、成果物の責務を分離した方が整合的である。
- Alternatives considered:
  - workflow JSON を破棄する: 方針転換の根拠が失われる。
  - 本番運用 workflow として repo の主導線へ置く: trial 学習基盤の feature scope から外れ、誤って運用資産として見なされやすい。

# 実装計画: Z-Image キャラクター LoRA 試作基盤

**Branch**: `024-zimage-character-lora` | **Date**: 2026-04-01 | **Spec**: [spec.md](/home/kounoike/ghq/github.com/kounoike/photopainter-updater/specs/024-zimage-character-lora/spec.md)  
**Input**: `/specs/024-zimage-character-lora/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

`Z-Image` 系モデル向けに、少数の参照画像からキャラクター LoRA を試作学習できる最小構成を定義する。対象は VRAM 12GB 程度のローカル環境での成立性確認であり、Docker ベースの `SimpleTuner` 実行環境、量子化・offload 前提の学習条件、少数画像向けデータ準備方針、学習成果物の最小限の再利用確認手順、将来の ComfyUI 自動生成フローへ接続するための責務分界を成果物として整理する。既存の参照画像ベース差し替え workflow は検討 artifact として保持し、本 feature では個別 inpaint 前提ではない `character fixed traits` の学習済み表現へ軸足を移す。

## Technical Context

<!--
  ACTION REQUIRED:
  この節は実装前提を具体値で置き換える。
  不明点は推測せず NEEDS CLARIFICATION または TODO: を使う。
-->

**Language/Version**: Python 3.10-3.13、Bash (POSIX shell)、Markdown  
**Primary Dependencies**: Docker Engine / Docker Compose v2、NVIDIA Container Toolkit、`SimpleTuner`、`bitsandbytes` または同等の量子化バックエンド、Hugging Face Hub、既存 ComfyUI / `Z-Image` 推論環境  
**Storage**: ローカルファイルと Docker bind mount（参照画像セット、学習設定、キャッシュ、LoRA 成果物、validation 画像）  
**Testing**: 試作学習の手動実行、validation 画像による目視確認、学習成果物の最小限の再利用確認、手順書追従確認  
**Target Platform**: ローカル Linux 開発環境、単一 GPU（VRAM 12GB 程度）、将来の ComfyUI ローカル運用環境  
**Project Type**: ローカル学習試作基盤 + 運用手順 / 設定設計  
**Performance Goals**: 半日以内に 1 キャラクターの試作学習を開始できること、1 回の試作で同一キャラクターとして識別可能か判断できる validation 画像を得られること  
**Constraints**: 12GB 級 GPU、量子化と offload 前提、少数画像、全自動運用志向、手動 inpaint 前提禁止、ComfyUI への本格統合は後続 feature に分離  
**Scale/Scope**: 単一キャラクターの trial LoRA、少数画像セット、ローカル利用者 1 名、まずは trial 学習と最小限の再利用確認まで

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されている

Phase 1 再確認結果:

- [x] 追加設計は `specs/024-zimage-character-lora/`、将来必要となる `scripts/` / 設定ファイル配置方針、agent context 更新に限定されている
- [x] trial 学習を `Z-Image` + 量子化 + offload の最小構成へ絞ることで、ローカル優先・運用単純性を維持している
- [x] 検証手順は trial 学習開始、validation 画像確認、LoRA 再利用確認、将来統合前提整理の 4 観点を満たしている

## Project Structure

### Documentation (this feature)

```text
specs/024-zimage-character-lora/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── zimage-trial-lora-contract.md
├── artifacts/
│   └── comfyui/
│       ├── extract-image-feature.json
│       ├── photopainter-image-generate.json
│       └── run.png
└── tasks.md
```

### Source Code (repository root)
<!--
  ACTION REQUIRED:
  実際の構成に置き換える。未使用の選択肢は削除し、実パスのみ残す。
-->

```text
.
├── comfyui/
│   └── workflow-ui/
├── comfyui-data/
│   ├── input/
│   ├── models/
│   └── output/
├── scripts/
│   ├── run-codex.sh
│   └── run-claude.sh
│   └── zimage-lora/
├── specs/
│   └── 024-zimage-character-lora/
├── server/
├── docs/
└── compose.yml
```

**Structure Decision**: trial 学習の実装は、既存アプリ本体 (`server/`, `firmware/`) へ混ぜず、将来的に追加されるローカル実行補助 script と学習設定ファイル群を repo ルートの補助領域へ分離する。今回の plan では、まず `specs/024-zimage-character-lora/` に設計・検証・artifact を閉じ、後続 implement で必要最小限の `scripts/` と設定テンプレートを追加する前提を採る。

## Phase 0: Research Summary

詳細は [research.md](/home/kounoike/ghq/github.com/kounoike/photopainter-updater/specs/024-zimage-character-lora/research.md) を参照。

- trial 学習のベースは `Z-Image Turbo` + `SimpleTuner` を採用する
- 12GB 級 GPU では `NF4 + bf16`、`batch_size=1`、`gradient_checkpointing`、group offload を trial の標準条件とする
- 少数画像 trial では高品質画像を優先し、caption はキャラクター恒常特徴中心の短い表現に寄せる
- 学習成果物の再利用確認は、LoRA を推論に適用して同一キャラクターとして識別できる validation 画像を得ることを最小条件とする
- 既存 ComfyUI workflow artifact は「参照画像差し替え方式が不安定だった根拠」として保持し、trial 学習成果物とは責務を分ける

## Phase 1: Design & Contracts

### Data Model

詳細は [data-model.md](/home/kounoike/ghq/github.com/kounoike/photopainter-updater/specs/024-zimage-character-lora/data-model.md) を参照。

- `ReferenceImageSet`
- `TrialTrainingProfile`
- `CharacterLoraArtifact`
- `ReuseValidationRun`
- `IntegrationBoundary`

### Contracts

詳細は [zimage-trial-lora-contract.md](/home/kounoike/ghq/github.com/kounoike/photopainter-updater/specs/024-zimage-character-lora/contracts/zimage-trial-lora-contract.md) を参照。

- trial 学習の入力束、実行前提、出力物、最小再利用確認を contract として固定する
- 生成フロー統合は将来拡張としつつ、LoRA 成果物の受け渡し単位を明確化する

### Quickstart

詳細は [quickstart.md](/home/kounoike/ghq/github.com/kounoike/photopainter-updater/specs/024-zimage-character-lora/quickstart.md) を参照。

- 参照画像セットを用意する
- trial 学習設定を用意する
- 12GB 前提の trial 条件で学習を開始する
- validation 画像を確認する
- LoRA を最小限再利用して継続可否を判断する

## Implementation Strategy

1. trial 学習に必要な入力束、設定束、成果物束を contract と data model で固定する
2. `SimpleTuner` を前提にした 12GB 向け縮退条件を quickstart と research へ落とす
3. 少数画像向けデータ準備方針と caption 方針を整理する
4. LoRA 成果物の最小限の再利用確認手順を定義する
5. 将来の ComfyUI 自動生成フローとは、scene 可変要素、`character fixed traits`、`outfit variable traits` の責務分界だけを残して今回のスコープを閉じる

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| なし | - | - |

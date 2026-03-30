# データモデル: ディザリング向け画像改善アイデア整理

**Branch**: `019-dither-image-ideas` | **Date**: 2026-03-30

## 概要

この feature は永続 DB を追加しない。比較実験に必要な実行時設定と評価記録を、server 側の設定構造と `specs/019-dither-image-ideas/` の文書成果物として扱う。

## エンティティ

### 1. ImprovementProfile

画像加工の改善候補を一意に表す実験単位。

| フィールド | 型 | 説明 |
|-----------|----|------|
| `id` | 文字列 | profile の一意キー。起動設定と比較記録の双方で参照する |
| `label` | 文字列 | 比較表に表示する短い名称 |
| `stage_scope` | 列挙 | `preprocess` / `dither` / `hybrid` |
| `preprocess_steps` | 配列 | 彩度補正、コントラスト補正、局所調整など前処理の構成 |
| `dither_options` | 構造体 | 既存または新規のディザリング設定値 |
| `hypothesis` | 文字列 | 何を改善したい候補か |
| `risks` | 配列 | 想定される副作用、破綻条件 |
| `status` | 列挙 | `planned` / `tested` / `hold` / `rejected` / `promoted` |

初回実装 profile:
- `baseline`
- `no-sat-boost`
- `color-priority`
- `hue-guard`
- `color-priority-hue-guard`

### 2. EvaluationImage

比較時に手動で差し替える入力画像。

| フィールド | 型 | 説明 |
|-----------|----|------|
| `id` | 文字列 | 画像の一意キー |
| `label` | 文字列 | 比較表に表示する画像名 |
| `path` | パス | `CONTENT_DIR` に手動配置する入力画像の場所 |
| `coverage_tags` | 配列 | 無彩色階調、低彩度写真、高彩度領域、輪郭重視など |
| `is_reference_candidate` | 真偽値 | 代表比較画像候補かどうか |

### 3. ExperimentRun

1 回の比較実験の記録単位。

| フィールド | 型 | 説明 |
|-----------|----|------|
| `profile_id` | 文字列 | 試した改善 profile |
| `input_image_id` | 文字列 | 使用した入力画像 |
| `compare_mode` | 列挙 | `single` / `split-with-baseline` |
| `split_direction` | 列挙 | `vertical` / `horizontal` / `none` |
| `execution_mode` | 列挙 | `bmp-preview` / `binary-preview` / `device-check` |
| `observations` | 配列 | 粒状感、輪郭保持、色の自然さ、破綻など観察結果 |
| `decision` | 列挙 | `advance` / `hold` / `reject` |
| `next_action` | 文字列 | 次に何を試すか |

### 4. ComparisonCriterion

候補間を横並びで評価するための観点。

| フィールド | 型 | 説明 |
|-----------|----|------|
| `name` | 文字列 | 比較観点名 |
| `description` | 文字列 | 何を観察するか |
| `required` | 真偽値 | すべての実験で記録必須か |

必須観点:
- 粒状感
- 輪郭保持
- 色の自然さ
- 破綻リスク
- 適用しやすい画像条件

## 関係

- `ImprovementProfile` 1 件に対し `ExperimentRun` は複数回ありうる
- `EvaluationImage` 1 件を複数 profile が共有する
- `ExperimentRun` は必須の `ComparisonCriterion` をすべて記録対象に持つ

## 状態遷移

### ImprovementProfile

```text
planned -> tested -> promoted
planned -> tested -> hold
planned -> tested -> rejected
hold -> tested
```

- `planned`: 実験前
- `tested`: 同じ入力画像で比較済み
- `hold`: 条件付きで保留
- `rejected`: 今回対象外
- `promoted`: 次の具体化または採用判断へ進める

## バリデーションルール

- `id` は比較記録内で一意でなければならない
- `ExperimentRun` は少なくとも 1 つの `EvaluationImage` を参照しなければならない
- `compare_mode=split-with-baseline` の run には `split_direction` が必須
- `decision=advance` の run には `next_action` が必須
- `status=tested` 以上の profile には少なくとも 1 件の `ExperimentRun` が必要

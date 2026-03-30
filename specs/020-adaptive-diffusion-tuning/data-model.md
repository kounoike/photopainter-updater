# データモデル: 写真調ディザリングの追加改善

**Branch**: `020-adaptive-diffusion-tuning` | **Date**: 2026-03-30

## 概要

この feature は永続 DB を追加しない。比較に必要な profile 定義、評価画像、比較結果を、server 側の設定構造と `specs/020-adaptive-diffusion-tuning/` の文書成果物として扱う。

## エンティティ

### 1. ExperimentalAlgorithm

写真調画像向けに試した追加改善アルゴリズム。runtime には残さず、再現用メモとして保持する。

| フィールド | 型 | 説明 |
|-----------|----|------|
| `id` | 文字列 | 実験アルゴリズムの一意キー |
| `label` | 文字列 | 表示用の短い名称 |
| `base_profile` | 列挙 | 既存比較基準。今回は `color-priority` を想定する |
| `blue_bias` | 数値 | 青系領域で有色候補を残しやすくする補正量 |
| `highlight_guard` | 数値 | 明るい低彩度面で誤差拡散を弱める補正量 |
| `skin_tone_guard` | 数値 | 肌寄りの暖色中間調で誤差拡散を弱める補正量 |
| `known_issues` | 配列 | 青空未改善、cyan/teal 系の服色が普通の水色へ寄るなどの判明問題 |
| `status` | 列挙 | `planned` / `tested` / `hold` / `rejected` |

### 2. EvaluationImage

比較時に手動で差し替える入力画像。

| フィールド | 型 | 説明 |
|-----------|----|------|
| `id` | 文字列 | 画像の一意キー |
| `label` | 文字列 | 比較表に表示する画像名 |
| `path` | パス | `server/testdata/dither-result-check/` 配下の入力画像 |
| `coverage_tags` | 配列 | `blue-sky`, `low-saturation`, `skin-tone`, `illustration` など |
| `notes` | 文字列 | 何を観察する画像か |

### 3. LocalCheckPoint

ローカルテストで確認する代表色域。

| フィールド | 型 | 説明 |
|-----------|----|------|
| `name` | 文字列 | 色域名 |
| `source_fixture` | パス | 参照する fixture |
| `expectation` | 文字列 | 新 profile で観察したい変化 |

### 4. ComparisonResult

既存基準と追加改善案を比較した結果。

| フィールド | 型 | 説明 |
|-----------|----|------|
| `profile_id` | 文字列 | 試した profile |
| `input_image_id` | 文字列 | 使用した入力画像 |
| `observations` | 配列 | 青保持、低彩度面、肌、中間調、ノイズなどの観察結果 |
| `decision` | 列挙 | `advance` / `hold` / `reject` |
| `next_action` | 文字列 | 次に何を試すか |

## 関係

- `ExperimentalAlgorithm` は 1 件以上の `ComparisonResult` を持つ
- `EvaluationImage` は複数の `ComparisonResult` から参照される
- `LocalCheckPoint` は新 profile のローカル回帰確認に使われる

## 状態遷移

### ExperimentalAlgorithm

```text
planned -> tested -> promoted
planned -> tested -> hold
planned -> tested -> rejected
hold -> tested
```

- `planned`: 実装前または比較前
- `tested`: ローカルまたは手動比較を実施済み
- `hold`: 条件付きで保留
- `rejected`: 今回は runtime 採用しない

## バリデーションルール

- `id` は profile 間で一意でなければならない
- `EvaluationImage` は少なくとも 2 件以上を比較対象に持つこと
- `ComparisonResult` は `blue-sky` または `low-saturation` の観察を最低 1 件含まなければならない
- `decision=advance` の結果には `next_action` が必須

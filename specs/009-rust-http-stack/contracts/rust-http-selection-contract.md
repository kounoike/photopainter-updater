# Contract: Rust HTTPスタック選定結果

## Purpose

後続の Rust サーバ実装 feature が、Rust 内の HTTP framework を再比較せずに参照できるように、選定結果の最小契約を定義する。

## Required Output

選定結果文書は少なくとも次を含まなければならない。

- `selected_candidate`
- `challenger_candidate`
- `reference_candidate`
- `comparison_axes`
- `adoption_reason`
- `rejection_reasons`
- `alignment_with_008`
- `re_evaluation_triggers`

## Expected Shape

```yaml
selected_candidate: axum
challenger_candidate: actix-web
reference_candidate: warp
comparison_axes:
  - image_processing_fit
  - telemetry_api_fit
  - maintainability
  - dependency_weight
  - deployment_ease
  - developer_experience
adoption_reason:
  - Tower ecosystem と接続しやすい
  - handler/extractor/state モデルが後続責務に対して素直
rejection_reasons:
  actix-web:
    - axum を覆す決定打までは確認できない
  warp:
    - filter 中心モデルが今回の長期責務では優位になりにくい
alignment_with_008: Rust 第一候補の結論を維持しつつ、Rust 内順位を明示した
re_evaluation_triggers:
  - 採用予定の画像処理 crate が特定 framework と強く結びつく
  - telemetry 要件が大きく変わり HTTP framework への要求が変化する
```

## Acceptance Rules

- `selected_candidate` は 008 の結論と矛盾しないか、矛盾する場合は差分理由を明示すること。
- `challenger_candidate` は比較済み候補であり、見送り理由を持つこと。
- `comparison_axes` は spec の FR-003 を満たすこと。
- `re_evaluation_triggers` は将来の再比較条件として実行可能な粒度で書くこと。


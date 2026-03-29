# Data Model: Rust HTTPスタック再評価

## Entity: Rust HTTP Candidate

- Purpose: Rust 内で比較する HTTP framework 候補を表す。
- Fields:
  - `name`: 候補名。`axum`、`actix-web`、`warp` など
  - `position`: `final`、`challenger`、`reference`
  - `runtime_model`: Tokio/Hyper 等との関係
  - `routing_model`: handler/extractor、filter などの構成モデル
  - `middleware_model`: Tower 利用可否、framework 独自 middleware の特徴
  - `image_processing_fit`: 画像前処理 workload との適合評価
  - `telemetry_fit`: JSON POST、状態注入、監視連携との適合評価
  - `maintainability_notes`: 長期保守上の留意点
  - `dependency_notes`: 依存の重さ、stack の複雑さ
  - `deployment_notes`: コンテナ化、配布容易性に関する所見
  - `developer_experience_notes`: 学習コスト、試作速度、説明しやすさ
  - `adoption_reason`: 採用または維持理由
  - `rejection_reason`: 見送り理由

## Entity: Evaluation Axis

- Purpose: 候補比較の共通基準を表す。
- Fields:
  - `name`: 比較軸名
  - `description`: 軸の意味
  - `required`: この feature で必須かどうか
- Required axes:
  - `image_processing_fit`
  - `telemetry_api_fit`
  - `maintainability`
  - `dependency_weight`
  - `deployment_ease`
  - `developer_experience`

## Entity: Rust HTTP Selection Result

- Purpose: 後続 feature が参照する最終判断を表す。
- Fields:
  - `selected_candidate`: 最終候補
  - `challenger_candidate`: 第一対抗候補
  - `reference_candidate`: 参考候補
  - `alignment_with_008`: 008 との整合説明
  - `re_evaluation_triggers`: 再評価が必要になる条件の一覧
  - `source_documents`: 根拠文書群

## Relationships

- `Rust HTTP Selection Result` は 1 つ以上の `Rust HTTP Candidate` を参照する。
- 各 `Rust HTTP Candidate` はすべての必須 `Evaluation Axis` に対する評価を持つ。

## Validation Rules

- `selected_candidate` は必ず `Rust HTTP Candidate` のいずれか 1 つである。
- `challenger_candidate` は `selected_candidate` と異なる候補でなければならない。
- `reference_candidate` は `selected_candidate` と `challenger_candidate` のいずれとも異なる候補でなければならない。
- すべての候補は、必須の比較軸に対する評価と採否理由を持たなければならない。
- `alignment_with_008` は、008 の結論を維持したか、差分を出したかのどちらかを明示しなければならない。

# Data Model: HTTPサーバ技術選定調査

## Entity: 技術候補

- Purpose: HTTP サーバの実装候補を比較可能な単位で表す。
- Fields:
  - `name`: 例 `Python + FastAPI`, `Rust + axum`, `Go + net/http`
  - `runtime_family`: `python`, `rust`, `go`
  - `adoption_status`: `preferred`, `alternative`, `reference_only`, `rejected`
  - `strengths`: 長所の一覧
  - `tradeoffs`: 注意点または短所の一覧
  - `fit_for_repo`: `high`, `medium`, `low`
- Validation Rules:
  - `preferred` は 1 候補のみ
  - すべての候補は `strengths` と `tradeoffs` を持つ

## Entity: 評価観点

- Purpose: 候補比較の軸を表す。
- Fields:
  - `name`: 例 `画像前処理適合性`, `telemetry API 適合性`, `ローカル運用適合性`, `保守性`, `依存の重さ`, `配布容易性`, `開発体験`
  - `priority`: `high`, `medium`, `low`
  - `description`: この観点で何を見るか
- Validation Rules:
  - `high` priority の観点は採用理由に必ず反映される

## Entity: 画像前処理要件

- Purpose: サーバが将来担う画像変換パイプラインを表す。
- Fields:
  - `input_format`: `PNG` などのフルカラー画像
  - `operations`: `rotate`, `scale`, `dither`, `index_to_6_colors`
  - `reference_source`: `ref/convert.py`
  - `output_goal`: デバイス向け加工済み画像またはバイナリ
- Validation Rules:
  - `operations` には少なくともディザリングと 6 色化を含む
  - 比較対象候補はこの要件への実現性評価を持つ

## Entity: デバイス telemetry 要件

- Purpose: デバイスが POST 送信する状態データの受信と監視連携要件を表す。
- Fields:
  - `input_method`: `HTTP POST`
  - `initial_signal`: `battery_level`
  - `observability_targets`: `Grafana`, 通知連携
  - `retention_path`: 時系列蓄積または外部メトリクス基盤連携
- Validation Rules:
  - `initial_signal` は将来拡張可能であること
  - 比較対象候補は受信 API と監視連携の実現性評価を持つ

## Entity: 技術選定結果

- Purpose: 今回の調査結果を後続 feature が参照できる形で固定する。
- Fields:
  - `preferred_candidate`: 第一候補
  - `fallback_candidate`: 第一対抗候補
  - `other_candidates`: その他候補一覧
  - `decision_rationale`: 第一候補を選んだ理由
  - `revisit_conditions`: 再比較が必要になる条件
- Validation Rules:
  - `preferred_candidate` は `技術候補.adoption_status=preferred` と一致する
  - `revisit_conditions` は将来の判断条件として具体的であること

## Relationships

- 評価観点 `scores` 技術候補
- 画像前処理要件 `constrains` 技術候補
- デバイス telemetry 要件 `constrains` 技術候補
- 技術選定結果 `summarizes` 技術候補
- 技術選定結果 `depends_on` 高優先度の評価観点

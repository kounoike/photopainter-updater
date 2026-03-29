# Contract: HTTPサーバ技術選定結果

## Contract Goal

後続の HTTP サーバ feature が参照できるよう、今回の技術選定結果を固定する。

## Required Candidates

- `Python + FastAPI`
- `Rust + axum`
- `Go + net/http`

## Required Comparison Axes

- 画像前処理適合性
- telemetry API 適合性
- ローカル運用適合性
- 保守性
- 依存の重さ
- 配布容易性

## Required Feature Assumptions

- 入力画像は `PNG` などのフルカラー画像
- 画像前処理には回転、スケーリング、ディザリング、6 色インデックス化を含む
- `ref/convert.py` を参考処理として扱う
- デバイスは `battery_level` などの telemetry を `HTTP POST` で送る
- 受信した telemetry は Grafana などの監視・通知系へ接続できることを評価する

## Accepted Outcome for This Feature

- 暫定第一候補: `Rust + axum`
- 第一対抗候補: `Python + FastAPI`
- 参考候補: `Go + net/http`
- この Accepted Outcome は `research.md` の Decision 1, 2, 3 と一致していなければならない

## Revisit Conditions

- 実装直前の PoC で、画像処理の主要部分を Python ライブラリで短期間に安全実装できる見通しが立つ
- HTTP サーバの役割が当面は軽量 API と静的配信に留まり、重い変換処理を担わないと判断される
- 逆に、画像前処理やデバイス向けバイナリ生成が主要責務として確定する

## Documentation Contract

- 後続 feature はこの判断を初期前提として使ってよい
- 再比較する場合は、上記 `Revisit Conditions` のいずれかを満たすことを明記する

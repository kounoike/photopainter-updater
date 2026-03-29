# Quickstart: Rust HTTPスタック再評価の確認

この手順は `tasks.md` の verification task 完了条件を兼ねる。各節の確認項目をすべて満たしたら、その対応 task を完了として扱う。

## 1. 比較対象を確認する

- `research.md` に `axum`、`actix-web`、`warp` の 3 候補が含まれていることを確認する。
- `spec.md` の比較軸と `research.md` の比較軸が一致していることを確認する。
- 完了条件: User Story 1 の候補比較レビューが可能である。

## 2. 画像前処理観点を確認する

- [convert.py](/workspaces/photopainter-updater/ref/convert.py) を根拠として、回転、スケーリング、ディザリング、6 色インデックス化が比較条件に入っていることを確認する。
- `research.md` に、HTTP framework 単体ではなく画像前処理 workload 全体との噛み合わせで評価した説明があることを確認する。
- 完了条件: User Story 2 の画像前処理レビューが可能である。

## 3. telemetry 観点を確認する

- `research.md` に、JSON POST、状態注入、監視連携を前提とした telemetry API 適合性が含まれていることを確認する。
- `research.md` に、候補ごとの telemetry に関する採否理由があることを確認する。
- 完了条件: User Story 2 の telemetry レビューが可能である。

## 4. 最終判断を確認する

- `research.md` と contract で、`selected_candidate` が `axum`、`challenger_candidate` が `actix-web`、`reference_candidate` が `warp` になっていることを確認する。
- 008 の結論との差分、または継続理由が明記されていることを確認する。
- 完了条件: User Story 3 の選定理由レビューが可能である。

## 5. 後続 feature で再利用できることを確認する

- 後続の Rust サーバ実装 feature が、この比較結果だけで HTTP framework を再比較せずに参照できることをレビューで確認する。
- 再評価が必要になる条件が `re_evaluation_triggers` として明記されていることを確認する。
- 完了条件: feature 全体の文書レビューが完了し、後続 feature 参照に十分と判断できる。

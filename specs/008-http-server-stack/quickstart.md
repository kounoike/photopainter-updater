# Quickstart: HTTPサーバ技術選定調査

## 目的

HTTP サーバの基本実装前に、主要候補を比較し、次 feature の第一候補を固定する。

## 調査手順

1. [ref/convert.py](/workspaces/photopainter-updater/ref/convert.py) を確認し、回転、スケーリング、ディザリング、6 色インデックス化の流れを把握する。
2. Rust + axum、Python + FastAPI、Go + `net/http` を比較対象として整理する。
3. 画像前処理適合性、telemetry API 適合性、ローカル運用、保守性、依存の重さ、配布容易性で比較する。
4. PNG などのフルカラー画像入力と、デバイス側 POST telemetry の想定を各候補で評価する。
5. 暫定第一候補、対抗候補、参考候補を決め、その理由を文書化する。
6. 後続 feature で再評価すべき条件がある場合は明記する。

## 検証手順

1. [research.md](/workspaces/photopainter-updater/specs/008-http-server-stack/research.md) に 3 候補の比較結果と採否があることを確認する。
2. [stack-selection-contract.md](/workspaces/photopainter-updater/specs/008-http-server-stack/contracts/stack-selection-contract.md) に第一候補と再評価条件が固定されていることを確認する。
3. [plan.md](/workspaces/photopainter-updater/specs/008-http-server-stack/plan.md) と [data-model.md](/workspaces/photopainter-updater/specs/008-http-server-stack/data-model.md) の用語が一致していることを確認する。
4. 後続 feature が「axum 前提案で最小サーバを作る」判断を、追加比較なしで参照できることを確認する。
5. 画像前処理要件、telemetry 要件、ローカル優先・最小構成との整合が比較結果に含まれていることを確認する。

## 完了条件

- 暫定第一候補が `Rust + axum` として明示されている。
- `Python + FastAPI` と `Go + net/http` の位置づけが明確である。
- 画像前処理要件と telemetry 要件が比較に含まれている。
- 再比較が必要になる条件が文書化されている。

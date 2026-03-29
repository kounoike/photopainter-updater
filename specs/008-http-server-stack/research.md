# Research: HTTPサーバ技術選定調査

## Baseline Comparison Conditions

- 比較対象は `Rust + axum`、`Python + FastAPI`、`Go + net/http` の 3 候補とする。
- 入力画像は `PNG` などのフルカラー画像を前提とする。
- 画像前処理要件は `ref/convert.py` を参考に、回転、スケーリング、ディザリング、6 色インデックス化を含む。
- telemetry 要件は `HTTP POST` による `battery_level` 受信を初期想定とし、Grafana などの監視・通知系への接続可能性まで評価する。
- 比較軸は、画像前処理適合性、telemetry API 適合性、ローカル運用適合性、保守性、依存の重さ、配布容易性、開発体験/試作速度とする。

## Decision 1: 暫定第一候補は Rust + axum とする

- Decision: HTTP サーバの基本的な枠組みを次 feature で実装する場合、暫定第一候補は Rust + axum とする。
- Rationale: 今回の本質的な比較軸は「今後必要になりそうな画像前処理やデバイス向け変換処理、telemetry 受信を含めても無理のない土台か」である。`ref/convert.py` が示す処理は、フルカラー画像入力、回転、スケーリング、ディザリング、6 色インデックス化という CPU 寄りパイプラインであり、今後さらに server-side pre-render へ進むと処理比重は増える可能性が高い。Rust はこの種の画像変換やバイナリ生成を本命責務に据えても性能上の不安が小さく、axum は HTTP 層と POST 受信を整理しやすい。さらに `rustup` での環境構築は十分現実的で、将来 Docker Compose 化する際も multi-stage build で軽量ランタイムイメージに寄せやすい。LAN 内常駐サーバとして長期的に育てる前提なら、初期導入コストよりも将来の保守性と配布容易性の価値が大きい。  
  Sources: [axum docs](https://docs.rs/axum/latest/axum/)
- Alternatives considered:
  - Python + FastAPI
    - 見送り理由: 試作速度は高いが、画像前処理が主要責務になった場合に本命基盤として据える根拠がやや弱い。
  - Go 標準 `net/http`
    - 却下理由: 単純サーバには非常に良いが、今回の比較軸では FastAPI のデータモデル表現と開発速度の方が有利。

## Decision 2: Python + FastAPI は対抗候補として残す

- Decision: Python + FastAPI は不採用ではなく、実装速度と周辺ライブラリ活用を重視する場合の第一対抗候補として記録する。
- Rationale: `ref/convert.py` は Pillow ベースで、すでに回転、スケーリング、ディザリング、6 色インデックス化の流れを具体的に示している。つまり画像前処理の試作容易性だけを見れば Python はむしろ最も強い。FastAPI も POST body、データモデル、API 記述に強く、バッテリー残量のような telemetry 受信 API を早く形にするのも得意である。したがって Python + FastAPI は「画像処理試作」と「telemetry API 試作」の両面で高い実現性を持つ。ただし長期的に画像処理が本格化し、server-side pre-render が中心責務になったときの本命基盤としては、Rust 側に分があると判断した。  
  Sources: [FastAPI homepage](https://fastapi.tiangolo.com/), [FastAPI features](https://fastapi.tiangolo.com/id/features/)
- Alternatives considered:
  - FastAPI を第一候補に据える
    - 見送り理由: 「既に Python がある」以上の決定打を、画像前処理中心の将来像に対して示しきれない。

## Decision 3: 参考候補として Go 標準 `net/http` を残す

- Decision: 他候補としては Go 標準 `net/http` を参考候補に残すが、今回は第一候補にしない。
- Rationale: Go 公式サイトは web applications 向けに標準ライブラリと単一バイナリ配布の利点を強調しており、`net/http` も標準ライブラリとして安定している。POST 受信や軽量 API、Prometheus 系 export との相性も良い。一方で、今回の要件では画像前処理が重要であり、ディザリングや 6 色変換パイプラインを組む際の参照実装が Python 側にあること、Rust/Python に比べて今回の調査軸で決定打が弱いことから、参考候補に留める。  
  Sources: [Go for Web Development](https://go.dev/solutions/webdev), [net/http package](https://pkg.go.dev/net/http)
- Alternatives considered:
  - Go を Rust / Python と並ぶ最有力候補に引き上げる
    - 却下理由: 現状の repo 接続性と将来の画像前処理周辺ライブラリ事情で、FastAPI ほどの即効性がない。

## Decision 4: 比較軸は「今後の画像処理 workload への適合性」を最優先にする

- Decision: 比較軸は、画像前処理・変換 workload への適合性を最優先とし、その次に telemetry 収集 API の組みやすさ、ローカル運用適合性、保守性、依存の重さ、配布容易性、開発体験/試作速度を置く。試作速度は重要だが最上位ではない。
- Rationale: このサーバは単なるファイル配信で終わらず、将来的にデバイス向けの加工済みデータ配信とデバイス状態収集の両方を担う可能性が高い。したがって、早く書けるかよりも「その先の責務に耐えるか」を優先して比較する方が一貫している。ただし PoC と初期 API 立ち上げでは開発体験も現実的な差になるため、明示的な比較軸として残す。
- Alternatives considered:
  - 既存簡易サーバとの近さを主比較軸にする
    - 却下理由: それは偶然の現状に引きずられた判断であり、将来の責務に対する比較として弱い。

## Decision 5: 各候補とも画像前処理と telemetry 収集は実現可能だが、強みの出方が異なる

- Decision: Rust + axum、Python + FastAPI、Go + `net/http` のいずれでも、画像前処理と telemetry 収集自体は実現可能と判断する。
- Rationale: 画像前処理はライブラリ差と実装労力の差であり、技術的に不可能な候補はない。telemetry 受信も単純な POST API と時系列転送であればどの候補でも十分成立する。差が出るのは、画像処理の試作速度、長期性能、配布形態、監視統合のしやすさである。Python は `ref/convert.py` を活かして試作しやすく、Rust は長期本命としての堅さがある。Go はシンプルな常駐 API と metrics export に向くが、今回の画像処理比較では主役ではない。

### Candidate Fit Notes

- Rust + axum
  - 画像前処理: 回転、スケーリング、ディザリング、6 色インデックス化をネイティブ実装でき、server-side pre-render の本命基盤として扱いやすい。
  - telemetry: POST 受信、軽量 API、時系列基盤への転送は整理しやすい。
  - 強み: 長期性能、型安全性、multi-stage build による軽量イメージ。
  - 弱み: 画像処理実装の立ち上がりは Python より重い。
- Python + FastAPI
  - 画像前処理: `ref/convert.py` と Pillow 系資産をそのまま活かしやすく、回転、スケーリング、ディザリング、6 色化の PoC が最短で書ける。
  - telemetry: POST body、validation、監視連携 API の試作が速い。
  - 強み: 試作速度、画像処理周辺のライブラリ資産、API 記述の速さ。
  - 弱み: 画像前処理が本命責務になったときの長期本命基盤としては Rust より説得力が弱い。
- Go + net/http
  - 画像前処理: 実装自体は可能だが、今回の参考実装や比較軸に対して強い優位が見えにくい。
  - telemetry: POST 受信や metrics export は堅実に実装しやすい。
  - 強み: 単一バイナリ配布、軽量な常駐 API。
  - 弱み: 今回の画像前処理中心比較では主役になりにくい。

## Deferred Follow-up: 実装 feature では axum 前提の最小サーバ案と、FastAPI 前提の比較撤退条件を明示する

- Decision: 次の HTTP サーバ実装 feature では Rust + axum を前提案として具体化しつつ、もし実装着手時に API 試作速度や画像処理ライブラリ事情から FastAPI が有利と分かった場合の撤退条件も明示する。
- Rationale: これなら主軸はぶらさずに、実装直前の実情に応じた再評価余地も残せる。

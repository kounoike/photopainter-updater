# 実装計画: HTTPサーバ技術選定調査

**Branch**: `008-http-server-stack` | **Date**: 2026-03-29 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/008-http-server-stack/spec.md)  
**Input**: `/specs/008-http-server-stack/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

HTTP サーバ機能の基本実装前に、Rust+axum、Python+FastAPI、その他候補を比較する。主判断軸は、将来的な画像前処理と telemetry 収集を含むサーバ責務にどれだけ適しているかである。画像前処理では `ref/convert.py` が示すように、フルカラー画像入力、回転、スケーリング、ディザリング、6 色インデックス化が必要になる。加えて、デバイス側からの POST でバッテリー残量などを受け取り、Grafana などの監視や低バッテリー通知へつなぐ想定も含めて比較する。比較結果として、暫定第一候補は Rust+axum、対抗候補は Python+FastAPI、参考候補は Go 標準 `net/http` とする。

## Technical Context

**Language/Version**: Python 3 系、Rust stable、Go stable の比較調査  
**Primary Dependencies**: FastAPI、axum、Go 標準 `net/http`、`ref/convert.py`、将来候補としての画像処理ライブラリ連携、監視基盤連携、コンテナ配布前提  
**Storage**: files (`server/contents/`) を中心としたローカルファイル運用  
**Testing**: 文書レビュー、比較表確認、後続 feature での参照可能性確認  
**Target Platform**: devcontainer 上のローカル開発環境、LAN 内 HTTP サーバ運用  
**Project Type**: server-side architecture decision record for local firmware companion service  
**Performance Goals**: ローカル LAN 内で十分な応答性を持ち、画像前処理と telemetry 収集が追加されても技術選定をやり直さずに済むこと  
**Constraints**: 今回は実装しない、ローカル優先、後続で Docker Compose やコンテナ配布へ進める余地を残す、画像処理 workload と telemetry 収集の両方を比較軸に含めること  
**Scale/Scope**: 単一家庭内または少数端末向けの companion HTTP サーバ想定。画像変換とデバイス telemetry 収集を将来責務に含む。

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は比較理由として明示されている

## Project Structure

### Documentation (this feature)

```text
specs/008-http-server-stack/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── stack-selection-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
server/
├── run.sh
└── contents/

docs/
├── firmware.md
└── firmware-http-epaper.md

specs/
├── 007-stream-http-render/
└── 008-http-server-stack/
```

**Structure Decision**: 今回は実装コードを追加せず、HTTP サーバ実装前の比較調査を `specs/008-http-server-stack/` に閉じて記録する。後続 feature はこの調査結果を参照して、画像前処理を含むサーバ実装先と技術スタックを決める。

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

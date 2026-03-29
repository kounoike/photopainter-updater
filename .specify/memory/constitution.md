<!--
Sync Impact Report
- Version change: template -> 1.0.0
- Modified principles:
  - Principle 1 placeholder -> I. 仕様駆動・段階実行
  - Principle 2 placeholder -> II. スコープ厳守
  - Principle 3 placeholder -> III. 日本語ドキュメント
  - Principle 4 placeholder -> IV. 検証可能性
  - Principle 5 placeholder -> V. ローカル優先・運用単純性
- Added sections:
  - 追加制約
  - 実行ワークフロー
- Removed sections:
  - None
- Templates requiring updates:
  - ✅ updated: .specify/templates/constitution-template.md
  - ✅ updated: .specify/templates/plan-template.md
  - ✅ updated: .specify/templates/spec-template.md
  - ✅ updated: .specify/templates/tasks-template.md
  - ✅ updated: .specify/templates/agent-file-template.md
  - ⚠ pending: .specify/templates/commands/*.md (directory not present)
  - ⚠ pending: specs/_shared/product.md (file not present)
  - ⚠ pending: specs/_shared/architecture.md (file not present)
  - ⚠ pending: specs/_shared/conventions.md (file not present)
  - ⚠ pending: specs/_shared/test-strategy.md (file not present)
- Follow-up TODOs:
  - TODO(SHARED_SPECS): `specs/_shared/*.md` を作成し、共通方針の参照元を明示する
-->
# PhotoPainter Updater 憲章

## Core Principles

### I. 仕様駆動・段階実行
すべての実装は `specify -> clarify -> plan -> tasks -> analyze -> implement`
の順で進めなければならない。各フェーズは対応する成果物を残し、後続工程は前段
成果物の根拠なしに開始してはならない。理由は、機能追加よりも前に要求・設計・
実装計画の整合性を固定し、誤実装を防ぐためである。

### II. スコープ厳守
作業は明示された Allowed Scope に限定し、Forbidden Scope への変更は禁止する。
必要な修正が許可範囲外に及ぶ場合は、推測で拡張せず BLOCKER を記録して停止しな
ければならない。理由は、Kanban カードを実行可能仕様として扱い、機能境界の破壊
を防ぐためである。

### III. 日本語ドキュメント
憲章、spec、plan、tasks、checklist、運用手順を含むプロジェクト文書は日本語で
記述しなければならない。英語の固有名詞、ライブラリ名、プロトコル名、コード識
別子は原文を保持してよいが、説明文・判断基準・利用者向け記述は日本語で統一し
なければならない。理由は、仕様レビューと運用判断を日本語で一貫して行うためで
ある。

### IV. 検証可能性
各 user story、要求、運用フローには独立した検証方法を定義しなければならない。
実装タスクには少なくとも一つの検証タスクまたは明示的な手動確認手順を含め、エ
ラー時の期待動作も記述しなければならない。理由は、段階ごとの完了判定を再現可
能にし、レビュー時の曖昧さをなくすためである。

### V. ローカル優先・運用単純性
本プロジェクトは LAN/WiFi 内で自律動作する構成を優先し、初期スコープでは外部
インターネットや複雑な分散基盤への依存を追加してはならない。採用技術と運用フ
ローは、既存要件を満たす最小構成を選ばなければならず、複雑化は plan で明示的
に正当化しなければならない。理由は、PhotoPainter の用途に対して保守しやすい
更新基盤を維持するためである。

## 追加制約

- Kanban カードには Goal、Done Criteria、Shared Specs、Allowed Scope、Forbidden
  Scope、Dependencies、Execution Rules を必ず含める。
- 情報不足があっても曖昧な仮定で埋めず、`TODO:` または `BLOCKER:` を明示する。
- 共有仕様 `specs/_shared/*.md` は本来の参照元である。現時点では未整備のため、
  追加時は本憲章との整合を最優先で確認する。
- グローバルアーキテクチャ変更、無関係なファイル変更、手順の省略は認めない。

## 実行ワークフロー

1. 実装開始前にカードまたは依頼文からスコープと完了条件を抽出し、不足があれば
   `TODO:` または `BLOCKER:` を記録する。
2. `specify` ではユーザー価値、要求、境界条件、成功基準を日本語で定義する。
3. `clarify` は局所的な曖昧さの解消に限定し、横断的な仕様変更は行わない。
4. `plan` では憲章チェック、技術選定理由、構成、検証方針を記録する。
5. `tasks` では user story 単位の独立実装と検証順序を定義する。
6. `analyze` では spec、plan、tasks の整合性と憲章違反を検査する。
7. `implement` では定義済みタスクのみ実施し、完了時に変更ファイル、実行テスト、
   残リスクを報告する。

## Governance

本憲章はプロジェクト内の他運用規約より優先される。すべてのレビュー、計画、タ
スク分解は本憲章への適合確認を必須とし、違反は修正または BLOCKER 化しなければ
ならない。改訂は `.specify/memory/constitution.md` の更新として行い、影響する
テンプレートと運用文書を同一変更で同期する。バージョン規則は Semantic Versioning
に従い、原則追加は MINOR、互換性を壊す原則変更は MAJOR、文言明確化は PATCH
とする。適合レビューでは少なくとも以下を確認しなければならない: フェーズ順守、
日本語文書化、スコープ逸脱の有無、検証手順の存在、ローカル優先方針との整合。

**Version**: 1.0.0 | **Ratified**: 2026-03-29 | **Last Amended**: 2026-03-29

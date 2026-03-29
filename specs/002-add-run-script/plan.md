# 実装計画: server 配信スクリプト追加

**Branch**: `002-add-run-script` | **Date**: 2025-09-05 | **Spec**: /home/kounoike/.cline/worktrees/d13fc/photopainter-updater/specs/002-add-run-script/spec.md  
**Input**: `/specs/002-add-run-script/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

`server/run.sh` を追加し、`server/contents/` をローカルでHTTP配信できるようにする。起動・停止が明確で、失敗時に理由が分かる出力を行う最小構成とする。

## Technical Context

**Language/Version**: Bash (POSIX shell) / Python 3 (実行環境)  
**Primary Dependencies**: Python 標準の HTTP 配信機能  
**Storage**: files (`server/contents/`)  
**Testing**: 手動確認 (起動、アクセス、停止、エラー)  
**Target Platform**: ローカル開発端末  
**Project Type**: 単一スクリプト追加  
**Performance Goals**: 起動後すぐに配信できること  
**Constraints**: ローカル用途、最小構成、既存スコープ外変更なし  
**Scale/Scope**: 単一端末での検証用途

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されている

## Project Structure

### Documentation (this feature)

```text
specs/002-add-run-script/
├── plan.md              # このファイル
├── research.md          # Phase 0 成果物
├── data-model.md        # Phase 1 成果物
├── quickstart.md        # Phase 1 成果物
├── contracts/           # Phase 1 成果物 (今回は作成不要)
└── tasks.md             # Phase 2 成果物 (/speckit.tasks)
```

### Source Code (repository root)

```text
server/
├── run.sh
└── contents/
```

**Structure Decision**: 既存構成が未整備のため、`server/` 配下に必要最小限のスクリプトと配信対象ディレクトリを配置する。

## Phase 0: Outline & Research

- 既知の要件を整理し、配信手順と失敗時の出力方針を明確化する

## Phase 1: Design & Contracts

- データモデルは不要 (ファイル配信のみ)
- 外部インターフェースは `server/run.sh` の実行手順のみであり、専用の contracts は作成しない
- quickstart に実行手順と確認方法を記載する

## Phase 2: Task Planning

- `server/run.sh` の追加
- `server/contents/` の存在確認と起動時のエラー表示
- 手動テスト手順の明記

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| なし | - | - |

## Constitution Check (Post-Design)

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されている

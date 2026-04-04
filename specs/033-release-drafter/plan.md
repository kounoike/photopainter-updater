# 実装計画: Release Drafter 導入

**Branch**: `033-release-drafter` | **Date**: 2026-04-04 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/033-release-drafter/spec.md)  
**Input**: `/specs/033-release-drafter/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

この feature では、GitHub 上の main 運用に対して次回リリース向けの draft を自動更新する導線を追加する。対象は `main` への `push` のみで、pull request metadata をもとに release draft の項目を分類し、分類対象外の変更も欠落させない既定区分を設ける。実装は repository 内の GitHub Actions workflow と release drafter 設定ファイルを追加し、README などの運用文書へ確認方法を追記する最小構成とする。publish 自動化や versioning policy の全面変更は行わない。

## Technical Context

**Language/Version**: YAML（GitHub Actions workflow / release drafter 設定）、Markdown  
**Primary Dependencies**: GitHub Actions、Release Drafter、既存 GitHub repository 運用、pull request labels  
**Storage**: GitHub repository 内の workflow / 設定ファイル、永続 DB なし  
**Testing**: workflow と設定ファイルの静的確認、README 手順確認、GitHub 上で release draft が更新される手動検証  
**Target Platform**: GitHub hosted repository、`main` branch 運用  
**Project Type**: repository automation / workflow configuration 追加  
**Performance Goals**: `main` への `push` 後、管理者が追加手作業なしで次回 release draft を確認できること  
**Constraints**: 更新契機は `main` への `push` のみに限定する、分類対象外の変更を欠落させない、release publish 自動化へ広げない、既存 CI/CD 全体を作り直さない  
**Scale/Scope**: 単一 repository の release note 下書き運用を対象とし、`.github/` 配下の workflow / 設定追加と関連文書更新に限定する

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されていない

## Project Structure

### Documentation (this feature)

```text
specs/033-release-drafter/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── release-drafter-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
.github/
├── workflows/
│   └── release-drafter.yml
└── release-drafter.yml

README.md
```

**Structure Decision**: 実装対象は `.github/` 配下の workflow と release drafter 設定、および repository の運用文書に限定する。既存 workflow が未導入のため、新規 `workflows/` ディレクトリと設定ファイルを最小追加し、運用導線は `README.md` へ集約する。追加のアプリケーションコードや外部ストレージは導入しない。

## Phase 0: Research Summary

- release draft の更新契機は `main` への `push` のみに固定し、PR 作成時点では更新しない
- pull request labels を分類 metadata として扱い、分類対象外は既定カテゴリへ集約して欠落を防ぐ
- 初回実行で draft が存在しない場合も生成できる設定とし、以後は同一 draft を更新する
- workflow と設定ファイルの責務を分離し、更新契機は workflow、分類と表示規則は release drafter 設定へ寄せる
- 運用者向けには README と quickstart で、設定場所、更新契機、確認方法、期待される反映タイミングを案内する

## Phase 1: Design & Contracts

### Data Model Output

- `ReleaseDraftConfig`: draft 名称、分類セクション、既定カテゴリを表す設定
- `ReleaseDraftTrigger`: `main` への `push` を起点とする更新契機
- `PullRequestCategory`: labels によって変更項目を分類する規則
- `ReleaseDraftUpdateResult`: draft 新規作成または既存更新の結果状態

### Contract Output

- `contracts/release-drafter-contract.md`: 更新契機、分類規則、既定カテゴリ、確認導線を定義する repository 契約

### Quickstart Output

- 設定ファイル配置の確認
- `main` への `push` 後に release draft を確認する手順
- 分類対象外 pull request の既定カテゴリ反映確認

## Post-Design Constitution Check

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を維持している
- [x] Allowed Scope / Forbidden Scope を超える変更を計画していない
- [x] 文書成果物は日本語で記述している
- [x] 各主要要求に対する検証方法を plan / quickstart / contract に反映した
- [x] repository 内設定と文書更新のみの最小構成を保ち、余計な自動化へ広げていない

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

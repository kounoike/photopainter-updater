# 実装計画: xiaozhi-esp32 構造解析ドキュメント

**Branch**: `004-document-xiaozhi-arch` | **Date**: 2026-03-29 | **Spec**: [/home/kounoike/ghq/github.com/kounoike/photopainter-updater/specs/004-document-xiaozhi-arch/spec.md](/home/kounoike/ghq/github.com/kounoike/photopainter-updater/specs/004-document-xiaozhi-arch/spec.md)  
**Input**: `/specs/004-document-xiaozhi-arch/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

`xiaozhi-esp32` の既存実装を対象に、全体構造、責務分担、代表的な主要フロー、関心領域ごとの実装位置を整理した日本語技術文書を `docs/` 配下へ追加する。実装コードには手を入れず、既存ソースと README 群を調査して、後続開発者が 15 分以内に構造を把握できることを狙う。

## Technical Context

**Language/Version**: Markdown（成果物）、既存参照実装は C/C++ on ESP-IDF  
**Primary Dependencies**: 既存 `xiaozhi-esp32` ソースツリー、既存 README 群、Spec Kit 成果物  
**Storage**: リポジトリ内ファイル（`docs/` と `specs/004-document-xiaozhi-arch/`）  
**Testing**: 手動レビュー、リンク確認、文書から実装位置を追えるかの目視検証  
**Target Platform**: ローカル作業環境上の Git リポジトリ、参照対象は ESP32 ファームウェアソース  
**Project Type**: 既存ファームウェアコードベース向けの構造解析ドキュメント追加  
**Performance Goals**: 初見の開発者が 15 分以内に主要構造を説明でき、少なくとも 6 つの関心領域の実装位置を特定できること  
**Constraints**: 日本語文書、`docs/` 配下に配置、コード挙動は変更しない、全モジュール網羅ではなく代表的主要フローに限定  
**Scale/Scope**: `xiaozhi-esp32` 1 ツリーを対象に、全体構造と起動・通信・音声を含む代表フロー、および主要関心領域を 1 文書で整理する

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

### Phase 0 Gate

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されていない

### Phase 1 Re-Check

- [x] 設計成果物は文書追加と検証導線に限定され、コード変更を含まない
- [x] 代表的主要フローにスコープを制限し、網羅的解析へ拡大していない
- [x] 検証は手動レビューと導線確認で定義され、完了判定が再現可能である
- [x] 追加する成果物はローカルファイルのみで、外部依存を増やしていない

## Project Structure

### Documentation (this feature)

```text
specs/004-document-xiaozhi-arch/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── documentation-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
docs/
└── xiaozhi-esp32-architecture.md     # 実装フェーズで追加する技術文書

specs/
└── 004-document-xiaozhi-arch/
    ├── spec.md
    ├── plan.md
    ├── research.md
    ├── data-model.md
    ├── quickstart.md
    └── contracts/

xiaozhi-esp32/
├── CMakeLists.txt
├── main/
│   ├── main.cc
│   ├── application.cc
│   ├── audio/
│   ├── display/
│   ├── protocols/
│   ├── boards/
│   ├── ota.cc
│   └── settings.cc
├── components/
└── scripts/
```

**Structure Decision**: 実装対象は `docs/` 配下の単一技術文書と最小限の導線追加に限定する。解析対象コードは `xiaozhi-esp32/` に集約されており、spec 成果物は `specs/004-document-xiaozhi-arch/` に閉じる構成とする。

## Phase 0: Research Outcome

Phase 0 では、文書配置先、記述粒度、主要調査起点、検証方法を確定し、`research.md` に判断根拠を記録した。`Technical Context` に `NEEDS CLARIFICATION` は残っていない。

## Phase 1: Design Outcome

- `data-model.md` で、文書成果物・関心領域・実装要素・主要フロー・導線の関係を定義する
- `contracts/documentation-contract.md` で、最終成果物の必須セクションと導線条件を定義する
- `quickstart.md` で、実装時の作業順序と手動検証手順を定義する
- 実装フェーズでは `docs/xiaozhi-esp32-architecture.md` を作成し、必要に応じて既存 README から導線を追加する

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

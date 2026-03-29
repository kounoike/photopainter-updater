# 実装計画: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]  
**Input**: `/specs/[###-feature-name]/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

[feature spec と research をもとに、主要要求と技術方針を日本語で要約する]

## Technical Context

<!--
  ACTION REQUIRED:
  この節は実装前提を具体値で置き換える。
  不明点は推測せず NEEDS CLARIFICATION または TODO: を使う。
-->

**Language/Version**: [例: Python 3.11 / NEEDS CLARIFICATION]  
**Primary Dependencies**: [例: FastAPI, Pillow / NEEDS CLARIFICATION]  
**Storage**: [例: files, SQLite, N/A]  
**Testing**: [例: pytest, manual device verification / NEEDS CLARIFICATION]  
**Target Platform**: [例: Linux server, ESP32, local LAN]  
**Project Type**: [例: firmware + server + workflow integration]  
**Performance Goals**: [ドメイン要求を日本語で記載]  
**Constraints**: [例: offline-capable, LAN-only, memory limits]  
**Scale/Scope**: [例: 5 devices, single household deployment]

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [ ] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [ ] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [ ] 文書・説明は日本語で記述されている
- [ ] 各 user story と主要要求に検証方法が定義されている
- [ ] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されている

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # このファイル
├── research.md          # Phase 0 成果物
├── data-model.md        # Phase 1 成果物
├── quickstart.md        # Phase 1 成果物
├── contracts/           # Phase 1 成果物
└── tasks.md             # Phase 2 成果物 (/speckit.tasks)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED:
  実際の構成に置き換える。未使用の選択肢は削除し、実パスのみ残す。
-->

```text
# Option 1: Single project
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# Option 2: Web application
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# Option 3: Mobile + API
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure]
```

**Structure Decision**: [採用した構成と根拠を日本語で記載する]

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [例: 外部サービス依存] | [必要理由] | [単純構成では満たせない理由] |

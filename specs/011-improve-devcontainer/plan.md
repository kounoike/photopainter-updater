# 実装計画: Devcontainer 起動改善

**Branch**: `011-improve-devcontainer` | **Date**: 2026-03-30 | **Spec**: [spec.md](/home/kounoike/ghq/github.com/kounoike/photopainter-updater/specs/011-improve-devcontainer/spec.md)  
**Input**: `/specs/011-improve-devcontainer/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

`.devcontainer` の初回利用時に発生する長い追加セットアップ待ちをなくし、devcontainer 再生成後も `codex` と `claude` の認証状態を維持できる構成へ改善する。変更対象は `.devcontainer/` と関連ドキュメントに限定し、対象 CLI はコンテナ利用可能時点で即利用できる状態に寄せる。認証情報は Git 管理対象から分離した永続領域で保持し、初回認証・再利用・意図的な初期化は文書化された手順で実行できるようにする。

## Technical Context

**Language/Version**: Dockerfile syntax、devcontainer.json、Bash、Node.js LTS ベースの開発 CLI  
**Primary Dependencies**: Dev Containers 構成、ESP-IDF v5.5.1、GitHub CLI、`@openai/codex`、`@anthropic-ai/claude-code`  
**Storage**: devcontainer 設定ファイル、Git 管理外の永続認証キャッシュ領域、`.devcontainer/.env`  
**Testing**: 手動の devcontainer build/rebuild/recreate 検証、`codex`/`claude` 起動確認、再認証不要確認、初期化手順確認  
**Target Platform**: ローカルの devcontainer 対応開発環境  
**Project Type**: 開発環境設定と運用ドキュメントの更新  
**Performance Goals**: 新規作成直後に `codex` と `claude` を追加セットアップ待ちなしで利用可能にし、再生成後も認証状態を維持する  
**Constraints**: ワークスペースの Git 管理対象へ認証情報を書かない、既存 ESP-IDF ベース環境を壊さない、変更は `.devcontainer/` と関連文書に閉じる  
**Scale/Scope**: このリポジトリを日常的に再生成しながら使う少人数開発向けの単一 devcontainer

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は認証保持のための最小永続化に限定されている

## Project Structure

### Documentation (this feature)

```text
specs/011-improve-devcontainer/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── devcontainer-readiness-contract.md
└── tasks.md
```

### Source Code (repository root)
```text
.devcontainer/
├── .env
├── .env.example
├── Dockerfile
└── devcontainer.json

docs/
├── firmware.md
└── firmware-http-epaper.md

specs/
└── 011-improve-devcontainer/
    ├── plan.md
    ├── research.md
    ├── data-model.md
    ├── quickstart.md
    └── contracts/
```

**Structure Decision**: 実装変更は `.devcontainer/Dockerfile` と `.devcontainer/devcontainer.json` に集中させる。運用説明は既存の devcontainer 前提を記述している `docs/firmware.md` を主対象とし、feature 成果物は `specs/011-improve-devcontainer/` に閉じる。アプリケーション本体には触れない。

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

## Validation Notes

- `node` による `.devcontainer/devcontainer.json` の構文確認は成功した。
- `docker build -f .devcontainer/Dockerfile .devcontainer -t photopainter-devcontainer-test` は成功し、Dockerfile に移した `codex` / `claude` の image build を確認した。
- `docker run --rm --entrypoint /bin/sh photopainter-devcontainer-test -c 'which codex; which claude; ls -d /home/vscode/.codex /home/vscode/.claude'` で `/usr/bin/codex`、`/usr/bin/claude`、`/home/vscode/.codex`、`/home/vscode/.claude` の存在を確認した。
- 一時 named volume を `/home/vscode/.codex` と `/home/vscode/.claude` に mount して probe file を書き込み、別コンテナ実行で同じ内容を再読込できることを確認した。
- `git diff --check` は成功した。VS Code からの devcontainer recreate そのものは未実施だが、Docker build と named volume 再 attach で同等の主要要件を検証した。

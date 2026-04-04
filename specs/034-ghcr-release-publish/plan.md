# 実装計画: Release 時の GHCR image publish

**Branch**: `034-ghcr-release-publish` | **Date**: 2026-04-04 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/034-ghcr-release-publish/spec.md)  
**Input**: `/specs/034-ghcr-release-publish/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

この feature では、GitHub の draft release を正式 publish したタイミングで `server` Docker image を build し、GHCR へ自動公開する release workflow を追加する。公開対象は明示的な publish target 定義で管理し、初期状態では `server` のみを有効化する。これにより今後 `comfyui` など別 image を追加するときも、workflow を作り直さず同じ枠組みへ対象を足せる。既存の Release Drafter は draft の生成・更新だけを担い、release publish 時の image 公開とは責務を分離する。運用文書には trigger、公開先、確認方法、将来の追加方針を追記する。

## Technical Context

**Language/Version**: YAML（GitHub Actions workflow）、Markdown、既存 Dockerfile syntax  
**Primary Dependencies**: GitHub Actions、GitHub Releases event、GHCR、Docker Buildx、`docker/login-action`、`docker/metadata-action`、`docker/build-push-action`  
**Storage**: GitHub repository 内 workflow / publish target 定義 / 文書、GHCR 上の container image  
**Testing**: workflow YAML の静的検証、publish target 定義の整合確認、README/quickstart 手順確認、GitHub 上で release publish 後の Actions と GHCR を見る手動検証  
**Target Platform**: GitHub hosted repository、GitHub Actions、GHCR、既存 `server/Dockerfile`  
**Project Type**: repository automation / release workflow integration  
**Performance Goals**: draft release を正式 publish した後、管理者が追加手作業なしで `server` image の publish 開始と tag 付与結果を確認できること  
**Constraints**: trigger は draft release の publish のみ、今回の必須対象は `server` のみ、未定義 image を publish しない、既存 release draft 更新運用を壊さない、GHCR 以外は扱わない  
**Scale/Scope**: 単一 repository の release workflow と image publish 定義を対象にし、`.github/` 配下設定、`server/` Docker build 入力、README / feature 文書更新に限定する

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は publish target 定義の最小抽象化に限定して正当化されている

## Project Structure

### Documentation (this feature)

```text
specs/034-ghcr-release-publish/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── release-image-publish-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
.github/
├── workflows/
│   ├── release-drafter.yml
│   └── release-image-publish.yml
└── release-image-publish.yml

server/
└── Dockerfile

compose.yml
README.md
```

**Structure Decision**: 実装対象は GitHub Actions workflow、publish target 定義ファイル、既存 `server/Dockerfile` を参照する release publish 導線、運用文書に限定する。`compose.yml` は build context の現行値を参照するための入力として扱うが、runtime 構成そのものは変更しない。publish target を workflow から分離した設定ファイルとして置くことで、将来 image 追加時の変更点を定義側へ寄せ、単一 image 専用の分岐増殖を避ける。

## Phase 0: Research Summary

- GitHub Actions の `release` event は `types: [published]` に限定することで、draft 編集時ではなく正式公開時だけ workflow を走らせられる
- GHCR publish は GitHub Actions の `GITHUB_TOKEN` と `packages: write` 権限を使う構成を基準にする
- Docker image tag と label は release version と repository 情報から一貫生成し、`docker/metadata-action` の出力を `docker/build-push-action` へ渡す構成を採る
- 将来の複数 image 対応は matrix を先に固定するより、publish target 定義を配列化して初期値に `server` だけ入れる方が現在スコープに対して単純で拡張しやすい
- 運用確認は GitHub Releases 画面、Actions 実行結果、GHCR package 画面の 3 点に集約する

## Phase 1: Design & Contracts

### Data Model Output

- `ReleasePublishTrigger`: draft release publish を契機とする workflow 起動条件
- `PublishTarget`: image 名、build context、Dockerfile、GHCR image 名、enabled 状態を持つ公開対象定義
- `ImageTagSet`: release version と補助タグ、OCI label をまとめた tag 生成結果
- `PublishRunResult`: 各 publish target ごとの build/push 成否と確認導線

### Contract Output

- `contracts/release-image-publish-contract.md`: trigger、publish target、tagging、visibility、scope guard を定義する repository 契約

### Quickstart Output

- release publish workflow と target 定義ファイルの確認
- draft release publish 後に Actions と GHCR を確認する手順
- 将来の image 追加時にどこを更新するかの確認

## Post-Design Constitution Check

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を維持している
- [x] Allowed Scope / Forbidden Scope を超える変更を計画していない
- [x] 文書成果物は日本語で記述している
- [x] 各主要要求に対する検証方法を plan / quickstart / contract に反映した
- [x] release publish 導線を最小構成に留め、複数 image 対応は publish target 定義の拡張性に限定している

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

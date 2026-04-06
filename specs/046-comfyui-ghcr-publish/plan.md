# 実装計画: ComfyUI GHCR 公開

**Branch**: `046-comfyui-ghcr-publish` | **Date**: 2026-04-06 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/046-comfyui-ghcr-publish/spec.md)  
**Input**: `/specs/046-comfyui-ghcr-publish/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

この feature では、既存の release publish 連動 GHCR 公開導線に `comfyui` image を追加する。既存 `server` image と同じ publish target 一覧方式をそのまま使い、`./comfyui` を build context、`./comfyui/runpod/Dockerfile` を Dockerfile とする target を追加する。workflow 本体には ComfyUI 専用の特別分岐を増やさず、README と quickstart も ComfyUI image が正式 release publish 時の公開対象に含まれる前提へ更新する。

## Technical Context

**Language/Version**: YAML（GitHub Actions workflow / publish target 定義）、Markdown、既存 Dockerfile syntax  
**Primary Dependencies**: GitHub Actions、GitHub Releases event、GHCR、Docker Buildx、`docker/login-action`、`docker/metadata-action`、`docker/build-push-action`、既存 `.github/workflows/release-image-publish.yml`、既存 `.github/release-image-publish.yml`  
**Storage**: GitHub repository 内 workflow / target 定義 / 文書、GHCR 上の container image  
**Testing**: workflow YAML の静的確認、publish target 定義の整合確認、README / quickstart 手順確認、GitHub 上で release publish 後の Actions と GHCR を見る手動検証  
**Target Platform**: GitHub hosted repository、GitHub Actions、GHCR、既存 `comfyui/runpod/Dockerfile`  
**Project Type**: repository automation / release workflow integration  
**Performance Goals**: draft release を正式 publish した後、管理者が追加手作業なしで `comfyui` image の publish 開始と tag 付与結果を確認できること  
**Constraints**: trigger は draft release の publish のみ、既存 `server` target を壊さない、ComfyUI target も既存 target 形式へ載せる、GHCR 以外は扱わない、runtime 自体の build 手順は変えない  
**Scale/Scope**: 単一 repository の release workflow と image publish 定義を対象にし、`.github/` 配下設定、`comfyui/runpod/Dockerfile` を参照する publish target、README / feature 文書更新に限定する

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は既存 publish target 枠組みへの追加に限定して正当化されている

## Project Structure

### Documentation (this feature)

```text
specs/046-comfyui-ghcr-publish/
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
│   └── release-image-publish.yml
└── release-image-publish.yml

comfyui/
└── runpod/
    └── Dockerfile

README.md
```

**Structure Decision**: 実装対象は release image publish workflow、publish target 定義、ComfyUI build 入力としての `comfyui/runpod/Dockerfile` 参照、運用文書に限定する。workflow 本体は既存 target 解決処理を維持し、追加差分は `.github/release-image-publish.yml` の target 定義と README / quickstart の公開説明へ寄せる。

## Phase 0: Research Summary

- 既存 workflow は `targets` 配列から enabled target を matrix 化しているため、ComfyUI 追加は target 定義ファイルへの追記だけで足りる
- `docker/build-push-action` は target ごとの `context` と `file` を matrix から受け取る構成なので、ComfyUI でも workflow 本体の分岐追加は不要
- release version tag と OCI label は既存 metadata 生成処理をそのまま再利用できる
- README では release publish 時の公開対象一覧に ComfyUI を追加し、GHCR repository 名を明示するだけで利用導線を補える

## Phase 1: Design & Contracts

### Data Model Output

- `PublishTarget`: image 名、enabled 状態、build context、Dockerfile、image repository、default label を持つ公開対象定義
- `ComfyUI Publish Target`: `./comfyui` と `./comfyui/runpod/Dockerfile` を参照し、`photopainter-comfyui` を GHCR repository 名とする公開対象
- `PublishRunResult`: target ごとの build/push 成否と GitHub Actions / GHCR 上の確認導線

### Contract Output

- `contracts/release-image-publish-contract.md`: trigger、publish target、ComfyUI build 入力、tagging、scope guard を定義する repository 契約

### Quickstart Output

- target 定義ファイルで `server` と `comfyui` の両 target を確認する
- draft release publish 後に GitHub Actions と GHCR の確認手順を追う
- ComfyUI image の公開先と release version tag を README / GHCR で確認する

## Post-Design Constitution Check

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を維持している
- [x] Allowed Scope / Forbidden Scope を超える変更を計画していない
- [x] 文書成果物は日本語で記述している
- [x] 各主要要求に対する検証方法を plan / quickstart / contract に反映した
- [x] 既存 publish target 方式を維持し、ComfyUI 追加の複雑化を最小に留めている

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

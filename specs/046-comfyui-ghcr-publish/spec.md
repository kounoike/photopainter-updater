# 機能仕様: ComfyUI GHCR 公開

**Feature Branch**: `046-comfyui-ghcr-publish`  
**Created**: 2026-04-06  
**Status**: Draft  
**Input**: ユーザー記述: "GitHub Actions で release publish 時に ComfyUI image も GHCR へ build and push したい。server image は既存の仕組みで公開済みなので、それと同じ設定方式で comfyui image を追加し、必要な build context と Dockerfile は current repository の comfyui/runpod/Dockerfile を使う。"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - release publish で ComfyUI image を公開する (Priority: P1)

保守者として、GitHub Release を publish したときに ComfyUI image も GHCR へ自動公開したい。これにより、手元で毎回手動 build/push せずに配布用 image を揃えられる。

**Why this priority**: 今回の中心価値は release publish 導線に ComfyUI image を追加すること自体にあるため。

**Independent Test**: publish workflow の設定から ComfyUI target が有効化され、release event で `comfyui/runpod/Dockerfile` を使う GHCR 公開導線を確認できれば完了。

**Acceptance Scenarios**:

1. **Given** 保守者が GitHub の release を publish する, **When** release image publish workflow が起動する, **Then** workflow は server image に加えて ComfyUI image の build/push を開始する。
2. **Given** ComfyUI image 公開対象が設定されている, **When** workflow が metadata を生成する, **Then** GHCR 上の ComfyUI image 名と tag は release version と一致する。

---

### User Story 2 - server と同じ設定方式で target を管理する (Priority: P2)

保守者として、ComfyUI image の公開設定も既存 server image と同じ target 一覧方式で管理したい。これにより、将来の image 追加時も workflow 本体を大きく変えずに済む。

**Why this priority**: 既存の公開導線と同じ構造に揃えることが保守性に直結するため。

**Independent Test**: target 設定ファイルだけで ComfyUI target の有効化、build context、Dockerfile、image repository 名を判断できれば完了。

**Acceptance Scenarios**:

1. **Given** release publish 設定ファイルを確認する保守者がいる, **When** target 一覧を見る, **Then** server と ComfyUI が同じ形式で定義されている。
2. **Given** workflow 本体を確認する保守者がいる, **When** target 解決処理を見る, **Then** ComfyUI 用の特別扱いではなく既存 target 処理で公開対象が選ばれている。

---

### User Story 3 - 公開手順を文書から判断できる (Priority: P3)

利用者または保守者として、ComfyUI image がどの契機で公開され、どの GHCR package 名になるかを README から判断したい。これにより、公開後の取得や確認手順を迷わず追える。

**Why this priority**: Actions 設定だけでは利用者向け導線が見えず、公開物の利用方法が不明確になりやすいため。

**Independent Test**: README を読んだ保守者が、release publish 後の ComfyUI image の公開先と確認場所を判断できれば完了。

**Acceptance Scenarios**:

1. **Given** 保守者が README を確認する, **When** release image publish の節を読む, **Then** ComfyUI image が GHCR 公開対象に含まれることを判断できる。
2. **Given** 利用者が公開 image 名を知りたい, **When** README を確認する, **Then** GHCR 上の repository 名と release version tag の対応を判断できる。

---

### Edge Cases

- server image だけを残して ComfyUI image が target 設定から外れていた場合、保守者が設定漏れに気づけること。
- ComfyUI image の build context や Dockerfile path が変わった場合でも、target 設定ファイルだけで追従箇所を判断できること。
- release publish workflow が複数 target を扱っても、ComfyUI 追加によって server image 公開導線を壊さないこと。
- ComfyUI image が大きくても、公開対象として意図的に有効化されたものであることを README から判断できること。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST release publish workflow の公開対象に ComfyUI image を追加しなければならない。
- **FR-002**: System MUST ComfyUI image の build context と Dockerfile として `./comfyui` と `./comfyui/runpod/Dockerfile` を利用できなければならない。
- **FR-003**: System MUST ComfyUI image の公開設定を既存 server image と同じ target 一覧方式で管理しなければならない。
- **FR-004**: System MUST ComfyUI image を GHCR 上の専用 repository 名で release version tag 付き公開物として扱わなければならない。
- **FR-005**: System MUST server image の既存公開導線を壊さずに ComfyUI image を追加しなければならない。
- **FR-006**: System MUST README または同等の運用文書で、ComfyUI image の公開契機、公開先、確認方法を案内しなければならない。

### Key Entities *(include if feature involves data)*

- **Publish Target**: release publish workflow が解決する 1 件分の image 公開設定。
- **ComfyUI Publish Target**: `./comfyui` と `./comfyui/runpod/Dockerfile` を使う ComfyUI image 向け公開設定。
- **Release Image Publish Workflow**: release published event を受けて target 一覧から image build/push を行う GitHub Actions workflow。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `.github/workflows/release-image-publish.yml` と `.github/release-image-publish.yml` の更新
- ComfyUI image 公開に必要な README や quickstart の更新
- ComfyUI image 名、publish target、確認手順の文書化

### Forbidden Scope

- firmware、`server/`、`xiaozhi-esp32/` 配下への変更
- release drafter や release publish 以外の GitHub Actions 設定変更
- ComfyUI runtime 自体の build 手順や Dockerfile 内容変更
- Docker Hub など GHCR 以外の registry 公開対応

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 保守者は release publish workflow の設定から、ComfyUI image が GHCR 公開対象に追加されたことを確認できる。
- **SC-002**: 保守者は target 設定ファイルだけで server と ComfyUI の両公開対象を判断できる。
- **SC-003**: README を読んだ利用者は ComfyUI image の公開先と tag 規則を判断できる。
- **SC-004**: ComfyUI image 追加後も server image 公開導線が同じ workflow で維持される。

## Assumptions

- GHCR への publish 認証と release publish workflow の既存権限は server image と同じ条件で ComfyUI image にも使える。
- ComfyUI image は現在の `comfyui/runpod/Dockerfile` をそのまま公開対象にしてよい。
- image サイズが大きいこと自体は今回の feature の blocker ではない。

## Documentation Impact

- root README の release image publish 説明に ComfyUI image を追加する必要がある。
- 必要なら `specs/046-comfyui-ghcr-publish/quickstart.md` で release publish 後の確認手順を示す必要がある。

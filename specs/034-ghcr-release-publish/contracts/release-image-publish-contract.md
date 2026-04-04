# Contract: Release 時の GHCR image publish

## Purpose

draft release を正式 publish したときに、明示的な publish target 一覧に基づいて container image を build し、GHCR へ公開する repository 契約を定義する。

## Repository File Contract

- Workflow file:
  - `.github/workflows/release-image-publish.yml`
- Publish target file:
  - `.github/release-image-publish.yml`

## Trigger Contract

- Trigger event:
  - GitHub Release の `published`
- Required condition:
  - release version が確定していること
- Non-trigger events:
  - draft release の作成・編集中イベント
  - `main` への通常 `push`
  - publish target 定義に含まれない image の変更

## Publish Target Contract

- Initial enabled target:
  - `server`
- Required fields per target:
  - target 名
  - build context
  - Dockerfile
  - GHCR image repository 名
  - enabled 状態
- Guard:
  - 定義されていない image は publish しない
  - 今回は `server` 以外を必須対象にしない
  - disabled target は publish しない

## Tagging Contract

- Primary tag:
  - release version に対応する tag
- Metadata:
  - source repository を含む OCI label
- Guard:
  - release version を tag 化できない場合は publish しない

## Visibility Contract

- 管理者は GitHub Actions で publish 成否を確認できる
- 管理者は GHCR package 画面で release version に対応する image tag を確認できる
- README と quickstart から trigger、対象 image、確認方法を辿れる
- live 確認は Releases で publish、Actions で実行結果、GHCR で tag 確認の順で追える

## Scope Guard Contract

- Release Drafter は draft 作成・更新のみを扱い、この契約では publish 後の image 公開のみを扱う
- GHCR 以外の registry は今回対象外
- image のアプリケーション内容変更は今回対象外
- `compose.yml` は `server` build 入力の参照元として扱うが、runtime 構成変更は対象外

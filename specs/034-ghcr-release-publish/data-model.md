# Data Model: Release 時の GHCR image publish

## 1. ReleasePublishTrigger

- Purpose:
  - image publish workflow を開始する GitHub Release 側の条件を表す。
- Fields:
  - `event`: `release`
  - `activity_type`: `published`
  - `draft_before_publish`: true / false
  - `release_version`
- Validation:
  - `activity_type` は今回スコープでは `published` のみ
  - `release_version` は空であってはならない
- Relationships:
  - `PublishTargetSet` を起動し、`ImageTagSet` の入力値になる

## 2. PublishTarget

- Purpose:
  - 1 つの Docker image をどう build / publish するかを表す定義単位。
- Fields:
  - `name`: 例 `server`
  - `enabled`
  - `build_context`
  - `dockerfile`
  - `image_repository`
  - `default_labels`
- Validation:
  - `name` は一意
  - `enabled=true` の target は `build_context` と `dockerfile` を必須とする
  - `image_repository` は GHCR の repository 名として解釈できること
- Relationships:
  - `PublishTargetSet.targets` の構成要素
  - `ImageTagSet` と組み合わせて `PublishRunResult` を生成する

## 3. PublishTargetSet

- Purpose:
  - release 時に処理対象となる publish target 一覧を表す。
- Fields:
  - `targets`
  - `default_registry`: `ghcr.io`
  - `future_extension_policy`
- Validation:
  - 少なくとも 1 件の enabled target を持てる
  - 今回は `server` を enabled target として必須で含む
  - 未定義 target は publish 対象に含めない
- Relationships:
  - `ReleasePublishTrigger` を受けて各 `PublishTarget` を順次または並列に処理する

## 4. ImageTagSet

- Purpose:
  - release version と publish target から生成される image tag / label の集合を表す。
- Fields:
  - `version_tag`
  - `additional_tags`
  - `oci_labels`
  - `source_repository`
- Validation:
  - `version_tag` は release version に対応して一意に定まる
  - `oci_labels` には source repository を含められること
- Relationships:
  - `PublishTarget` ごとの build/push 入力になる

## 5. PublishRunResult

- Purpose:
  - 各 publish target の build / push 実行結果を表す。
- Variants:
  - `succeeded`
  - `failed`
  - `skipped`
- Fields:
  - `target_name`
  - `release_version`
  - `package_url`
  - `workflow_run_url`
  - `reason`
- Rules:
  - enabled でない target は `skipped`
  - build または push に失敗した target は `failed`
  - GHCR へ到達し tag が確認可能な target は `succeeded`

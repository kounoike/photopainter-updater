# Data Model: Release Drafter 導入

## 1. ReleaseDraftTrigger

- Purpose:
  - いつ release draft 更新処理を走らせるかを表す。
- Fields:
  - `event`: `push`
  - `target_branch`: `main`
  - `enabled`: true / false
- Validation:
  - `target_branch` は `main` のみ
  - `event` は今回スコープでは `push` のみ
- Relationships:
  - `ReleaseDraftConfig` を起動する契機になる

## 2. ReleaseDraftConfig

- Purpose:
  - 次回 release draft の表示方針と分類規則を表す。
- Fields:
  - `name_template`: draft 名称の雛形
  - `category_rules`: `PullRequestCategory` の一覧
  - `fallback_category`: 未分類変更の掲載先
  - `autolabeler_enabled`: 分類補助を使うかどうか
- Validation:
  - `fallback_category` を必須とし、未分類変更が欠落しないこと
  - category 名は重複しないこと
- Relationships:
  - `PullRequestMetadata` を入力として `ReleaseDraftUpdateResult` を生成する

## 3. PullRequestMetadata

- Purpose:
  - release draft へ反映される pull request の分類材料を表す。
- Fields:
  - `number`
  - `title`
  - `labels`
  - `merged_into_main`
- Validation:
  - `merged_into_main` が true の変更のみ draft 更新対象になること
  - labels が空でも `fallback_category` へ掲載できること
- Relationships:
  - `PullRequestCategory` の判定対象になる

## 4. PullRequestCategory

- Purpose:
  - pull request をどの区分へ掲載するかを表す規則。
- Fields:
  - `name`
  - `match_labels`
  - `display_order`
- Validation:
  - `name` は release draft 内で一意
  - `match_labels` は 1 つ以上の label を受け取れる
- Relationships:
  - `ReleaseDraftConfig.category_rules` の構成要素になる

## 5. ReleaseDraftUpdateResult

- Purpose:
  - release draft 更新処理の結果状態を表す。
- Variants:
  - `created`
  - `updated`
  - `skipped`
- Fields:
  - `draft_exists_before`
  - `draft_exists_after`
  - `included_pull_requests`
- Rules:
  - 初回実行では `created` を許可する
  - 対象外 event では `skipped` になり、draft を触らない

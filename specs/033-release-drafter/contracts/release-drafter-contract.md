# Contract: Release Drafter 更新契約

## Purpose

repository 上で次回リリース向け draft を自動維持し、main へ取り込まれた変更を分類済みの状態で確認できるようにする契約を定義する。

## Trigger Contract

- Update event:
  - `main` への `push` のみ
- Non-trigger events:
  - `main` 以外への `push`
  - merge 前の `pull_request` 更新

## Draft Lifecycle Contract

- 初回実行:
  - draft が存在しなければ新規作成する
- 継続運用:
  - 既存 draft があれば更新する
- 対象外 event:
  - draft を更新しない

## Classification Contract

- Primary input:
  - pull request labels
- Behavior:
  - label が一致する変更は対応カテゴリへ掲載する
  - 一致しない変更も既定カテゴリへ掲載し、欠落させない

## Visibility Contract

- 管理者は repository の Releases 画面から次回 release draft を確認できる
- README から設定場所と確認方法を辿れる

## Scope Guard Contract

- 今回の自動化対象は draft 更新まで
- 実際の release publish 操作や versioning policy 全面変更は対象外

# Quickstart: Release Drafter 導入

## 前提

1. repository の default branch は `main` で運用する。
2. GitHub 上で Actions が有効である。
3. pull request に分類用 labels を付与できる。

## 1. 設定ファイルを確認する

確認対象:

- `.github/workflows/release-drafter.yml`
- `.github/release-drafter.yml`

期待結果:

- workflow 側で `main` への `push` が更新契機として定義されている
- release drafter 設定側で draft 名称、分類ルール、version resolver、既定扱いの方針が定義されている

## 2. `main` への変更反映後に draft を確認する

手順:

1. main 向け pull request を merge する
2. merge 後の `main` への `push` を契機に workflow が動くことを確認する
3. repository の Releases 画面で draft を確認する

期待結果:

- 次回リリース向け draft が作成または更新される
- merge された pull request が draft に反映される
- GitHub の Releases 画面で draft の本文を確認できる

## 3. 分類ルールを確認する

確認:

- label 付き pull request が対応するカテゴリへ入る
- label が付いていない pull request も draft から欠落しない
- GitHub の Releases 画面で分類済み一覧を確認できる

期待結果:

- draft 内で変更一覧が分類済みで見える
- 未分類変更が draft 本文に残る

## 4. 対象外イベントで更新されないことを確認する

確認:

- `main` 以外への push では draft 更新対象にしない
- merge 前の pull request 更新だけでは draft を更新しない

期待結果:

- 不要な draft 更新が発生しない

## 5. 運用文書を確認する

確認:

- README から設定場所と確認方法を辿れる
- troubleshooting と対象外イベントを quickstart で確認できる

期待結果:

- 管理者が release draft の更新契約と確認導線を理解できる

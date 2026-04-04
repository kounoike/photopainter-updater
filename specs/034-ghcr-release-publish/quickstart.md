# Quickstart: Release 時の GHCR image publish

## 前提

1. GitHub Releases を使って draft release を publish できる。
2. GitHub Actions が有効で、GHCR へ publish 可能な権限が repository に付与されている。
3. `server/Dockerfile` が release 時の build 入力として利用できる。

## 1. 設定ファイルを確認する

確認対象:

- `.github/workflows/release-image-publish.yml`
- `.github/release-image-publish.yml`

期待結果:

- workflow 側で `release` の `published` のみが trigger になっている
- publish target 定義側で `server` が enabled になっている
- GHCR 向け image repository 名、build context、Dockerfile が target ごとに確認できる
- `server` target の build context と Dockerfile が `compose.yml` の server build 入力と一致している

## 2. draft release を正式 publish する

手順:

1. GitHub の Releases 画面で draft release を開く
2. release version が意図どおりであることを確認する
3. draft を正式 release として publish する

期待結果:

- release publish を契機に image publish workflow が起動する
- `main` への通常 push だけでは image publish workflow は起動しない

## 3. GitHub Actions の結果を確認する

確認:

- Actions で release image publish workflow が 1 回起動している
- `server` target の build と push が成功している
- 失敗時は対象 target 名と失敗ステップを特定できる
- 手元の静的確認として `.github/workflows/release-image-publish.yml` と `.github/release-image-publish.yml` の YAML 構文が通る

期待結果:

- 管理者が Actions 画面だけで publish 成否を追える

## 4. GHCR 上の公開結果を確認する

確認:

- `server` image が GHCR に存在する
- release version に対応する tag が付いている
- package が repository と関連付いて見える
- Actions の成功 run と GHCR 上の tag が同じ release version を指している

期待結果:

- 管理者が GHCR 画面で対象 release と image を対応づけて確認できる

## 5. 将来の image 追加ポイントを確認する

確認:

- publish target 定義に `server` 以外の target を追加できる構造になっている
- 未定義 target は今回 publish 対象に含まれない
- 追加時に更新すべき場所が workflow 本体ではなく target 定義中心である
- target 追加時の必須項目は `name` `enabled` `build_context` `dockerfile` `image_repository` である

期待結果:

- 次回以降の `comfyui` などの追加作業範囲を把握できる

## 6. 運用文書を確認する

確認:

- README から release image publish の契機と確認場所を辿れる
- feature 文書から target 定義と確認手順を追える
- live 確認では Releases、Actions、GHCR の順に見る

期待結果:

- 管理者が release publish と GHCR 確認の流れを追加説明なしで理解できる

## 7. 手元での静的確認

確認:

- `.github/workflows/release-image-publish.yml` の YAML 構文確認
- `.github/release-image-publish.yml` の YAML 構文確認
- `git diff --check` で差分体裁確認

期待結果:

- GitHub 側の live 確認を除き、repository 内で事前確認できる項目が完了する

## 8. live 確認

手順:

1. draft release を publish する
2. GitHub Actions の `Release Image Publish` が起動し、`server` target が成功していることを確認する
3. GHCR で `ghcr.io/<repository_owner>/photopainter-server:<release-version>` が見えることを確認する

期待結果:

- `server` image の publish 開始と成功を GitHub 上で確認できる
- release version に対応する tag を GHCR 上で確認できる
- live 確認が未実施でも、残作業が GitHub 側確認だけだと判断できる

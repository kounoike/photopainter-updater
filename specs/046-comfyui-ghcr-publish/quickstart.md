# Quickstart: ComfyUI GHCR 公開

## 前提

1. GitHub Releases を使って draft release を publish できる。
2. GitHub Actions が有効で、GHCR へ publish 可能な権限が repository に付与されている。
3. `comfyui/runpod/Dockerfile` が release 時の build 入力として利用できる。

## 1. 設定ファイルを確認する

確認対象:

- `.github/workflows/release-image-publish.yml`
- `.github/release-image-publish.yml`

期待結果:

- workflow 側で `release` の `published` のみが trigger になっている
- publish target 定義側で `server` と `comfyui` が enabled になっている
- `comfyui` target の build context が `./comfyui` である
- `comfyui` target の Dockerfile が `./comfyui/runpod/Dockerfile` である

## 2. draft release を正式 publish する

手順:

1. GitHub の Releases 画面で draft release を開く
2. release version が意図どおりであることを確認する
3. draft を正式 release として publish する

期待結果:

- release publish を契機に image publish workflow が起動する
- `server` と `comfyui` の両 target が公開対象として扱われる

## 3. GitHub Actions の結果を確認する

確認:

- Actions で release image publish workflow が 1 回起動している
- `server` target の build と push が成功している
- `comfyui` target の build と push が成功している
- 失敗時は対象 target 名と失敗ステップを特定できる

期待結果:

- 管理者が Actions 画面だけで target ごとの publish 成否を追える

## 4. GHCR 上の公開結果を確認する

確認:

- `server` image が GHCR に存在する
- `comfyui` image が GHCR に存在する
- それぞれ release version に対応する tag が付いている
- package が repository と関連付いて見える

期待結果:

- `ghcr.io/<repository_owner>/photopainter-server:<release-version>`
- `ghcr.io/<repository_owner>/photopainter-comfyui:<release-version>`

の両方を確認できる

## 5. README を確認する

確認:

- root README の `Release Images` 節に ComfyUI image が追記されている
- 公開契機、公開先、確認場所を README から追える

期待結果:

- 利用者または保守者が workflow 設定を開かなくても ComfyUI image 公開先を判断できる

## 6. 手元での静的確認

確認:

- `.github/workflows/release-image-publish.yml` の YAML 構文確認
- `.github/release-image-publish.yml` の YAML 構文確認
- `git diff --check` で差分体裁確認

期待結果:

- GitHub 側 live 確認を除き、repository 内で事前確認できる項目が完了する

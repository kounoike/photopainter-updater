# Contract: Release Image Publish Targets

## Purpose

release published event を契機に、repository が定義する enabled publish target を GHCR へ build / push する契約を定義する。

## Trigger Contract

- workflow file: `.github/workflows/release-image-publish.yml`
- event: `release`
- types: `published` のみ

### Guarantees

- draft release の編集では起動しない
- 正式 release publish 時のみ image 公開導線が起動する
- release tag が image tag として使われる

## Publish Target Contract

- target config file: `.github/release-image-publish.yml`
- enabled target は次の情報を持つ
  - `name`
  - `enabled`
  - `build_context`
  - `dockerfile`
  - `image_repository`
  - `default_labels`

### ComfyUI Target

- `name`: `comfyui`
- `build_context`: `./comfyui`
- `dockerfile`: `./comfyui/runpod/Dockerfile`
- `image_repository`: `photopainter-comfyui`

### Scope Guards

- server target を削除または無効化しない
- workflow 本体に ComfyUI 専用分岐を追加しない
- GHCR 以外の registry 公開を導入しない

## Tagging Contract

- image registry: `ghcr.io`
- image name: `ghcr.io/<repository_owner>/<image_repository>`
- tag: `${release_version}`
- labels:
  - `org.opencontainers.image.source`
  - `org.opencontainers.image.version`
  - `org.opencontainers.image.title`

## Verification Contract

次を満たしたら contract 準拠とみなす。

1. target 設定で `server` と `comfyui` の両方が enabled target として確認できる
2. ComfyUI target が `./comfyui` と `./comfyui/runpod/Dockerfile` を参照している
3. workflow 本体が target 一覧の matrix 解決を維持している
4. README から `photopainter-comfyui` の公開先と確認契機を判断できる

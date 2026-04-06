# Data Model: ComfyUI GHCR 公開

## Entity: Publish Target

- Description: release image publish workflow が matrix 化して扱う 1 件分の image 公開設定。
- Fields:
  - `name`: target 識別名
  - `enabled`: 公開対象に含めるかどうか
  - `build_context`: Docker build context
  - `dockerfile`: build に使う Dockerfile path
  - `image_repository`: GHCR 上の repository 名
  - `default_labels`: OCI label の既定値
- Validation Rules:
  - enabled target は `build_context` と `dockerfile` を必須とする
  - `image_repository` は target ごとに一意であること
  - workflow 本体で特別分岐せず matrix 化できる形であること

## Entity: ComfyUI Publish Target

- Description: `comfyui/runpod/Dockerfile` を公開対象にする ComfyUI image 用 target。
- Fields:
  - `name`: `comfyui`
  - `enabled`: `true`
  - `build_context`: `./comfyui`
  - `dockerfile`: `./comfyui/runpod/Dockerfile`
  - `image_repository`: `photopainter-comfyui`
  - `default_labels.org.opencontainers.image.title`: `photopainter-comfyui`
- Validation Rules:
  - local / RunPod 共通 runtime の現行 Dockerfile を参照すること
  - server target と同じ target schema に従うこと

## Entity: Publish Run Result

- Description: release publish 後に target ごとに確認する build / push 結果。
- Fields:
  - `target_name`: 対象 target 名
  - `release_version`: release tag
  - `image`: GHCR image 名
  - `status`: success / failure
  - `verification_points`: Actions run、GHCR package、README 導線
- Validation Rules:
  - target ごとの成否を個別に判定できること
  - release version tag と GHCR 上の tag が一致すること

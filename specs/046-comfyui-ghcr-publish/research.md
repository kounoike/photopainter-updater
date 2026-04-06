# Research: ComfyUI GHCR 公開

## Decision 1: 既存 publish target 方式へ ComfyUI target を追加する

- Decision: `.github/release-image-publish.yml` の `targets` 配列に `comfyui` target を追加し、workflow 本体は既存の matrix 解決処理をそのまま使う。
- Rationale: `server` image 公開ですでに target 解決処理が動いており、ComfyUI 追加のために workflow 本体へ特別分岐を足す必要がない。最小変更で既存の拡張方針にも合う。
- Alternatives considered:
  - ComfyUI 専用 workflow を新設する: 公開契機と運用確認が分散し、既存の target 抽象化を活かせないため不採用。
  - workflow 本体へ `if: matrix.name == 'comfyui'` のような分岐を増やす: target 設定ファイルだけで管理できる利点が失われるため不採用。

## Decision 2: ComfyUI build 入力は `./comfyui` と `./comfyui/runpod/Dockerfile` に固定する

- Decision: ComfyUI target の build context は `./comfyui`、Dockerfile は `./comfyui/runpod/Dockerfile` とする。
- Rationale: local / RunPod 共通 runtime として現行運用している build 入力をそのまま公開対象へ使える。README や `compose.yml` の現行導線とも整合する。
- Alternatives considered:
  - repo root を build context にする: 不要ファイルの巻き込み範囲が広がり、現行 build 前提ともずれるため不採用。
  - 別 Dockerfile を release 用に追加する: runtime 定義の二重管理になるため不採用。

## Decision 3: README の Release Images 節へ ComfyUI 公開先を追加する

- Decision: root README の `Release Images` 節に ComfyUI image を公開対象として追記し、GHCR package 名と確認場所を明記する。
- Rationale: 利用者や保守者は最初に README を見るため、workflow 設定だけでなく公開先を文書から判断できるようにする必要がある。
- Alternatives considered:
  - feature quickstart のみに記載する: repository 全体の公開導線が README から見えなくなるため不採用。
  - README では server だけ残し、ComfyUI は hidden target にする: 公開対象追加の目的と矛盾するため不採用。

# 調査メモ: ComfyUI 自前イメージ構築

## Decision 1: `comfyui` service は repo 管理 Dockerfile から build する

**Decision**: `compose.yml` の `comfyui` service は repo 内 Dockerfile を build source にし、公開 image の直接参照から移行する。  
**Rationale**: 現状は upstream image 前提で runtime 状態に依存しやすく、再起動・再作成時の差分原因を repo 側で制御しにくい。Dockerfile を repo 管理にすると、ComfyUI 実行環境の基準が feature と同じ履歴で追跡でき、再 build の導線も手順化しやすい。  
**Alternatives considered**:
- `image:` で公開 image を使い続ける: runtime 側の差分が残りやすい
- 手元で修正した container を commit/push して使い回す: 再現性とレビュー性が低い

## Decision 2: base image は CUDA 対応 Python image にする

**Decision**: 自前 image は公開 ComfyUI runtime image ではなく、CUDA 対応の Python base image から組み立てる。  
**Rationale**: 問題の中心は upstream runtime image 任せの不透明さにあり、Python base から組み立てる方が ComfyUI / PyTorch / 追加依存の導入順序を repo 側で明示できる。ComfyUI upstream README の manual install 手順を Dockerfile 化することで、再構築性を高めつつ手順の根拠も upstream と揃えられる。  
**Alternatives considered**:
- 既存の ComfyUI runtime image を pinned 利用する: image 内状態の差分要因を repo 側で制御しにくい
- CUDA / Python まで完全自作する: scope が過大

## Decision 2a: Python 依存は `uv` で管理する

**Decision**: Python 依存の導入は `pip` 直接ではなく `uv` を使う。  
**Rationale**: user 指示どおり `uv` 管理へ寄せることで、Python 環境の再現性と Docker build 時の導入速度を改善しやすい。ComfyUI upstream manual install の `pip install` 手順は、Dockerfile では `uv` ベースに読み替えて固定する。  
**Alternatives considered**:
- `pip install` をそのまま使う: 要件に合わない
- Poetry や別の tool を導入する: 今回の scope 外

## Decision 2b: NVIDIA 専用 image として PyTorch CUDA wheel を明示する

**Decision**: 今回は NVIDIA/CUDA 専用 image とし、PyTorch は ComfyUI README の NVIDIA 手順を踏まえた CUDA wheel index を使って導入する。  
**Rationale**: repo の既存 compose も NVIDIA GPU 前提であり、AMD/Intel まで同時対応すると設計と検証が拡散する。ComfyUI README でも GPU 種別ごとに導入手順が分かれているため、今回は NVIDIA に限定する方が plan の単純性に合う。  
**Alternatives considered**:
- GPU ベンダー共通 image を狙う: 検証軸が増えすぎる
- CPU fallback を含める: 現行の ComfyUI 運用方針とずれる

## Decision 3: runtime で変化しやすい初期状態は Dockerfile と entrypoint へ寄せる

**Decision**: 起動前提となる repo 管理構成や初期導線は container 起動後の手作業ではなく Dockerfile と entrypoint 側で整える。  
**Rationale**: 問題の本質は「container を作り直すと同じ状態へ戻らない」ことにある。build 時に固定できる要素を Dockerfile へ寄せると、再起動だけでなく再 build・再作成でも同じ起点を得やすい。  
**Alternatives considered**:
- 初回起動後に毎回手作業で調整する: 人依存が残る
- README で補足するだけに留める: 不安定さの原因を構造的に減らせない

## Decision 4: 利用者データは既存 bind mount を継続利用する

**Decision**: `COMFYUI_DATA_DIR` 配下のモデル、利用者 custom node、output、input、user 設定、cache などは既存 bind mount を維持する。  
**Rationale**: 自前 image へ移行しても、利用者資産まで image 側へ閉じ込めると既存運用との互換性を壊す。再作成で失って困るものは host 側に残し、image は「再現可能な実行環境」、bind mount は「利用者資産」の責務へ分離するのが自然である。  
**Alternatives considered**:
- named volume へ全面移行する: 既存ホスト上データの扱いが変わる
- model や output まで image へ含める: サイズと更新運用が悪化する

## Decision 5: `comfyui` service 名と既存起動入口は変えない

**Decision**: 利用者の操作入口は引き続き `docker compose up -d comfyui` とし、service 名 `comfyui` と `COMFYUI_PORT` を維持する。  
**Rationale**: 今回の主目的は運用安定化であって、利用導線の刷新ではない。起動入口や到達 URL を維持すると、既存利用者の移行コストと文書差分を最小化できる。  
**Alternatives considered**:
- service 名を `comfyui-builder` などへ変更する: 利用者の導線が壊れる
- build 用と runtime 用で複数 service に分ける: compose 運用が複雑になる

## Decision 6: 再起動だけでなく再作成の検証手順を first-class に扱う

**Decision**: quickstart と検証観点には `restart` だけでなく `down` → `up` の再作成手順を明示する。  
**Rationale**: 問題が発生しやすいのは container 再作成時であり、再起動確認だけでは十分ではない。利用者が「何をすれば再現確認になるか」を同じ文書から辿れるようにしておく必要がある。  
**Alternatives considered**:
- 起動確認だけを残す: 再作成時の問題を検出しにくい
- 実装後に口頭で補足する: 再現性がない

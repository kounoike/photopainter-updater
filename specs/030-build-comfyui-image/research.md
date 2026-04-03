# 調査メモ: ComfyUI 自前イメージ構築

## Decision 1: `comfyui` service は repo 管理 Dockerfile から build する

**Decision**: `compose.yml` の `comfyui` service は repo 内 Dockerfile を build source にし、公開 image の直接参照から移行する。  
**Rationale**: 現状は upstream image 前提で runtime 状態に依存しやすく、再起動・再作成時の差分原因を repo 側で制御しにくい。Dockerfile を repo 管理にすると、ComfyUI 実行環境の基準が feature と同じ履歴で追跡でき、再 build の導線も手順化しやすい。  
**Alternatives considered**:
- `image:` で公開 image を使い続ける: runtime 側の差分が残りやすい
- 手元で修正した container を commit/push して使い回す: 再現性とレビュー性が低い

## Decision 2: 自前 image でも upstream base は pinned な既知ランタイムを土台にする

**Decision**: 自前 image は既知の upstream ComfyUI ランタイムを土台にし、tag を固定したうえで repo 管理の構成だけを追加する。  
**Rationale**: ComfyUI 本体と CUDA/PyTorch まで完全自作すると scope が急拡大する。安定した既知ランタイムを pinned base として使い、その上に repo 管理の初期構成と依存固定を重ねる方が、今回の目的である「再構築可能性」と「運用単純性」に合う。  
**Alternatives considered**:
- CUDA / PyTorch / ComfyUI 本体まで完全自作する: 保守範囲が大きすぎる
- `latest` 系の floating tag を使う: build ごとの差分原因が増える

## Decision 3: runtime で変化しやすい初期状態は Dockerfile へ寄せる

**Decision**: 起動前提となる repo 管理構成や初期導線は container 起動後の手作業ではなく Dockerfile 側で整える。  
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

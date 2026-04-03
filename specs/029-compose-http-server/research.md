# 調査メモ: HTTPサーバ Compose 統合

## Decision 1: server 専用 Dockerfile を新設する

**Decision**: `server/Dockerfile` を追加し、HTTP サーバ用 container image を compose から起動する。  
**Rationale**: 既存 `run.sh` は local Rust toolchain 前提であり、compose から直接置き換えにくい。server 専用 Dockerfile に切り出すと責務が明確で、他 compose サービスとも同じ管理方法へ寄せられる。  
**Alternatives considered**:
- compose から host の `cargo run` を直接叩く: container 管理にならない
- root に共通 Dockerfile を置く: server 専用責務がぼやける

## Decision 1a: Dockerfile は multi-stage build を採用する

**Decision**: builder と runtime を分けた multi-stage Dockerfile を採用する。  
**Rationale**: Docker の公式 best practice は multi-stage build による build 環境と runtime 環境の分離を推奨している。Rust 向け Docker Docs も builder と最終 runtime を分けた構成を案内している。これに従うと final image を小さくでき、attack surface も下げやすい。  
**Alternatives considered**:
- 単一 stage で `cargo build --release` から直接実行する: image が大きくなり、build toolchain が runtime に残る

## Decision 1b: builder には pinned Rust official image、runtime には小さめの runtime image を使う

**Decision**: builder stage は version を固定した Rust official image 系、runtime stage は build tool を含まない小さめの runtime image を使う。  
**Rationale**: Docker Docs は trusted source の base image と pinned version を推奨している。Rust 公式 image は妥当な builder 基盤であり、runtime を分離すると不要なツールを final image に持ち込まない。  
**Alternatives considered**:
- `latest` のような floating tag: 再現性が落ちる
- builder と runtime に同じ大きい image を使う: final image が過大になる

## Decision 1c: BuildKit cache と `.dockerignore` を活用する

**Decision**: Dockerfile では BuildKit cache を使い、`.dockerignore` で不要ファイルを build context から除外する。  
**Rationale**: Docker の best practice と Rust 向け公式 guide は cache の活用と `.dockerignore` を推奨している。Rust build は依存解決が重いので、Cargo registry / git / target cache の再利用は有効である。  
**Alternatives considered**:
- cache を使わない: 再 build が遅い
- repo 全体を無差別に context へ含める: build が重くなり、不要ファイルも巻き込む

## Decision 2: 配信データは既存 `server/contents/` を bind mount で継続利用する

**Decision**: image source / upload 保存先は `server/contents/` をそのまま compose service に渡す。  
**Rationale**: API 契約を変えず、既存データもそのまま使える。volume 名や別保存先へ移す必要はない。  
**Alternatives considered**:
- Docker named volume に移す: 既存ファイルの扱いが変わる
- image を container 内へ bake-in する: upload 更新と相性が悪い

## Decision 2a: container 内 port は 8000 固定、host 公開 port だけを設定可能にする

**Decision**: server process の `PORT` は container 内で 8000 固定とし、host へ公開する port だけを `SERVER_EXPOSE_PORT` で切り替える。  
**Rationale**: container 内 port まで可変にすると compose 設定と server process 設定が二重化しやすい。runtime の listen port は固定し、外部公開だけを変数化する方が運用が単純である。  
**Alternatives considered**:
- `SERVER_PORT` をそのまま container 内 `PORT` と host 側公開の両方に使う: 設定が混線しやすい

## Decision 3: server は compose 内でも独立起動可能にする

**Decision**: `docker compose up -d server` で単体起動でき、必要なら他サービスと同時起動もできる形にする。  
**Rationale**: spec は compose 内共存を求めつつ、不要な他サービス起動を強制しないことも求めている。  
**Alternatives considered**:
- 常に全サービス起動前提にする: ローカル運用が重い

## Decision 4: `server/run.sh` は置き換え後に削除する

**Decision**: 起動導線が compose へ移った段階で `server/run.sh` を廃止し、README / server README もそれに合わせる。  
**Rationale**: script を残すと二重導線になり、spec の一本化要件を満たしにくい。  
**Alternatives considered**:
- run.sh を互換ラッパとして残す: 運用導線が分裂したままになる

# 調査メモ: HTTPサーバ Compose 統合

## Decision 1: server 専用 Dockerfile を新設する

**Decision**: `server/Dockerfile` を追加し、HTTP サーバ用 container image を compose から起動する。  
**Rationale**: 既存 `run.sh` は local Rust toolchain 前提であり、compose から直接置き換えにくい。server 専用 Dockerfile に切り出すと責務が明確で、他 compose サービスとも同じ管理方法へ寄せられる。  
**Alternatives considered**:
- compose から host の `cargo run` を直接叩く: container 管理にならない
- root に共通 Dockerfile を置く: server 専用責務がぼやける

## Decision 2: 配信データは既存 `server/contents/` を bind mount で継続利用する

**Decision**: image source / upload 保存先は `server/contents/` をそのまま compose service に渡す。  
**Rationale**: API 契約を変えず、既存データもそのまま使える。volume 名や別保存先へ移す必要はない。  
**Alternatives considered**:
- Docker named volume に移す: 既存ファイルの扱いが変わる
- image を container 内へ bake-in する: upload 更新と相性が悪い

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

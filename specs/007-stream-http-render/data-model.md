# Data Model: HTTP画像の直接表示検討

## Entity: 更新方式判定

- Purpose: HTTP 取得画像をどの経路で e-paper へ反映するかの採用判断を表す。
- Fields:
  - `mode`: `sd_cached_render` または `direct_stream_render`
  - `decision`: `accepted` または `rejected`
  - `reason`: 採用または不採用の根拠
  - `scope_impact`: `low`, `medium`, `high`
- Validation Rules:
  - `decision=rejected` のとき `reason` は必須
  - `mode=direct_stream_render` を `accepted` にするには、既存 failure semantics と表示互換性を維持できることが前提
- State Transitions:
  - `undecided -> accepted`
  - `undecided -> rejected`

## Entity: 更新画像一時キャッシュ

- Purpose: HTTP 取得画像を描画前に保持する一時ファイルの存在有無を表す。
- Fields:
  - `path`: 現行では `/sdcard/download.bmp`
  - `storage`: `sdcard`
  - `lifecycle`: `overwrite_on_each_update`
  - `required_for_render`: `true` または `false`
- Validation Rules:
  - 現行方式では `required_for_render=true`
  - 直接表示方式が不採用である限り、描画前提の実体として残る

## Entity: 表示更新経路

- Purpose: HTTP 取得から e-paper 表示までの責務分割を表す。
- Fields:
  - `download_stage`: `http_to_sd_file`
  - `validate_stage`: `bmp_header_validation`
  - `render_stage`: `path_based_gui_bmp_render`
  - `failure_categories`: `config_error`, `wifi_error`, `http_error`, `image_error`
- Validation Rules:
  - `download_stage` 失敗は `http_error`
  - `validate_stage` または `render_stage` 失敗は `image_error`
  - 経路変更時も failure category は維持される必要がある

## Relationships

- 更新方式判定 `governs` 更新画像一時キャッシュの必要性
- 表示更新経路 `uses` 更新画像一時キャッシュ
- 更新方式判定 `documents` 表示更新経路の採用理由

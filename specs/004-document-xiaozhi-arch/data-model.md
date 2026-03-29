# Data Model: xiaozhi-esp32 構造解析ドキュメント

## Entity: 構造解析ドキュメント

- Purpose: `xiaozhi-esp32` の構造、責務分担、主要フロー、関心領域別の実装位置を記録する最終成果物。
- Fields:
  - `path`: `docs/` 配下の配置先
  - `title`: 文書タイトル
  - `summary`: 全体像の要約
  - `scope_statement`: 対象範囲と対象外
  - `sections`: 必須見出しの集合
  - `evidence_style`: 確認済み・推定・対象外の区別ルール
- Validation Rules:
  - 日本語で記述されること
  - `xiaozhi-esp32` の全体構造と代表的主要フローを含むこと
  - 少なくとも 6 つの関心領域の実装位置を辿れること
  - 読者が調査起点として使えるファイルまたはディレクトリ参照を含むこと

## Entity: 関心領域

- Purpose: 読者が目的別に参照する技術的な切り口。
- Fields:
  - `name`: 例: 起動、音声、表示、通信、設定、OTA、ボード差分
  - `description`: その関心領域の責務説明
  - `entry_points`: 関連する代表ファイルやディレクトリ
  - `related_flows`: 関連する主要フロー
- Validation Rules:
  - 各関心領域は少なくとも 1 つの実装要素に紐づくこと
  - 文書内の名称は一貫していること

## Entity: 実装要素

- Purpose: 構造解析の対象となるコードベース上の具体単位。
- Fields:
  - `path`: ディレクトリまたはファイルパス
  - `kind`: `directory` / `module` / `entrypoint` / `readme`
  - `responsibility`: 観察された責務
  - `shared_or_specific`: 共通実装かボード固有実装か
- Validation Rules:
  - 責務は名称ではなく実装観察に基づくこと
  - ボード固有要素は共通実装との境界説明を伴うこと

## Entity: 主要フロー

- Purpose: 起動・通信・音声など、読者が全体動作を追うための代表的処理経路。
- Fields:
  - `name`: フロー名
  - `trigger`: 開始条件
  - `sequence`: 主要な処理段階
  - `participating_elements`: 関与する実装要素
  - `notes`: 推定や対象外があればその注記
- Validation Rules:
  - 少なくとも起動、通信、音声を含むこと
  - sequence は高レベルに留め、詳細実装の網羅に拡張しないこと

## Entity: 導線

- Purpose: リポジトリ利用者が最終文書へ到達するための入口。
- Fields:
  - `source_path`: README などのリンク元
  - `target_path`: `docs/` 配下の文書
  - `link_text`: 誘導文言
- Validation Rules:
  - 導線は存在する場合、実ファイルを指していること
  - 新規導線は最小限に留めること

## Relationships

- 構造解析ドキュメント `contains` 関心領域
- 構造解析ドキュメント `contains` 主要フロー
- 関心領域 `maps_to` 実装要素
- 主要フロー `references` 実装要素
- 導線 `points_to` 構造解析ドキュメント

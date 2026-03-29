# Data Model: Devcontainer 起動改善

## Entity: 開発 CLI 提供状態

- Purpose: devcontainer 接続直後に主要 CLI を利用できる状態を表す。
- Fields:
  - `cli_name`: `codex` や `claude` など対象 CLI の識別子
  - `available_on_attach`: 接続直後に利用可能か
  - `install_timing`: 利用可能になる準備タイミング
  - `requires_manual_step`: 追加手動セットアップの要否

## Entity: 認証キャッシュ領域

- Purpose: devcontainer 再生成後も保持される認証関連ファイルの保存先を表す。
- Fields:
  - `location_type`: 永続領域の種別
  - `tracked_by_git`: Git 管理対象かどうか
  - `persists_across_recreate`: recreate 後も残るか
  - `reset_method`: 意図的に初期化する方法
  - `scoped_tools`: どの CLI の認証状態を保持するか

## Entity: Devcontainer 運用手順

- Purpose: 初回利用、再接続、再生成、初期化の判断手順を表す。
- Fields:
  - `entry_point`: 開発者が参照する手順の入口
  - `first_time_login_required`: 初回認証の要否
  - `reuse_expected_after_rebuild`: rebuild/recreate 後に再利用を期待するか
  - `verification_steps`: 利用者が確認する手順

## Relationships

- `開発 CLI 提供状態` は `Devcontainer 運用手順` の接続直後確認手順で検証される。
- `認証キャッシュ領域` は `開発 CLI 提供状態` のうち認証が必要な CLI を支える。
- `Devcontainer 運用手順` は `認証キャッシュ領域.reset_method` を参照して初期化手順を定義する。

## Validation Rules

- 対象 CLI は devcontainer 接続直後に追加セットアップなしで利用できなければならない。
- 認証キャッシュ領域は Git 管理対象から分離されていなければならない。
- devcontainer の rebuild または recreate 後、対象 CLI の認証再利用を確認できなければならない。
- 初回認証と意図的な初期化の手順は、利用者が文書だけで判断できなければならない。

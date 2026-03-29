# Data Model: run.sh 配信設定改善

## Entity: 起動設定

- Purpose: `run.sh` がサーバ起動前に確定させる実行条件を表す。
- Fields:
  - `port`: 待受ポート番号。未指定時は既定値を使う。
  - `content_dir_input`: 利用者が起動時に与える配信元ディレクトリ指定。未指定可。
  - `content_dir_resolved`: 実際に配信へ使う絶対パス化済みディレクトリ。
  - `default_content_dir`: スクリプト配置場所から解決される既定ディレクトリ。
  - `working_directory`: 利用者が `run.sh` を呼び出した時点のカレントディレクトリ。結果に影響してはならない。

## Entity: 配信公開情報

- Purpose: 起動後に利用者へ案内する接続情報を表す。
- Fields:
  - `bind_scope`: ローカルホスト限定ではない待受状態であることを示す属性。
  - `local_example_url`: 同一ホスト上での確認用 URL。
  - `lan_usage_note`: 別端末からはホストの LAN アドレスを使う旨の案内。
  - `image_path_note`: 配信元ディレクトリまたは代表ファイルの実体を示す案内。

## Entity: 起動失敗

- Purpose: 起動前検証で検知した失敗を利用者へ返す結果を表す。
- Fields:
  - `reason_type`: 依存不足、ポート指定不正、配信元ディレクトリ不正などの分類。
  - `message`: 利用者が原因を判断できる説明文。
  - `blocking`: サーバ起動を中止すべき失敗であることを示す。

## Relationships

- `起動設定` は `配信公開情報` の内容を決定する。
- `起動設定.content_dir_resolved` が有効なときのみサーバ起動へ進める。
- `起動設定` の検証に失敗した場合は `起動失敗` が返り、`配信公開情報` は生成されない。

## Validation Rules

- `content_dir_input` が未指定でも `default_content_dir` に解決できなければならない。
- `content_dir_resolved` はカレントディレクトリではなくスクリプト位置または利用者指定から決定されなければならない。
- `bind_scope` は別端末利用を妨げない内容で案内されなければならない。
- `起動失敗.message` は利用者が修正行動を選べる粒度であること。

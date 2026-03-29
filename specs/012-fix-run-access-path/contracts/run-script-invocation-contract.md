# Contract: run.sh Invocation

## Purpose

`run.sh` を使う利用者が、既定構成のままでも、任意ディレクトリ指定でも、一貫した方法で配信を開始できるように起動契約を定義する。

## Default Invocation

- Invocation:
  - Command: `server/run.sh`
- Preconditions:
  - `cargo` が利用可能である
  - 既定の配信元ディレクトリが存在する
- Expected Behavior:
  - 既定の配信元ディレクトリを使ってサーバを起動する
  - 利用者へローカル確認用 URL と、別端末ではホストの LAN アドレスを使う旨を案内する
  - `run.sh` の起動結果から localhost 用案内と LAN 用案内を区別して判別できる
  - 追加の必須設定を要求しない

## Invocation With Content Directory Override

- Invocation:
  - Command: `server/run.sh [content directory override]`
- Preconditions:
  - 指定ディレクトリが存在する
- Expected Behavior:
  - 指定ディレクトリを配信元として使う
  - 起動元カレントディレクトリに関わらず同じディレクトリを解決する
  - 起動案内に、実際に使われる配信元が判別できる情報を出す
  - 配信対象は指定ディレクトリ配下の `image.bmp` とする

## Failure Conditions

- Missing dependency:
  - Condition: `cargo` が利用できない
  - Result: 起動を中止し、依存不足と対処を示す

- Invalid port:
  - Condition: ポート指定が無効
  - Result: 起動を中止し、ポート指定誤りと判別できる説明を出す

- Missing content directory:
  - Condition: 既定値または指定値の配信元ディレクトリが存在しない
  - Result: 起動を中止し、不正なディレクトリ指定であると分かる説明を出す

- Empty content directory:
  - Condition: 配信元ディレクトリは存在するが `image.bmp` が存在しない
  - Result: サーバは起動でき、配信要求時に内容不足または未配置と判別できる応答を返す

## Behavioral Rules

- `run.sh` はどのカレントディレクトリから起動しても同じ既定ディレクトリを解決しなければならない。
- 任意ディレクトリ指定は既定ディレクトリより優先されなければならない。
- 今回の契約は配信 route の追加や設定ファイル永続化を含まない。

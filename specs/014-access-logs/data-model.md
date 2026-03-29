# Data Model: HTTP アクセスログ追加

## Entity: アクセスログ行

- Purpose: 1 回の HTTP リクエストを運用者が追跡できるようにする記録単位。
- Fields:
  - `timestamp`: リクエスト処理時刻
  - `remote_addr`: アクセス元を判別する情報
  - `method`: HTTP method
  - `path`: 対象 path
  - `status`: 応答ステータス
  - `outcome`: 成功または失敗の判別

## Entity: 応答結果情報

- Purpose: ログから成功/失敗と失敗種別を切り分けるための補助情報。
- Fields:
  - `status_code`: 実際の応答コード
  - `category`: success / input-missing / transform-failed / not-found などの区分
  - `summary`: 1 行で読める短い説明

## Relationships

- `アクセスログ行` は各 HTTP リクエストに対して 1 件生成される。
- `応答結果情報` は `アクセスログ行` の一部として記録される。
- 同じ path への複数回アクセスでも、時刻やアクセス元により個別の `アクセスログ行` として区別される。

## Validation Rules

- 1 リクエストにつき 1 件のログ行でなければならない。
- ログ行は path と status を必ず含まなければならない。
- 取得可能な場合はアクセス元情報を含まなければならない。
- 成功系、入力画像未配置、変換不能、存在しない path のすべてでログ行が生成されなければならない。

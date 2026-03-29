# Contract: Access Log Output

## Purpose

HTTP サーバが各リクエストで出力するアクセスログの最小契約を定義する。

## Log Output Rules

- Trigger:
  - `GET /`
  - `GET /image.bmp`
  - 存在しない path へのアクセス
- Behavior:
  - 1 リクエストにつき 1 行のアクセスログを出力する
  - 既存レスポンス契約は変更しない

## Required Fields

- `request_id`: 複数回アクセスを区別しやすくする識別子
- `timestamp`: リクエスト発生タイミングが分かる情報
- `remote`: 取得可能な場合のアクセス元情報。取得できない場合はフォールバック値を出してもよい
- `method`: HTTP method
- `path`: アクセス対象 path
- `status`: 応答ステータス
- `outcome`: 成功または失敗種別が分かる結果情報

## Success Case

- Conditions:
  - 変換済み BMP を正常に返せる
- Expected log meaning:
  - 対象 path へのアクセスが成功し、成功ステータスで応答したことが分かる

## Failure Cases

- Conditions:
  - `image.png` 未配置
  - `image.png` 変換不能
  - 存在しない path
- Expected log meaning:
  - 失敗したことと、そのときの応答ステータスが分かる

## Behavioral Constraints

- ログ追加によって `/` と `/image.bmp` の応答 body や Content-Type を変更してはならない。
- ログは標準出力または既存の起動導線から確認できなければならない。

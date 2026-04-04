# Data Model: Hello 動作確認エンドポイント

## 1. HelloProbeRequest

- Purpose:
  - 利用者または運用者が server 稼働確認のために送る `/hello` request を表す。
- Fields:
  - `method`: 想定は `GET`、確認系 client では `HEAD` もあり得る
  - `path`: `/hello`
  - `remote`: request の送信元識別子
- Validation:
  - `path` は常に `/hello` であること
  - 疎通確認用途であり、body や画像入力を前提にしないこと
- Relationships:
  - `HelloProbeResponse` と `AccessLogEvent` を生成する入力になる

## 2. HelloProbeResponse

- Purpose:
  - server が応答可能であることを利用者へ伝える成功レスポンスを表す。
- Fields:
  - `status`: 成功 status
  - `content_type`: text 系の応答種別
  - `message`: server 稼働確認に使う固定メッセージ
  - `depends_on_image_state`: 常に `false`
- Validation:
  - 画像ファイルの有無や decode 成否に関係なく生成できること
  - 利用者が成功と判別できる短い文言を含むこと
- Relationships:
  - `HelloProbeRequest` に対する直接の応答になる

## 3. AccessLogEvent

- Purpose:
  - `/hello` を含む各 request の結果を既存ログ導線に記録する 1 件のイベント。
- Fields:
  - `method`
  - `path`
  - `status`
  - `remote`
  - `outcome`
- Validation:
  - `/hello` request でも 1 request につき 1 行のログを残すこと
  - 既存 route の outcome 命名と矛盾しないこと
- Relationships:
  - `HelloProbeResponse` 成功時の記録先になる
  - 既存の `/`、`/image.bmp`、`/image.bin`、`/upload` と同じログ構造を共有する

## 4. RouteContractSet

- Purpose:
  - `/hello` 追加後の route 全体の共存条件を表す。
- Members:
  - `hello_probe`: `/hello`
  - `image_root`: `/`
  - `image_bmp`: `/image.bmp`
  - `image_binary`: `/image.bin`
  - `image_upload`: `/upload`
  - `fallback`: 未定義 path
- Rules:
  - `hello_probe` は画像状態に依存しない
  - 既存 route の path と主要応答種別は変更しない
  - `fallback` は引き続き未定義 path を `404` として扱う

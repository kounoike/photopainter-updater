# Data Model: HTTPサーバ構成整理

## 1. ServerConfig

- Purpose:
  - サーバ起動時に解決される設定の正準表現。環境変数、既定値、検証結果を 1 箇所で扱う。
- Fields:
  - `port`: HTTP 待受ポート
  - `content_dir`: 入力画像を探索するディレクトリ
  - `use_lab`: 色距離計算方式の切替
  - `use_atkinson`: ディザ方式の切替
  - `diffusion_rate`: 誤差拡散の強度
  - `use_zigzag`: 走査順の切替
- Validation:
  - `port` は有効なポート番号でなければならない
  - `content_dir` は存在するディレクトリとして解決できることが望ましい
  - `diffusion_rate` は許容範囲内へ正規化されるか、入力不正として案内される
- Relationships:
  - `AppState` の初期化元になる
  - `StartupLogSummary` の材料になる

## 2. AppState

- Purpose:
  - request handler が共有する実行時状態。設定値とログ出力先を束ねる。
- Fields:
  - `content_dir`
  - `logger`
  - `request_counter`
  - `dither_options`
- Rules:
  - handler から参照される情報のみを持ち、起動専用ロジックは含めない
  - `ServerConfig` から派生して生成される

## 3. DitherOptions

- Purpose:
  - 画像変換パイプラインに渡すディザ関連オプションをまとめる。
- Fields:
  - `use_lab`
  - `use_atkinson`
  - `diffusion_rate`
  - `use_zigzag`
- Rules:
  - 画像ロードや HTTP route から独立した純粋な変換パラメータとして扱う
  - 変換関数群は個別の bool/float をばらばらに受け取らず、この単位で受け取る

## 4. AccessLogEvent

- Purpose:
  - 1 リクエスト分のアクセス記録と失敗分類を表す。
- Fields:
  - `request_id`
  - `timestamp`
  - `remote`
  - `method`
  - `path`
  - `status`
  - `outcome`
- Outcome variants:
  - `success`
  - `input-missing`
  - `transform-failed`
  - `not-found`
  - `internal-error`
- Rules:
  - すべての主要 route と失敗系で 1 件ずつ生成される
  - 出力形式が変わっても、意味上の項目は保持される

## 5. ImagePipelineRequest

- Purpose:
  - 入力画像の読込から応答生成までに必要な情報を束ねる。
- Fields:
  - `input_path`
  - `response_format`
  - `dither_options`
- Rules:
  - route 層は request から必要最小限の変換要求を組み立て、詳細処理は pipeline 層へ委譲する

## 6. ImagePipelineResult

- Purpose:
  - 変換成功時の応答 payload と失敗時の分類を表す。
- Variants:
  - `bmp_response`
  - `binary_response`
  - `input_error`
  - `transform_error`
- Relationships:
  - `response` モジュールが HTTP `Response` へ変換する
  - `AccessLogEvent.outcome` の決定材料になる

## 7. ModuleBoundary

- Purpose:
  - 今回の refactor で維持すべき責務境界を設計上明示する。
- Variants:
  - `config`: 設定読込と検証
  - `app`: 起動配線と `AppState` 初期化
  - `routes`: router と handler
  - `logging`: 起動ログとアクセスログの出力
  - `response`: HTTP response 生成
  - `image_pipeline`: 画像読込、ディザ、BMP/Binary 生成
- Rules:
  - 各 boundary は単独で理解可能であること
  - route から image pipeline への依存は許容するが、image pipeline から route へ逆依存させない

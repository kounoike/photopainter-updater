# Data Model: POST画像保存

## 1. UploadRequest

- Purpose:
  - `POST /upload` で受け取った入力を、handler から保存処理へ渡すための正準表現。
- Fields:
  - `transport`: `raw_body` または `multipart_form`
  - `content_type`: リクエストの `Content-Type`
  - `filename_hint`: multipart の filename、または raw body では空
  - `body_bytes`: 保存候補の元データ
  - `remote`: 送信元識別子
- Validation:
  - `body_bytes` は空であってはならない
  - multipart の場合、保存対象として扱う単一の画像 field を特定できること
- Relationships:
  - `UploadCandidate` の入力元になる

## 2. UploadCandidate

- Purpose:
  - decode 済みの受理可能画像と、保存前の判定結果を表す。
- Fields:
  - `decoded_image`: decode 済みの画像
  - `detected_format`: 受信内容から判定した元画像形式
  - `source_dimensions`: 元画像の幅と高さ
- Validation:
  - 受理可能な画像形式として decode 成功していること
  - 元画像サイズが 0x0 でないこと
- Relationships:
  - `NormalizedImage` の生成元になる

## 3. NormalizedImage

- Purpose:
  - 保存条件を満たすよう正規化された画像を表す。
- Fields:
  - `png_bytes`: PNG 形式へ再 encode した保存内容
  - `width`: 480
  - `height`: 800
  - `normalization_applied`: 形式変換やリサイズ・中央クロップの有無
- Validation:
  - 常に PNG として encode できること
  - 常に 480x800 であること
- Relationships:
  - `CurrentImageFile` の置換対象になる

## 4. CurrentImageFile

- Purpose:
  - 現在配信で参照される `image.png` と一時置換の状態を表す。
- Fields:
  - `target_path`: 現在画像の本体パス
  - `temp_path`: 一時保存パス
  - `exists_before_update`: 更新前に現在画像が存在したか
- State Transitions:
  - `idle`: 更新前の安定状態
  - `writing_temp`: 一時ファイルへ保存中
  - `replacing`: 一時ファイルを現在画像へ置換中
  - `committed`: 新しい現在画像へ更新完了
  - `rolled_back`: 一時ファイル破棄後、旧現在画像を維持
- Rules:
  - `committed` 前に `target_path` を壊してはならない
  - 失敗時は `rolled_back` に戻り、既存画像を維持する

## 5. UploadResult

- Purpose:
  - upload 処理全体の結果と応答生成材料を表す。
- Variants:
  - `success`
  - `unsupported_media`
  - `invalid_payload`
  - `save_failed`
  - `internal_error`
- Fields per result:
  - `status`
  - `message`
  - `replaced_existing_image`
  - `normalization_summary`
- Relationships:
  - `response.rs` の upload 応答生成に使う
  - `AccessLogEvent.outcome` の upload 分類に使う

## 6. AccessLogEvent Extension

- Purpose:
  - 既存アクセスログへ upload の成否を統合する際の拡張観点を明示する。
- Added outcome candidates:
  - `upload-success`
  - `upload-invalid`
  - `upload-save-failed`
- Rules:
  - `POST /upload` も 1 request につき 1 件のログを出す
  - GET route 用の既存 outcome と矛盾しない命名にする

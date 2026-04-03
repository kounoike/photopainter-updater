# データモデル: ComfyUI PNG POSTノード

## 1. 送信対象画像

| 項目 | 型 | 必須 | 説明 |
|------|----|------|------|
| `image` | ComfyUI `IMAGE` | 必須 | workflow から渡される画像入力 |
| `batch_size` | `int` | 必須 | 入力画像枚数。`1` のみ許可 |
| `png_bytes` | `bytes` | 実行時生成 | HTTP request body として送る PNG バイト列 |

### Validation Rules

- `image` が未接続または `None` の場合は失敗
- `batch_size != 1` の場合は失敗
- PNG 変換後の `png_bytes` が空の場合は失敗

## 2. 送信先 URL

| 項目 | 型 | 必須 | 説明 |
|------|----|------|------|
| `url` | `str` | 必須 | node widget から入力する送信先 |
| `scheme` | `str` | 必須 | `http` または `https` |
| `normalized_url` | `str` | 実行時生成 | validation 後に送信へ使う URL |

### Validation Rules

- 空文字は失敗
- `http` / `https` 以外は失敗
- host が欠落している URL は失敗

## 3. HTTP 送信要求

| 項目 | 型 | 必須 | 説明 |
|------|----|------|------|
| `method` | `str` | 必須 | 常に `POST` |
| `headers.content_type` | `str` | 必須 | 常に `image/png` |
| `body` | `bytes` | 必須 | `png_bytes` をそのまま使用 |

### Invariants

- multipart/form-data にしない
- body に追加メタデータや envelope を付けない
- 026 の `POST /upload` にそのまま送れる形式を保つ

## 4. 送信結果

| 項目 | 型 | 必須 | 説明 |
|------|----|------|------|
| `success` | `bool` | 必須 | `200 OK` のときのみ `true` |
| `status_code` | `int \| None` | 条件付き | 応答を受けた場合のみ保持 |
| `response_excerpt` | `str` | 条件付き | 応答本文の短い要約 |
| `error_kind` | `str \| None` | 条件付き | `invalid_url` / `invalid_image` / `network_error` / `http_error` など |
| `ui_message` | `str` | 必須 | ComfyUI 上で利用者に見せる summary |

### State Transitions

1. `pending`
2. `validated`
3. `encoded`
4. `posted`
5. `succeeded` または `failed`

`failed` へ遷移した場合は例外送出により workflow 実行を失敗扱いにする。

## 5. 配置エンティティ

| 項目 | 型 | 必須 | 説明 |
|------|----|------|------|
| `repo_source_path` | `path` | 必須 | `comfyui/custom_node/comfyui-photopainter-custom` |
| `runtime_install_path` | `path` | 必須 | `comfyui-data/custom_nodes/comfyui-photopainter-custom` |
| `install_mode` | `enum` | 必須 | `symlink` または `copy` |

### Validation Rules

- ComfyUI の node discovery は `runtime_install_path` 側に存在していることを前提とする
- repo 配下のソースだけでは ComfyUI に自動認識されない

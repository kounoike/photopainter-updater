# Contract: ComfyUI PNG POSTノード

## 1. Node Metadata

| 項目 | 値 |
|------|----|
| Category | `photopainter/http` |
| Display Name | `PhotoPainter PNG POST` |
| Output Node | `true` |
| Return Types | `()` |

## 2. Inputs

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `image` | `IMAGE` | Yes | 単一画像 1 枚を受け取る |
| `url` | `STRING` | Yes | `http` または `https` の送信先 URL |

## 3. Request Contract

node は実行時に次の HTTP request を生成する。

| 項目 | 値 |
|------|----|
| Method | `POST` |
| Header | `Content-Type: image/png` |
| Body | PNG raw bytes |

### Compatibility Target

- 026 の `POST /upload`
- その他、`Content-Type: image/png` の raw body を受け取る任意 endpoint

## 4. Success Contract

### Preconditions

- `image` が接続されている
- `image` は単一画像である
- `url` は有効な `http` / `https` URL である

### Success Condition

- HTTP response status が `200`

### Success Result

node は ComfyUI UI に summary を返す。

例:

```text
POST success: 200 OK -> http://127.0.0.1:8000/upload
```

戻り値は終端ノードとして以下を想定する。

```python
{
    "ui": {
        "text": ["POST success: 200 OK -> http://127.0.0.1:8000/upload"]
    },
    "result": (),
}
```

## 5. Failure Contract

次のケースでは node 実行を失敗扱いにし、例外を送出する。

| Failure Kind | Condition | Expected Handling |
|--------------|-----------|------------------|
| `invalid_url` | URL 空、scheme 不正、host 欠落 | request を送らず失敗 |
| `invalid_image` | 入力不足、複数画像バッチ、PNG 化失敗 | request を送らず失敗 |
| `network_error` | DNS / 接続 / timeout 失敗 | workflow 失敗 |
| `http_error` | `200` 以外の status | status と本文要約を含めて失敗 |

例:

```text
POST failed: 400 Bad Request -> invalid payload
```

## 6. Batch Handling

- `image` が複数画像を含む場合、先頭だけ送らない
- 1 node 実行で複数回 POST しない
- 利用者へ単一画像のみ対応であることをエラーメッセージで示す

## 7. Installation Boundary

- 実装ソース: `comfyui/custom_node/comfyui-photopainter-custom/`
- ComfyUI 実行時配置: `comfyui-data/custom_nodes/comfyui-photopainter-custom/`
- 本 feature では ComfyUI registry 公開や ComfyUI Manager 登録までは扱わない

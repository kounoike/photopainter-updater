# Contract: HTTPサーバ Compose Runtime

## 1. Service Contract

compose は HTTP サーバ service を提供する。

| 項目 | 期待値 |
|------|--------|
| 起動単位 | `docker compose up -d server` で単体起動可能 |
| 共存 | 既存 `comfyui` / `ollama` / `ai-toolkit` と同一 compose 内で共存 |
| 停止 | compose で停止可能 |
| ログ | compose で標準出力ログを確認可能 |

## 2. Endpoint Compatibility

compose 起動後も次を維持する。

| Path | Expected |
|------|----------|
| `/` | 現在画像の基本取得導線 |
| `/image.bmp` | BMP 配信 |
| `/image.bin` | binary 配信 |
| `/upload` | 現在画像更新 |

## 3. Storage Contract

- host 側 `server/contents/` を継続利用する
- upload 後の `image.png` 更新は host 側にも反映される

## 4. Documentation Contract

- root README は compose 起動手順を案内する
- `server/run.sh` への案内は残さない
- server 関連文書は compose 下での起動、停止、ログ確認、疎通確認を説明する

# Contract: RunPod Ollama runtime

## 1. Image Contract

| 項目 | 契約 |
|------|------|
| image 入口 | RunPod serverless 用の custom Docker image を repo 内 asset から build できる |
| upstream 依存 | upstream `worker-comfyui` の `/start.sh` と handler 導線を再利用する |
| Ollama 導入 | image 起動時に `ollama serve` を実行できる |
| ローカル互換 | ローカル Docker で同じ image を起動して挙動確認できる |

## 2. Runtime Contract

- 起動時に Ollama sidecar を先に起動する
- `http://127.0.0.1:11434/api/version` の readiness 成功後に upstream `/start.sh` へ委譲する
- Ollama API は localhost 限定で使う
- `KEEP_ALIVE` は image で固定しない
- model pull warning があっても worker 起動自体は継続する

## 3. Storage Contract

- `/runpod-volume` が利用可能な場合は、Ollama model directory をその配下へ向ける
- `/runpod-volume` が利用不能な場合は、コンテナ内一時領域へフォールバックする
- 起動ログから persistent / ephemeral のどちらが選ばれたか判断できる
- persistent mode ではコンテナ再作成後も取得済み model の再利用を期待できる
- ephemeral mode ではコンテナ再作成後の model 維持を保証しない

## 4. Configuration Contract

| 項目 | 契約 |
|------|------|
| Ollama bind | `127.0.0.1:11434` を使う |
| model directory | `OLLAMA_MODELS` に実保存先を設定する |
| pull model list | 単一 env 値のカンマ区切り一覧で受け取る |
| empty pull list | 事前 pull を行わず起動継続する |
| pull failure | failure model 名を warning として残す |

## 5. Verification Contract

- ローカル擬似検証手順は `/runpod-volume` bind mount あり・なしの両方を含む
- ローカル擬似検証手順は Ollama API readiness 確認を含む
- ローカル擬似検証手順は test payload を worker へ送る確認を含む
- RunPod 本番向け手順は Network Volume 接続位置と確認方法を含む

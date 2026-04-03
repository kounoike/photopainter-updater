# 調査メモ: ComfyUI PNG POSTノード

## Decision 1: ノード実装は従来型の Python custom node API を使う

**Decision**: `NODE_CLASS_MAPPINGS`、`INPUT_TYPES`、`RETURN_TYPES`、`FUNCTION`、`OUTPUT_NODE` を持つ従来型の Python custom node として実装する。  
**Rationale**: 既存の ComfyUI 公式ドキュメントではこの属性群が backend custom node の基本契約として整理されている。ローカルにも `websocket_image_save.py` の実例があり、今回の終端ノード要件に最短で合う。  
**Alternatives considered**:
- `comfy_api.latest` ベースの新しい extension API を使う: 将来性はあるが、この repo の既存 runtime とローカル実例から離れ、実装量も増える
- フロントエンド JS 拡張込みの複合ノードにする: 今回は不要

## Decision 2: HTTP 送信は Python 標準ライブラリ `urllib.request` を使う

**Decision**: `urllib.parse` で URL を検証し、`urllib.request.Request` で `POST` を送る。  
**Rationale**: `Content-Type: image/png` の raw body 送信、timeout 指定、status 取得、HTTP エラー処理までは標準ライブラリで十分に満たせる。追加の `requests` 依存を持ち込まない方が install 手順が単純で、ComfyUI 側の依存衝突リスクも減る。  
**Alternatives considered**:
- `requests` を追加する: API は分かりやすいが依存追加が必要
- `aiohttp` を使う: ComfyUI 側の async 文脈に載せる必要があり過剰

## Decision 3: 画像変換は tensor -> `numpy` -> `Pillow` -> PNG bytes とする

**Decision**: ComfyUI の `IMAGE` 入力を `255.0 * image.cpu().numpy()` で `uint8` 化し、`Pillow` で PNG にエンコードする。  
**Rationale**: ComfyUI 公式 repo の議論やローカル custom node 例は、`IMAGE` をこの系列で PIL 画像へ変換する実装慣習を示している。026 側が PNG raw body を受けるため、途中ファイルを作らず memory 上で PNG bytes を得るのが最小構成。  
**Alternatives considered**:
- 一時ファイルへ PNG 保存してから読み直す: ディスク I/O が増え、不要
- OpenCV 等を追加する: 依存過多

## Decision 4: 成功時は UI summary、失敗時は例外で workflow を止める

**Decision**: node は `RETURN_TYPES = ()` の終端ノードとし、成功時に `{"ui": {"text": [...]}, "result": ()}` を返し、失敗時は `ValueError` または `RuntimeError` を投げる。  
**Rationale**: spec は送信専用終端ノードであることと、送信失敗を workflow 失敗として扱うことを要求している。出力ソケットを持たず、UI summary と例外で状態を分ける方が node の意味が明確になる。  
**Alternatives considered**:
- 文字列 output を返す: workflow 継続が可能になり、失敗見逃しの余地が増える
- 失敗しても成功扱いで summary だけ返す: spec に反する

## Decision 5: repo パスと ComfyUI 実行パスは分離し、symlink/copy で接続する

**Decision**: 実装ソースは `comfyui/custom_node/comfyui-photopainter-custom` に置き、ComfyUI の実行時には `comfyui-data/custom_nodes/comfyui-photopainter-custom` から参照させる。  
**Rationale**: ユーザー要求の repo パスは `custom_node` 単数形だが、既存 compose の mount は `custom_nodes` 複数形である。compose を変更せずに両立させるには、repo 管理ソースと runtime install 位置を分けるのが最も安全。  
**Alternatives considered**:
- compose を変えて repo パスを直接 mount する: scope を広げすぎる
- 実装ソースも `comfyui-data/custom_nodes` 直下に置く: ユーザー要求の配置を満たさない

## Decision 6: バッチ画像はサポートせず、単一画像のみ受け付ける

**Decision**: `IMAGE` 入力に複数枚が含まれる場合は送信エラーとする。  
**Rationale**: spec は単一画像送信を対象としており、026 の upload も 1 request = 1 current image 更新という意味を持つ。バッチを暗黙に連続送信したり先頭だけ送ったりすると、利用者の期待がぶれる。  
**Alternatives considered**:
- 先頭画像だけ送る: 暗黙の欠落が起きる
- バッチ全件を順に送る: 1 実行で複数回 POST になり spec の単純性を壊す

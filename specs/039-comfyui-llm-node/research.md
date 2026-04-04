# 調査メモ: ComfyUI local LLM node

## Decision 1: node は既存 `comfyui-photopainter-custom` に追加する単一 Python custom node とする

**Decision**: `NODE_CLASS_MAPPINGS` を使う既存の Python custom node 方式を維持し、ローカル LLM 推論 node を同一ライブラリへ追加する。  
**Rationale**: repo にはすでに `PhotoPainter PNG POST` が同方式で実装されており、配布経路、読み込み方法、test 配置、README 導線が揃っている。別 package や別 runtime を作ると scope が広がる。  
**Alternatives considered**:
- 新しい custom node package を別ディレクトリに切る: 分離は明確だが配布・build・README を増やす
- ComfyUI 本体や別サービスへ組み込む: scope 外

## Decision 2: backend は `transformers` と `llama-cpp-python` の adapter で吸収する

**Decision**: node 入力で `transformers` / `llama-cpp` を選ばせ、内部では backend ごとの小さな adapter 関数へ委譲する。  
**Rationale**: spec は backend 切替を要求しているが、node 自体は薄く保つ必要がある。adapter 層に閉じると input contract を固定したまま backend 差分だけを隔離できる。  
**Alternatives considered**:
- backend ごとに別 node を作る: UI とテストが重複する
- backend 判定を model 名の規約に埋め込む: 利用者に暗黙ルールを強いる

## Decision 3: `think_mode` は共通列挙値 `off` / `generic` / `qwen` / `gemma` / `deepseek_r1` に限定し、prompt formatting preset として扱う

**Decision**: node の widget は backend 非依存の `think_mode` を持ち、`off`、`generic`、`qwen`、`gemma`、`deepseek_r1` の 5 値を初期対応とする。`think_mode` は backend 固有 API の切替ではなく、system/user prompt へ適用する prompt formatting preset として扱う。`generic` は family 固有最適化を持たない best-effort preset とする。  
**Rationale**: user は family ごとの差異を明示的に制御したい一方、backend ごとの細かい proprietary flag を node contract に露出したくない。prompt formatting に寄せると `transformers` と `llama-cpp` の両方で同じ入力 contract を保てる。  
**Alternatives considered**:
- boolean のみ: family 差分を表現できない
- backend ごとに別 widget や別 API にする: node contract が肥大化する
- `granite` 等まで最初から広げる: adapter と test が増える

## Decision 4: model 保存先は環境変数 `COMFYUI_LLM_MODEL_CACHE_DIR` を一次参照する

**Decision**: model cache / 保存先の統一入力として `COMFYUI_LLM_MODEL_CACHE_DIR` を採用し、`.env` から `compose.yml` 経由で ComfyUI container へ渡す。未設定時は backend の既定保存先へ fallback する。  
**Rationale**: Hugging Face 系と llama.cpp 系で既定の保存場所や環境変数が異なるため、node 側で 1 本の env var を見る方が利用者向け説明が単純になる。`.env` を node 自身に読ませるのではなく compose で注入する方が責務も薄い。  
**Alternatives considered**:
- backend 固有環境変数だけをそのまま使う: 利用者が backend ごとの差を理解する必要がある
- 環境変数を必須にする: 試験導入の負担が増える

## Decision 4a: `model_id` は Hugging Face Hub の `user/repo` とし、`llama-cpp` は任意の `model_file` で補助する

**Decision**: `model_id` は backend 共通で Hugging Face Hub の `user/repo` として扱う。`llama-cpp` は追加で任意入力 `model_file` を受け取り、repo 内に複数 GGUF がある場合の選択に使う。  
**Rationale**: `model_id` の意味を backend 間で揃えつつ、GGUF ファイル選択だけを `model_file` へ切り出すと input contract がわかりやすい。  
**Alternatives considered**:
- `llama-cpp` だけローカルファイル path を `model_id` に入れる: backend ごとに意味が変わりすぎる
- `model_file` を持たない: 複数 GGUF repo の扱いが曖昧になる

## Decision 5: JSON/schema 検証は `jsonschema` を使う

**Decision**: `json_output=true` の場合はまず `json.loads()` を行い、`json_schema` が非空のときだけ `jsonschema` で検証する。  
**Rationale**: schema 検証を自前実装すると node が厚くなりやすく、必須キー・型・追加プロパティなどの扱いで曖昧さが残る。依存追加は増えるが、責務を小さく維持できる。  
**Alternatives considered**:
- parse 成功だけを見る: schema 利用価値が下がる
- 自前の最小 validator を書く: edge case と test が増える

## Decision 6: retry は parse 失敗または schema 不一致だけに限定する

**Decision**: retry 対象は JSON parse 失敗と schema 不一致のみとし、上限は小さく固定できる設計にする。model load 失敗、OOM、backend import 失敗、未対応 `think_mode` は即失敗とする。  
**Rationale**: local LLM の出力ぶれは retry で改善することがあるが、backend や model 自体の失敗は retry しても改善しにくい。対象を絞ることで node の責務過多を防げる。  
**Alternatives considered**:
- すべて retry する: 遅いだけで原因を隠す
- retry を持たない: JSON mode の運用安定性が下がる

## Decision 7: node は後続利用向けの単一 `STRING` 出力を持つ非終端 node とする

**Decision**: node は output node にせず、成功時は単一の `STRING` 出力だけを返す通常 node とする。text mode では plain text、JSON mode では valid JSON string を返す。必要に応じて UI summary を付与し、失敗時は例外で workflow を止める。  
**Rationale**: spec の主用途は prompt planner など後続ノードでの文字列再利用であり、出力契約は 1 本の方が workflow 接続と contract test が単純になる。  
**Alternatives considered**:
- 終端 node にする: 結果を downstream へ渡しにくい
- `text` と `json_text` を分けて返す: output 契約が複雑になる

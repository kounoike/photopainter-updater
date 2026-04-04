# 機能仕様: ComfyUI local LLM node

**Feature Branch**: `039-comfyui-llm-node`  
**Created**: 2026-04-04  
**Status**: Draft  
**Input**: ユーザー記述: "モデルをある程度自由に Hugging Face から選べること、think は複数形式から選べること、モデル保存先ディレクトリは環境変数で指定できること。既存の comfyui/custom_node/comfyui-photopainter-custom に薄い LLM 推論ノードを追加したい"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Clarifications

### Session 2026-04-04

- Q: `think` の指定方式はどうするか → A: node 共通の列挙値 `think_mode` で持つ
- Q: schema の与え方はどうするか → A: `json_schema` 文字列入力のみに絞る
- Q: model 保存先環境変数は必須か → A: 任意。未設定時は backend 既定保存先を使う
- Q: 初期対応する `think_mode` はどこまでか → A: `off`、`generic`、`qwen`、`gemma`、`deepseek_r1` まで

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Workflow 内でローカル LLM 推論を使いたい (Priority: P1)

ComfyUI の利用者は、既存の PhotoPainter custom node ライブラリに追加された単一ノードから、`system_prompt` と `user_prompt` を渡してローカル LLM 推論を実行し、その結果を後続ノードへ渡したい。これにより、画像生成 workflow の中で prompt planning などの言語処理を分散させずに扱える。

**Why this priority**: この feature の中心価値は、ComfyUI workflow の中だけでローカル LLM 推論を扱えることにあるため。

**Independent Test**: ComfyUI 起動後に追加ノードを workflow へ配置し、`transformers` または `llama-cpp` を選択して `system_prompt` と `user_prompt` を渡したとき、単発のローカル推論が成功し、単一 `STRING` 出力の生成結果を後続ノードで参照できることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が ComfyUI 上で local LLM node を利用できる状態, **When** `system_prompt`、`user_prompt`、backend、`model_id`、必要に応じて `model_file` を与えて実行する, **Then** node は Python 内でローカル推論を実行し、生成結果を workflow 内で利用できる単一 `STRING` 出力として返す。
2. **Given** 利用者が同じ workflow を異なる model 指定で再実行する状態, **When** `model_id` または `model_file` を切り替えて node を実行する, **Then** node は指定された model 解決結果を使って独立に推論し、model 切替以外の workflow 構成を変更させない。

---

### User Story 2 - JSON 出力を固定し schema に合わせたい (Priority: P2)

ComfyUI の利用者は、LLM 出力を自由文ではなく JSON として固定し、必要な場合は期待 schema に一致した結果だけを成功扱いにしたい。これにより、prompt planner などの後続ノードが構造化された出力を前提に処理できる。

**Why this priority**: prompt planning 用途では、自由文よりも構造化出力を安定して得られることが再利用性に直結するため。

**Independent Test**: `json_output=true` で node を実行し、schema なしでは JSON parse 成功で通ること、schema ありでは必須キーや型が一致した JSON のみ成功扱いになることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が `json_output=true` を設定している状態, **When** node が JSON として解釈可能な応答を生成する, **Then** node はその結果を単一 `STRING` 出力の JSON 文字列として返し、workflow で再利用できる。
2. **Given** 利用者が `json_schema` を設定している状態, **When** node が schema に一致する JSON を生成する, **Then** node は schema 検証済みの結果として成功扱いにする。

---

### User Story 3 - schema 不一致時に限定リトライしたい (Priority: P3)

ComfyUI の利用者は、JSON parse 失敗や schema 不一致が起きた場合にだけ限定回数の自動リトライを行い、それでも満たせない場合は失敗理由を切り分けたい。これにより、軽微な出力ぶれだけを吸収しつつ、不正な結果を黙って通さずに運用できる。

**Why this priority**: local LLM の出力揺れは現実的に起こるため、限定リトライがないと workflow の安定性が落ちる一方、無制限リトライは責務過多になるため。

**Independent Test**: schema に合わない JSON を返す条件を用意し、node が parse 失敗または schema 不一致に限って上限回数まで再試行し、最終的に不一致のままなら workflow error として失敗することを確認する。

**Acceptance Scenarios**:

1. **Given** `json_output=true` と schema 検証が有効である状態, **When** 最初の生成結果が JSON parse 失敗または schema 不一致になる, **Then** node は設定された上限回数の範囲で再試行する。
2. **Given** 再試行後も結果が schema を満たさない状態, **When** node が最終判定に到達する, **Then** node は成功扱いにせず、JSON 失敗理由を判別できる実行エラーとして終了する。

### Edge Cases

- `json_output=false` のときは `json_schema` が与えられていても検証を要求しないこと。
- schema 自体が不正な JSON または不正な schema 形式である場合は、推論前に設定不備として失敗すること。
- backend が選択された `think_mode` に対応しない場合でも、未対応設定を黙って別意味へ変換しないこと。
- `think_mode` の初期対応範囲は `off`、`generic`、`qwen`、`gemma`、`deepseek_r1` に限定し、それ以外を暗黙対応しないこと。
- `generic` は family 固有最適化を持たない best-effort の汎用 thinking preset であり、特定 model での動作保証を意味しないこと。
- `model_id` は Hugging Face Hub の `user/repo` 形式を前提とし、`llama-cpp` で repo 内に複数 GGUF がある場合は `model_file` 未指定のまま成功扱いにしないこと。
- model の保存先ディレクトリ環境変数 `COMFYUI_LLM_MODEL_CACHE_DIR` が未設定の場合は backend 既定保存先を使い、無効な値が与えられた場合は利用者が保存先設定不備を判別できること。
- model 指定が Hugging Face 上に存在しても、実行環境に対して不適切なサイズや形式でロード失敗した場合は、JSON 失敗とは別の model/backend error として扱うこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST `comfyui/custom_node/comfyui-photopainter-custom` 配下の既存 custom node ライブラリに追加できる ComfyUI node として local LLM 推論機能を提供しなければならない。
- **FR-002**: System MUST node 入力として少なくとも `system_prompt` と `user_prompt` を受け取り、単発のローカル推論を実行できなければならない。
- **FR-003**: Users MUST be able to node ごとに `transformers` または `llama-cpp` を選択できなければならない。
- **FR-004**: Users MUST be able to `model_id` として Hugging Face Hub の `user/repo` 形式を node へ与え、backend が解決可能な範囲で切り替えられなければならない。
- **FR-004a**: Users MUST be able to 任意入力 `model_file` を指定でき、主に `llama-cpp` backend で repo 内の対象 GGUF を明示できなければならない。
- **FR-005**: System MUST model 保存先ディレクトリを環境変数 `COMFYUI_LLM_MODEL_CACHE_DIR` で任意指定でき、その設定がある場合は local model 解決に反映し、未設定時は backend 既定保存先を使わなければならない。
- **FR-006**: System MUST `think` に相当する推論モードを node 共通入力 `think_mode` の列挙値として選択できるようにし、node は backend 固有 API ではなく prompt formatting preset として扱わなければならない。
- **FR-006a**: System MUST `think_mode` の初期対応値を `off`、`generic`、`qwen`、`gemma`、`deepseek_r1` に限定しなければならない。
- **FR-006b**: System MUST `generic` を family 固有最適化を持たない best-effort preset として扱い、特定 model での動作保証を示してはならない。
- **FR-007**: System MUST `json_output` を有効化した場合、生成結果を JSON として解釈し、parse 不能な結果を成功扱いにしてはならない。
- **FR-008**: System MUST `json_schema` を multiline 文字列として受け取れるようにし、空でない場合は schema として解釈しなければならない。
- **FR-009**: System MUST `json_output=true` かつ schema が指定されている場合、JSON parse と schema 検証の両方に成功した結果だけを成功扱いにしなければならない。
- **FR-010**: System MUST parse 失敗または schema 不一致の場合に限り、設定された上限回数まで再試行できなければならない。
- **FR-011**: System MUST model load 失敗、backend 実行失敗、設定不備、JSON parse 失敗、schema 不一致を区別できる形で利用者へ示さなければならない。
- **FR-012**: System MUST 最終成功時に workflow 内で再利用できる単一 `STRING` 出力を返し、`json_output=true` のときはその文字列が valid JSON でなければならない。
- **FR-013**: System MUST node 内に prompt planning 固有ロジック、画像生成ロジック、長期会話保持、無制限自動修正ループを持ち込んではならない。
- **FR-014**: System MUST 利用者向け文書に backend 選択、model 指定、保存先環境変数、JSON schema 指定、retry 挙動を説明しなければならない。

### Key Entities *(include if feature involves data)*

- **Local LLM Node 設定**: backend、`model_id`、任意の `model_file`、`system_prompt`、`user_prompt`、`think_mode`、`json_output`、schema 指定、retry 回数など、1 回の node 実行条件を表す設定集合。`think_mode` の初期対応値は `off`、`generic`、`qwen`、`gemma`、`deepseek_r1` とする。
- **Model 保存先ディレクトリ**: 環境変数 `COMFYUI_LLM_MODEL_CACHE_DIR` で任意指定される local model cache または保存先。未設定時は backend 既定保存先を使い、設定がある場合は backend が model 解決時に参照する。
- **JSON 出力契約**: `json_output` の有無、`json_schema`、retry 上限、検証成否から成る構造化出力条件。
- **推論結果**: node が返す単一 `STRING` 出力。成功時は workflow で再利用され、`json_output=true` の場合は JSON 文字列として扱われる。失敗時は原因区分を伴う実行エラーとして扱われる。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `comfyui/custom_node/comfyui-photopainter-custom` 配下への local LLM node 追加
- 当該 node の利用手順、設定項目、エラー挙動、環境変数運用の文書化
- node の unit test と contract test の追加
- backend ごとの think 形式選択と JSON schema 検証に関する最小限の実装
- `think_mode` の初期対応を `off`、`generic`、`qwen`、`gemma`、`deepseek_r1` に限定した最小実装
- `compose.yml` を介した `COMFYUI_LLM_MODEL_CACHE_DIR` の runtime 注入

### Forbidden Scope

- ComfyUI 本体コアコードの無関係な改変
- server、firmware、既存 upload endpoint の仕様変更
- prompt planning 専用テンプレートや画像生成 workflow 固有ロジックの同時追加
- 新しい常駐サービス、HTTP 待受 API、外部ジョブキューの導入
- model 学習、LoRA 管理、会話履歴永続化、複数候補 rerank などの高度機能追加

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は ComfyUI workflow から 1 回の node 実行で local LLM 推論を行い、単一 `STRING` 出力の生成結果を後続ノードへ渡せる。
- **SC-002**: 利用者は backend と model 指定を切り替えても、custom node ライブラリや workflow の構造を作り直さずに再利用できる。
- **SC-003**: `json_output=true` のとき、利用者は JSON parse 失敗と schema 不一致を成功扱いにせず判別できる。
- **SC-004**: schema 不一致や parse 失敗のような出力揺れに対して、node は限定リトライで吸収を試み、それでも不正な結果を workflow 成功として通さない。

## Assumptions

- 利用者は ComfyUI を Docker Compose または同等環境で起動し、既存 `comfyui-photopainter-custom` ノード群を使える。
- 利用者は推論対象 model の `user/repo` 識別子を把握しており、必要に応じて `llama-cpp` 用の `model_file` を追加指定する。
- model 保存先ディレクトリは必要に応じて環境変数 `COMFYUI_LLM_MODEL_CACHE_DIR` として node 実行環境に注入できる。
- schema 検証は `json_output=true` のときだけ意味を持ち、自由文生成時の品質保証までは今回扱わない。
- retry は parse 失敗または schema 不一致の吸収に限定し、model load や backend 失敗の自動復旧は対象外とする。

## Documentation Impact

- `comfyui/custom_node/comfyui-photopainter-custom/README.md` に新 node の入出力、backend 選択、`model_id` / `model_file`、`think_mode`、保存先環境変数を追記する必要がある。
- JSON output、schema 指定、retry 条件、失敗分類を説明する利用手順文書が必要になる。
- 必要に応じて compose や devcontainer の運用文書へ model 保存先環境変数の設定例を追加する必要がある。

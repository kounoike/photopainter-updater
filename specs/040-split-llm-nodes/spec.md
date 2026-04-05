# 機能仕様: ComfyUI LLM ノード分離

**Feature Branch**: `040-split-llm-nodes`  
**Created**: 2026-04-05  
**Status**: Draft  
**Input**: ユーザー記述: "ComfyUI の local LLM custom node を backend ごとに分離し、transformers ノードと llama-cpp ノードを別々にする。transformers 側には quantization_mode を自然に持たせ、llama-cpp 側には GGUF 前提の model_file や量子化向け運用を閉じ込める。既存の UI と責務を整理し、backend 固有の think 制御や制約をそれぞれのノードへ寄せたい。"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Clarifications

### Session 2026-04-05

- Q: 既存の単一 `PhotoPainter LLM Generate` ノードをどう扱うか → A: 既存ノードは即時削除する
- Q: `transformers` 専用ノードで `model_file` をどう扱うか → A: `transformers` 専用ノードは `model_file` を完全に持たない
- Q: `llama-cpp` 専用ノードで `think_mode` をどう扱うか → A: `llama-cpp` 専用ノードは `think_mode` を持たない

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Transformers ノードへ責務を集約したい (Priority: P1)

ComfyUI の利用者は、`transformers` backend を使う local LLM 推論を専用ノードとして利用し、Qwen/Gemma 系の think 制御、structured output、量子化設定を backend 固有の UI で扱いたい。これにより、GGUF 前提の入力項目に邪魔されず、主力 workflow を安定して構成したい。

**Why this priority**: 現状もっとも重要なのは `transformers` を本命経路として使いやすくし、Qwen 系の documented control と量子化設定を素直に扱えるようにすることだから。

**Independent Test**: `transformers` 専用ノードを workflow に配置し、`model_id`、`think_mode`、`quantization_mode`、`json_output` を指定して実行したとき、量子化設定、Qwen/Gemma 系の documented think 制御、retry 理由を debug 出力で確認しつつ、最終出力を後続ノードへ渡せることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が `transformers` を使いたい状態, **When** 専用ノードへ `model_id`、`think_mode`、`quantization_mode` を設定して実行する, **Then** node は `transformers` 向け UI だけを見せ、`llama-cpp` 専用の入力を要求せずに推論できる。
2. **Given** 利用者が Qwen 系 model で `think_mode=off` または `qwen` を指定している状態, **When** 専用ノードを実行する, **Then** node は documented control と debug 出力で requested think 設定を確認できる。
3. **Given** 利用者が Gemma 系 model で `think_mode=gemma` を指定している状態, **When** 専用ノードを実行する, **Then** node は Gemma documented control を backend 固有実装として適用し、その適用有無を debug 出力で確認できる。

---

### User Story 2 - llama-cpp ノードを GGUF 専用として使いたい (Priority: P2)

ComfyUI の利用者は、`llama-cpp` backend を使う local LLM 推論を別ノードとして利用し、GGUF repo と `model_file` を前提にした設定だけを扱いたい。これにより、`transformers` 向けの量子化や think 制御 UI と混ざらず、GGUF 運用を単純化したい。

**Why this priority**: `llama-cpp` の model 指定や think 制御は `transformers` と前提が違い、1 ノードで両方を抱えると UI と失敗要因が混線するため。

**Independent Test**: `llama-cpp` 専用ノードを workflow に配置し、GGUF repo と `model_file` を与えて推論したとき、`transformers` 専用入力を持たずに実行できること、`model_file` 未指定など GGUF 特有の設定不備を明示 failure にすることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が GGUF model を使いたい状態, **When** `llama-cpp` 専用ノードへ `model_id` と `model_file` を与えて実行する, **Then** node は GGUF 前提の実行条件で推論し、結果を workflow 内で利用できる。
2. **Given** 利用者が `llama-cpp` 専用ノードを使っている状態, **When** `transformers` 向けの量子化設定を期待する, **Then** node はその設定自体を UI に持ち込まず、backend 固有責務を分離した状態を保つ。

---

### User Story 3 - backend 差を workflow レベルで明確にしたい (Priority: P3)

ComfyUI の利用者は、どの workflow が `transformers` 本命経路で、どの workflow が `llama-cpp` の軽量 GGUF 経路なのかをノード名だけで識別したい。これにより、トラブルシュート時に backend 固有の制約をすぐ切り分けたい。

**Why this priority**: 現状の単一ノードは backend の前提差が大きく、debug 出力で見ないと責務が分かりにくいため。

**Independent Test**: ComfyUI 上で両専用ノードを同時に配置し、ノード名、入力項目、debug 出力が backend 固有であることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が同じ workflow で両 backend を比較したい状態, **When** 2 種類の専用ノードを配置する, **Then** ノード名と入力欄だけで backend の違いを識別できる。
2. **Given** 利用者が backend 固有の問題を切り分けたい状態, **When** debug 出力や error を確認する, **Then** backend ごとの制約が単一ノード時代より明確に見える。

### Edge Cases

- 既存の単一 `PhotoPainter LLM Generate` ノードは削除し、backend ごとの専用ノードへ置き換えること。
- `transformers` 専用ノードは `model_file` を持たず、GGUF 前提の UI を一切露出しないこと。
- `llama-cpp` 専用ノードに `quantization_mode` や `BitsAndBytesConfig` 前提の UI を残さないこと。
- `transformers` と `llama-cpp` の両ノードが同時に存在しても、debug 出力やエラー分類が混同されないこと。
- backend 分離後も `json_output` と `json_schema` の契約を崩さず、既存の structured output 検証を維持すること。
- retry は JSON parse failure または schema validation failure のときだけ発生し、backend load failure や think 制御 failure では再試行しないこと。
- debug 出力から retry が発生したか、発生した場合の理由種別が分かること。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST 既存の単一 local LLM node を backend ごとに分離し、少なくとも `transformers` 専用ノードと `llama-cpp` 専用ノードを提供しなければならない。
- **FR-002**: System MUST `transformers` 専用ノードに `quantization_mode` を自然な入力として持たせ、`none`、`bnb_8bit`、`bnb_4bit` を選択できなければならない。
- **FR-002a**: System MUST `transformers` 専用ノードに `model_file` を持ち込んではならない。
- **FR-003**: System MUST `llama-cpp` 専用ノードに GGUF 運用向けの `model_file` 入力を持たせ、`transformers` 専用の量子化 UI や `think_mode` を露出してはならない。
- **FR-004**: System MUST 両専用ノードで `system_prompt`、`user_prompt`、`json_output`、`json_schema`、retry、debug 出力の基本契約を維持しなければならない。
- **FR-004a**: System MUST retry を JSON parse failure または schema validation failure の場合に限定し、その他の backend error では再試行してはならない。
- **FR-005**: System MUST `transformers` 専用ノードで family ごとの documented think 制御を優先し、Qwen/Gemma 系の制御を backend 固有実装として閉じ込めなければならない。
- **FR-006**: System MUST `llama-cpp` 専用ノードで GGUF / `model_file` / context window 前提の validation を担い、`transformers` 側の loader 制約や family 固有 think 制御と混在させてはならない。
- **FR-007**: Users MUST be able to node 名と UI だけで backend の違いを判別できなければならない。
- **FR-008**: System MUST debug 出力に backend 固有設定を含め、少なくとも `transformers` 側では quantization 設定、think 制御要求、raw/sanitized 情報を確認できなければならない。
- **FR-008a**: System MUST debug 出力で retry の発生有無と理由種別を確認できなければならない。
- **FR-009**: System MUST 既存の単一 `PhotoPainter LLM Generate` ノードを削除し、README または関連文書にノード名変更と移行手順を記載しなければならない。
- **FR-010**: System MUST 画像生成本体の VRAM を優先する前提を維持し、両専用ノードとも生成後の backend メモリ解放方針を崩してはならない。

### Key Entities *(include if feature involves data)*

- **Transformers LLM Node**: `transformers` backend 専用の ComfyUI custom node。`model_id`、`quantization_mode`、family ごとの think 制御、structured output 契約を持ち、`model_file` は持たない。
- **LlamaCpp LLM Node**: `llama-cpp` backend 専用の ComfyUI custom node。GGUF 前提の `model_id`、`model_file`、context window validation、structured output 契約を持ち、`think_mode` は持たない。
- **Backend 固有 UI 契約**: どの入力をどの専用ノードに見せるか、どの debug 項目を返すかを定めるノード別入出力契約。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `comfyui/custom_node/comfyui-photopainter-custom` 配下の local LLM node 分離
- backend ごとの node 名、入力項目、debug 出力、validation の整理
- README、workflow 例、tests の更新
- 必要に応じた Dockerfile の依存整理

### Forbidden Scope

- server や firmware 側の仕様変更
- 外部常駐推論サービスの導入
- prompt planner 固有ロジックや画像生成ロジックの追加
- モデル学習や LoRA 管理の導入

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は ComfyUI 上で `transformers` 専用ノードと `llama-cpp` 専用ノードを backend ごとに明確に使い分けられる。
- **SC-002**: `transformers` 専用ノードでは `quantization_mode` を UI から選択でき、`llama-cpp` 専用ノードではその設定が見えない。
- **SC-003**: `llama-cpp` 専用ノードでは GGUF 前提の `model_file` 運用が維持され、`transformers` 専用ノードでは GGUF 前提入力が主経路に出ない。
- **SC-004**: backend ごとの debug / error / 入力契約が明確化され、単一ノード時代よりトラブルシュートがしやすくなる。

## Assumptions

- 利用者は当面 `transformers` を本命経路として使い、`llama-cpp` は GGUF 軽量運用向けの別経路として扱う。
- backend ごとの専用ノード化により、既存の単一ノードより UI と責務の分離が改善する。
- `transformers` 側の量子化は `bitsandbytes` と `accelerate` を使った load-time quantization を前提とする。
- backend 分離後も、structured output と debug 出力の基本思想は維持する。

## Documentation Impact

- `comfyui/custom_node/comfyui-photopainter-custom/README.md` に 2 種類の専用ノード、旧単一ノード削除、backend 別 UI、移行手順を追記する必要がある。
- 必要に応じて workflow 例を backend 別に分けて更新する必要がある。
- テスト文書や contract test の説明も backend 分離後の node 名と入出力に合わせて更新が必要である。

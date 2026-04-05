# 機能仕様: ComfyUI think off 強制

**Feature Branch**: `041-enforce-think-off`  
**Created**: 2026-04-05  
**Status**: Draft  
**Input**: ユーザー記述: "ComfyUI の transformers LLM custom node で think_mode=off を実効的にし、未対応経路では黙って reasoning を流さず明示 failure にする。off 指定時の処理時間差で internal thinking が走っていないことを判断しやすくし、debug 出力で強制無効化の成否を確認できるようにしたい。"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - `off` を実効化したい (Priority: P1)

ComfyUI の利用者は、`PhotoPainter Transformers LLM Generate` ノードで `think_mode=off` を選んだとき、モデルが内部 reasoning を継続する曖昧な成功を避けたい。これにより、処理時間の増加や `<think>` 相当の混入を見て「実は think していた」という状態を防ぎたい。

**Why this priority**: `off` が効かないまま成功扱いになると、応答時間と出力品質の期待が崩れ、workflow の調整が成立しないため。

**Independent Test**: `think_mode=off` を指定して Qwen 系モデルと documented control 未対応モデルをそれぞれ実行し、前者は明示的な think 無効化付きで成功し、後者は黙って成功せず failure になることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が Qwen 系など documented に think 無効化できるモデルで `think_mode=off` を選んでいる, **When** ノードを実行する, **Then** ノードは documented control を強制無効化として適用し、最終出力だけを返す。
2. **Given** 利用者が documented な think 無効化手段を持たないモデルで `think_mode=off` を選んでいる, **When** ノードを実行する, **Then** ノードは best-effort 成功へ落とさず、`think_mode=off` を保証できないことを明示 failure として返す。

---

### User Story 2 - `off` の成否を debug で確認したい (Priority: P2)

ComfyUI の利用者は、`think_mode=off` 実行時に「無効化要求を出したか」「保証できたか」「sanitize で救済しただけではないか」を debug 出力から判別したい。これにより、処理時間差や出力差の原因を workflow 上で切り分けたい。

**Why this priority**: `off` の体感差を見ても、制御が効いたのか偶然短く返っただけか分からないと再現性がないため。

**Independent Test**: `think_mode=off` で成功するモデルと失敗するモデルの双方を実行し、debug 出力から enforced 状態、利用した control 種別、failure 理由を区別できることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が `think_mode=off` で成功した実行結果を確認している, **When** `debug_json` を見る, **Then** documented control の利用有無と `off` を保証できたかどうかを判別できる。
2. **Given** 利用者が `think_mode=off` で失敗した実行結果を確認している, **When** エラーと `debug_json` を見る, **Then** 失敗理由が unsupported な `off` 制御であることを判別できる。

---

### User Story 3 - sanitize 依存の成功を避けたい (Priority: P3)

ComfyUI の利用者は、`think_mode=off` の成功判定が「出てしまった reasoning を後処理で削ったから OK」という経路に依存しないことを求める。これにより、hidden reasoning が継続してトークンや時間を消費する実行を成功扱いにしないようにしたい。

**Why this priority**: 今回欲しいのは見た目の整形ではなく、実行コストを含めた `off` の実効性だから。

**Independent Test**: `think_mode=off` で `<think>...</think>` を含む生出力を返すケースを模擬し、その場合に sanitize 後の成功ではなく `off` 契約違反として扱われることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が `think_mode=off` を指定している, **When** backend が reasoning block を含む生出力を返す, **Then** ノードは sanitize で成功扱いにせず、`off` 契約違反として failure にする。
2. **Given** 利用者が `think_mode=qwen` など `off` 以外を指定している, **When** backend が family 仕様に応じた reasoning block を返す, **Then** ノードは既存の family ごとの挙動を維持し、今回の厳格化を `off` 以外へ不要に拡張しない。

### Edge Cases

- tokenizer/chat template が `enable_thinking=False` のような documented control 引数を受け付けない場合、そのモデルは `think_mode=off` 成功扱いにしないこと。
- `think_mode=off` で `<think>` 相当の生出力が観測された場合、最終テキストの sanitize 有無にかかわらず契約違反として扱うこと。
- `llama-cpp` ノードは `think_mode` を持たないため、今回の厳格化対象に含めないこと。
- `json_output=true` でも `think_mode=off` の保証不能は JSON retry 対象にせず、backend / think 制御 failure として即時失敗すること。
- Gemma 系のように `off` の documented disable を持たない family では、「think トークンを付けないだけ」で `off` 保証済みと見なさないこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST `PhotoPainter Transformers LLM Generate` ノードの `think_mode=off` を best-effort 指示ではなく、強制無効化を保証できる場合だけ成功する契約として扱わなければならない。
- **FR-002**: System MUST `think_mode=off` で documented な think 無効化手段を適用できる model family について、その documented control を generation 開始前に有効化しなければならない。
- **FR-003**: System MUST `think_mode=off` で documented な think 無効化手段を適用できない model family または runtime 経路について、final-only prompt による best-effort 成功へ落とさず明示 failure を返さなければならない。
- **FR-004**: System MUST `think_mode=off` 実行時に backend が reasoning block または同等の think 痕跡を返した場合、sanitize による救済成功ではなく契約違反の failure として扱わなければならない。
- **FR-005**: System MUST `think_mode=off` の debug 出力に、少なくとも requested disable の有無、利用した control 種別、`off` 保証の成否、sanitize 依存の有無を含めなければならない。
- **FR-006**: System MUST `think_mode=off` の failure を JSON parse failure や schema failure と区別し、retry 対象外の think control failure として利用者へ示さなければならない。
- **FR-007**: System MUST `think_mode=off` の厳格化を `transformers` ノードに限定し、`llama-cpp` ノードの契約や UI を変更してはならない。
- **FR-008**: System MUST 既存の `think_mode=qwen`、`gemma`、`deepseek_r1`、`generic` の挙動を不要に壊さず、今回の変更が `off` の実効性強化に限定されるようにしなければならない。
- **FR-009**: System MUST 利用者向け文書で `think_mode=off` が「保証できない経路では失敗する」契約に変わったこと、成功条件、主な未対応ケース、debug の見方を説明しなければならない。

### Key Entities *(include if feature involves data)*

- **Think Off Enforcement Plan**: `think_mode=off` 実行時に、model family、runtime 経路、documented control の可否、保証成否、failure 理由を保持する判定結果。
- **Think Off Debug Contract**: `requested_enable_thinking`、`control_kind`、`documented_control_available` に加え、`off` 保証成否と unsupported / violated 理由を表す debug 情報。
- **Think Trace**: `<think>...</think>` など、backend が内部 reasoning を外部出力へ漏らした痕跡。`off` では成功判定を阻害する。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` の `think_mode=off` 判定、validation、debug 出力の更新
- `comfyui/custom_node/comfyui-photopainter-custom/tests/` の unit test / contract test 更新
- `comfyui/custom_node/comfyui-photopainter-custom/README.md` の `think_mode=off` 契約説明更新

### Forbidden Scope

- server や firmware 側の仕様変更
- `llama-cpp` ノードへの `think_mode` 追加や UI 変更
- `off` 以外の mode に対する大規模な制御方針変更
- 新しい外部推論サービスや計測基盤の導入

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は `think_mode=off` で documented disable が使えるモデルを実行したとき、成功結果と debug 出力から `off` が強制適用されたと判断できる。
- **SC-002**: 利用者は `think_mode=off` で documented disable を保証できないモデルを実行したとき、曖昧な成功ではなく明示 failure を受け取り、内部 reasoning が走った可能性を切り分けられる。
- **SC-003**: `think_mode=off` の実装は、reasoning block を sanitize して成功扱いにする経路を持たず、`off` の実効性を処理時間と出力契約の両面で担保できる。
- **SC-004**: README と debug 情報だけで、利用者が `off` 成功条件と主な未対応ケースを理解できる。

## Assumptions

- 当面 `think_mode=off` の厳格な強制対象は `transformers` backend に限る。
- documented な disable API を持たない family へ一律の `off` を保証することは今回のスコープ外であり、その場合は failure が正しい。
- Qwen 系では chat template 経路の `enable_thinking=False` が主な保証手段であり、その経路が使えない runtime は unsupported と見なす。
- 利用者は処理時間差と debug 出力を併用して `off` の実効性を判断する。

## Documentation Impact

- `comfyui/custom_node/comfyui-photopainter-custom/README.md` に `think_mode=off` の成功条件、unsupported 時の failure、debug 項目を追記する必要がある。
- `tests/test_node_logic.py` と `tests/test_contract.py` の期待値説明を `off` 厳格化に合わせて更新する必要がある。

# 機能仕様: ComfyUI 長文回答 continuation

**Feature Branch**: `042-continue-long-answers`  
**Created**: 2026-04-05  
**Status**: Draft  
**Input**: ユーザー記述: "ComfyUI の local LLM custom node で、max_tokens や response_budget の上限に達して回答本文が途中で切れた場合、最終回答が完結するまで自動で続きを取得したい。まずは単純に回答だけが長いケースを対象にし、途中打ち切りで本文が終わっていない場合は継続生成して最後まで読み出せるようにする。"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 長い最終回答を最後まで受け取りたい (Priority: P1)

ComfyUI の利用者は、`PhotoPainter LLM Generate (Transformers)` で回答本文が長く、1 回目の生成が token 上限で途中終了した場合でも、自動で続きを取得して最終回答を完結させたい。これにより、回答の後半が欠けたまま workflow に流れる状態を避けたい。

**Why this priority**: まず解決したいのは、thinking ではなく最終回答本文が単純に長いケースであり、ここが欠けると workflow の利用価値が直接下がるため。

**Independent Test**: 模擬 backend が 1 回目で途中までの本文、2 回目で残り本文を返すケースを用意し、node が両者を連結して単一の完結した `output_text` を返すことを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が長文回答を要求している状態, **When** 1 回目の generation が token 上限到達で本文途中までしか返さない, **Then** node は continuation を行い、最終的に完結した本文を単一 `STRING` として返す。
2. **Given** 利用者が短い回答を要求している状態, **When** 1 回目の generation だけで本文が完結している, **Then** node は不要な continuation を追加せず、そのまま結果を返す。

---

### User Story 2 - continuation の発生有無を debug で確認したい (Priority: P2)

ComfyUI の利用者は、長文回答が複数回の generation で連結されたのか、1 回で終わったのかを debug 出力から判別したい。これにより、回答遅延や token 消費の原因を workflow 上で切り分けたい。

**Why this priority**: continuation が見えないと、遅くなった理由や予算不足の調整点が分からないため。

**Independent Test**: continuation が 0 回のケースと 1 回以上発生したケースを比較し、`debug_json` から continuation 回数と最終連結結果を判別できることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が continuation を伴って成功した実行結果を確認している, **When** `debug_json` を見る, **Then** continuation の回数と continuation に使った追加 generation があったことを判別できる。
2. **Given** 利用者が 1 回の generation だけで完結した実行結果を確認している, **When** `debug_json` を見る, **Then** continuation が不要だったことを判別できる。

---

### User Story 3 - continuation の上限や不適切なケースを制御したい (Priority: P3)

ComfyUI の利用者は、continuation が無限に続いたり、`think_mode=off` や JSON mode の契約を壊したりしないことを求める。これにより、長文回答救済のための追加 generation が別の failure を隠さないようにしたい。

**Why this priority**: continuation は便利だが、条件を誤ると token 浪費や契約破壊を起こすため。

**Independent Test**: continuation 上限到達、`think_mode=off`、`json_output=true` のケースを模擬し、許可される場合だけ continuation が走り、不適切なケースでは既存契約どおり failure または非 continuation 扱いになることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が `think_mode=off` または `json_output=true` を使っている状態, **When** 1 回目の generation が token 上限で途中終了する, **Then** node は continuation の可否を既存契約に従って判断し、`off` 契約や strict JSON 契約を壊さない。
2. **Given** 利用者が continuation 上限を超えるほど長い回答を要求している状態, **When** 上限回数まで continuation しても本文が完結しない, **Then** node は無限継続せず、上限到達を判別できる failure または明示状態で停止する。

### Edge Cases

- continuation は回答本文の単純な途中終了を主対象とし、`think_mode=off` の reasoning trace 救済には使わないこと。
- `json_output=true` では strict JSON / schema 契約を壊さないこと。必要なら JSON 専用 continuation 条件を別扱いにし、曖昧な自由文 rescue をしないこと。
- 1 回目の generation が空文字、同一断片の繰り返し、または進展のない continuation を返した場合は無限ループに入らないこと。
- continuation により `raw_text` と `output_text` が複数回 generation の連結結果になる場合、debug からその事実を判別できること。
- `llama-cpp` と `transformers` の両ノードで continuation を許可するかは、backend ごとの実装能力に応じて明示されること。未対応 backend を黙って fallback させないこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST local LLM node で回答本文が token 上限到達により途中終了したと判定できる場合、自動 continuation により残り本文の取得を試みられなければならない。
- **FR-002**: System MUST continuation 成功時、複数回の generation 断片を利用者が後続 workflow で扱える単一 `STRING` 出力へ連結しなければならない。
- **FR-003**: System MUST continuation が不要な完結回答に対して、追加 generation を実行してはならない。
- **FR-004**: System MUST continuation の回数または追加 token 予算に明示上限を持ち、進展のない継続や無限ループを防がなければならない。
- **FR-005**: System MUST `debug_json` に continuation の発生有無、continuation 回数、上限到達または停止理由を含めなければならない。
- **FR-006**: System MUST `think_mode=off` の厳格契約を壊してはならず、reasoning trace を continuation で救済成功に変えてはならない。
- **FR-007**: System MUST `json_output=true` の strict JSON / schema 契約を壊してはならず、自由文 continuation による曖昧な救済へ落としてはならない。
- **FR-008**: System MUST backend ごとの continuation 対応有無を明示し、未対応 backend では silent fallback ではなく明示 failure または非対応状態を返さなければならない。
- **FR-009**: System MUST 利用者向け文書に continuation の対象ケース、上限、debug の見方、`think_mode=off` / JSON mode との関係を説明しなければならない。

### Key Entities *(include if feature involves data)*

- **Continuation Plan**: continuation を許可するか、対象 backend、上限回数、停止条件、既存契約との整合を表す判定結果。
- **Continuation State**: 各 generation 断片、現在までの連結本文、continuation 回数、進展有無、停止理由を保持する実行状態。
- **Continuation Debug Contract**: continuation の有無、回数、停止理由、上限到達有無を利用者へ返す debug 情報。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `comfyui/custom_node/comfyui-photopainter-custom/__init__.py` の generation continuation ロジック更新
- `comfyui/custom_node/comfyui-photopainter-custom/tests/` の unit test / contract test 更新
- `comfyui/custom_node/comfyui-photopainter-custom/README.md` の continuation 説明更新

### Forbidden Scope

- server や firmware 側の仕様変更
- 外部ジョブキューや常駐サービスの導入
- `think_mode=off` 契約の緩和
- prompt planner 固有ロジックや画像生成ロジックの追加

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は長文回答が 1 回目で途中終了しても、node から完結した本文を受け取れる。
- **SC-002**: 利用者は `debug_json` から continuation が発生したか、何回追加 generation したかを判別できる。
- **SC-003**: continuation 実装は無限ループせず、上限到達や非対応条件を明示できる。
- **SC-004**: `think_mode=off` と JSON mode の既存厳格契約を壊さずに、長文回答 continuation を導入できる。

## Assumptions

- 主対象は「最終回答本文が長いだけ」のケースであり、reasoning block の continuation 救済は今回の対象外である。
- 既存 backend 実装から token-level finish reason を常に取得できるとは限らないため、途中終了判定は backend ごとに利用可能な信号へ依存する。
- まずは `transformers` backend を本命として continuation を検討し、`llama-cpp` は実装可能性に応じて同等対応または明示非対応とする。

## Documentation Impact

- `comfyui/custom_node/comfyui-photopainter-custom/README.md` に continuation の条件、上限、debug 項目、既知の非対象ケースを追記する必要がある。
- `tests/test_node_logic.py` と `tests/test_contract.py` の期待値説明を continuation 契約に合わせて更新する必要がある。

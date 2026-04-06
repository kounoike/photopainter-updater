# 機能仕様: RunPod Ollama sidecar

**Feature Branch**: `043-runpod-ollama-sidecar`  
**Created**: 2026-04-06  
**Status**: Draft  
**Input**: ユーザー記述: "RunPod serverless向けComfyUIイメージでコンテナ起動時にOllamaサーバを常駐起動し、永続領域からモデルを利用できるようにする。モデルpullを初期化手順に組み込み、どのモデルをpullするかを設定できるようにする。KEEP_ALIVEはノード側で0を指定する前提で、Dockerfile側では設定しない。"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Clarifications

### Session 2026-04-06

- Q: 起動時に指定モデルの pull が一部失敗した場合、コンテナ起動をどう扱うか → A: pull 失敗は警告だけ出して、起動はそのまま続ける
- Q: Ollama API の公開範囲をどうするか → A: Ollama API は同一コンテナ内 localhost のみで使える前提にする
- Q: pull 対象モデルの指定形式をどうするか → A: 1 つの設定値にカンマ区切りでモデル一覧を入れる
- Q: RunPod Network Volume が未接続のときの起動方針をどうするか → A: 永続領域がなくても一時領域で起動を続ける

## User Scenarios & Testing *(mandatory)*

### User Story 1 - ComfyUI コンテナ内で Ollama を常駐利用する (Priority: P1)

RunPod serverless で ComfyUI を使う運用者として、同じコンテナ起動の流れの中で Ollama サーバも自動起動し、ComfyUI ノードから追加の手作業なしで LLM 推論を呼び出したい。これにより、`transformers` 直呼びではなく Ollama を前提にした安定した推論基盤へ寄せられる。

**Why this priority**: Ollama をコンテナ起動時に利用可能にできなければ、この feature の主目的である RunPod serverless 上の一体運用が成立しないため。

**Independent Test**: RunPod 向け ComfyUI image を起動し、コンテナ内の Ollama API 疎通確認と ComfyUI ノードからの推論呼び出しの両方が追加の手動起動なしで成功すれば完了。

**Acceptance Scenarios**:

1. **Given** RunPod serverless 用にカスタマイズした ComfyUI image が起動している, **When** 利用者がコンテナ内の Ollama API 疎通を確認する, **Then** Ollama サーバは同じコンテナの起動フローの一部として利用可能になっている。
2. **Given** Ollama サーバが同居起動している, **When** ComfyUI の Ollama 利用ノードからモデル推論を実行する, **Then** 利用者は別コンテナや外部ホストを用意せずに推論結果を取得できる。
3. **Given** Ollama サーバが同居起動している, **When** API の接続先を確認する, **Then** 利用者は同一コンテナ内 localhost 前提で利用することを判断できる。

---

### User Story 2 - 永続領域のモデルを再利用する (Priority: P2)

RunPod serverless の運用者として、Ollama モデルを永続領域に保持し、コンテナ再作成後も毎回取り直さずに再利用したい。これにより、起動時間とネットワーク転送量を抑えながら継続運用できる。

**Why this priority**: serverless 環境で毎回モデルを再取得する運用は、初回応答遅延とネットワークコストの面で実用性を下げるため。

**Independent Test**: 永続領域にモデルを配置した状態でコンテナを再作成し、同じモデルが再 pull なしで利用可能であれば完了。

**Acceptance Scenarios**:

1. **Given** 利用者が指定した Ollama モデルを一度取得済みである, **When** RunPod serverless コンテナを再作成する, **Then** 取得済みモデルは永続領域から再利用される。
2. **Given** 永続領域に対象モデルがまだ存在しない, **When** 初期化手順が実行される, **Then** 必要なモデル取得が行われ、その後の推論に利用できる。
3. **Given** RunPod Network Volume が未接続である, **When** 初期化手順が実行される, **Then** 一時領域を使って起動を継続し、再利用不能であることを運用者が判断できる。

---

### User Story 3 - pull 対象モデルを運用設定で切り替える (Priority: P3)

RunPod serverless の運用者として、どの Ollama モデルを事前取得するかを設定で切り替えたい。これにより、用途に応じて軽量モデルと高性能モデルを同じカスタマイズ基盤で運用できる。

**Why this priority**: モデル名が固定だと image や起動導線を毎回作り直す必要があり、運用柔軟性が低くなるため。

**Independent Test**: モデル指定設定を変更して再起動し、指定したモデル群だけが取得対象として扱われることを確認できれば完了。

**Acceptance Scenarios**:

1. **Given** 運用者が事前取得したいモデル一覧を設定している, **When** 初期化手順が実行される, **Then** 指定したモデルだけが取得対象になる。
2. **Given** モデル一覧の一部は永続領域に既に存在する, **When** 初期化手順が実行される, **Then** 既存モデルは再利用され、不足分だけが取得対象になる。
3. **Given** 運用者が単一の設定値でモデル一覧を指定している, **When** 初期化手順がその設定を読む, **Then** カンマ区切り一覧として解釈される。

---

### Edge Cases

- Ollama サーバの起動が ComfyUI より遅い場合でも、最初の推論要求が即失敗扱いにならず、疎通確認の成否を判断できること。
- 永続領域が未マウントまたは書き込み不可の場合、モデル再利用不能であることを運用者が判別できること。
- pull 対象に存在しないモデル名や取得不能なモデル名が含まれる場合、失敗モデルを切り分けて判断できること。
- 一部モデルの pull に失敗しても、起動自体は継続し、失敗モデルだけを警告として判別できること。
- モデル一覧を空にした場合でも、Ollama サーバ自体の起動可否と事前取得を行わない状態を区別できること。
- モデル一覧設定の空白や区切りの揺れがあっても、取得対象解釈が曖昧にならないこと。
- RunPod Network Volume が未接続のとき、一時領域へ切り替わったことと、再作成後にモデルが残らないことを運用者が判別できること。
- KEEP_ALIVE はノード側で制御する前提を維持し、コンテナ起動設定がその方針と競合しないこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST RunPod serverless 向け ComfyUI コンテナの起動フロー中で Ollama サーバを自動起動し、ComfyUI ノードが同一コンテナ内の Ollama API を利用できるようにしなければならない。
- **FR-002**: System MUST Ollama サーバ起動後に API 疎通を確認し、利用可能状態になる前に推論クライアントを成功扱いにしてはならない。
- **FR-002a**: System MUST Ollama API を同一コンテナ内 localhost 利用前提で提供し、外部公開を前提にしてはならない。
- **FR-003**: System MUST Ollama のモデル保存先を RunPod の永続領域で運用できるようにし、コンテナ再作成後も取得済みモデルを再利用できなければならない。
- **FR-003a**: System MUST RunPod Network Volume が未接続でも一時領域を使って起動を継続できなければならない。ただし一時領域利用中であることと再利用不能であることを運用者へ判別可能にしなければならない。
- **FR-004**: System MUST 初期化手順の中で、運用設定から指定された 0 個以上の Ollama モデルを取得対象として扱えなければならない。
- **FR-005**: System MUST 既に永続領域へ存在するモデルを再利用し、不足しているモデルのみを取得対象として扱わなければならない。
- **FR-006**: System MUST どのモデルを取得対象にするかを、image 再作成なしに運用設定から変更できなければならない。
- **FR-006a**: System MUST モデル取得設定を単一の設定値で受け取り、カンマ区切りの 0 個以上のモデル名一覧として解釈しなければならない。
- **FR-007**: System MUST モデル取得失敗、永続領域未接続、Ollama 起動失敗、API 未到達を運用者が区別できるようにしなければならない。
- **FR-007a**: System MUST 一部モデルの取得に失敗しても、Ollama サーバ起動と ComfyUI 起動を継続し、失敗モデルを警告として記録しなければならない。
- **FR-008**: System MUST KEEP_ALIVE の制御をコンテナ起動設定へ固定埋め込みせず、ノード側で `0` を指定する現行運用と競合しないようにしなければならない。
- **FR-009**: System MUST RunPod serverless 向けのカスタマイズ手順書に、永続領域前提、モデル取得設定、起動時の確認方法を記載しなければならない。
- **FR-010**: System MUST 既存の ComfyUI serverless 起動導線を壊さず、Ollama 同居に伴う追加前提だけを利用者が判断できるようにしなければならない。

### Key Entities *(include if feature involves data)*

- **Ollama 同居ランタイム**: RunPod serverless 用 ComfyUI コンテナ内で、ComfyUI と同時に利用可能になる LLM 推論サービス。
- **モデル取得設定**: 起動時に事前取得すべき Ollama モデル名の一覧を表す単一設定値。カンマ区切りで 0 件以上を指定できる。
- **永続モデル領域**: RunPod の永続領域上にある、Ollama モデルと関連データを再利用するための保存先。
- **一時モデル領域**: RunPod Network Volume が未接続のときに使うコンテナ内保存先。コンテナ再作成後の再利用は保証しない。
- **起動初期化手順**: コンテナ起動時に Ollama サーバ起動、疎通確認、必要モデル取得を順に処理する運用導線。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- RunPod serverless 向け ComfyUI image のカスタマイズ定義更新
- コンテナ起動初期化手順の見直し
- Ollama モデル保存先とモデル取得設定の導線追加
- RunPod 向け運用文書、quickstart、README 相当の手順更新
- ComfyUI から同居 Ollama を利用する前提の疎通確認手順追加
- localhost 限定の Ollama API 利用前提を反映した起動設定更新

### Forbidden Scope

- Ollama モデル自体の作成、量子化、チューニング
- ComfyUI-ollama ノードの機能変更や KEEP_ALIVE 仕様変更
- RunPod 以外のホスティング基盤向けのデプロイフロー変更
- 既存 custom node の推論ロジックを `transformers` から Ollama へ全面移植すること
- firmware、`server/`、`xiaozhi-esp32/` 配下への変更
- Ollama API を外部公開する前提の設計変更

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は RunPod serverless 用 ComfyUI コンテナを 1 回起動するだけで、追加の手動プロセス起動なしに Ollama API 疎通確認まで完了できる。
- **SC-002**: 利用者は一度取得したモデルを永続領域から再利用し、コンテナ再作成後も同じモデルで推論を再開できる。
- **SC-003**: 利用者は運用設定を変更するだけで、事前取得するモデル一覧を切り替えられる。
- **SC-004**: 運用者は起動ログまたは手順書から、Ollama 起動失敗、API 未到達、モデル取得失敗、永続領域未接続を切り分けて判断できる。

## Assumptions

- RunPod serverless 環境では ComfyUI image のカスタマイズと永続領域利用が可能である。
- Ollama モデルは image に焼き込まず、RunPod Network Volume がある場合は永続領域へ配置して再利用する。
- RunPod Network Volume がない環境では、一時領域での継続起動を許容する。
- KEEP_ALIVE は ComfyUI 側ノード入力で `0` を指定する運用を継続し、コンテナ起動設定では固定しない。
- ComfyUI と Ollama は同一コンテナ内で共存し、追加の別サービス分離は今回の前提に含めない。
- 取得対象モデルは運用設定で与えられ、空一覧も有効な設定として扱う。
- Ollama API は同一コンテナ内 localhost 接続のみを前提とし、外部クライアント接続は対象外とする。

## Documentation Impact

- RunPod serverless 向け ComfyUI カスタマイズ手順に、Ollama 同居起動と永続領域前提を追加する必要がある。
- モデル取得設定の定義、設定例、永続領域上の保存方針を quickstart または README に追記する必要がある。
- モデル取得設定が単一設定値のカンマ区切り一覧であることを設定例付きで文書化する必要がある。
- 起動確認手順に Ollama API 疎通確認とモデル取得状態の確認方法を追加する必要がある。
- 起動時に一部モデル pull が失敗しても継続起動することと、その警告確認方法を手順へ追加する必要がある。
- Ollama API が localhost 限定であることと、ComfyUI からの接続先前提を文書へ追加する必要がある。
- RunPod Network Volume 未接続時は一時領域へ切り替わることと、再利用されないことを手順へ追加する必要がある。

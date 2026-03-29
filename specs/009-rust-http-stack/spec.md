# 機能仕様: Rust HTTPスタック再評価

**Feature Branch**: `009-rust-http-stack`  
**Created**: 2026-03-29  
**Status**: Draft  
**Input**: ユーザー記述: "rustでhttpサーバを実装するにあたって、008-http-server-stackではRust+axumと決め打ちしたが、Rustの中でもaxum以外の選択肢を考える必要はないのか"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Rust 内の候補比較をやり直したい (Priority: P1)

開発者は Rust ベースで HTTP サーバを実装する前に、`axum` をそのまま採用してよいかを確認し、必要なら他の Rust 候補も比較したい。

**Why this priority**: 008 では Rust を第一候補にしたが、Rust 内の framework 比較を省略したまま実装へ進むと、早い段階で不適切な選択を固定する可能性があるため。

**Independent Test**: `axum` と他の Rust HTTP 候補 1 つ以上について、同一観点で比較され、`axum` 維持または別候補採用の判断が文書化されていることを確認する。

**Acceptance Scenarios**:

1. **Given** Rust ベースでサーバ実装を進める前提がある状態, **When** Rust 内の候補比較を行う, **Then** 開発者は `axum` を維持すべきか、別候補を採るべきかを説明できる。
2. **Given** 比較結果がまとまった状態, **When** 後続の Rust サーバ実装 feature に進む, **Then** 開発者は Rust HTTP スタックの前提を再比較なしで参照できる。

---

### User Story 2 - 画像前処理と telemetry に対する適合性を見たい (Priority: P2)

開発者は Rust の HTTP 候補が、画像前処理パイプラインとデバイス telemetry API の両方にどれだけ適するかを把握したい。

**Why this priority**: このサーバは単なる API ではなく、画像変換と状態収集を含むため、HTTP framework の選択もその責務に照らして見る必要があるため。

**Independent Test**: 各 Rust 候補について、画像前処理、POST 受信、監視連携、保守性の観点が比較結果に含まれていることを確認する。

**Acceptance Scenarios**:

1. **Given** 画像前処理と telemetry 収集を前提に比較する状態, **When** Rust 候補を評価する, **Then** 各候補の向き不向きが機能観点に即して説明される。

---

### User Story 3 - `axum` を維持する場合も理由を残したい (Priority: P3)

開発者は、最終的に `axum` を維持する場合でも、その理由と他候補を見送った理由を明確に残したい。

**Why this priority**: 再評価した結果が「やはり `axum`」であっても、比較過程が残っていないと将来また同じ問いを繰り返すため。

**Independent Test**: 調査結果に、`axum` 維持または別候補採用の理由と、見送った Rust 候補の理由が残っていることを確認する。

**Acceptance Scenarios**:

1. **Given** Rust 候補比較が完了した状態, **When** 最終判断を記録する, **Then** `axum` 維持または別候補採用の理由と見送り理由が文書化される。

### Edge Cases

- `axum` と他候補の差が決定的でない場合でも、後続実装に進める暫定結論または再評価条件を残すこと。
- 他候補を追加しても、008 で整理した画像前処理と telemetry の比較軸を失わないこと。
- Rust 内候補を比較した結果、HTTP framework よりも先に共通ライブラリ選定や非同期 runtime 方針を決める必要があると分かった場合、その理由を残すこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST `axum` を比較対象として含む Rust HTTP スタック調査結果を残さなければならない。
- **FR-002**: System MUST `axum` 以外に、少なくとも 1 つ以上の Rust HTTP 候補を比較対象として記録しなければならない。
- **FR-003**: System MUST 比較結果に、画像前処理適合性、telemetry API 適合性、保守性、依存の重さ、配布容易性、開発体験を含めなければならない。
- **FR-004**: System MUST 後続の Rust サーバ実装 feature が参照できる最終候補または暫定候補を明示しなければならない。
- **FR-005**: System MUST `axum` を維持する場合でも、その理由と他候補を見送る理由を記録しなければならない。
- **FR-006**: System MUST 008 の結論と矛盾する場合、その差分理由を開発文書に残さなければならない。

### Key Entities *(include if feature involves data)*

- **Rust HTTP 候補**: `axum` を含む Rust の HTTP framework または関連スタック候補。
- **評価観点**: 画像前処理、telemetry API、保守性、依存、配布容易性、開発体験などの比較軸。
- **Rust HTTP 選定結果**: 維持候補、対抗候補、見送り理由、再評価条件をまとめた記録。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- Rust HTTP framework 候補の比較調査
- 008 の結論との整合確認
- 比較結果の設計文書・調査文書への反映

### Forbidden Scope

- `server/` や新規サーバ実装コードの追加
- Rust サーバの PoC 実装
- `firmware/` 側の仕様変更
- `xiaozhi-esp32/` 配下の直接変更

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `axum` と他の Rust 候補 1 つ以上について、同一文書内で比較と採否が確認できる。
- **SC-002**: 後続の Rust サーバ実装 feature が、Rust HTTP スタックの再比較なしで候補を参照できる。
- **SC-003**: `axum` 維持または変更の理由が文書化され、読み手が再評価の必要性を即判断できる。
- **SC-004**: 008 の画像前処理・telemetry 前提と矛盾しない比較結果になっている。

## Assumptions

- 008 での「Rust を第一候補にする」判断自体は維持しつつ、その中身だけ再評価する。
- 比較対象は Rust の HTTP framework / stack に限定する。
- 画像前処理と telemetry 収集は引き続き主要責務として扱う。

## Documentation Impact

- `specs/009-rust-http-stack/` 配下の設計成果物に Rust 内候補比較の結果を残す必要がある。
- 必要なら `specs/008-http-server-stack/` から参照できる差分理由を残す必要がある。

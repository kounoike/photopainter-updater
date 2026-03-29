# 機能仕様: ACT LED アクティビティ表示

**Feature Branch**: `006-activity-led-indicator`  
**Created**: 2026-03-29  
**Status**: Draft  
**Input**: ユーザー記述: "SDカード・HTTPアクセス・画面更新などをしている間、ACT LEDを点滅させる"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 更新中を見分けたい (Priority: P1)

利用者は起動時または BOOT ボタン更新時に、更新処理が進行中かどうかを実機で利用可能な活動表示 LED の点滅で即座に判別できる。

**Why this priority**: いま処理待ちなのか停止したのか判別しづらく、更新体験とトラブル切り分けの両方に直接影響するため。

**Independent Test**: 正常な `config.txt` を置いた状態で起動し、SD 読込から表示更新完了まで ACT LED が点滅し、完了後に点滅が止まることを確認する。

**Acceptance Scenarios**:

1. **Given** 正常な設定と取得可能な画像がある状態, **When** デバイスが起動して更新処理を開始する, **Then** 更新中は ACT LED が点滅し、更新完了後は点滅が止まる。
2. **Given** 起動後に待機している状態, **When** 利用者が BOOT ボタンで再更新を開始する, **Then** 再更新中も ACT LED が点滅し、更新完了後は点滅が止まる。

---

### User Story 2 - 長い待ち時間を誤解したくない (Priority: P2)

利用者は HTTP 取得や e-paper 描画に時間がかかっても、活動表示 LED が動作していることでフリーズではないと判断できる。

**Why this priority**: 通信や表示に数秒から十数秒かかる場面で、無反応に見える時間を減らす価値が高いため。

**Independent Test**: 意図的に応答や描画完了まで時間がかかる更新条件で起動し、待機中も ACT LED の点滅が継続することを確認する。

**Acceptance Scenarios**:

1. **Given** 画像取得や表示完了まで時間を要する状態, **When** 更新ジョブが進行中である, **Then** 利用者は ACT LED の点滅を継続して観測できる。

---

### User Story 3 - 失敗時も進行終了を見分けたい (Priority: P3)

利用者は設定不備や通信失敗で更新が途中終了した場合でも、活動表示 LED の点滅停止によって進行中処理が終わったことを把握できる。

**Why this priority**: 正常完了だけでなく失敗終了時にも点滅が続きっぱなしだと、状態表示として信用できなくなるため。

**Independent Test**: `config.txt` 欠落や Wi-Fi 接続失敗を発生させ、失敗が確定した時点で ACT LED の点滅が止まり、その後の終了動作に移ることを確認する。

**Acceptance Scenarios**:

1. **Given** 更新開始後に設定不備または通信失敗が発生する状態, **When** 更新処理が継続不能と判断される, **Then** ACT LED の点滅は停止し、利用者は進行中処理が終わったと判断できる。

### Edge Cases

- 更新対象がすぐに失敗する場合でも、ACT LED は進行開始後に点滅へ入り、失敗確定後には停止する。
- 起動時更新と BOOT ボタン更新が重複要求された場合でも、ACT LED 表示は 1 つの進行中処理に対して一貫した点滅状態を保つ。
- 待機状態では ACT LED が点灯しっぱなしや点滅しっぱなしにならない。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST 更新ジョブの進行開始時に ACT LED の点滅を開始しなければならない。
- **FR-002**: System MUST 活動表示 LED の点滅中に、既定の単一パターンを維持し、利用者が目視で活動中と判断できる周期で点滅させなければならない。
- **FR-003**: System MUST SD カード読込、ネットワーク接続、画像取得、表示更新など、更新ジョブに含まれる待機時間を伴う処理中も ACT LED の点滅を継続しなければならない。
- **FR-004**: System MUST 更新ジョブが正常完了した時点で ACT LED の点滅を停止しなければならない。
- **FR-005**: System MUST 更新ジョブが失敗終了した時点でも ACT LED の点滅を停止しなければならない。
- **FR-006**: System MUST 起動時更新と BOOT ボタン更新のどちらでも、同じ ACT LED 活動表示ルールを適用しなければならない。
- **FR-007**: System MUST 待機状態に入った後、ACT LED が進行中を誤認させる点滅状態を残してはならない。
- **FR-008**: System MUST 更新ジョブの直列実行制御と矛盾しないよう、同時に複数の点滅状態を発生させてはならない。

### Key Entities *(include if feature involves data)*

- **更新ジョブ活動状態**: 起動時更新または BOOT ボタン更新が現在進行中かどうかを表す状態。
- **ACT LED 表示状態**: 消灯、点滅中など、利用者へ活動状況を伝えるための LED 状態。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `firmware/` 配下での更新ジョブ進行状態と ACT LED 表示制御の追加・調整
- 既存の起動時更新および BOOT ボタン更新フローに対する活動表示の統合
- 必要最小限の利用者向け文書更新

### Forbidden Scope

- `xiaozhi-esp32/` 配下の直接変更
- ACT LED 以外の新規 UI 表示や追加インジケータの導入
- 更新対象、設定ファイル形式、HTTP 取得仕様、e-paper 更新仕様の変更
- 定期点滅通知や常時状態監視など、更新ジョブ外の LED 用途追加

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 起動時更新の手動確認で、進行開始から進行終了まで利用者が ACT LED の点滅を連続して観測できる。
- **SC-002**: BOOT ボタン再更新の手動確認で、進行開始から進行終了まで利用者が ACT LED の点滅を連続して観測できる。
- **SC-003**: 設定不備、Wi-Fi 失敗、HTTP 失敗、画像不正の各失敗確認で、失敗確定後に ACT LED の点滅が停止する。
- **SC-004**: 更新待機中の観察で、利用者が ACT LED を見て誤って「まだ更新中」と判断しない状態を維持できる。

## Assumptions

- TODO: 対象ハードウェアで利用する活動表示 LED と GPIO 割り当てを実機確認し、実装前に確定する。
- ACT LED の既存用途がある場合でも、更新ジョブ進行中表示を優先してよい。
- 利用者は主に目視で進行中か停止済みかを判定し、厳密な点滅周期の数値計測までは求めない。

## Documentation Impact

- `specs/006-activity-led-indicator/` 配下の設計成果物をこの仕様に沿って作成・更新する必要がある。
- `docs/firmware-http-epaper.md` に、更新中は ACT LED が点滅する運用説明を追加する必要がある。

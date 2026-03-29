# 機能仕様: SDカード設定 HTTP e-paper 更新ファーム

**Feature Branch**: `005-sdcard-http-epaper`  
**Created**: 2026-03-29  
**Status**: Draft  
**Input**: ユーザー記述: "xiaozhi-esp32を元にして、本命のファームウエアを作りたい。SDカードのルートにWiFiのSSID/パスワード、画像を取得するURLを記載したJSONファイルがあって、起動時とBOOTボタンを押したときに画像をHTTPで取りに行ってe-paperの表示内容を更新する"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Clarifications

### Session 2026-03-29

- Q: 設定 JSON の画像取得先は単一 URL と複数候補のどちらにするか → A: JSON は単一の画像 URL だけを持ち、毎回その 1 枚を取得する
- Q: 更新失敗時のデバイス挙動はどうするか → A: 更新失敗時はそのまま終了してシャットダウンする
- Q: SDカード上の設定ファイル名は何に固定するか → A: SDカードルートの `config.txt` に固定する
- Q: 実装コードはどこに置き、`xiaozhi-esp32/` をどう扱うか → A: 実装コードは `firmware/` 配下に置き、`xiaozhi-esp32/` は参照専用で書き換えない

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 起動時に最新画像を表示できる (Priority: P1)

利用者は電源投入後、SDカードルートの `config.txt` に置かれた設定情報をもとにデバイスが WiFi に接続し、設定 JSON に記載された単一の画像 URL から画像を取得して e-paper 表示を更新できる。

**Why this priority**: この機能が成立しなければ、本命ファームウェアとしての基本価値である「設定済み画像の自動取得と表示」が実現できないため。

**Independent Test**: SDカードルートに正しい `config.txt` を置いた状態で起動し、追加操作なしで画像取得と e-paper 更新が完了することを確認する。

**Acceptance Scenarios**:

1. **Given** SDカードのルートに有効な `config.txt` があり、WiFi と画像 URL が利用可能な状態で、 **When** デバイスが起動する, **Then** デバイスは設定を読み込み、WiFi 接続後に画像を取得して e-paper 表示を更新する。
2. **Given** 起動時の画像取得が成功した状態で, **When** 表示更新が完了する, **Then** 利用者は取得した画像が e-paper に反映されたことを確認できる。

---

### User Story 2 - BOOTボタンで手動更新できる (Priority: P2)

利用者は起動後に BOOT ボタンを押すことで、その時点で再度画像取得を実行し、e-paper 表示を更新できる。

**Why this priority**: 起動時だけでなく任意タイミングで更新できることにより、運用時の再起動依存を避けられるため。

**Independent Test**: 起動後に BOOT ボタンを押し、再度 HTTP 取得が実行されて表示内容が更新されることを確認する。

**Acceptance Scenarios**:

1. **Given** デバイスが起動済みで設定読込済みの状態で, **When** 利用者が BOOT ボタンを押す, **Then** デバイスは設定済み URL から画像を再取得し、e-paper 表示を更新する。
2. **Given** BOOT ボタンによる更新要求が実行中の状態で, **When** 更新が完了する, **Then** デバイスは表示更新結果を安定して維持する。

---

### User Story 3 - 設定不備や取得失敗時でも原因を切り分けられる (Priority: P3)

利用者または開発者は、SDカード設定不備や HTTP 取得失敗があっても、デバイスが失敗理由を判断できる形で更新処理を終了し、シャットダウンする挙動を把握できる。

**Why this priority**: 現場運用では設定ミスやネットワーク不調が起こり得るため、失敗時にどのように停止するかが明確でないと復旧や保守が難しくなるため。

**Independent Test**: 設定ファイル欠落、WiFi 接続失敗、HTTP 応答異常などを個別に発生させ、デバイスが失敗理由を判断できる形で更新処理を終了し、シャットダウンすることを確認する。

**Acceptance Scenarios**:

1. **Given** SDカードに設定 JSON が存在しない、または必要項目が不足している状態で, **When** デバイスが起動する, **Then** デバイスは更新処理を続行せず、失敗理由を判断できる形で終了し、シャットダウンする。
2. **Given** WiFi または HTTP 取得が失敗する状態で, **When** 起動時または BOOT ボタン押下時に更新を試みる, **Then** デバイスは失敗理由を判断できる形で更新処理を終了し、シャットダウンする。

### Edge Cases

- SDカードは読めるが JSON の形式が壊れている場合、設定読込失敗として扱われること。
- JSON に WiFi 情報はあるが画像 URL が空または不正な場合、HTTP 取得を開始しないこと。
- BOOT ボタンが連続で押された場合でも、同時に複数の更新処理が走らず表示状態が破綻しないこと。
- HTTP 取得した画像が e-paper で扱えない形式やサイズだった場合、表示破損ではなく失敗として扱われること。
- 起動時または BOOT ボタン更新に失敗した場合、更新処理は終了し、そのままシャットダウンすること。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST `xiaozhi-esp32` を参照元として活用しつつ、画像取得と e-paper 更新を主目的とした専用ファームウェアを `firmware/` 配下に提供する。
- **FR-002**: System MUST 起動時に SDカードのルートにある `config.txt` を読み込み、WiFi 接続情報と単一の画像取得先 URL を取得する。
- **FR-003**: System MUST 設定 JSON に基づいて WiFi へ接続し、画像取得に必要な通信準備を完了しなければならない。
- **FR-004**: System MUST 起動時に設定済み URL へ HTTP でアクセスし、取得した画像で e-paper 表示を更新しなければならない。
- **FR-005**: Users MUST be able to BOOT ボタンを押すことで、設定済み URL からの画像再取得と e-paper 再更新を手動で実行できる。
- **FR-006**: System MUST 同時多発的な更新要求を避け、起動時更新または BOOT ボタン更新のいずれか 1 件ずつを直列に処理しなければならない。
- **FR-007**: System MUST 設定 JSON の欠落、不正形式、必須項目不足を検出し、更新失敗として扱わなければならない。
- **FR-008**: System MUST WiFi 接続失敗、HTTP 応答失敗、画像不正時に、更新処理を終了してシャットダウンしなければならない。
- **FR-009**: System MUST 失敗時に、利用者または開発者が設定不備・接続不良・取得失敗を切り分けられる状態を残した上で、更新処理を終了しシャットダウンしなければならない。
- **FR-010**: System MUST 利用者が SDカード上の `config.txt` を差し替えるだけで、WiFi 接続先や単一の画像取得先を変更できるようにしなければならない。

### Key Entities *(include if feature involves data)*

- **設定 JSON (`config.txt`)**: SDカードのルートに置かれる設定ファイル。中身は JSON として解釈し、WiFi の SSID、パスワード、単一の画像取得 URL を保持する。
- **更新ジョブ**: 起動時または BOOT ボタン押下時に発生する、設定読込、WiFi 接続、HTTP 取得、e-paper 更新までの一連処理。
- **表示画像**: HTTP で取得され、e-paper に反映される対象画像。取得成功時のみ表示更新に使われる。
- **失敗状態**: 設定不備、WiFi 接続失敗、HTTP 失敗、画像不正などを区別し、シャットダウン前に判断可能にするための状態情報。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `firmware/` 配下への専用ファームウェア実装。
- `xiaozhi-esp32/` を参照元として調査し、その内容を設計へ反映すること。
- SDカード上の `config.txt` 読込、WiFi 接続、HTTP 画像取得、e-paper 更新、BOOT ボタン再更新の実装。
- 失敗時の安全な振る舞いと切り分けしやすい失敗状態の整備。
- 必要な設定例や利用手順などの関連文書更新。

### Forbidden Scope

- 音声対話、MQTT 会話、クラウド会話機能の拡張。
- SDカード上での複数画像管理やスライドショー機能。
- 設定 JSON に複数画像 URL や画像選択ロジックを持たせること。
- 画像取得を HTTP 以外の方式に広げること。
- OTA や遠隔管理基盤の全面刷新。
- `xiaozhi-esp32/` 配下の既存ソースを直接書き換えること。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 正しい `config.txt` と利用可能なネットワーク環境がある場合、起動後 60 秒以内に画像取得と e-paper 表示更新が完了する。
- **SC-002**: 利用者が BOOT ボタンを押した場合、90% 以上の試行で 60 秒以内に表示更新結果を確認できる。
- **SC-003**: 設定不備、WiFi 接続失敗、HTTP 取得失敗の各ケースで、デバイスが失敗理由を判断できる形で更新処理を終了し、シャットダウンする。
- **SC-004**: 利用者は SDカード上の `config.txt` を差し替えるだけで、再書き込みなしに WiFi 接続先と単一の画像取得先を変更できる。

## Assumptions

- 対象デバイスは SDカード、WiFi、e-paper、BOOT ボタンを利用できるハードウェア構成を備えている。
- `xiaozhi-esp32/` は実装参考用の同梱コードとして扱い、派生実装は `firmware/` 配下に新設する前提とする。
- `config.txt` はデバイス起動前に利用者が SDカードのルートへ配置する運用とする。
- 画像取得先 URL は LAN 内または通常の HTTP 到達範囲にあり、認証なしで取得できる前提とする。
- 起動時と BOOT ボタン押下時の都度更新を初期スコープとし、定期更新は対象外とする。
- e-paper に適した画像形式への前処理は取得先またはファームウェア側で解決可能な前提だが、仕様上は「表示可能な画像を扱うこと」を要求する。
- 更新失敗時はリトライ継続やエラー画面表示ではなく、更新処理を終了してシャットダウンする運用とする。

## Documentation Impact

- `specs/005-sdcard-http-epaper/contracts/config-and-update-contract.md` に、専用ファームウェア用の `config.txt` 仕様を維持更新する必要がある。
- `specs/005-sdcard-http-epaper/quickstart.md` に、SDカードへの設定配置方法、起動時更新、BOOT ボタン更新、失敗時確認の手順を維持更新する必要がある。
- 利用者向け文書として `docs/firmware-http-epaper.md` を作成または更新し、`config.txt` の配置方法と運用手順を説明できる状態にする必要がある。

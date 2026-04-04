# 機能仕様: config insecure HTTPS

**Feature Branch**: `037-https-insecure-flag`  
**Created**: 2026-04-04  
**Status**: Draft  
**Input**: ユーザー記述: "config.txtにboolフラグのinsecure設定を追加し、insecure: trueならばHTTPS証明書未検証でも通信出来るようにする"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - HTTPS 検証省略で更新する (Priority: P1)

自己署名証明書や検証未整備の HTTPS サーバーを使う利用者として、`config.txt` に `insecure: true` を設定したときだけ、証明書未検証でも画像取得を実行したい。これにより、閉域網や検証用サーバーでも既存の更新フローを止めずに e-paper 更新を実行できる。

**Why this priority**: 利用者の主要求そのものであり、HTTPS サーバーへ切り替えても運用継続できることが今回の中核価値だから。

**Independent Test**: `image_url` に `https://` を設定し、証明書検証を通常では通過できないサーバーを用意する。`insecure: true` を設定して起動または BOOT ボタン更新を行い、画像取得から表示更新まで成功することを確認する。

**Acceptance Scenarios**:

1. **Given** `config.txt` に `https://` の `image_url` と `insecure: true` が設定されている, **When** 更新処理を開始する, **Then** 利用者は証明書未検証の HTTPS サーバーから画像取得を継続できる。
2. **Given** `config.txt` に `https://` の `image_url` と `insecure: true` が設定されている, **When** 取得した画像が表示条件を満たす, **Then** 利用者は既存の成功時と同様に e-paper 更新完了を確認できる。

---

### User Story 2 - 既定では安全側を維持する (Priority: P2)

通常運用の利用者として、`insecure` を明示的に有効化しない限り、既存の安全側の扱いを維持したい。これにより、誤設定で証明書未検証通信へ落ちることを避けられる。

**Why this priority**: 例外的な緩和設定を追加しても、既定挙動を不用意に弱めないことが運用品質上重要だから。

**Independent Test**: `https://` の `image_url` に対して `insecure` を省略または `false` にした `config.txt` を用意する。更新処理を実行し、証明書検証を通過できない HTTPS サーバーへの接続は失敗として扱われることを確認する。

**Acceptance Scenarios**:

1. **Given** `config.txt` に `https://` の `image_url` があり `insecure` が未設定または `false` である, **When** 証明書検証を通過できない HTTPS サーバーへ接続する, **Then** 更新処理は失敗として終了し利用者は失敗扱いを確認できる。
2. **Given** `config.txt` に `http://` の `image_url` がある, **When** `insecure` が未設定または `false` で更新する, **Then** 利用者は既存の HTTP 更新フローを継続利用できる。

---

### User Story 3 - 設定ミスを判別する (Priority: P3)

運用者として、`config.txt` の `insecure` 値が不正な場合や意図しない組み合わせの場合に、設定不備か通信失敗かを区別したい。これにより、現地での切り分けを早められる。

**Why this priority**: 新しい設定項目を追加すると設定不備の発生可能性が上がるため、既存の障害判別性を維持する必要がある。

**Independent Test**: `config.txt` の `insecure` に boolean 以外の値を設定して更新処理を実行し、通信開始前に設定不備として扱われることを確認する。

**Acceptance Scenarios**:

1. **Given** `config.txt` の `insecure` が boolean ではない, **When** 設定読込を行う, **Then** 更新処理は通信開始前に設定不備として失敗する。
2. **Given** `config.txt` の他の必須項目が正しい, **When** `insecure` だけが不正である, **Then** 利用者は画像取得失敗ではなく設定不備として扱われる。

### Edge Cases

- `image_url` が `https://` でも、サーバー証明書が正しく検証できる場合は `insecure: false` または未設定でも成功できること。
- `insecure: true` でも、URL が不正、サーバーへ到達不能、HTTP ステータス異常、画像形式不正など証明書検証以外の失敗は従来通り失敗扱いにすること。
- `insecure` が文字列 `"true"` や数値 `1` など boolean 以外で与えられた場合は、暗黙変換せず設定不備として扱うこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST allow `config.txt` to include an optional boolean field named `insecure`.
- **FR-002**: System MUST treat omitted `insecure` as `false`.
- **FR-003**: System MUST accept both `http://` and `https://` values for `image_url`.
- **FR-004**: System MUST permit HTTPS image retrieval without server certificate verification only when `image_url` uses `https://` and `insecure` is `true`.
- **FR-005**: System MUST continue to require normal certificate verification for HTTPS retrieval when `insecure` is omitted or `false`.
- **FR-006**: System MUST reject `config.txt` as invalid before starting network access when `insecure` is present but is not a boolean.
- **FR-007**: System MUST preserve the existing update flow for successful HTTP retrieval, successful verified HTTPS retrieval, BMP rendering, binary frame rendering, and failure categorization outside the certificate-verification exception.
- **FR-008**: System MUST leave `http://` retrieval behavior unchanged regardless of the `insecure` value.
- **FR-009**: System MUST make the insecure setting understandable in the user-facing documentation for `config.txt`, including that it disables HTTPS certificate verification only when explicitly enabled.

### Key Entities *(include if feature involves data)*

- **設定 JSON (`config.txt`)**: SDカードルートに置かれる更新設定。`wifi_ssid`、`wifi_password`、`image_url`、任意の `insecure` を持つ。
- **`insecure` フラグ**: HTTPS 接続時だけ意味を持つ boolean 設定。`true` のときだけ証明書未検証通信を許可し、未設定または `false` では安全側の既定挙動を維持する。
- **画像取得設定**: `image_url` と `insecure` の組み合わせで通信方式と検証方針が決まる取得条件。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `config.txt` の仕様に任意 boolean `insecure` を追加すること。
- `https://` の `image_url` を設定可能にすること。
- `insecure: true` の場合だけ HTTPS 証明書未検証通信を許可すること。
- 上記仕様に合わせて関連文書と検証手順を更新すること。

### Forbidden Scope

- `xiaozhi-esp32/` 配下を直接変更すること。
- `config.txt` の既存必須項目やファイル配置を変更すること。
- HTTPS 以外の認証方式、クライアント証明書、証明書ピンニング、証明書保存方式など別機能を同時追加すること。
- `insecure: true` を既定値にすること、または利用者の明示設定なしに証明書未検証通信へ自動移行すること。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は `config.txt` に `https://` の更新先を設定でき、`insecure: true` を指定した場合は証明書未検証の HTTPS サーバーに対しても 1 回の更新操作で画像取得から表示更新完了まで到達できる。
- **SC-002**: 利用者は `insecure` を省略または `false` にしたまま、証明書未検証の HTTPS サーバーへ接続した場合に、成功扱いにならず失敗として判別できる。
- **SC-003**: 運用者は `insecure` の値が不正な `config.txt` を使った場合、通信失敗ではなく設定不備として切り分けられる。
- **SC-004**: 利用者は更新後の文書だけで、`insecure` の既定値、効果範囲、利用時の注意点を理解できる。

## Assumptions

- 利用者は閉域網や検証環境など、自己署名証明書または未整備の証明書を使う HTTPS サーバーを運用する可能性がある。
- 既存の HTTP 更新、画像形式判定、失敗状態保存、起動時更新と BOOT ボタン更新の流れは本機能でも維持する。
- `insecure` は設定の明示性を優先し、未設定時は安全側の `false` とみなす。

## Documentation Impact

- `config.txt` の契約文書に `insecure` の定義、既定値、`https://` 許可条件を追加する。
- quickstart と運用文書に、証明書未検証 HTTPS の利用手順と注意点を追加する。
- 必要に応じて失敗系テスト手順へ `insecure` 未設定時の HTTPS 失敗と `insecure: true` 時の HTTPS 成功を反映する。

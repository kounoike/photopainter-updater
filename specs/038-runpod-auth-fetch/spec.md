# 機能仕様: RunPod authenticated fetch

**Feature Branch**: `038-runpod-auth-fetch`  
**Created**: 2026-04-04  
**Status**: Draft  
**Input**: ユーザー記述: "RunPodのような外部HTTPSサーバから画像を取得できるようにし、config.txtにinsecure設定とBearerトークン設定を追加する"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Bearer 認証付き HTTPS で更新する (Priority: P1)

外部 HTTPS サーバーを使う利用者として、`config.txt` に Bearer トークンを設定し、認証付きの画像取得で e-paper 更新を成功させたい。これにより、RunPod のような外部公開サーバーでも認証付きで安全に更新元を運用できる。

**Why this priority**: 外部 HTTPS 更新の実用価値は認証付き取得が成立して初めて出るため、この story が今回の中心価値になる。

**Independent Test**: `https://` の `image_url` と有効な `bearer_token` を設定した `config.txt` を用意し、認証付き更新元へ起動時または BOOT ボタン更新でアクセスして、画像取得から表示更新まで成功することを確認する。

**Acceptance Scenarios**:

1. **Given** `config.txt` に `https://` の `image_url` と有効な `bearer_token` が設定されている, **When** 更新処理を開始する, **Then** 利用者は Bearer 認証付き HTTPS サーバーから画像取得して表示更新できる。
2. **Given** Bearer 認証付き HTTPS サーバーが BMP または binary 形式を返す, **When** 利用者が更新を実行する, **Then** 利用者は既存の route 選択ルールを維持したまま画像更新完了を確認できる。

---

### User Story 2 - 外部 HTTPS 運用の例外設定を扱う (Priority: P2)

自己署名証明書や検証未整備の外部 HTTPS サーバーを使う利用者として、必要なときだけ `insecure: true` を設定して認証付き取得を継続したい。これにより、検証環境や暫定運用でも Bearer 認証付き更新を止めずに使える。

**Why this priority**: 外部サーバー運用では HTTPS と認証の両方が必要であり、証明書事情により暫定的な例外設定が必要になる場面があるため。

**Independent Test**: `https://` の `image_url`、有効な `bearer_token`、`insecure: true` を設定し、通常の証明書検証では通過できない更新元に対して更新処理を実行し、認証付き更新が成功することを確認する。

**Acceptance Scenarios**:

1. **Given** `https://` の `image_url`、有効な `bearer_token`、`insecure: true` が設定されている, **When** 証明書検証を通常では通過できない HTTPS サーバーへ接続する, **Then** 利用者は Bearer 認証付き取得を継続できる。
2. **Given** `https://` の `image_url` と有効な `bearer_token` があり `insecure` が未設定または `false` である, **When** 証明書検証を通過できないサーバーへ接続する, **Then** 更新処理は成功扱いにならず失敗として終了する。

---

### User Story 3 - 認証設定ミスを判別する (Priority: P3)

運用者として、Bearer トークンや `insecure` の設定が誤っている場合に、設定不備、認証失敗、通信失敗を切り分けたい。これにより、外部サーバー運用時の障害調査を早められる。

**Why this priority**: 外部サーバー化により設定項目が増えるため、失敗理由を判別できないと運用負荷が高くなるため。

**Independent Test**: `bearer_token` の型不正、空文字、認証拒否応答、`insecure` 型不正の各ケースを用意し、通信開始前の設定不備と通信後の認証失敗が区別されることを確認する。

**Acceptance Scenarios**:

1. **Given** `bearer_token` または `insecure` の型が不正である, **When** 設定読込を行う, **Then** 更新処理は通信開始前に設定不備として失敗する。
2. **Given** `bearer_token` の形式は正しいが値が無効である, **When** 認証付き更新元へアクセスする, **Then** 更新処理は認証または HTTP 失敗として終了し設定不備とは区別される。

### Edge Cases

- `bearer_token` が未設定でも、認証不要な `http://` または `https://` 更新元は既存どおり利用できること。
- `bearer_token` が存在しても空文字は有効値として扱わず、設定不備として拒否すること。
- `insecure: true` でも Bearer トークン不正、HTTP status 異常、payload 不正、画像不正など証明書検証以外の失敗は従来どおり失敗扱いにすること。
- `bearer_token` は送信時に `Authorization: Bearer <token>` として扱われ、他の認証方式へ自動変換しないこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST allow `config.txt` to include an optional boolean field named `insecure`.
- **FR-002**: System MUST allow `config.txt` to include an optional string field named `bearer_token`.
- **FR-003**: System MUST treat omitted `insecure` as `false`.
- **FR-004**: System MUST accept both `http://` and `https://` values for `image_url`.
- **FR-005**: System MUST send `Authorization: Bearer <token>` on image retrieval requests only when `bearer_token` is configured with a non-empty string.
- **FR-006**: System MUST permit HTTPS image retrieval without server certificate verification only when `image_url` uses `https://` and `insecure` is `true`.
- **FR-007**: System MUST continue to require normal certificate verification for HTTPS retrieval when `insecure` is omitted or `false`.
- **FR-008**: System MUST reject `config.txt` as invalid before starting network access when `insecure` is present but is not a boolean.
- **FR-009**: System MUST reject `config.txt` as invalid before starting network access when `bearer_token` is present but is not a non-empty string.
- **FR-010**: System MUST preserve the existing update flow for unauthenticated HTTP retrieval, authenticated HTTPS retrieval, BMP rendering, binary frame rendering, and failure categorization outside the added authentication and certificate-verification branches.
- **FR-011**: System MUST leave `http://` retrieval behavior unchanged when `bearer_token` is omitted and `insecure` is omitted or `false`.
- **FR-012**: System MUST make the `insecure` and `bearer_token` settings understandable in the user-facing documentation for `config.txt`, including default values, effect scope, and operational cautions.

### Key Entities *(include if feature involves data)*

- **設定 JSON (`config.txt`)**: SDカードルートに置かれる更新設定。`wifi_ssid`、`wifi_password`、`image_url`、任意の `insecure`、任意の `bearer_token` を持つ。
- **`bearer_token`**: 画像取得時に `Authorization: Bearer <token>` を送るための任意 string 設定。未設定時は認証ヘッダを送らない。
- **`insecure` フラグ**: HTTPS 接続時だけ意味を持つ boolean 設定。`true` のときだけ証明書未検証通信を許可する。
- **画像取得設定**: `image_url`、`bearer_token`、`insecure` の組み合わせで認証有無と検証方針が決まる取得条件。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `config.txt` の仕様に `insecure` と `bearer_token` を追加すること。
- `https://` の `image_url` を設定可能にすること。
- Bearer トークン付き画像取得を追加すること。
- `insecure: true` の場合だけ HTTPS 証明書未検証通信を許可すること。
- 上記仕様に合わせて firmware 文書と検証手順を更新すること。

### Forbidden Scope

- `xiaozhi-esp32/` 配下を直接変更すること。
- `config.txt` の既存必須項目やファイル配置を変更すること。
- Bearer 以外の認証方式、トークン更新 API、署名付き URL 発行機構、証明書ピンニング、秘密情報暗号化保存など別機能を同時追加すること。
- `insecure: true` を既定値にすること、または利用者の明示設定なしに証明書未検証通信へ自動移行すること。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は `https://` の更新先と有効な `bearer_token` を設定した `config.txt` で、1 回の更新操作につき画像取得から表示更新完了まで到達できる。
- **SC-002**: 利用者は `insecure: true` を明示設定した場合だけ、証明書未検証の HTTPS 更新元に対しても Bearer 認証付き更新を成功させられる。
- **SC-003**: 利用者は `insecure` を省略または `false` にしたまま証明書未検証 HTTPS 更新元へ接続した場合に、成功扱いにならず失敗として判別できる。
- **SC-004**: 運用者は `bearer_token` または `insecure` の設定不備と、通信後の認証失敗を区別して判断できる。

## Assumptions

- 外部サーバー運用では Bearer 認証付き HTTPS が主用途であり、RunPod はその代表例である。
- `bearer_token` は `config.txt` に平文保存する運用を当面許容し、秘密情報保護の高度化は今回のスコープ外とする。
- 既存の HTTP 更新、画像形式判定、失敗状態保存、起動時更新と BOOT ボタン更新の流れは本機能でも維持する。

## Documentation Impact

- `config.txt` の契約文書に `insecure` と `bearer_token` の定義、既定値、`https://` 許可条件を追加する。
- quickstart と運用文書に Bearer 認証付き更新、未検証 HTTPS 利用手順、設定上の注意点を追加する。
- 必要に応じて失敗系テスト手順へ認証失敗、`bearer_token` 型不正、`insecure` 未設定時の HTTPS 失敗を反映する。

# 機能仕様: server 配信スクリプト追加

**Feature Branch**: `002-add-run-script`  
**Created**: 2025-09-05  
**Status**: Draft  
**Input**: ユーザー記述: "Spec Kit workflowに従って、server/contents/ ディレクトリを python3 -m http.server で配信するシェルスクリプトを server/run.sh に追加する。specify -> clarify(必要時のみ) -> plan -> tasks -> analyze -> implement -> commit -> merge の順で進め、必要な成果物を作成し、実装とテストまで行う。"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - ローカル配信開始 (Priority: P1)

開発者がローカルで `server/contents/` の内容をHTTPで配信し、ブラウザから確認できる。

**Why this priority**: 配信ができないと内容確認が進まないため最優先。

**Independent Test**: スクリプト実行後にブラウザで配信内容へアクセスできることを確認する。

**Acceptance Scenarios**:

1. **Given** リポジトリがローカルに存在する, **When** スクリプトを実行する, **Then** `server/contents/` のファイルがHTTPで配信される
2. **Given** 配信が起動している, **When** ブラウザで配信URLにアクセスする, **Then** 期待するファイル内容が表示される

---

### User Story 2 - 停止と再起動 (Priority: P2)

開発者が配信を停止し、必要に応じて再起動できる。

**Why this priority**: 作業中に再起動が必要になるため。

**Independent Test**: 停止操作後に配信が終了し、再実行で再度配信できることを確認する。

**Acceptance Scenarios**:

1. **Given** 配信が起動している, **When** 停止操作を行う, **Then** 配信が終了する

---

### Edge Cases

- 配信ポートが既に使用中の場合は、分かる形で失敗を通知する
- `server/contents/` が存在しない場合は、分かる形で失敗を通知する

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST `server/run.sh` を提供する
- **FR-002**: System MUST `server/contents/` をHTTPで配信できる
- **FR-003**: Users MUST be able to 配信を停止できる
- **FR-004**: System MUST 起動失敗時に理由が分かる出力を行う
- **FR-005**: System MUST `server/contents/` と `.gitignore` を用意する
- **FR-006**: System MUST 追加の設定なしで実行できる

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `server/run.sh` の追加
- ローカル配信用の最小限の実行導線

### Forbidden Scope

- 既存アプリケーションの配信方式変更
- `server/contents/` 配下の内容変更

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 初回利用で追加設定なしに配信を開始できる
- **SC-002**: 代表ファイルへアクセスし内容確認ができる
- **SC-003**: 起動失敗時に利用者が原因を把握できる

## Assumptions

- 利用者はローカル端末でシェルスクリプトを実行できる
- 配信はローカル検証用途であり外部公開は想定しない
- `server/contents/` は本機能で新規作成する

## Documentation Impact

- `server/run.sh` の利用方法を簡単に説明する記載が必要であれば追加する

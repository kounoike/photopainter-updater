# 機能仕様: ComfyUI PNG POSTノード

**Feature Branch**: `027-comfyui-post-node`  
**Created**: 2026-04-02  
**Status**: Draft  
**Input**: ユーザー記述: "026で作ったPOSTに送ることを想定しつつ、任意のURLに対してPNG画像をPOSTするComfyUIのカスタムノードをcomfyui/custom_node/comfyui-photopainter-custom以下に作って"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT:
  - user story は重要度順に並べる
  - 各 story は独立して実装・検証・デモ可能でなければならない
  - 受け入れ条件と Independent Test は日本語で具体的に書く
-->

### User Story 1 - Workflow から画像を送信したい (Priority: P1)

ComfyUI の利用者は workflow 内で生成または加工した画像を、追加の手動保存や別ツールなしで任意の URL へ PNG として POST したい。

**Why this priority**: この feature の中心価値は、ComfyUI の画像出力をそのまま外部 endpoint へ渡せることにあるため。

**Independent Test**: ComfyUI で画像を生成し、カスタムノードに 026 の `POST /upload` URL を設定して実行したとき、HTTP サーバ側が `200 OK` を返し、更新後画像が反映されることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が ComfyUI workflow で画像を扱える状態, **When** 利用者が PNG POST ノードへ画像と送信先 URL を渡して実行する, **Then** ノードは画像を PNG として送信し、送信先から返った成功応答を確認できる。
2. **Given** 利用者が 026 の `POST /upload` を送信先 URL に設定している状態, **When** workflow を実行する, **Then** 送信先サーバは画像を受理し、PhotoPainter 用の現在画像更新に使える。

---

### User Story 2 - 送信失敗を判別したい (Priority: P2)

ComfyUI の利用者は URL 不正、到達失敗、失敗 status などが起きたときに、workflow 上で送信に失敗したことを判別したい。

**Why this priority**: 外部 HTTP 送信を含む workflow では、成功時だけでなく失敗時に原因を切り分けられないと運用しづらいため。

**Independent Test**: 不正 URL、接続不能 URL、`400` または `500` を返すテスト先を設定し、ノードが失敗として扱い、利用者が判別できることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が到達不能または不正な URL を設定している状態, **When** workflow を実行する, **Then** ノードは送信失敗を成功扱いせず、失敗理由を判別できる。
2. **Given** 送信先が失敗 status を返す状態, **When** ノードが画像を送信する, **Then** ノードは失敗として扱い、返却 status を利用者が確認できる。

---

### User Story 3 - 繰り返し使いやすくしたい (Priority: P3)

ComfyUI の利用者は同じ workflow を繰り返し使う際に、送信先 URL や送信結果を workflow 上で一貫して扱いたい。

**Why this priority**: 一度動くだけでは不十分で、workflow 部品として再利用しやすいことがノード価値になるため。

**Independent Test**: 同じ workflow を URL を変えて複数回実行し、毎回ノードが入力画像を送信し、送信結果の表示や出力が一貫していることを確認する。

**Acceptance Scenarios**:

1. **Given** 利用者が workflow 内で送信先 URL を変更できる状態, **When** 別の URL で同じ workflow を再実行する, **Then** ノードは設定された URL へその都度送信する。
2. **Given** 利用者が複数回 workflow を実行する状態, **When** 各回で送信が成功または失敗する, **Then** ノードは毎回一貫した方法で送信結果を返す。

---

### Edge Cases

- 送信先 URL が空、HTTP/HTTPS 以外、または構文不正な場合は送信を開始しないこと。
- 入力画像が未接続または無効な場合は、空送信を行わず利用者が入力不足を判別できること。
- 送信先が `200` 以外の status を返した場合でも、成功扱いにせず status を確認できること。
- 026 の `POST /upload` を送信先にした場合でも、026 側専用の追加手動変換を利用者へ要求しないこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST ComfyUI のカスタムノードとして `comfyui/custom_node/comfyui-photopainter-custom` 配下に配置できなければならない。
- **FR-002**: System MUST workflow から受け取った画像を PNG として任意の URL へ HTTP POST できなければならない。
- **FR-003**: Users MUST be able to ノード入力として送信先 URL を指定できなければならない。
- **FR-004**: System MUST 送信成功時に、利用者が成功と判別できる送信結果を返さなければならない。
- **FR-005**: System MUST URL 不正、接続失敗、失敗 status、入力不足の各ケースで、成功扱いせず失敗理由を判別できなければならない。
- **FR-006**: System MUST 026 の `POST /upload` を送信先にした場合、追加の手動画像変換なしで PNG 送信を成立させなければならない。
- **FR-007**: System MUST 同じ workflow を複数回実行したとき、各回の入力画像と送信先 URL に基づいて独立に送信処理を行わなければならない。
- **FR-008**: System MUST 送信先が返した HTTP status と本文の要約、または同等の結果情報を利用者が確認できる形で保持しなければならない。

### Key Entities *(include if feature involves data)*

- **送信対象画像**: ComfyUI workflow からノードへ渡される画像。送信時には PNG 表現へ変換される。
- **送信先 URL**: 利用者がノードへ指定する HTTP 送信先。任意 URL を取りうるが、送信可能な URL である必要がある。
- **送信結果**: HTTP status、応答本文要約、成功/失敗判定を含むノード出力または表示情報。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- `comfyui/custom_node/comfyui-photopainter-custom` 配下へのカスタムノード追加
- PNG POST ノードの利用手順、設定項目、エラー挙動の文書化
- 026 の `POST /upload` と整合する送信契約の確認

### Forbidden Scope

- 既存 `server/` の 026 upload 機能仕様の変更
- ComfyUI 本体コアコードの無関係な改変
- 画像生成アルゴリズムや PhotoPainter firmware の変更
- 認証基盤、キュー基盤、永続ジョブ管理の新規導入

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は 1 回の workflow 実行で、ComfyUI 内の画像を指定 URL へ PNG として送信できる。
- **SC-002**: 026 の `POST /upload` を送信先にした場合、利用者は追加の手動変換なしで PhotoPainter 更新用画像を送れる。
- **SC-003**: 送信失敗時、利用者は 1 回の node 実行結果確認で送信失敗とその原因区分を判断できる。
- **SC-004**: 利用者は送信先 URL を切り替えて同じ workflow を再利用しても、送信手順を作り直さずに繰り返し実行できる。

## Assumptions

- 利用者は ComfyUI が動作する環境でカスタムノードを配置できる。
- 送信先は HTTP または HTTPS の到達可能な URL を想定する。
- 026 の `POST /upload` は PNG 画像を受理できる既存 endpoint として利用される。
- ノードは単一画像送信を対象とし、複数画像一括送信や非同期ジョブ管理までは必須範囲に含めない。

## Documentation Impact

- カスタムノードの配置方法、ComfyUI 上での使い方、送信先 URL の設定方法を案内する文書更新が必要になる。
- 026 の `POST /upload` と組み合わせる利用例を文書へ含める必要がある。
- 失敗時の status や到達失敗時の確認方法を利用者向けに記載する必要がある。

# Feature Specification: ComfyUI Docker Compose 統合

**Feature Branch**: `022-add-comfyui-compose`  
**Created**: 2026-03-30  
**Status**: Draft  
**Input**: User description: "配信する画像を生成するためにComfyUIをプロジェクトに追加したい。ゆくゆくはhttpサーバも合わせてdocker composeで起動するようにする。まずはComfyUIだけ起動するようcompose.ymlを作成する"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - ComfyUI をローカルで起動する (Priority: P1)

開発者がリポジトリをクローンした後、`compose.yml` を使って ComfyUI をローカル環境で起動できる。ブラウザから ComfyUI の Web UI にアクセスし、画像生成ワークフローを実行できる。

**Why this priority**: ComfyUI を使った画像生成がこのフィーチャーの中心目的であり、他のシナリオより先に動作が確認できる必要がある。

**Independent Test**: `docker compose up` を実行してブラウザで ComfyUI Web UI にアクセスし、デフォルトワークフローが表示されれば完了。

**Acceptance Scenarios**:

1. **Given** Docker および Docker Compose がインストール済みの環境で、**When** リポジトリルートで `docker compose up` を実行したとき、**Then** ComfyUI サービスが起動し、ブラウザで Web UI にアクセスできる。
2. **Given** ComfyUI が起動している状態で、**When** ブラウザからアクセスしたとき、**Then** デフォルトの画像生成ワークフローが表示される。
3. **Given** ComfyUI が起動している状態で、**When** `docker compose down` を実行したとき、**Then** サービスが正常に停止する。

---

### User Story 2 - モデルや生成画像をホストに永続保存する (Priority: P2)

開発者がダウンロードしたモデルファイルや生成した画像が、コンテナを停止・削除しても失われないよう、ホスト側のディレクトリに永続保存される。

**Why this priority**: モデルファイルは大容量であり、コンテナ再起動のたびに再ダウンロードするのは非効率。画像生成結果も保持する必要がある。

**Independent Test**: モデルを配置してコンテナ再起動後も同じモデルが利用できること、生成画像がホスト側に残ることを確認。

**Acceptance Scenarios**:

1. **Given** ホスト上のモデルディレクトリにモデルファイルを配置した状態で、**When** ComfyUI を起動したとき、**Then** そのモデルが ComfyUI 上で利用可能になる。
2. **Given** ComfyUI で画像を生成した後に、**When** `docker compose down` してコンテナを削除したとき、**Then** 生成画像がホスト側のディレクトリに残っている。

---

### User Story 3 - GPU アクセラレーションを利用する (Priority: P3)

GPU 搭載環境の開発者が、ComfyUI での画像生成時に GPU を活用して高速に処理できる。GPU がない環境でも CPU フォールバックで動作する。

**Why this priority**: 画像生成の実用速度に直結するが、機能検証自体は CPU でも可能なため優先度は低い。

**Independent Test**: GPU 搭載マシンで起動し、ComfyUI のシステム情報に GPU が認識されていることを確認。

**Acceptance Scenarios**:

1. **Given** NVIDIA GPU とドライバがインストール済みの環境で、**When** ComfyUI を起動したとき、**Then** ComfyUI が GPU を認識して利用できる。
2. **Given** GPU が搭載されていない環境で、**When** ComfyUI を起動したとき、**Then** CPU モードで正常に起動し画像生成が実行できる。

---

### Edge Cases

- GPU ドライバが未インストールの場合、エラーではなく CPU モードで起動する。
- 使用するポートがすでに占有されている場合、起動に失敗し明確なエラーが表示される。
- ホスト側のボリュームディレクトリが存在しない場合、Docker が自動作成するか、または README に事前手順が明記される。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `compose.yml` は ComfyUI サービスを定義し、`docker compose up` で起動できなければならない。
- **FR-002**: ComfyUI の Web UI はホストのブラウザからアクセス可能なポート（デフォルト 8188）でリッスンしなければならない。
- **FR-003**: ComfyUI のモデルディレクトリ、出力ディレクトリはホスト側ボリュームにマウントされ、コンテナ削除後も永続しなければならない。
- **FR-004**: GPU 環境では NVIDIA GPU がコンテナに渡され、ComfyUI が GPU を利用できなければならない。
- **FR-005**: `compose.yml` は将来の HTTP サーバサービス追加を考慮した構成（サービス分割、ネットワーク定義）でなければならない。
- **FR-006**: ボリュームのホスト側ディレクトリパスは環境変数または `.env` ファイルで変更できることが望ましい。

### Key Entities

- **ComfyUI サービス**: Docker Compose で管理される画像生成ワークフローサービス。モデル・出力・設定の各ディレクトリをボリュームとして持つ。
- **ボリューム**: モデルファイル（大容量）と生成画像を永続化するホスト側ディレクトリ。
- **ネットワーク**: 将来の HTTP サーバとの連携を見越した内部ネットワーク。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `docker compose up` の実行から ComfyUI Web UI へのアクセスが可能になるまで、初回起動（イメージ pull 済み）で 30 秒以内に完了する。
- **SC-002**: コンテナを再起動しても、事前に配置したモデルファイルと生成済み画像がすべて保持されている。
- **SC-003**: GPU 搭載環境で ComfyUI が GPU を認識して利用でき、CPU 環境でもエラーなく起動できる。
- **SC-004**: 新たな Docker サービスを `compose.yml` に追加する際、既存の ComfyUI サービスの定義変更が最小限（ネットワーク参照のみ）で済む。

## Assumptions

- 開発者のマシンには Docker および Docker Compose（v2 以降）がインストール済みであることを前提とする。
- GPU サポートは NVIDIA GPU + NVIDIA Container Toolkit を想定する（AMD/Apple Silicon は対象外）。
- ComfyUI の公式 Docker イメージ（または信頼できる公開イメージ）を使用する。独自ビルドは行わない。
- HTTP サーバとの Docker Compose 統合は本フィーチャーのスコープ外（将来フィーチャーで対応）。
- ポート番号はデフォルト 8188 を使用し、変更が必要な場合は `.env` ファイルで対応する。

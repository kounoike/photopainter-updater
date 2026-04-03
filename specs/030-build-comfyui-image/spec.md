# 機能仕様: ComfyUI 自前イメージ構築

**Feature Branch**: `030-build-comfyui-image`  
**Created**: 2026-04-04  
**Status**: Draft  
**Input**: ユーザー記述: "ComfyUIの環境がdockerコンテナ再起動したりするとおかしくなるから、ちゃんと自分でイメージ作って構築する"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 同じ ComfyUI 環境を再起動後も維持したい (Priority: P1)

ComfyUI の利用者は、コンテナ再起動や再作成のあとでも、同じ構成の環境をそのまま使い続けたい。これにより、起動するたびに依存関係や導入状態が変わって壊れる状況を避けられる。

**Why this priority**: 再起動のたびに利用可否が変わると、ComfyUI を作業基盤として使えず、この feature の価値が成立しないため。

**Independent Test**: 利用者が repo の案内に従って ComfyUI 環境を構築し、停止・再起動・再作成を行ったあとでも、同じ UI 到達確認と node 利用確認を再現できれば完了。

**Acceptance Scenarios**:

1. **Given** 利用者が repo 管理の手順で ComfyUI 環境を構築済みで, **When** ComfyUI コンテナを停止して再起動する, **Then** 再度同じ ComfyUI UI へ到達できる。
2. **Given** 利用者が一度構築した ComfyUI 環境を使っている状態で, **When** コンテナを再作成する, **Then** 利用者が追加の手作業なしで同じ構成を再現できる。

---

### User Story 2 - repo 管理の構成として再現したい (Priority: P2)

開発者として、ComfyUI 環境の構成要素を repo 側で管理し、他の環境でも同じ手順で再現したい。これにより、個別コンテナの場当たり的な修正ではなく、共有可能な構築方法として維持できる。

**Why this priority**: 手元だけで直る状態では、別マシンや将来の再構築時に同じ問題を繰り返すため。

**Independent Test**: 新しい利用者が repo 内の手順だけを使い、事前に修正済みコンテナを配布されなくても ComfyUI 環境を構築できれば完了。

**Acceptance Scenarios**:

1. **Given** 利用者がこの repo を clone した直後で, **When** ComfyUI 用の案内に従って構築する, **Then** repo 管理の構成だけで必要な環境を準備できる。
2. **Given** 既存の事前構築済みコンテナがない状態で, **When** 利用者が ComfyUI 環境を立ち上げる, **Then** 利用者は repo 管理の構築手順を起点に環境を再現できる。

---

### User Story 3 - 既存のデータと導線を壊したくない (Priority: P3)

ComfyUI の利用者は、自前イメージ方式へ移行しても、既存のモデル保存先、出力先、custom node 導線、関連サービスとの共存を壊したくない。これにより、環境の安定化を進めつつ既存作業を中断せずに済む。

**Why this priority**: 安定化のための変更で既存データや利用手順が壊れると、運用回帰のコストが大きいため。

**Independent Test**: 既存のデータディレクトリを保持したまま新しい構成へ移行し、ComfyUI の主要導線と repo 管理 custom node が継続利用できれば完了。

**Acceptance Scenarios**:

1. **Given** 既存のモデル、出力、input、user 設定を保持している利用者がいる状態で, **When** 自前イメージ方式へ切り替える, **Then** 既存データ保存先を継続利用できる。
2. **Given** repo 管理 custom node と関連サービス導線が存在する状態で, **When** 新しい ComfyUI 構成を使う, **Then** 既存の custom node 利用と他サービスとの共存前提は維持される。

### Edge Cases

- 初回構築時に必要な準備が不足していても、利用者がどこを確認すべきか判断できること。
- ComfyUI コンテナを削除して再作成しても、永続化対象のデータは不意に失われないこと。
- 既存の repo 管理 custom node や利用者追加の custom node が、新しい構成で見えなくならないこと。
- ComfyUI 以外の既存 compose サービスを使う利用者が、今回の変更で不要な再設定を強いられないこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST ComfyUI 環境を、repo 管理の構築物から再現可能な形で提供しなければならない。
- **FR-002**: System MUST ComfyUI 環境の再起動または再作成後も、利用者が同じ構成を継続利用できるようにしなければならない。
- **FR-003**: Users MUST be able to 事前に手修正されたコンテナへ依存せず、repo 内の手順だけで ComfyUI 環境を構築または再構築できなければならない。
- **FR-004**: System MUST 既存のモデル保存先、入力、出力、利用者設定などの継続利用対象を保持できなければならない。
- **FR-005**: System MUST repo 管理 custom node の利用導線を維持しなければならない。
- **FR-006**: System MUST ComfyUI の起動、再起動、再作成、障害切り分けの基本手順を利用者へ明示しなければならない。
- **FR-007**: System MUST 既存の Ollama、HTTP サーバ、AI Toolkit などの compose 導線を不必要に壊してはならない。
- **FR-008**: System MUST 利用者が現在使っている既定の ComfyUI 到達先と利用開始手順を大きく変えずに移行できるようにしなければならない。

### Key Entities *(include if feature involves data)*

- **ComfyUI 構築物**: repo 管理の手順から生成または取得される、ComfyUI 実行環境の基準となる構成。
- **ComfyUI 永続データ**: モデル、custom node、output、input、user 設定、キャッシュなど、コンテナ再作成後も継続利用したい利用者資産。
- **ComfyUI 運用導線**: 構築、起動、再起動、再作成、状態確認、障害時確認の利用者向け手順。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- ComfyUI サービスの構築方式を repo 管理へ寄せるための compose 構成見直し
- ComfyUI 実行環境の再現性を高めるための repo 内構築資産追加
- 既存データ保存先と custom node 導線を維持するための設定整理
- README と ComfyUI 関連 quickstart の更新
- 再起動、再作成、復旧確認のための最小限の検証導線整備

### Forbidden Scope

- ComfyUI workflow 自体の作り直し
- firmware や HTTP サーバの通信仕様変更
- Ollama や AI Toolkit の機能追加
- モデル配布物や生成物そのものの内容変更
- クラウド配備や複数ホスト運用まで含む大規模基盤設計

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は repo の案内を読み始めてから 20 分以内に、ComfyUI 環境を構築して UI 到達可否を判断できる。
- **SC-002**: 同じ利用者が ComfyUI コンテナの停止・再起動・再作成を行っても、追加の場当たり修正なしで同じ利用開始状態へ戻せる。
- **SC-003**: 既存のデータ保存先を使っている利用者が移行後もモデル、出力、custom node 導線を継続利用できる。
- **SC-004**: 起動失敗時に、利用者が手順書から少なくとも最初の確認先を 1 つ特定できる。

## Assumptions

- 現在の問題の主因は、既成イメージ依存またはコンテナ内の手修正により再現性が崩れている点にある。
- 利用者は引き続き Docker Compose ベースで ComfyUI を起動する。
- 今回の self-build 構成は NVIDIA GPU と CUDA を前提とし、CPU/AMD/Intel 向け最適化や互換対応は対象外とする。
- ComfyUI の既存公開ポートや基本的な到達方法は、可能な限り維持する。
- 既存の `comfyui-data` 系保存先と repo 管理 custom node は継続利用対象である。
- 今回の主目的は安定した再構築導線の確立であり、ComfyUI の新機能追加は主目的ではない。

## Documentation Impact

- README の ComfyUI 導線を、自前イメージ前提の構築手順へ更新する必要がある。
- ComfyUI 関連 quickstart を、初回構築・再起動・再作成・確認手順まで含めて見直す必要がある。
- 後続 phase では、compose 定義、環境変数テンプレート、ComfyUI 関連文書の整合を取る必要がある。

# 機能仕様: AI Toolkit 試用環境

**Feature Branch**: `025-ai-toolkit-env`  
**Created**: 2026-04-01  
**Status**: Draft  
**Input**: ユーザー記述: "AI Toolkitを試すための環境を作る"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Clarifications

### Session 2026-04-01

- Q: AI Toolkit 試用環境の正式導線は何を中心に定義するか? → A: Docker Compose サービス群中心
- Q: AI Toolkit 試用環境は既存資産のどこを土台にするか? → A: 既存 ComfyUI と Ollama を土台にし、不足分だけ追加する
- Q: 何をもって AI Toolkit を試せたとみなすか? → A: 主要サービスが起動し、`docker compose exec comfyui curl -fsS http://ollama:11434/api/version` が成功できれば試用成功

## User Scenarios & Testing *(mandatory)*

### User Story 1 - すぐ試せる作業開始導線 (Priority: P1)

開発者として、既存 ComfyUI と Ollama を土台に必要分だけ追加した Docker Compose サービス群を中心とする AI Toolkit 試用環境を使い、必要な準備がそろった状態から作業を始めたい。これにより、個別調査や手作業の初期設定に時間を使わず、検証したい操作へすぐ入れる。

**Why this priority**: 試用環境がすぐ使えなければ、この feature の価値が成立しないため。

**Independent Test**: 新しい開発者が案内に従って Docker Compose ベースの試用環境を立ち上げ、追加の探索なしに `docker compose exec comfyui curl -fsS http://ollama:11434/api/version` まで到達できれば完了。

**Acceptance Scenarios**:

1. **Given** 開発者がこのリポジトリを取得した直後で, **When** Docker Compose 中心の試用環境案内に従って準備を行う, **Then** AI Toolkit を試し始めるまでに必要な手順が一続きで分かる
2. **Given** 開発者が Docker Compose ベースの試用環境を起動した状態で, **When** `docker compose exec comfyui curl -fsS http://ollama:11434/api/version` を実行する, **Then** 試用成功可否を判断できる

---

### User Story 2 - 再現可能な共有環境 (Priority: P2)

開発者として、同じ既存 ComfyUI と Ollama を土台にした Docker Compose ベースの前提で AI Toolkit を繰り返し試せる再現可能な環境がほしい。これにより、毎回の試行で手順や判定基準がぶれず、同じ条件で検証をやり直せる。

**Why this priority**: 再現性がないと検証結果の共有や切り分けが難しくなり、試用の価値が下がるため。

**Independent Test**: 同じ利用者が別タイミングで同じ案内を使って環境を準備し、同等の初期状態から主要サービス起動と代表操作成功まで再到達できれば完了。

**Acceptance Scenarios**:

1. **Given** 同じ利用者が別タイミングで同じ成果物を参照する, **When** それぞれ試用環境を準備する, **Then** 主要な前提条件と開始手順の解釈がぶれない
2. **Given** 利用者が一度試用環境を中断した後で, **When** 再度試用環境へ入る, **Then** 同じ確認手順で利用開始可否を判断できる

---

### User Story 3 - 既存作業への影響を抑える (Priority: P3)

開発者として、AI Toolkit の試用環境を用意しても既存の開発導線や成果物を壊したくない。これにより、新しい試行を行いながら、今の作業を安全に継続できる。

**Why this priority**: 既存の利用導線を壊すと、新規試用より回帰リスクの方が大きくなるため。

**Independent Test**: 試用環境を追加した後も、既存の主要な開発導線が従来どおり参照・利用できることを確認できれば完了。

**Acceptance Scenarios**:

1. **Given** 既存の開発手順を使っている開発者がいる, **When** AI Toolkit 試用環境が追加される, **Then** 既存手順を置き換えずに並行して利用方針を理解できる
2. **Given** AI Toolkit をまだ使わない開発者がいる, **When** リポジトリの主要な導線を参照する, **Then** 既存作業に不要な混乱を招かない

### Edge Cases

- 必要な前提条件が満たされない場合でも、何が不足しているかを利用者が判断できること
- 途中で試用を中断しても、再開時に最初から調べ直さず続きから進められること
- AI Toolkit を使わない開発者が既存の導線だけを使う場合、試用環境追加による誤解や強制が起きないこと
- 外部サービス固有の認証差異は今回の対象外とし、Compose 起動とローカル疎通確認に必要な前提のみ扱うこと

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST AI Toolkit を試用する利用者向けに、既存 ComfyUI と Ollama を土台に必要分だけ追加する Docker Compose サービス群を中心とした開始条件、準備手順、最初の確認手順を一連で提供しなければならない
- **FR-002**: System MUST 利用者が追加の口頭説明なしで試用を始められるよう、必要な前提条件と不足時の対処方針を明示しなければならない
- **FR-003**: Users MUST be able to 共通の手順で主要サービス起動後に `docker compose exec comfyui curl -fsS http://ollama:11434/api/version` を実行し、利用開始可否を判断できなければならない
- **FR-004**: System MUST 同じ利用者が同じ Docker Compose ベースの初期条件から試用を再現できるよう、環境の前提と利用手順を統一して記録しなければならない
- **FR-008**: System MUST 既存 ComfyUI と Ollama の利用方針を壊さず、AI Toolkit 試用に必要な追加要素だけを識別できるようにしなければならない
- **FR-005**: System MUST 試用環境の追加が既存の主要な開発導線を置き換えないことを明示しなければならない
- **FR-006**: System MUST AI Toolkit を使わない利用者にも影響範囲が分かるよう、試用対象と非対象を区別して示さなければならない
- **FR-007**: System MUST 試用中に発生しやすい失敗パターンと復帰方法を利用者が参照できる状態にしなければならない

### Key Entities *(include if feature involves data)*

- **AI Toolkit 試用環境**: 利用者が AI Toolkit の動作確認や初期検証を行うための共通の作業前提、開始手順、確認導線。
- **Compose 試用導線**: AI Toolkit を試すための正式な起動・確認の流れ。複数サービスの起動順序、確認方法、停止方法を含む。
- **既存基盤サービス**: 試用環境の土台として継続利用する ComfyUI と Ollama。AI Toolkit 導入後も役割を維持する前提のサービス群。
- **利用開始手順**: 利用者が準備から最初の確認まで進むために順番に参照する案内。
- **前提条件一覧**: 試用前に満たすべき条件、不足時の確認観点、対象外事項をまとめた情報。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- AI Toolkit 試用環境のうち、既存 ComfyUI と Ollama を土台にした Docker Compose サービス群を中心にした開始条件、利用手順、確認方法の整理
- `compose.yml` の補助説明やコメント整理
- `.env.example` の AI Toolkit 試用向け説明更新
- `README.md` の AI Toolkit 入口追加と既存導線の整理
- 試用環境と既存開発導線の関係整理
- 試用時の前提条件、不足条件、復帰方法の文書化
- `specs/025-ai-toolkit-env/` 配下の成果物作成

### Forbidden Scope

- 本命ファームウェア機能や `server/` 本体機能の追加
- `xiaozhi-esp32/` 配下の直接変更
- AI Toolkit を前提にした既存開発導線の全面置換
- 試用範囲を超える本番運用設計や恒久運用の確定

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 新規利用者が案内を読み始めてから 15 分以内に、主要サービスの起動と `docker compose exec comfyui curl -fsS http://ollama:11434/api/version` の成功可否を自力で判断できる
- **SC-002**: 同じ利用者が同じ Docker Compose 中心の案内を使って別タイミングでも、主要な準備手順の解釈差異なく試用開始まで到達できる
- **SC-003**: AI Toolkit を使わない開発者が主要導線を参照した際に、試用環境が任意であることを 1 回の読了で理解できる
- **SC-004**: 試用中の代表的な失敗時に、利用者が追加質問なしで次の確認先または代表操作成功までの復帰手順を特定できる

## Assumptions

- この feature の対象利用者は、既存リポジトリを使って開発や検証を行う開発者である
- AI Toolkit はまず「試す」ことが目的であり、本番利用の設計確定までは求めない
- 既存の Compose 資産は試用導線の主要な土台として参照され、devcontainer は必要に応じた補助導線として扱う
- ComfyUI と Ollama は既存資産として継続利用し、今回の変更はそれらを置き換えず拡張する前提とする
- 外部サービス固有の認証や権限制御の差異は今回のスコープ外とし、ローカル Compose 利用に必要な前提だけを扱う
- 試用環境の価値は、最小限の準備で検証を始められることと、チームで前提を共有できることにある

## Documentation Impact

- `specs/025-ai-toolkit-env/spec.md` に AI Toolkit 試用環境の目的、利用者価値、境界を記載する
- 後続 phase で `plan.md`、`tasks.md`、必要に応じて `research.md`、`quickstart.md` を作成する
- 実装時には `README.md`、`.env.example`、必要に応じて `compose.yml` と関連運用文書へ、試用環境の入口と既存導線との関係を反映する必要がある

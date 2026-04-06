# 機能仕様: Local RunPod image 統一

**Feature Branch**: `044-local-runpod-image`  
**Created**: 2026-04-06  
**Status**: Draft  
**Input**: ユーザー記述: "ローカルのComfyUI環境もRunPod向けと同じworker-comfyuiベースのイメージを使うように統一したい。既存のローカル専用ComfyUI Dockerfileやcompose前提の仕組みは廃止してよい。ローカルでも/runpod-volumeをbind mountしてRunPodと同じモデル配置前提に寄せる。"

**記述ルール**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Clarifications

### Session 2026-04-06

- Q: local で共通 image を使う入口をどう統一するか → A: 既存 `compose.yml` の `comfyui` service を共通 image ベースへ置き換える
- Q: local の Ollama 接続方式をどう統一するか → A: local でも独立 `ollama` service は廃止し、`comfyui` コンテナ内 Ollama に統一する
- Q: local の `/runpod-volume` bind mount を必須にするか → A: local でも `/runpod-volume` bind mount を必須にする

## User Scenarios & Testing *(mandatory)*

### User Story 1 - ローカルでも RunPod と同じ ComfyUI image を使う (Priority: P1)

ローカル運用者として、ComfyUI をローカルで起動するときも RunPod 向けと同じ `worker-comfyui` ベース image を使いたい。これにより、ローカル確認対象と本番投入対象の差分を減らし、動いたはずなのに本番でだけ壊れる状況を避けられる。

**Why this priority**: 今回の中心価値は local と RunPod の runtime 差分をなくすことにあるため。

**Independent Test**: ローカルの ComfyUI 起動手順が RunPod 用 image を使う形に切り替わり、旧 local 専用 Dockerfile を使わずに Web UI 到達と Ollama 同居起動確認ができれば完了。

**Acceptance Scenarios**:

1. **Given** ローカルで ComfyUI を起動したい利用者がいる, **When** ローカル起動手順に従う, **Then** 利用者は RunPod と同じ image を使って ComfyUI を起動できる。
2. **Given** ローカル起動後に ComfyUI と Ollama の両方を確認したい, **When** 利用者が疎通確認手順を実行する, **Then** RunPod と同じ同居 runtime 前提で動作確認できる。
3. **Given** 利用者が既存の `docker compose up -d comfyui` 導線を使っている, **When** 新しい local 構成へ移行する, **Then** 利用者は service 名を変えずに共通 image ベースへ移行できる。
4. **Given** 利用者が従来の独立 `ollama` service を前提にしていた, **When** 新しい local 構成を確認する, **Then** 利用者は Ollama が `comfyui` コンテナ内へ統合されたと判断できる。

---

### User Story 2 - ローカルでも `/runpod-volume` 前提で model を扱う (Priority: P2)

ローカル運用者として、model 保存先や model 探索パスも RunPod と同じ `/runpod-volume` 前提に揃えたい。これにより、local と RunPod で model 配置の考え方を統一できる。

**Why this priority**: image だけ同じでも model path が違うと、workflow や troubleshooting の差分が残るため。

**Independent Test**: ローカル compose または同等の起動導線で `/runpod-volume` 相当の bind mount を使い、ComfyUI と Ollama がその配下を前提に model を扱えることを確認できれば完了。

**Acceptance Scenarios**:

1. **Given** ローカル利用者が model を永続化したい, **When** `/runpod-volume` 相当の host path を bind mount して起動する, **Then** 利用者は RunPod と同じ path 前提で model を配置・再利用できる。
2. **Given** ComfyUI と Ollama の両方が model を参照する, **When** ローカル構成を確認する, **Then** 利用者は `/runpod-volume/models` と `/runpod-volume/ollama/models` の役割を誤解なく判断できる。
3. **Given** 利用者が local 起動手順を実行する, **When** `/runpod-volume` bind mount が無い状態で起動しようとする, **Then** 利用者は現行導線の前提を満たしていないと判断できる。

---

### User Story 3 - 旧 local 専用構成を廃止する (Priority: P3)

保守者として、旧 local 専用の ComfyUI Dockerfile や compose 前提の仕組みを廃止し、どれが現行導線かを明確にしたい。これにより、保守対象を減らし、文書と実装のズレを防げる。

**Why this priority**: 2 系統の runtime を同時に維持すると、今後の変更が常に二重管理になるため。

**Independent Test**: 旧 local 専用導線が廃止されたことを文書と構成ファイルから確認でき、現行導線が新しい共通 image ベースだけになっていれば完了。

**Acceptance Scenarios**:

1. **Given** 保守者が repo の ComfyUI 関連構成を確認する, **When** 現行導線を探す, **Then** local / RunPod の両方が同じ image ベース前提で説明されている。
2. **Given** 利用者が古い local 専用手順を参照していた, **When** README や quickstart を読む, **Then** 旧導線が廃止され、新しい共通導線へ移行すべきと判断できる。

---

### Edge Cases

- ローカルで `/runpod-volume` を bind mount し忘れた場合、model が見つからない理由を利用者が判断できること。
- ローカルで `/runpod-volume` bind mount が無い状態を成功導線として誤認しないこと。
- 旧 local 用の環境変数や volume path が残っていても、現行導線として誤案内しないこと。
- RunPod 向け image を local にも使うことで port や起動シーケンスが変わっても、利用者が確認方法を追えること。
- 既存の `ollama` 独立 service を廃止または役割変更する場合でも、ComfyUI からの LLM 利用前提が壊れないこと。
- `compose.yml` の `comfyui` service を置き換えたあとも、利用者が別 service 名を探さずに済むこと。
- local の独立 `ollama` service を廃止したあとも、利用者が誤って `http://ollama:11434` 前提を残さないこと。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST ローカル ComfyUI 起動でも RunPod 向け `worker-comfyui` ベース image を共通 runtime として使えるようにしなければならない。
- **FR-002**: System MUST 旧 local 専用の ComfyUI Dockerfile と local 専用 runtime 導線を現行構成から外し、新しい共通導線を唯一の推奨手順として示さなければならない。
- **FR-003**: System MUST ローカル利用時も `/runpod-volume` を bind mount する前提で model 保存先と model 探索パスを構成しなければならない。
- **FR-003a**: System MUST local 利用手順では `/runpod-volume` bind mount を必須前提として扱い、省略可能な導線として説明してはならない。
- **FR-004**: System MUST ローカル利用時に ComfyUI と Ollama の model path 役割を `/runpod-volume` 配下で区別して案内しなければならない。
- **FR-005**: System MUST ローカルの compose または同等の起動導線を、共通 image を使う形へ更新しなければならない。
- **FR-005a**: System MUST `compose.yml` の既存 `comfyui` service を共通 image ベースへ置き換え、新しい service 名を利用者へ要求してはならない。
- **FR-005b**: System MUST local 構成の独立 `ollama` service を廃止し、`comfyui` コンテナ内 Ollama を唯一の推奨導線にしなければならない。
- **FR-006**: System MUST ローカル起動後の確認手順として、ComfyUI 到達確認、Ollama API 疎通確認、model path 確認を文書化しなければならない。
- **FR-007**: System MUST 旧 local 専用導線から新しい共通導線への移行時に、利用者が必要な host 側 directory 準備や bind mount を判断できるようにしなければならない。
- **FR-008**: System MUST RunPod 用 runtime を local にも流用しても、既存 custom node と `comfyui-ollama` 利用前提を維持しなければならない。

### Key Entities *(include if feature involves data)*

- **共通 ComfyUI image**: local と RunPod の両方で使う `worker-comfyui` ベース runtime image。
- **ローカル `/runpod-volume` bind mount**: local 環境で RunPod の永続領域を模す host 側 mount 入口。
- **旧 local 導線**: 既存 `comfyui/Dockerfile` とそれに依存した local 専用 compose / README 手順。
- **移行後 local 導線**: 共通 image を使い、`/runpod-volume` 前提で ComfyUI と Ollama を扱う local 起動手順。

## Scope Boundaries *(mandatory)*

### Allowed Scope

- local ComfyUI 起動導線の共通 image ベースへの移行
- `compose.yml`、README、quickstart、関連文書の更新
- 旧 local 専用 ComfyUI runtime 資産の廃止または役割整理
- `/runpod-volume` bind mount 前提の local model path 整理

### Forbidden Scope

- firmware、`server/`、`xiaozhi-esp32/` 配下への変更
- Ollama API の外部公開方針変更
- custom node の機能追加や workflow 自体の仕様変更
- RunPod serverless 専用 feature の削除

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 利用者は local と RunPod で同じ ComfyUI image を使って起動できる。
- **SC-002**: 利用者は local でも `/runpod-volume` bind mount 前提で model 配置と再利用を説明できる。
- **SC-003**: 保守者は旧 local 専用導線を追わずに、1 つの共通 runtime 導線だけを保守すればよい状態になる。
- **SC-004**: README と quickstart を読んだ利用者が、local 起動、ComfyUI 確認、Ollama 確認、model path 確認を迷わず実施できる。

## Assumptions

- local 環境でも `/runpod-volume` 相当の host bind mount を用意できる。
- local 用の ComfyUI も RunPod 用 image をそのまま使う方針でよい。
- 旧 local 専用 Dockerfile と導線は廃止して問題ない。
- `keep_alive` は引き続き node 側で `0` を指定する運用を継続する。
- local の独立 `ollama` service は不要であり、廃止して問題ない。
- local 利用者は `/runpod-volume` を bind mount する運用へ移行する。

## Documentation Impact

- root README の ComfyUI 起動手順を共通 image 前提へ更新する必要がある。
- `compose.yml` の既存 `comfyui` service を置き換える移行であることを README と quickstart に明記する必要がある。
- RunPod 用 README と feature quickstart に local 利用手順を統合または整理する必要がある。
- local 独立 `ollama` service 廃止と、接続先が localhost 前提へ変わることを文書へ反映する必要がある。
- local でも `/runpod-volume` bind mount が必須であることを文書へ明記する必要がある。
- 旧 local 専用 ComfyUI 手順が残っていれば、廃止または移行案内へ置き換える必要がある。

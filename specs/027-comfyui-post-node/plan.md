# 実装計画: ComfyUI PNG POSTノード

**Branch**: `027-comfyui-post-node` | **Date**: 2026-04-03 | **Spec**: [spec.md](./spec.md)  
**Input**: `/specs/027-comfyui-post-node/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

`comfyui/custom_node/comfyui-photopainter-custom` 配下に、ComfyUI workflow の `IMAGE` 入力を PNG raw body に変換して任意 URL へ `POST` する終端ノードを追加する。026 で実装済みの `POST /upload` と直接つながることを優先し、HTTP 送信形式は `Content-Type: image/png` 固定、失敗時は node 実行エラーとして workflow 全体を失敗扱いにする。実行時の ComfyUI は既存 compose により `custom_nodes/` を bind mount しているため、repo 内ソースと ComfyUI 実行ディレクトリの接続方法も文書化する。

## Technical Context

**Language/Version**: Python 3.x（ComfyUI ランタイム同梱版）  
**Primary Dependencies**: ComfyUI custom node backend API（`NODE_CLASS_MAPPINGS` / `OUTPUT_NODE`）、Python 標準ライブラリ `urllib` / `io`、既存ランタイムに含まれる `Pillow`、`numpy`  
**Storage**: ローカルファイル（repo 内 `comfyui/custom_node/comfyui-photopainter-custom/`、実行時 `comfyui-data/custom_nodes/`）  
**Testing**: Python 構文チェック、ローカル import 確認、ComfyUI 手動実行、026 の `POST /upload` への疎通確認  
**Target Platform**: Docker Compose で起動した ComfyUI、または同等の ComfyUI ローカル実行環境  
**Project Type**: ComfyUI カスタムノード + 利用手順ドキュメント  
**Performance Goals**: LAN 内の通常運用で 1 回の workflow 実行中に単一 PNG POST を完了し、余計な中間ファイルを生成しない  
**Constraints**: `Content-Type: image/png` の raw body 固定、終端ノード、`200 OK` 以外は失敗、追加の外部 Python 依存を増やさない、既存 `server/` 仕様は変更しない  
**Scale/Scope**: 単一利用者のローカル workflow、1 回の実行につき単一画像 1 枚送信を対象とする

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
  - Allowed: `comfyui/custom_node/comfyui-photopainter-custom` 配下のノード追加、利用手順と契約文書の追加
  - Forbidden: 既存 `server/` upload 仕様変更、ComfyUI 本体コアの無関係な改変、firmware 変更
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されている

**Phase 1 再確認（Design 後）**:
- [x] 新たな常駐サービスやジョブ基盤を追加していない
- [x] 既存 compose / server へ影響を広げず、ComfyUI custom node の最小変更に留めている
- [x] 検証手順は成功導線、失敗導線、026 互換導線をそれぞれ持つ

## Project Structure

### Documentation (this feature)

```text
specs/027-comfyui-post-node/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── comfyui-png-post-node-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
comfyui/
└── custom_node/
    └── comfyui-photopainter-custom/
        ├── __init__.py
        └── README.md

comfyui-data/
└── custom_nodes/
    └── comfyui-photopainter-custom -> ../../comfyui/custom_node/comfyui-photopainter-custom

README.md
```

**Structure Decision**: ユーザー要求どおり repo 管理下のソースは `comfyui/custom_node/comfyui-photopainter-custom` に置く。一方で ComfyUI の実行時探索先は既存 compose の `comfyui-data/custom_nodes` なので、実装では repo 側ソースを symlink または copy で取り込む前提を quickstart に明記する。

## Phase 0: Research 成果物

→ [research.md](./research.md) 参照

**主要決定事項**:

| 決定事項 | 採用内容 | 根拠 |
|---------|---------|------|
| ノード API 形式 | `NODE_CLASS_MAPPINGS` を使う従来型 Python custom node | 既存 runtime とローカルサンプルに最も近く、実装が最小 |
| HTTP 実装 | Python 標準ライブラリ `urllib.request` | 追加依存なしで `POST`、header、timeout、error handling を満たせる |
| 画像変換 | ComfyUI `IMAGE` tensor を `numpy` + `Pillow` で PNG bytes 化 | ComfyUI 慣習に沿い、026 の raw PNG body 契約に直結する |
| 成功/失敗の表現 | 成功時は UI 用 summary を返し、失敗時は例外を投げる | workflow で失敗を見逃さない要件に合致する |
| 配置方式 | repo 内ソース + 実行時 `custom_nodes` への symlink/copy | 既存 compose を変えずにユーザー要求のパスを守れる |

## Phase 1: Design

### Node 設計

- ノード名: `PhotopainterPngPost`
- 表示名: `PhotoPainter PNG POST`
- 種別: `OUTPUT_NODE = True` の終端ノード
- 入力:
  - `image`: `IMAGE`
  - `url`: `STRING`
- 出力:
  - `RETURN_TYPES = ()`
  - 成功時は ComfyUI UI に結果文字列を表示
- 主要処理:
  1. URL が空でないこと、`http` または `https` であることを検証
  2. `IMAGE` 入力が単一画像 1 枚であることを検証
  3. tensor を `uint8` RGB PNG bytes に変換
  4. `Content-Type: image/png` で `POST`
  5. `200 OK` のときだけ成功として summary を返す
  6. URL 不正、入力不足、接続失敗、`200` 以外の status は例外で失敗

### Runtime/Install 設計

- repo 内実装先は `comfyui/custom_node/comfyui-photopainter-custom/`
- ComfyUI 実行時は `comfyui-data/custom_nodes/comfyui-photopainter-custom/` に配置されている必要がある
- 開発時は symlink を推奨する
  - 変更即反映と Git 管理の両立が容易なため
- zip 配布や registry 対応は対象外

### Validation 設計

- 自動検証:
  - `python -m py_compile` 相当の構文確認
  - URL バリデーション、PNG 変換、HTTP レスポンス判定の単体テスト
- 手動検証:
  - ComfyUI 起動後にノードが一覧へ表示される
  - 026 の `POST /upload` へ送って `image.png` が更新される
  - 不正 URL / 接続失敗 / `400` 応答で workflow が失敗する

## Phase 1: Contracts

→ [contracts/comfyui-png-post-node-contract.md](./contracts/comfyui-png-post-node-contract.md) 参照

外部 HTTP endpoint を新設する feature ではないが、ComfyUI ノードの入出力と 026 upload への送信契約が実装とテストの基準になるため contract を文書化する。

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| なし | なし | なし |

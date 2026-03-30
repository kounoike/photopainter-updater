# 実装計画: デフォルト画像フォーマットを .bin に変更

**Branch**: `016-default-bin-format` | **Date**: 2026-03-30 | **Spec**: [spec.md](spec.md)
**Input**: `/specs/016-default-bin-format/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

サーバーの `/` ルートが現在 BMP を返しているのを `.bin` フォーマット（バイナリフレーム）返却に切り替える。あわせてファームウェアのフォーマット判定ロジックを反転し、URL が明示的に `.bmp` で終わる場合のみ BMP パスを選択し、それ以外はデフォルトでバイナリパスを使用するよう変更する。変更は最小構成で、既存の `serve_binary_image` ハンドラと `DownloadBinaryFrameToDisplay` パスをそのまま再利用する。

## Technical Context

**Language/Version**: Rust（サーバー）、C++（ファームウェア）
**Primary Dependencies**: axum（ルーティング）、既存バイナリフレーム処理実装
**Storage**: ファイルシステム（SD カード、サーバー側画像ストレージ）
**Testing**: サーバーは `cargo test`、ファームウェアは実機または単体関数テスト
**Target Platform**: Linux サーバー + ESP32S3
**Project Type**: firmware + server
**Performance Goals**: 既存の `.bin` ルートと同等のレスポンスタイムを維持
**Constraints**: LAN/WiFi 内オフライン動作、既存 BMP フォールバック維持
**Scale/Scope**: シングルデバイス・家庭内 LAN 運用

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない（`/image.bin` ルート自体は変更しない）
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている（spec.md の Independent Test 参照）
- [x] ローカル優先・最小構成を満たし、複雑化は明示的に正当化されている（既存実装の再利用のみ）

**Phase 1 design 後再確認:**
- [x] data-model.md の変更範囲が Allowed Scope に限定されている
- [x] contracts/server-routes.md が `image.bin` ルート自体を変更していない
- [x] 新規データ構造・依存ライブラリの追加なし

## Project Structure

### Documentation (this feature)

```text
specs/016-default-bin-format/
├── plan.md              # このファイル
├── research.md          # Phase 0 成果物 ✅
├── data-model.md        # Phase 1 成果物 ✅
├── quickstart.md        # Phase 1 成果物 ✅
├── contracts/
│   └── server-routes.md # Phase 1 成果物 ✅
└── tasks.md             # Phase 2 成果物 (/speckit.tasks)
```

### Source Code (変更対象ファイル)

```text
server/
└── src/
    └── main.rs              # L173: ルート変更 (serve_image → serve_binary_image)

firmware/
└── main/
    └── config.cc            # L35-41, L104-106: フォーマット判定ロジック変更
```

**Structure Decision**: サーバーとファームウェアの 2 コンポーネント構成。変更は各コンポーネントの既存ファイル 1 か所ずつに限定する。

## 実装詳細

### サーバー変更 (server/src/main.rs)

**変更箇所:** L173

```rust
// 変更前
.route("/", get(serve_image))

// 変更後
.route("/", get(serve_binary_image))
```

`serve_binary_image` は既存実装であり、`/image.bin` ルートで使用済み。追加実装不要。

### ファームウェア変更 (firmware/main/config.cc)

**変更箇所:** L35-41 周辺（`HasBmpSuffix` 追加）および L104-106（`IsBinaryImageUrl` 変更）

```c
// 追加: BMP サフィックス判定
bool HasBmpSuffix(const char* value) {
    if (value == nullptr) {
        return false;
    }
    size_t length = strlen(value);
    return length >= 4 && strcmp(value + length - 4, ".bmp") == 0;
}

// 変更: デフォルトをバイナリに（.bmp の場合のみ BMP パス）
bool IsBinaryImageUrl(const char* image_url) {
    return !HasBmpSuffix(image_url);
}
```

既存の `HasBinSuffix` は `IsBinaryImageUrl` から使われなくなるが、他で使用されている可能性を考慮して削除しない（使用箇所を確認した上でタスクで判断する）。

## 検証方針

| 対象 | 検証方法 | 期待結果 |
|------|---------|---------|
| `GET /` のフォーマット | `curl -I http://localhost:<port>/` | `Content-Type: application/vnd.photopainter-frame` |
| `GET /` と `GET /image.bin` の一致 | 両エンドポイントのレスポンスを比較 | 同一内容 |
| `.bmp` フォールバック | `GET /image.bmp` でリクエスト | `Content-Type: image/bmp` |
| ファームウェア: `image_url` が `/` の場合 | `IsBinaryImageUrl("http://s/")` | `true`（バイナリパス） |
| ファームウェア: `image_url` が `.bmp` の場合 | `IsBinaryImageUrl("http://s/image.bmp")` | `false`（BMP パス）|
| ファームウェア: `image_url` が `.bin` の場合 | `IsBinaryImageUrl("http://s/image.bin")` | `true`（後方互換） |

## Complexity Tracking

*憲章チェック違反なし。複雑化なし。*

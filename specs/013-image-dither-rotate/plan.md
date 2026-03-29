# 実装計画: 画像ディザリング回転配信

**Branch**: `013-image-dither-rotate` | **Date**: 2026-03-29 | **Spec**: [spec.md](/workspaces/photopainter-updater/specs/013-image-dither-rotate/spec.md)  
**Input**: `/specs/013-image-dither-rotate/spec.md` の仕様

**記述言語**: この文書は日本語で記述する。固有名詞、コード識別子、ライブラリ名のみ原文維持可。

## Summary

既存の HTTP サーバが `image.bmp` をそのまま返している構成を、`image.png` を入力として変換済み 24bit BMP を生成して返す構成へ拡張する。変換順序は、彩度を大きく強調し、`ref/convert.py` の palette と dithering 方針を基準に量子化し、その結果を右 90 度回転した後に BMP として配信する。既存の取得先 `/` と `/image.bmp` は維持し、入力画像未配置や変換失敗時には入力画像由来の問題と判別できる失敗応答を返す。彩度強調の合否は `server/testdata/image-dither-rotate/pre.png` と `post.png` の比較 fixture で判定する。

## Technical Context

**Language/Version**: Rust stable + 参照用 Python 3 スクリプト  
**Primary Dependencies**: `axum`、Tokio、Rust 標準ライブラリのファイルアクセス、画像変換ライブラリ、`ref/convert.py` を参照したディザリング方針  
**Storage**: ローカルファイル (`server/contents/image.png` 入力、配信時に生成される 24bit BMP 出力)  
**Testing**: Rust 自動テスト、`server/testdata/image-dither-rotate/pre.png` / `post.png` の代表座標 `(4,4)` `(12,4)` `(4,12)` `(20,12)` `(12,20)` `(4,28)` `(12,28)` `(20,28)` を各チャネル差 `±3` で比較する彩度 fixture テスト、参照変換との比較確認、HTTP 手動確認  
**Target Platform**: devcontainer またはローカル開発環境、LAN 内の BMP 配信サーバ  
**Project Type**: ローカル常駐 HTTP サーバの画像変換付き配信機能  
**Performance Goals**: 利用者が 1 回の取得で変換済み BMP を受け取れ、入力画像差し替え後の次回取得に新しい変換結果が反映されること  
**Constraints**: 既存 route `/` と `/image.bmp` を維持する、入力画像は単一の `image.png`、`ref/convert.py` 参照範囲は palette と dithering 方針に限定する、彩度強調は fixture 比較で判定し代表座標は `server/testdata/image-dither-rotate/README.md` に揃える、`firmware/` や `xiaozhi-esp32/` は変更しない  
**Scale/Scope**: 単一画像を少数端末へ配信するローカル用途。まずは 1 枚の PNG から 1 枚の BMP を生成する最小構成

## Constitution Check

*GATE: Phase 0 research 前に必ず確認し、Phase 1 design 後に再確認する。*

- [x] `specify -> clarify -> plan -> tasks -> analyze -> implement` の順序を破っていない
- [x] Allowed Scope / Forbidden Scope が明記され、範囲外変更がない
- [x] 文書・説明は日本語で記述されている
- [x] 各 user story と主要要求に検証方法が定義されている
- [x] ローカル優先・最小構成を満たし、画像変換追加は既存取得先維持のための限定的拡張として扱う

## Project Structure

### Documentation (this feature)

```text
specs/013-image-dither-rotate/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── transformed-bmp-response-contract.md
└── tasks.md
```

### Source Code (repository root)
```text
server/
├── contents/
│   └── .gitignore
├── Cargo.toml
├── Cargo.lock
├── run.sh
└── src/
    └── main.rs
    
server/testdata/
└── image-dither-rotate/
    ├── pre.png
    ├── post.png
    └── README.md

ref/
└── convert.py

specs/
├── 010-http-bmp-server/
├── 012-fix-run-access-path/
└── 013-image-dither-rotate/
```

**Structure Decision**: 実装は既存サーバの責務を保つため `server/` 配下へ閉じる。`server/src/main.rs` に画像読込、彩度変換、参照相当ディザリング、右 90 度回転、BMP 応答生成を統合し、`ref/convert.py` は palette と dithering 方針の品質基準として参照専用で扱う。彩度強調の基準画像は `server/testdata/image-dither-rotate/` に置く。新規 route は増やさず、既存の `/` と `/image.bmp` を維持する。

## Complexity Tracking

> **憲章チェック違反を例外承認する場合のみ記入する**

該当なし。

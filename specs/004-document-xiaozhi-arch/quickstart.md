# Quickstart: xiaozhi-esp32 構造解析ドキュメント

## 目的

`xiaozhi-esp32` の構造解析結果を `docs/` 配下へ追加し、後続開発者が調査起点として使える状態にする。

## 実装手順

1. `spec.md`、`plan.md`、`research.md`、`data-model.md`、`contracts/documentation-contract.md` を確認する。
2. `docs/xiaozhi-esp32-architecture.md` を作成し、概要、対象範囲、ディレクトリ構成、主要関心領域、主要フロー、共通実装とボード固有実装の境界、注意点を配置する。
3. `xiaozhi-esp32/main/main.cc` と `xiaozhi-esp32/main/application.cc` を起点に、起動フローと主要責務を整理する。
4. `xiaozhi-esp32/main/protocols/`、`main/audio/README.md`、`main/display/`、`main/settings.*`、`main/ota.*`、`main/boards/README.md` を読み、関心領域ごとの実装位置を整理する。
5. 必要なら既存 README から新規文書への最小限の導線を追加する。
6. 文書中で、確認済み事項、推定事項、対象外事項の区別を明示し、名称ではなく観察した責務ベースで説明する。

## 検証手順

1. User Story 1: 文書だけを読み、主要ディレクトリの役割と起動・通信・音声の代表的主要フローを説明できることを確認する。
2. User Story 2: 文書から、起動、音声、表示、通信、設定、OTA、ボード差分の実装位置を一意に追えることを確認する。
3. User Story 3: リポジトリ利用者が `docs/xiaozhi-esp32-architecture.md` を見つけられ、必要なら README 導線から到達できることを確認する。
4. 文書中で、確認済み事項、推定事項、対象外事項が区別されていることを確認する。
5. 名称と責務がずれて見える箇所でも、観察した責務ベースで説明されていることを確認する。
6. Forbidden Scope にあるコード挙動変更や網羅的ボード仕様一覧化が含まれていないことを確認する。

## 完了条件

- `docs/` 配下に技術文書が追加されている。
- 文書が日本語で記述されている。
- 代表的主要フローと主要関心領域が含まれている。
- 導線追加が必要な場合は、既存 README から参照できる。
- 観察した責務ベースの説明と、確認済み/推定/対象外の区別が含まれている。

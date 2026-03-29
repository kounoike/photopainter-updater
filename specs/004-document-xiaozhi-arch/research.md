# Research: xiaozhi-esp32 構造解析ドキュメント

## Decision 1: 成果物は `docs/` 配下の単一文書として追加する

- Decision: ルート `docs/` を最終成果物の配置先とし、`xiaozhi-esp32` 構造解析用の単一技術文書を追加する。
- Rationale: `spec` の clarify で配置先を `docs/` と確定済みであり、コード対象に依存しすぎず、リポジトリ全体から見つけやすい恒久文書として扱えるため。
- Alternatives considered:
  - `specs/004-document-xiaozhi-arch/` 内に成果物を残す: 設計成果物と最終利用者向け技術文書が混在し、継続参照先として弱い。
  - `xiaozhi-esp32/` 配下へ直接配置する: 対象に近いが、プロジェクト共通の技術文書置き場としては拡張性が低い。

## Decision 2: 文書粒度は「全体構造 + 代表的主要フロー」に制限する

- Decision: 文書は全モジュール網羅ではなく、全体構造、責務分担、関心領域別の実装位置、起動・通信・音声などの代表的主要フローに限定する。
- Rationale: 改修の着手点を得るには概要だけでは不十分だが、全ボード・全モジュール詳細まで広げると Allowed Scope を超えやすくなるため。
- Alternatives considered:
  - ディレクトリ構成のみを説明する: 実装読解の起点として不足する。
  - ほぼ全モジュールを詳細解説する: スコープ肥大化と保守コスト増大を招く。

## Decision 3: 調査の一次情報はコード入口と既存補助文書に限定する

- Decision: 調査起点は `xiaozhi-esp32/main/main.cc`、`xiaozhi-esp32/main/application.cc`、`xiaozhi-esp32/main/protocols/`、`xiaozhi-esp32/main/audio/README.md`、`xiaozhi-esp32/main/boards/README.md`、`xiaozhi-esp32/main/settings.*`、`xiaozhi-esp32/main/ota.*` を中心にする。
- Rationale: 入口、主要オーケストレータ、通信抽象、音声系補助文書、ボード差分、設定、OTA を押さえれば、spec が要求する主要関心領域を過不足なく説明できるため。
- Alternatives considered:
  - `components/` 全域を先に網羅する: 初手として広すぎ、代表フローの把握効率が悪い。
  - README のみで構造説明を組み立てる: 実コードとの対応が弱く、責務の裏付けに欠ける。

### 調査メモ

- `main/main.cc` は `app_main()` から NVS 初期化とモード分岐を行う、最初の読解起点として有効。
- `main/application.cc` は assets 更新、OTA、表示通知、状態管理を横断する中核オーケストレータ。
- `main/protocols/protocol.h` は MQTT / WebSocket 両実装の共通抽象を示すため、通信層の把握起点として使いやすい。
- `main/audio/README.md` は音声パイプラインの概要把握に有効で、コード本体を読む前の入口として価値が高い。
- `main/boards/README.md` は `boards/` 配下の共通パターンを説明しており、個別ボード調査前の地図として使える。

## Decision 4: 検証は「文書だけで辿れるか」を中心に手動確認する

- Decision: 実装フェーズの検証は、文書から主要関心領域の実装位置を辿れるか、代表的主要フローを追えるか、導線から文書に到達できるかを手動で確認する。
- Rationale: 今回の成果物はコードではなく技術文書であり、自動テストよりも読解可能性と導線確認が完了条件に直結するため。
- Alternatives considered:
  - 自動 lint や構文チェックのみで完了判定する: 文書品質や調査起点としての有効性を十分に検証できない。
  - コードのビルドや実機動作まで含める: Forbidden Scope であり、今回の目的と一致しない。

## Decision 5: 契約成果物は「文書構成契約」として残す

- Decision: `contracts/` には API ではなく、最終成果物が満たすべき文書構成・導線・対象範囲を定義した `documentation-contract.md` を置く。
- Rationale: この feature の外部インターフェースは開発者が参照する技術文書そのものであり、最終成果物の構造要件を先に固定すると tasks と implement のブレを抑えられるため。
- Alternatives considered:
  - `contracts/` を作らない: plan テンプレート上の設計成果物が欠け、完了条件の共有が弱くなる。
  - 実装対象ファイル名だけ決める: セクション要件や導線条件が抜け落ちる。

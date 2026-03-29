# Research: SDカード設定 HTTP e-paper 更新ファーム

## Decision 1: 設定は SDカードルートの `config.txt` に固定する

- Decision: 設定ファイル名は SDカードルートの `config.txt` に固定し、中身は JSON として解釈する。
- Rationale: clarify で確定済みであり、利用者向け説明、実装の探索ロジック、手動運用を最も単純にできるため。
- Alternatives considered:
  - `settings.json`: 意味は通るが、既存コードや利用手順の説明上で優位性がない。
  - 複数候補のファイル名探索: 実装と検証が複雑化する。

## Decision 2: 画像取得先は単一 URL に限定する

- Decision: `config.txt` は単一の画像 URL のみを持ち、毎回その 1 枚を取得する。
- Rationale: 今回の目的は確実な起動時更新と手動更新であり、複数画像管理や選択ロジックは scope を超えるため。
- Alternatives considered:
  - 画像 URL 配列: JSON 仕様と表示ルールが膨らむ。
  - ベース URL + パラメータ: API 的な拡張を生み、現時点では不要。

## Decision 3: 失敗時は更新処理を終了し、そのままシャットダウンする

- Decision: 設定不備、WiFi 失敗、HTTP 失敗、画像不正時は更新処理を継続せず、失敗理由を判断できる状態を残してシャットダウンする。
- Rationale: ユーザー要求で明示されており、リトライやエラー表示を追加しない最小構成として一貫しているため。
- Alternatives considered:
  - 既存表示維持のまま待機: 失敗時の最終状態が曖昧になる。
  - e-paper にエラー表示: 追加 UI 設計が必要になり scope が広がる。

## Decision 4: 既存 BSP と基礎部品を再利用する

- Decision: `components/sdcard_bsp`、`components/button_bsp`、`components/epaper_port`、既存のネットワーク/HTTP 周辺部品を再利用候補とする。
- Rationale: 既存ツリーに SDカード初期化、BOOT ボタン、e-paper 表示、HTTP 関連部品がすでにあるため、専用ファームとしては再利用が最短である。
- Alternatives considered:
  - すべて新規実装: 既存コードベースを土台にする利点が薄れる。
  - 既存の会話系アプリ層をそのまま流用: 単機能ファームには責務が重すぎる。

### 実装反映メモ

- `sdcard_bsp` は `/sdcard` の mount とファイル I/O 前提を再利用する。
- `button_bsp` は `boot_groups` の single click event を BOOT ボタン更新トリガとして再利用する。
- `epaper_port` と `epaper_src` は `GUI_ReadBmp_RGB_6Color()` と `epaper_port_display()` の経路で再利用する。
- `http_client_bsp` は天気取得向けの固定用途 API なのでそのままは流用せず、`firmware/` 側で `esp_http_client` を画像取得専用に包む。

## Decision 5: 検証は正常系 2 本と失敗系 4 本を最小セットにする

- Decision: 検証は「起動時更新成功」「BOOT ボタン更新成功」に加え、「設定ファイル欠落」「WiFi 失敗」「HTTP 失敗」「画像不正」の失敗系を最小セットとする。
- Rationale: spec の成功基準と edge case を最小限でカバーし、実装完了判定を明確にできるため。
- Alternatives considered:
  - 正常系だけ確認する: 失敗時シャットダウン要件を満たしたか判断できない。
  - ボード差分まで網羅的に確認する: 初期スコープとして過剰。

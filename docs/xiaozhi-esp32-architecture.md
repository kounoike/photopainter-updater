# xiaozhi-esp32 構造解析メモ

## 概要

この文書は、リポジトリに同梱されている `xiaozhi-esp32/` の実装構造を、後続開発者の調査起点として整理した技術メモである。目的は API リファレンスの網羅ではなく、どこに何の責務があり、どのファイルから読むと全体像を掴みやすいかを短時間で把握できるようにすることにある。

### 記法

- 確認済み: 実ファイルを読んで責務や流れを確認した内容
- 推定: 命名や周辺コードから合理的に推定した内容
- 対象外: 今回は深掘りしていない領域

## 対象範囲と対象外

### 対象範囲

- `xiaozhi-esp32/` 直下の主要ディレクトリ構造
- `main/main.cc` と `main/application.cc` を中心とした起動経路
- 音声、表示、通信、設定、OTA、ボード差分の実装位置
- 起動、通信、音声の代表的な主要フロー

### 対象外

- 全ボードの個別仕様の網羅
- 全コンポーネントの API 詳細
- 実装の正しさ、性能、上流との差分検証
- `xiaozhi-esp32` 自体の挙動変更

## ディレクトリ構成と責務分担

### ルート直下

- `xiaozhi-esp32/CMakeLists.txt`
  - 確認済み: ESP-IDF の project 定義と `PROJECT_VER` を持つ最上位 CMake 入口。
- `xiaozhi-esp32/sdkconfig*`
  - 確認済み: チップ種別やビルド差分向けの設定群。
- `xiaozhi-esp32/partitions/`
  - 推定: OTA や assets 配置を含むパーティション構成定義。
- `xiaozhi-esp32/scripts/`
  - 確認済み: 変換・生成補助スクリプト群。実行時本体ではなく開発補助寄り。

### `main/`

- 役割
  - 確認済み: アプリ本体の入口と高レベルのオーケストレーション層。
- 主なファイル
  - `main/main.cc`
    - 確認済み: `app_main()` を持つエントリポイント。NVS 初期化、モードフラグ読み出し、`User_Mode_init()`、`Application::GetInstance().Start()` などの分岐を担当。
  - `main/application.cc`
    - 確認済み: デバイス状態、assets 更新、OTA、表示通知、音声/通信との接続などを束ねる中核オーケストレータ。
  - `main/audio/`
    - 確認済み: 音声処理・入出力・wake word 周辺。
  - `main/display/`
    - 確認済み: `Display` 抽象と LCD/OLED 実装。
  - `main/protocols/`
    - 確認済み: `Protocol` 抽象の上に MQTT / WebSocket 実装を載せる通信層。
  - `main/settings.*`
    - 確認済み: NVS をラップする設定永続化ヘルパ。
  - `main/ota.*`
    - 確認済み: バージョン確認、設定配布、アクティベーション、ファーム更新。
  - `main/boards/`
    - 確認済み: 各ボードごとの `config.h`、`config.json`、`*.cc`、README を持つ板別実装置き場。

### `components/`

- 役割
  - 確認済み: 板別や機能別に再利用される補助コンポーネント群。
- 例
  - `audio_bsp`, `codec_board`, `http_client_bsp`, `http_server_bsp`, `led_bsp`, `i2c_bsp`
    - 推定: ハードウェアや周辺機能の下位抽象を提供する層。
- 読み方
  - 対象外: 今回は `components/` 全域の詳細までは追っていない。必要になったら `main/` から参照されるものを辿るのが起点として適切。

## 主要関心領域ごとの実装位置

### 1. 起動

- 主な起点: `xiaozhi-esp32/main/main.cc`
- 確認済み:
  - 既定イベントループ生成
  - NVS 初期化
  - `PhotoPainter` 名前空間でモード系キーを読む
  - `PhotPainterMode` に応じて xiaozhi mode / basic mode / network mode / mode selection に分岐
- 次に読む場所:
  - `xiaozhi-esp32/main/application.cc`
  - `xiaozhi-esp32/components/user_app_bsp/`

### 2. アプリ全体制御

- 主な起点: `xiaozhi-esp32/main/application.cc`
- 確認済み:
  - デバイス状態を文字列とイベントで管理
  - assets 更新、OTA、アクティベーション、表示通知を統括
  - Board, Display, AudioService, Protocol を横断的に扱う
- 次に読む場所:
  - `xiaozhi-esp32/main/device_state.h`
  - `xiaozhi-esp32/main/assets.*`
  - `xiaozhi-esp32/main/mcp_server.*`

### 3. 音声

- 主な起点: `xiaozhi-esp32/main/audio/README.md`
- 確認済み:
  - `AudioService` が中心オーケストレータ
  - `AudioCodec` が物理 codec との入出力を担当
  - `AudioProcessor` が AEC / VAD などの前処理を担当
  - `WakeWord` が独立に起動ワード検出を担う
  - FreeRTOS task と queue で uplink/downlink を分離
- 次に読む場所:
  - `xiaozhi-esp32/main/audio/audio_service.cc`
  - `xiaozhi-esp32/main/audio/audio_codec.*`

### 4. 通信

- 主な起点: `xiaozhi-esp32/main/protocols/protocol.h`
- 確認済み:
  - `Protocol` が音声・JSON・接続イベントの共通インターフェースを定義
  - `MqttProtocol` は MQTT 制御 + UDP 音声送信 + AES 文脈を持つ
  - `WebsocketProtocol` は WebSocket ベースの別実装
- 推定:
  - 起動後はサーバ設定や配布設定に応じて MQTT / WebSocket のどちらかが使い分けられる
- 次に読む場所:
  - `xiaozhi-esp32/main/protocols/mqtt_protocol.cc`
  - `xiaozhi-esp32/main/protocols/websocket_protocol.cc`

### 5. 表示

- 主な起点: `xiaozhi-esp32/main/display/display.h`
- 確認済み:
  - `Display` 抽象が status、notification、emotion、chat message など UI 更新の共通面を提供
  - 実装は `lcd_display.cc` と `oled_display.cc` に分かれる
  - power save や lock/unlock も display 抽象配下で扱う
- 次に読む場所:
  - `xiaozhi-esp32/main/display/lcd_display.cc`
  - `xiaozhi-esp32/main/display/oled_display.cc`

### 6. 設定

- 主な起点: `xiaozhi-esp32/main/settings.h`
- 確認済み:
  - `Settings` は NVS namespace ごとの読み書きラッパ
  - string / int / bool の取得・設定、key/all erase を提供
  - destructor で dirty 時に commit する
- 次に読む場所:
  - `xiaozhi-esp32/main/settings.cc`
  - `xiaozhi-esp32/main/main.cc`
  - `xiaozhi-esp32/main/ota.cc`

### 7. OTA と設定配布

- 主な起点: `xiaozhi-esp32/main/ota.cc`
- 確認済み:
  - `ota_url` は `wifi` namespace の設定または `CONFIG_OTA_URL` を使用
  - バージョン確認 API の応答から `activation`、`mqtt`、`websocket`、`server_time`、`firmware` を解析
  - OTA だけでなく通信設定配布もここで行っている
- 次に読む場所:
  - `xiaozhi-esp32/main/ota.h`
  - `xiaozhi-esp32/main/application.cc`

### 8. ボード差分

- 主な起点: `xiaozhi-esp32/main/boards/README.md`
- 確認済み:
  - 各ボードは独立ディレクトリを持ち、典型的には `config.h`、`config.json`、`*.cc`、`README.md` で構成される
  - `config.h` はピン配置や表示/音声条件、`config.json` はターゲット chip と build 設定を持つ
  - `*.cc` が板別の初期化と具象実装の寄せ場になる
- 次に読む場所:
  - `xiaozhi-esp32/main/boards/README.md`
  - 任意の代表ボードディレクトリ

## 代表的主要フロー

### フロー 1: 起動とモード分岐

1. `app_main()` が既定イベントループを作成する
2. NVS を初期化し、`PhotoPainter` namespace のモード設定を読む
3. `User_Mode_init()` を呼んでボタン系初期化を行う
4. `PhotPainterMode` の値に応じて xiaozhi mode / basic mode / network mode / mode selection に分岐する
5. xiaozhi mode の場合は `Application::GetInstance().Start()` に処理が渡る

確認済み: 1-5  
対象外: `User_Basic_mode_app_init()` 以降の下位実装詳細

### フロー 2: 音声 uplink / downlink

1. `AudioInputTask` が `AudioCodec` から PCM を読む
2. 入力は `AudioProcessor` や `WakeWord` に渡される
3. `OpusCodecTask` が encode queue から PCM を取り、Opus 化して send queue に積む
4. 通信層が Opus packet をサーバへ送る
5. 受信側では decode queue に積まれた Opus packet を PCM に戻し、playback queue を介して speaker 再生する

確認済み: 音声 README 上の構成  
推定: `Application` と `Protocol` が send / receive の結線を担う詳細

### フロー 3: OTA と通信設定の反映

1. `Application` が `Ota` を通して新バージョン確認を行う
2. `Ota::CheckVersion()` が HTTP でサーバに問い合わせる
3. 応答 JSON から activation、MQTT、WebSocket、server_time、firmware を読む
4. 必要に応じて `Settings` 経由で `mqtt` / `websocket` namespace を更新する
5. 新ファームが必要なら更新を進め、そうでなければ現在バージョンを valid 扱いにする

確認済み: 2-5  
推定: 1 の呼び出しタイミングの全分岐

## 共通実装とボード固有実装の境界

### 共通実装

- `main/`
  - アプリ全体制御、表示抽象、通信抽象、設定、OTA などの中核処理
- `components/`
  - 複数ボードで使い回す BSP / 補助コンポーネント

### ボード固有実装

- `main/boards/<board-name>/`
  - `config.h`: ピン、表示サイズ、codec 条件など
  - `config.json`: ターゲットチップと build 差分
  - `*.cc`: 板別の初期化や具象クラス

### 境界の見方

- 新しい機能が「すべてのボードで共有される制御」に属するなら `main/` または `components/` を疑う
- 特定基板のピンや表示/音声デバイス差分なら `main/boards/` を疑う
- 名称だけで判断せず、`main/application.cc` や board README から実際の参照関係を追うほうが安全

## 調査時の注意点と未確認事項

- `components/` 配下は広く、今回の調査では `main/` から辿る範囲に留めている
- `PhotoPainter` 名前空間や `PhotPainterMode` など、命名に揺れが見える箇所がある。名称ではなく実際の読み書き箇所で責務を判断したほうがよい
- `main/audio/README.md` は構造理解の起点として有用だが、詳細挙動を確定するには `audio_service.cc` の実装確認が必要
- `main/boards/` は数が多く、共通パターン把握に留めた。個別ボード対応時は対象ディレクトリを直接読む前提とする
- OTA 応答仕様の外部仕様書 URL はコメントに示されているが、今回その外部文書自体は確認していない

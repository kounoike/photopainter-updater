# 実験設定コントラクト: 写真調追加改善 profile

**Branch**: `020-adaptive-diffusion-tuning` | **Date**: 2026-03-30

## 目的

写真調向け追加改善を、既存の server 起動経路と比較ワークフローの中で再現可能にするため、profile と検証入力の責務を定義する。

## 維持するインターフェース

| 項目 | 内容 |
|------|------|
| `GET /` | 既存どおり binary frame を返す |
| `GET /image.bin` | 既存どおり binary frame を返す |
| `GET /image.bmp` | 既存どおり BMP を返す |
| `CONTENT_DIR` | 手動で差し替える入力画像ディレクトリ |
| `PORT` | 待受ポート |

## 追加・更新する設定インターフェース

### `IMAGE_PROFILE`

| 項目 | 内容 |
|------|------|
| 型 | 文字列 |
| デフォルト | `baseline` |
| 許容値 | 既存 profile に加えて、新しい写真調向け profile `adaptive-photo` を追加する |
| 役割 | 既存上位候補と追加改善案を切り替える |
| エラー時 | 未知の key は起動エラーとして扱う |

### `DITHER_DIFFUSION_RATE`

| 項目 | 内容 |
|------|------|
| 型 | 数値 |
| デフォルト | profile 既定値 |
| 役割 | profile が持つ既定の拡散率を必要時に上書きする |
| エラー時 | 数値以外は起動エラーとして扱う |

## 互換性ルール

- `IMAGE_PROFILE=baseline` かつ比較設定なしは現行挙動と互換であること
- 既存 profile の key と比較モードは破壊しないこと
- 新 profile は `COMPARE_WITH_BASELINE` または `COMPARE_PROFILE` と組み合わせて比較できること
- firmware から見える URL と payload 形式は変更しないこと

## 評価入力の最小要件

今回の feature では、比較入力として少なくとも次を持てること:

| 項目 | 説明 |
|------|------|
| `image7` 相当 | 明るい低彩度面と肌の中間調を評価できる写真調画像 |
| `image8` 相当 | 青系の広い面を評価できる写真調画像 |
| `image6` 相当 | 既存イラスト調の回帰をざっくり確認できる画像 |

## 非目標

- HTTP API のクエリパラメータ化
- firmware 向けの新しい制御コマンド
- 転送 payload への評価メタデータ埋め込み
- 自動化された実機比較ハーネスの導入

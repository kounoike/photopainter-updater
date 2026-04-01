# Integration Boundary

今回の feature は trial 学習基盤までに限定し、生成フロー側の本格統合は後続へ分離する。

## 今回固定するもの

- Docker ベースの `SimpleTuner` 実行環境
- `Z-Image Turbo` 用 trial config
- 少数画像 dataset と caption の最低限の layout
- LoRA artifact と validation prompt の受け渡し単位

## 今回やらないもの

- ComfyUI での end-to-end 自動生成
- `keep / adapt / replace` の分岐実装
- Qwen Image Edit による個別修正

## 責務分界

### LoRA 側

- `character fixed traits`
- trial 中に最低限保持したい identity

### 将来の生成フロー側

- `outfit variable traits`
- scene variable traits
- event / weather / pose / composition

## 将来拡張点

- `keep`
  - 参照衣装を維持
- `adapt`
  - 元衣装ベースで小さな季節差分を入れる
- `replace`
  - イベント衣装へ置換する

この 3 値は今回実装しないが、artifact と prompt の責務分界を崩さない範囲で後続 feature に持ち込めるように整理する。

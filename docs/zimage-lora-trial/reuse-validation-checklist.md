# Reuse Validation Checklist

## 前提

- [ ] LoRA artifact の保存先が確定している
- [ ] validation prompt テンプレートが render されている
- [ ] 出力先ディレクトリを決めた
- [ ] 既存のローカル `Z-Image` 推論環境を使える

## 実行

- [ ] `scripts/zimage-lora/validate-reuse.sh --artifact ...` を実行した
- [ ] reuse manifest が生成された
- [ ] `VALIDATE_REUSE_COMMAND` が設定されている場合は、その command が実行された
- [ ] 1 件以上の生成画像が出力された

## 判定

- [ ] 同一キャラクターとして識別可能か判断した
- [ ] 顔 drift をメモした
- [ ] 色 drift をメモした
- [ ] 衣装 drift をメモした
- [ ] 継続 / reject を決めた

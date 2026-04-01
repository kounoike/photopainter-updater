# Manual Checklist

## 学習開始前

- [ ] `docker --version` と `docker compose version` が通る
- [ ] `nvidia-smi` が通る
- [ ] `scripts/zimage-lora/configs/trial-12gb.env` を作成した
- [ ] `HF_TOKEN` を env に設定した
- [ ] `scripts/zimage-lora/validate-trial-layout.sh` が pass した

## build / 起動

- [ ] `scripts/zimage-lora/train-trial.sh --build-only` で image build が通る
- [ ] build log に GPU runtime 関連の致命エラーがない
- [ ] rendered config が `scripts/zimage-lora/workspace/runtime/<training_name>/` に生成される

## 学習実行中

- [ ] `simpletuner train --config ...` が開始された
- [ ] dataset path 不足や config parse error で即時終了していない
- [ ] startup OOM が出た場合は `ZLORA_FORCE_INT8=1` や `ZLORA_FORCE_FP16=1` の縮退を試した
- [ ] validation steps 到達まで進行している

## 学習後

- [ ] LoRA 成果物が `output/<training_name>/` 配下に出力された
- [ ] validation 画像が保存された
- [ ] trial 条件、OOM 有無、drift 観察結果をメモした

## 再利用確認

- [ ] `VALIDATE_REUSE_COMMAND` のテンプレートを確認した
- [ ] `scripts/zimage-lora/validate-reuse.sh` で manifest を生成した
- [ ] 既存のローカル `Z-Image` 推論環境で 1 件以上の validation 画像を確認した

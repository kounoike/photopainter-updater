# リサーチ: 写真調ディザリングの追加改善

**Branch**: `020-adaptive-diffusion-tuning` | **Date**: 2026-03-30

## 1. 追加改善の記録単位

**Decision:** 今回の改善は runtime の新 profile としては残さず、再現用アルゴリズム文書として残す。

**Rationale:** `019` では `color-priority + DITHER_DIFFUSION_RATE=0.8` が暫定上位候補になっており、これを基準として残したまま差分比較できる状態が必要である。今回の試作は実機で有意差を示せず、新しい問題点も見つかったため、runtime に残すより再現可能な文書として残す方が妥当である。

**Alternatives considered:**
- `color-priority` の既定値をそのまま書き換える: 比較基準が消え、回帰評価がしにくい。
- 試作コードをそのまま残す: 差が見えない状態で複雑さだけが残る。

---

## 2. 青保持の改善方針

**Decision:** 青系の広い面には、候補色選択時に白黒への吸い込みを追加で抑え、青候補を選びやすくする色域依存の補正を入れる。

**Rationale:** `019` では `color-priority` 系でも空の青が十分に戻らないという所見が残った。青保持は全色一律の彩度ブーストよりも、「青として扱うべき領域で青が白や黒へ吸われない」ことが重要である。

**Alternatives considered:**
- 全色に対する彩度ブーストを戻す: 写真調の自然さを崩しやすい。
- `hue-guard` だけを強める: 飛び色抑制には効いても、青そのものを残す効果は限定的。

---

## 3. 明るい低彩度面と肌の中間調の改善方針

**Decision:** 高明度かつ低彩度の画素、および肌寄りの暖色中間調では、誤差拡散を弱める局所制御を導入する。

**Rationale:** `019` の比較では、写真調画像で明るい低彩度面がざらつきや白面化を起こしやすく、肌も平板になりやすかった。固定の `DITHER_DIFFUSION_RATE=0.8` は全体のバランスは良いが、弱点のある色域だけを保護できない。局所的に拡散を弱めれば、淡色面の濁りと肌のノイズ増幅を抑えやすい。

**Alternatives considered:**
- `DITHER_DIFFUSION_RATE` をさらに全体で下げる: 暗部や輪郭まで均しすぎる。
- `Atkinson` を再採用する: イラスト調で均しすぎる傾向があり、今回の第一候補にはしにくい。

---

## 4. 評価画像の扱い

**Decision:** 既存 `image7` に加えて、青系の広い面を含む写真調画像を `server/testdata/dither-result-check/` に追加し、青保持を別軸で確認する。

**Rationale:** `019` は `image7` が青空を含まないため、青保持の良し悪しを十分に判定できなかった。今回の feature では、写真調の 2 系統以上で比較結果を残すことが成功条件に含まれる。

**Alternatives considered:**
- `image7` だけで評価を続ける: 青保持の論点が未評価のまま残る。
- 人工 fixture だけで代替する: 色域の傾向は見えても写真調の破綻を評価しきれない。

---

## 5. ローカル検証の最小単位

**Decision:** ローカル検証では、既存 fixture に加えて色域別の差分確認テストを追加し、新 profile が青寄り、低彩度高明度、肌寄り暖色で狙った方向へ動くことを確認する。

**Rationale:** 実機主判定だけに頼ると、コード変更と画質変化の関係を追いにくい。ローカルで色域別の振る舞いを確認できれば、実機所見とのつながりを説明しやすい。

**Alternatives considered:**
- 実機確認だけで進める: 回帰切り分けが遅くなる。
- 画像全体の golden 比較だけを使う: どの色域を改善したかが見えにくい。

---

## 6. 試作アルゴリズムの再現メモ

**Decision:** 今回試した追加改善は、次の 3 要素を組み合わせた実験アルゴリズムとして文書に残す。

**Rationale:** 実装コードは残さないが、後で同じ案を再試行できる程度の再現性は必要である。

### 再現用アルゴリズム要約

1. ベースは `color-priority + DITHER_DIFFUSION_RATE=0.8`
2. 青系の広い面では、白黒候補より青候補を選びやすくする補正を追加する
3. 高明度かつ低彩度の領域では、誤差拡散を局所的に弱める
4. 肌寄りの暖色中間調でも、誤差拡散を局所的に弱める

### 試作時に見えた限界

- 実機では `color-priority + DITHER_DIFFUSION_RATE=0.8` と有意差が見えにくかった
- 青空保持の改善は確認できなかった
- `image8` の cyan/teal 系の独特な服色が、実機では普通の水色に寄って見える問題が残った

### ローカル確認結果

- 試作コードを外した状態で `cargo test` が通ることを確認した
- 現行 runtime は `color-priority + DITHER_DIFFUSION_RATE=0.8` を基準に据えたまま維持する

---

## 7. 実機観察メモ欄

以下は、今回の追加改善検討で見えた所見を残す欄として使う。

### 7-0. `image6` 補足メモ

- artifact: `specs/020-adaptive-diffusion-tuning/artifacts/image6/image6_splitview.jpg`
- compare_target: `color-priority + DITHER_DIFFUSION_RATE=0.8`
- observation:
  - 右半分の方が全体に黄色が鮮やかに見え、第一印象は良い
  - 写真調向け追加改善案を当てても、少なくともこのイラスト調入力では大きな破綻より先に「黄の鮮やかさ向上」が目についた
- interpretation:
  - イラスト調に対して即座に悪化するタイプの回帰は今のところ強く見えていない
  - ただし `image6` は今回の主判定画像ではないため、採否判断の主根拠には使わず補足所見として扱う

### 7-1. `image7`

- compare_target: `color-priority + DITHER_DIFFUSION_RATE=0.8`
- artifact: `specs/020-adaptive-diffusion-tuning/artifacts/image7/image7_splitview.jpg`
- skin_midtones:
  - 右半分の方が全体に見栄えが良く、左半分より破綻感が少ない
- bright_low_saturation:
  - 左半分はゴミが乗っているような見え方があり、右半分の方が素直に見える
- noise_or_black_speckles:
  - 左半分でノイズや汚れに見える粒が目立つ
- false_color_risk:
  - 現時点では右半分で強い悪化は感じない
- decision: `hold`
- next_action:
  - 肌と明るい背景のどちらに効いているかを単独表示でも一度だけ確認する

### 7-2. `image8`

- compare_target: `color-priority + DITHER_DIFFUSION_RATE=0.8`
- artifact: `specs/020-adaptive-diffusion-tuning/artifacts/image8/image8_splitview.jpg`
- blue_retention:
  - 右半分の方が全体としては良く見えるが、どちらの半分にも青空が十分には見えない
- bright_background_grain:
  - 左半分はゴミが乗っているようなノイズ感があり、右半分の方が見やすい
- side_effects:
  - 右半分が優勢でも、今回主目的だった青空保持は未達のまま残っている
- neutral_absorption:
  - 左半分は白灰や汚れ寄りに見えやすい
- cyan_teal_clothing:
  - 元画像の緑よりの水色っぽい独特の服色が、実機では普通の水色に寄って見える
- decision: `reject` 寄りの `hold`
- next_action:
  - 青空保持だけでなく、cyan/teal 系衣服の色崩れも新しい問題設定として追加する

## 8. 判定基準と現時点の結論

### 8-1. `advance` / `hold` / `reject` の基準

- `advance`
  - ローカル回帰に加え、写真調 2 系統以上の比較で既存上位候補を上回る改善が確認できる
- `hold`
  - ローカルでは改善意図が確認できるが、実機比較または代表画像での優位性確認が未完了である
- `reject`
  - ローカル回帰で既存 profile を悪化させる、または狙った色域に対する改善が確認できない

### 8-2. 現時点の結論

- `reject`: 写真調向け追加改善の runtime 採用
  - `image6` / `image7` / `image8` を見比べても、既存 `color-priority + DITHER_DIFFUSION_RATE=0.8` に対する有意差は見えにくかった
  - 一部で左半分の「ゴミが乗ったような」ノイズ感との差は見えたが、profile 追加に見合うだけの明確な前進とは言いにくい
  - `image8` では青空保持が改善せず、さらに cyan/teal 系の服色が普通の水色に寄る新しい問題点が見つかった
  - したがって、今回の試作は runtime に残さず、再現用アルゴリズム文書だけを保持する

### 8-3. 次アクション

- 現行 runtime は `color-priority + DITHER_DIFFUSION_RATE=0.8` を維持する
- 次に写真調改善を再開する場合は、青空保持だけでなく cyan/teal 系衣服の色崩れも問題設定に含める
- 実験を再開する場合は、本書の再現用アルゴリズム要約を出発点にして別 feature として切り直す

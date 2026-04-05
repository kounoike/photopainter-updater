# research.md

## Decision 1: continuation の主対象は自由文の長い最終回答に限定する
- Decision: continuation はまず text mode の長文回答を対象にし、1 回目で本文が切れたと判断できるときだけ追加 generation を行う。
- Rationale: ユーザーの主訴は「回答本文が長くて最後まで読めない」ことであり、最小スコープで価値を出すには自由文 continuation に絞るのが妥当である。
- Alternatives considered:
  - すべての生成結果に continuation を適用する: JSON や `think_mode=off` と衝突しやすく、契約破壊のリスクが高いため不採用。
  - continuation を行わず `max_tokens` 調整だけに委ねる: 長文回答の完結性を node 側で担保できないため不採用。

## Decision 2: `think_mode=off` の reasoning trace 救済には使わない
- Decision: `think_mode=off` で reasoning trace が出た場合は continuation せず、既存どおり failure とする。
- Rationale: 042 は長文回答 continuation の feature であり、041 で導入した strict `off` 契約を崩してはならない。trace 救済を許すと hidden reasoning を再び成功扱いにする。
- Alternatives considered:
  - `off` でも final answer 部分だけ取れるまで continuation する: strict `off` の意図に反するため不採用。

## Decision 3: JSON mode は自由文 continuation に混ぜない
- Decision: `json_output=true` では continuation を自由文向けと切り分け、曖昧な連結 rescue を行わない。
- Rationale: strict JSON / schema 契約では、途中終了と不正 JSON の境界が曖昧になりやすい。まずは text mode continuation だけを安定させた方が安全である。
- Alternatives considered:
  - 未完了 JSON に continuation を適用する: 将来的にはあり得るが、停止判定と schema 契約を別途設計すべきなので今回は不採用。

## Decision 4: continuation は明示上限と進展検知を持つ
- Decision: continuation 回数上限と「同じ断片しか増えていない」「空文字が返る」などの停止条件を実装する。
- Rationale: 長文回答 continuation は便利だが、モデルが進展しない場合に token を浪費しやすい。上限と進展検知がないと無限継続の危険がある。
- Alternatives considered:
  - 完結するまで無制限 continuation: 停止保証がなく危険なため不採用。
  - 回数上限だけで進展検知なし: 同じ断片を繰り返すケースを防ぎきれないため不採用。

## Decision 5: continuation の事実は debug へ出す
- Decision: continuation の発生有無、回数、停止理由、最終停止が「完結」「上限到達」「進展なし」のどれかを `debug_json` に出す。
- Rationale: 長文回答が遅かった理由や、どこで停止したかを利用者が workflow 上で切り分けられる必要がある。
- Alternatives considered:
  - 連結後の最終テキストだけ返す: 途中で追加 generation が走ったことが見えず、遅延原因を判断できないため不採用。

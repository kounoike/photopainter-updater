# Research: Release Drafter 導入

## Decision 1: 更新契機は `main` への `push` のみにする

- Decision:
  - release draft 更新は `main` への `push` のみを対象とする。
- Rationale:
  - ユーザー clarification で、PR 作成時点では反映させず、merge 後だけ更新したい意図が明確になった。
  - 運用者が「公開候補に確定した変更だけを draft に載せたい」という期待に一致し、draft のノイズも減る。
- Alternatives considered:
  - `pull_request` 更新でも draft を更新する: merge 前の変更が見えてしまい、今回の意図とずれる。
  - 手動更新にする: 自動導線が失われ、spec の主目的を満たさない。

## Decision 2: 分類情報は pull request labels を前提にする

- Decision:
  - draft 内の分類は pull request labels を起点に整理し、未分類は既定カテゴリへ送る。
- Rationale:
  - labels は GitHub 上で既に一般的な運用単位であり、追加の入力チャネルを増やさずに分類できる。
  - 既定カテゴリを持たせることで FR-005 の「欠落させない」を満たしやすい。
- Alternatives considered:
  - branch 名だけで分類する: 意味の粒度が粗く、運用者の意図を反映しにくい。
  - 本文解析で分類する: 複雑化し、最小構成の原則に反する。

## Decision 3: 設定責務は workflow と release drafter 設定に分離する

- Decision:
  - 更新契機は workflow 側、分類ルールと表示内容は release drafter 設定側に分ける。
- Rationale:
  - 更新タイミングと changelog 構造を分離した方が、運用者がどこを変更すべきか判断しやすい。
  - 後続の分類調整だけを行う場合にも、workflow の変更を避けられる。
- Alternatives considered:
  - すべて workflow 内へ寄せる: 設定変更の見通しが悪くなる。
  - 文書だけ追加して設定を曖昧にする: 実装時の検証可能性が下がる。

## Decision 4: README に確認導線を追加する

- Decision:
  - repository ルートの README へ release draft の設定場所と確認方法を追記する。
- Rationale:
  - 運用者が最初に見る導線として README が最も見つけやすく、FR-007 を満たしやすい。
  - `.github/` の設定ファイルだけでは導入後の運用方法が伝わらない。
- Alternatives considered:
  - feature 配下文書だけに閉じる: 実装時の補助にはなるが、運用導線として弱い。
  - 別の docs を新設する: 今回スコープでは過剰。

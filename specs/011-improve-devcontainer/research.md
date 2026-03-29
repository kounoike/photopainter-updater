# Research: Devcontainer 起動改善

## Decision 1: 主要 CLI はコンテナ作成後ではなく image build 時点で利用可能にする

- Decision: `@openai/codex` と `@anthropic-ai/claude-code` は devcontainer 利用可能時点で存在する状態を標準とし、接続後に長い追加セットアップを走らせる前提をやめる。
- Rationale: 現在の `.devcontainer/devcontainer.json` は `postCreateCommand` で global install を実行しており、コンテナ再作成のたびに待ち時間が発生する。今回の最優先要求は「接続直後に作業開始できること」なので、接続後処理ではなく image 側で準備を完了させる方が要求に直接一致する。
- Alternatives considered:
  - `postCreateCommand` を維持する
    - 見送り理由: コンテナ再生成のたびに待ち時間が再発し、FR-001/FR-002 を満たしにくい。
  - 各開発者が手動で CLI を都度インストールする
    - 見送り理由: 手順のばらつきが大きく、再現性がない。

## Decision 2: 認証状態はワークスペース外の永続領域で保持する

- Decision: Codex などの認証関連ファイルはワークスペースの Git 管理対象に置かず、devcontainer 再生成後も残る永続領域へ分離して保持する前提で設計する。
- Rationale: spec の FR-003/FR-004 は「再生成後も再利用できること」と「Git 管理対象と分離すること」を同時に要求している。認証情報をワークスペースに置く運用は機密情報混入のリスクが高く、逆にコンテナ内部の一時領域だけでは再生成時に消える。永続領域を分離するのが最小で妥当な折衷である。
- Alternatives considered:
  - 認証情報をワークスペース配下へ保存する
    - 見送り理由: Forbidden Scope の「Git 管理対象へ認証情報を保存しない」に反する運用を招きやすい。
  - 毎回再認証を前提にする
    - 見送り理由: SC-002 に反し、日常運用の手間が大きい。

## Decision 3: 永続化対象は「認証継続に必要な最小範囲」に限定する

- Decision: 永続化するのは対象 CLI の認証状態と関連キャッシュに限定し、アプリケーションの build 成果物やリポジトリ内容まで拡張しない。
- Rationale: 本 feature の目的は開発環境の起動時間短縮と再認証削減であり、広範な永続化は要求されていない。保持対象を最小限に絞ることで、複雑化と情報保持範囲の不透明さを避けられる。
- Alternatives considered:
  - ホームディレクトリ全体を広く永続化する
    - 見送り理由: 何が残るか把握しづらく、不要ファイルや意図しない設定まで持ち越しやすい。
  - 永続化を一切行わない
    - 見送り理由: 認証消失問題を解決できない。

## Decision 4: 検証は devcontainer の実運用フローに沿った手動確認を主とする

- Decision: この feature の受け入れ確認は自動テストではなく、devcontainer の新規作成・再生成・再接続・CLI 起動・再認証不要確認を含む手動検証を中心に定義する。
- Rationale: 対象は container lifecycle とローカル認証状態であり、リポジトリ内だけで完結する自動テスト化が難しい。今回必要なのは実際の利用フローで問題が消えることなので、手動確認の手順を Quickstart として固定するのが適切である。
- Alternatives considered:
  - リポジトリ内ユニットテストのみで確認する
    - 見送り理由: devcontainer 再生成や認証保持の成否を十分に表現できない。

## Decision 5: 運用説明は既存の devcontainer 前提文書を更新して一箇所に寄せる

- Decision: 開発者向けの説明は既存の devcontainer 前提を記載している文書を主対象に更新し、初回認証、再生成後の再利用、意図的な初期化方法をまとめる。
- Rationale: 既存 `docs/firmware.md` と `docs/firmware-http-epaper.md` は devcontainer 前提での作業をすでに案内している。別文書を増やすより既存導線を更新した方が、利用者が迷いにくい。
- Alternatives considered:
  - feature 専用 README を新規追加する
    - 見送り理由: 導線が分散し、運用説明が重複する。

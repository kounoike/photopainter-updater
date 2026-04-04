# Research: Release 時の GHCR image publish

## Decision 1: trigger は `release.published` に限定する

- Decision:
  - image publish workflow は GitHub Actions の `release` event に対して `types: [published]` のみを購読する。
- Rationale:
  - GitHub 公式ドキュメントでは、release workflow は `published` を使うことで stable release と draft から publish された pre-release の両方を拾える。
  - draft の `created` や `edited` では workflow が走らないため、spec の「正式 release publish 時のみ開始する」に合う。
  - `main` push と切り離すことで、既存 Release Drafter の責務と衝突しない。
- Alternatives considered:
  - `push tag` ベースにする: release UI からの publish と責務が分かれ、spec の trigger 定義がぶれる。
  - `release.released` や `prereleased` を併用する: draft からの公開挙動が複雑になり、`published` 単独より説明しづらい。

## Decision 2: GHCR publish は `GITHUB_TOKEN` と job permissions を使う

- Decision:
  - GHCR への認証は GitHub Actions 標準の `GITHUB_TOKEN` を使い、job permissions に `packages: write` と `contents: read` を付ける。
- Rationale:
  - GitHub 公式の GHCR publish 例は `docker/login-action` に `ghcr.io`、`${{ github.actor }}`、`${{ secrets.GITHUB_TOKEN }}` を渡す構成を示している。
  - repository に紐づく package として公開しやすく、追加 secret を増やさずに最小構成を維持できる。
  - package 権限を workflow ごとに閉じられるため、運用説明も単純になる。
- Alternatives considered:
  - personal access token を別 secret で管理する: 権限管理が重くなり、今回スコープには過剰。
  - ローカルや外部 CI から publish する: repository 内 release 運用から外れ、確認導線も分散する。

## Decision 3: tag と OCI metadata は release version 起点で統一する

- Decision:
  - image tag は release version を基準に生成し、あわせて repository source などの OCI label も付与する。
- Rationale:
  - GHCR 上で release と image の対応を追うには、release version を含む tag が最も直接的である。
  - `docker/metadata-action` は release / tag 情報から tags と labels を一貫生成でき、`build-push-action` にそのまま渡せる。
  - source repository label を付けると GitHub package 画面で repository との関連付けを明確にしやすい。
- Alternatives considered:
  - 手書きで tags を組み立てる: version 取り回しや label の重複管理が増える。
  - `latest` のみ付与する: release 単位の識別要件を満たせない。

## Decision 4: 複数 image 対応は publish target 定義ファイルで拡張可能にする

- Decision:
  - workflow 内に image ごとの固定分岐を増やさず、publish target の一覧を repository 内設定として持つ。
- Rationale:
  - 今回必須なのは `server` だけだが、`comfyui` などの将来追加を見越すなら、build context / Dockerfile / image 名を target 単位で切り出すのが最も変更点を閉じやすい。
  - target が 1 件でも配列構造にしておけば、後続 feature は同じ schema に要素を追加するだけで済む。
  - 「未定義 image を暗黙に publish しない」という spec 要件にも合う。
- Alternatives considered:
  - まず `server` 専用 workflow を作り、後で作り直す: 近い将来の拡張で差し替えコストが出る。
  - いきなり `server` と `comfyui` の両方を publish 対象にする: 今回の forbidden scope を超える。

## Decision 5: 運用確認導線は Releases / Actions / GHCR の 3 点に固定する

- Decision:
  - 運用文書では、release publish 実行後の確認先を GitHub Releases、Actions、GHCR package 画面の 3 つに固定して案内する。
- Rationale:
  - 管理者が「契機」「実行結果」「公開物」をそれぞれ 1 か所ずつ追えれば十分で、追加ダッシュボードは不要。
  - README と quickstart で案内すべき確認場所が明快になり、SC-004 を満たしやすい。
- Alternatives considered:
  - GHCR だけを確認先にする: 失敗時の原因追跡ができない。
  - Actions だけを確認先にする: 公開物の到達確認が弱い。

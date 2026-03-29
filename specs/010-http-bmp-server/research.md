# Research: BMP配信HTTPサーバ

## Decision 1: 最小実装は Rust + `axum` を採用する

- Decision: 最初の HTTP サーバ実装は Rust + `axum` で行う。
- Rationale: 008 で Rust が第一候補、009 で Rust 内候補として `axum` が最終候補に整理されている。今回は単一ファイル配信だけだが、後続で画像前処理や telemetry を追加する可能性があるため、ここで別 stack に寄せずに `axum` で開始する方が後方互換性が高い。最小スコープでも `axum` の導入コストは限定的で、以後の拡張先を固定できる価値がある。
- Alternatives considered:
  - Python の簡易 HTTP サーバを継続する
    - 見送り理由: 既存の `server/run.sh` は最小確認には使えるが、次 feature で Rust 側へ寄せ直す手戻りが発生する。
  - `actix-web`
    - 見送り理由: 009 で第一対抗候補に留まっており、今回の最小サーバを始める起点として `axum` を覆す理由がない。

## Decision 2: 配信元ファイルは `server/contents/image.bmp` に固定する

- Decision: ルート `/` が返す配信元ファイルは `server/contents/image.bmp` とする。
- Rationale: 既存 `server/contents/` は配信素材置き場としてすでに存在し、`.gitignore` で実ファイルをコミットしない構成になっている。今回ユーザーは `image.bmp` を必要になったタイミングで用意するとしているため、サーバ実装はその固定パスだけを見ればよい。これにより、起動方法とファイル配置場所を単純に説明できる。
- Alternatives considered:
  - 実行引数や環境変数でファイルパスを切り替える
    - 見送り理由: 初期スコープでは運用単純性を優先し、設定面は増やさない。

## Decision 3: `GET /` だけを実装し、`image.bmp` 未配置時は失敗応答を返す

- Decision: ルート `/` のみを提供し、`image.bmp` が無い場合は未配置と分かる失敗応答を返す。
- Rationale: PhotoPainter はルート URL で画像取得できればよく、今回の目的もそこに限定されている。ファイルが無い状態で空レスポンスや 200 OK を返すと切り分けが難しいため、失敗応答と短い説明文を返す方が運用しやすい。
- Alternatives considered:
  - 自動でダミー画像を返す
    - 見送り理由: 利用者が画像をまだ用意していない状況を隠してしまい、誤動作に見えやすい。

## Decision 4: `image.bmp` の差し替えは再起動なしで次回配信に反映させる

- Decision: リクエストごとにファイルを読み直し、`image.bmp` の差し替えを次回アクセスから反映させる。
- Rationale: 今回のユースケースは「後から利用者が BMP を置く/差し替える」ことを含む。サーバ内部で長期キャッシュすると、差し替え後に即反映しない挙動になり、最小運用の分かりやすさを損なう。単一ファイル配信であれば毎回読み込みのコストは小さい。
- Alternatives considered:
  - 起動時に一度だけメモリへ読み込む
    - 見送り理由: 差し替え時に再起動が必要になり、SC-003 に反する。

## Decision 5: 起動導線は `server/run.sh` を維持しつつ Rust サーバ実行へ差し替える

- Decision: 利用者向けの起動導線は `server/run.sh` を維持し、その中身を Rust サーバ起動に合わせる。
- Rationale: 既存 repo にはすでに `server/run.sh` があり、利用者視点でも起動入口が一つの方が分かりやすい。新しい Rust バイナリを直接叩かせるより、まずは既存入口を使い回して最小変更で移行するのが自然である。
- Alternatives considered:
  - 新しい起動スクリプトを追加する
    - 見送り理由: 入口が増えて初期導線が複雑になる。

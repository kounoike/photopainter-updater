# Contract: Devcontainer Readiness and Auth Persistence

## Purpose

devcontainer の利用者が、`codex` と `claude` の接続直後利用可否と再生成後の認証保持について同じ期待値で運用できるよう、最小契約を定義する。

## Container Ready Contract

- Trigger:
  - 開発者が新規に devcontainer を build/create して接続する
- Preconditions:
  - `.devcontainer/` の標準構成が適用されている
- Expected Result:
  - `codex` と `claude` が追加セットアップなしで利用可能である
  - 開発者は接続後すぐにコマンド起動確認へ進める

## Auth Persistence Contract

- Trigger:
  - 開発者が一度認証した後に devcontainer を rebuild または recreate する
- Preconditions:
  - 認証情報を保持する永続領域が有効である
  - 保持対象 CLI は `codex` と `claude` である
- Expected Result:
  - 再接続後、`codex` と `claude` は再認証を要求しない
  - 認証情報はワークスペースの Git 管理対象には現れない

## First-Time Login Contract

- Trigger:
  - 永続領域が空の状態で初めて devcontainer を利用する
- Expected Result:
  - 開発者は通常の認証手順で `codex` と `claude` を認証できる
  - 認証完了後の再生成では、その状態が再利用される

## Reset Contract

- Trigger:
  - 開発者が意図的に `codex` または `claude` の認証状態を初期化したい
- Expected Result:
  - 手順書に、初期化対象と永続領域の消去方法が明記されている
  - 初期化後は再度通常の認証手順へ戻れる

## Behavioral Rules

- 認証保持の対象は、`codex` と `claude` の継続利用に必要な最小範囲に限定する。
- 認証情報や関連キャッシュは、リポジトリ配下の Git 管理対象へ保存してはならない。
- 本契約は `.devcontainer` の利用体験を対象とし、アプリケーション本体の build 成果物保持は保証しない。

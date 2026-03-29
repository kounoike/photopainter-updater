#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
contents_dir="${script_dir}/contents"
port="${PORT:-8000}"

if ! command -v cargo >/dev/null 2>&1; then
  echo "ERROR: cargo が見つかりません。Rust toolchain をインストールしてください。" >&2
  exit 1
fi

if [[ ! -d "${contents_dir}" ]]; then
  echo "ERROR: ${contents_dir} が存在しません。" >&2
  exit 1
fi

if [[ ! "${port}" =~ ^[0-9]+$ ]]; then
  echo "ERROR: PORT は数値で指定してください。" >&2
  exit 1
fi

echo "Starting Rust HTTP BMP server with contents in ${contents_dir}"
echo "Serving at http://127.0.0.1:${port}/ and http://127.0.0.1:${port}/image.bmp"
echo "Stop: Ctrl+C"

cd "${script_dir}"
export PORT="${port}"
exec cargo run --release

#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
contents_dir="${script_dir}/contents"
port="${PORT:-8000}"

if ! command -v python3 >/dev/null 2>&1; then
  echo "ERROR: python3 が見つかりません。python3 をインストールしてください。" >&2
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

echo "Serving ${contents_dir} at http://127.0.0.1:${port}/"
echo "Stop: Ctrl+C"

cd "${contents_dir}"
exec python3 -m http.server "${port}"

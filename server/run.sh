#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
default_contents_dir="${script_dir}/contents"
port="${PORT:-8000}"

usage() {
  cat <<'EOF'
Usage: server/run.sh [CONTENT_DIR]

CONTENT_DIR:
  Optional directory containing image.png to transform and serve as image.bmp/image.bin.
  If omitted, server/contents is used.
EOF
}

resolve_dir() {
  local input="$1"
  if [[ "${input}" = /* ]]; then
    (cd -- "${input}" && pwd)
  else
    (cd -- "${input}" && pwd)
  fi
}

detect_lan_ip() {
  if command -v hostname >/dev/null 2>&1; then
    local lan_ip
    lan_ip=$(hostname -I 2>/dev/null | awk '{for (i = 1; i <= NF; i++) if ($i !~ /^127\./) { print $i; exit }}')
    if [[ -n "${lan_ip}" ]]; then
      printf '%s\n' "${lan_ip}"
      return 0
    fi
  fi
  return 1
}

if [[ "${1:-}" = "--help" || "${1:-}" = "-h" ]]; then
  usage
  exit 0
fi

if [[ "$#" -gt 1 ]]; then
  echo "ERROR: 引数は 1 つまでです。" >&2
  usage >&2
  exit 1
fi

requested_contents_dir="${1:-${default_contents_dir}}"

if ! command -v cargo >/dev/null 2>&1; then
  echo "ERROR: cargo が見つかりません。Rust toolchain をインストールしてください。" >&2
  exit 1
fi

if ! contents_dir=$(resolve_dir "${requested_contents_dir}" 2>/dev/null); then
  echo "ERROR: 配信元ディレクトリ ${requested_contents_dir} が存在しません。" >&2
  exit 1
fi

if [[ ! "${port}" =~ ^[0-9]+$ ]]; then
  echo "ERROR: PORT は数値で指定してください。" >&2
  exit 1
fi

echo "Starting Rust HTTP image server on 0.0.0.0:${port} with input image in ${contents_dir}"
echo "Local: http://127.0.0.1:${port}/ , http://127.0.0.1:${port}/image.bmp , http://127.0.0.1:${port}/image.bin"
if lan_ip=$(detect_lan_ip); then
  echo "LAN:   http://${lan_ip}:${port}/ , http://${lan_ip}:${port}/image.bmp , http://${lan_ip}:${port}/image.bin"
else
  echo "LAN:   このホストの LAN IP を使って http://<host-ip>:${port}/ へアクセスしてください"
fi
echo "Routing: firmware uses binary only when image_url ends with .bin; otherwise BMP routes stay unchanged"
echo "Access logs: each request is written as one line to stdout"
echo "Stop: Ctrl+C"

cd "${script_dir}"
export PORT="${port}"
export CONTENT_DIR="${contents_dir}"
exec cargo run --release

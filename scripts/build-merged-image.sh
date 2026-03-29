#!/usr/bin/env bash
set -euo pipefail

# Mirrors the xiaozhi-esp32 release flow, but only for firmware/ and with a
# simple merged-image output suitable for GUI flash tools.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
FIRMWARE_DIR="${REPO_ROOT}/firmware"
BUILD_DIR="${FIRMWARE_DIR}/build"

TARGET="esp32s3"
OUTPUT="${BUILD_DIR}/merged-flash.bin"
SKIP_BUILD=0

usage() {
    cat <<EOF
Usage: $(basename "$0") [options]

Build firmware/ for the selected target and create a merged flash image.

Options:
  --target <chip>   ESP chip target. Default: ${TARGET}
  --output <path>   Output merged image path. Default: ${OUTPUT}
  --skip-build      Reuse existing firmware/build artifacts
  -h, --help        Show this help

Examples:
  $(basename "$0")
  $(basename "$0") --skip-build
  $(basename "$0") --target esp32s3 --output firmware/build/merged-s3.bin
EOF
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --target)
            TARGET="${2:?missing value for --target}"
            shift 2
            ;;
        --output)
            OUTPUT="${2:?missing value for --output}"
            shift 2
            ;;
        --skip-build)
            SKIP_BUILD=1
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown option: $1" >&2
            usage >&2
            exit 2
            ;;
    esac
done

if [ -f /opt/esp/idf/export.sh ]; then
    # shellcheck disable=SC1091
    . /opt/esp/idf/export.sh >/dev/null
fi

command -v python3 >/dev/null 2>&1 || {
    echo "python3 not found" >&2
    exit 1
}
command -v idf.py >/dev/null 2>&1 || {
    echo "idf.py not found. Source ESP-IDF export.sh first." >&2
    exit 1
}

mkdir -p "$(dirname "${OUTPUT}")"

if [ "${SKIP_BUILD}" -eq 0 ]; then
    echo "Setting target to ${TARGET}"
    idf.py -C "${FIRMWARE_DIR}" set-target "${TARGET}"
    echo "Building firmware/"
    idf.py -C "${FIRMWARE_DIR}" build
fi

SDKCONFIG_TARGET="$(python3 - "${FIRMWARE_DIR}/sdkconfig" <<'PY'
import pathlib
import re
import sys

sdkconfig = pathlib.Path(sys.argv[1])
if not sdkconfig.exists():
    print("")
    raise SystemExit(0)

text = sdkconfig.read_text(encoding="utf-8", errors="replace")
match = re.search(r'^CONFIG_IDF_TARGET="([^"]+)"$', text, re.MULTILINE)
print(match.group(1) if match else "")
PY
)"

if [ -z "${SDKCONFIG_TARGET}" ]; then
    echo "Could not determine CONFIG_IDF_TARGET from ${FIRMWARE_DIR}/sdkconfig" >&2
    exit 1
fi

if [ "${SDKCONFIG_TARGET}" != "${TARGET}" ]; then
    echo "sdkconfig target is '${SDKCONFIG_TARGET}', expected '${TARGET}'." >&2
    echo "Run without --skip-build or rebuild with: idf.py -C firmware set-target ${TARGET}" >&2
    exit 1
fi

FLASHER_ARGS_JSON="${BUILD_DIR}/flasher_args.json"
if [ ! -f "${FLASHER_ARGS_JSON}" ]; then
    echo "Missing ${FLASHER_ARGS_JSON}. Build firmware/ first." >&2
    exit 1
fi

python3 - "${FLASHER_ARGS_JSON}" "${BUILD_DIR}" "${TARGET}" "${OUTPUT}" <<'PY'
import json
import pathlib
import shlex
import subprocess
import sys

flasher_args_path = pathlib.Path(sys.argv[1])
build_dir = pathlib.Path(sys.argv[2])
target = sys.argv[3]
output = pathlib.Path(sys.argv[4])

data = json.loads(flasher_args_path.read_text(encoding="utf-8"))
chip = data.get("extra_esptool_args", {}).get("chip")
if chip != target:
    raise SystemExit(
        f"flasher_args.json was generated for chip '{chip}', expected '{target}'. Rebuild firmware/ for the correct target."
    )

cmd = [sys.executable, "-m", "esptool", "--chip", chip, "merge_bin", "-o", str(output.resolve())]
cmd.extend(data.get("write_flash_args", []))

flash_files = data.get("flash_files", {})
for offset, rel_path in sorted(flash_files.items(), key=lambda item: int(item[0], 0)):
    cmd.extend([offset, str((build_dir / rel_path).resolve())])

print("Running:")
print(" ".join(shlex.quote(part) for part in cmd))
subprocess.run(cmd, check=True)
print(f"Merged image written to {output.resolve()}")
PY

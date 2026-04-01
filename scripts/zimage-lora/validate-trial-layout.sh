#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
# shellcheck source=lib/common.sh
source "${SCRIPT_DIR}/lib/common.sh"

usage() {
  cat <<'EOF'
Usage: validate-trial-layout.sh [--env-file PATH] [--character-id ID]
EOF
}

TRIAL_ENV_FILE="${SCRIPT_DIR}/configs/trial-12gb.env"
CHARACTER_ID_OVERRIDE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --env-file)
      TRIAL_ENV_FILE="$2"
      shift 2
      ;;
    --character-id)
      CHARACTER_ID_OVERRIDE="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      zimage_die "unknown argument: $1"
      ;;
  esac
done

TRIAL_ENV_FILE="$(zimage_abspath "${TRIAL_ENV_FILE}")"
zimage_require_cmds bash python docker
zimage_load_env_file "${TRIAL_ENV_FILE}"
zimage_apply_defaults
if [[ -n "${CHARACTER_ID_OVERRIDE}" ]]; then
  export CHARACTER_ID="${CHARACTER_ID_OVERRIDE}"
  export TRAINING_NAME="${TRAINING_NAME:-${CHARACTER_ID}-trial}"
fi
zimage_prepare_dirs
zimage_resolve_training_paths

dataset_dir="${ZLORA_RENDERED_DATASET_DIR}"
[[ -d "${dataset_dir}" ]] || zimage_die "dataset dir not found: ${dataset_dir}"

mapfile -t images < <(find "${dataset_dir}" -maxdepth 1 -type f \( -iname '*.png' -o -iname '*.jpg' -o -iname '*.jpeg' -o -iname '*.webp' \) | sort)
image_count="${#images[@]}"
if [[ "${image_count}" -lt "${ZLORA_MIN_IMAGE_COUNT}" ]]; then
  zimage_die "image count ${image_count} is below minimum ${ZLORA_MIN_IMAGE_COUNT}"
fi

missing_captions=0
for image in "${images[@]}"; do
  caption="${image%.*}.txt"
  if [[ ! -f "${caption}" ]]; then
    zimage_warn "missing caption: ${caption}"
    missing_captions=1
    continue
  fi
  if [[ ! -s "${caption}" ]]; then
    zimage_warn "empty caption: ${caption}"
    missing_captions=1
  fi
done
if [[ "${missing_captions}" -ne 0 ]]; then
  zimage_die "caption validation failed"
fi

python - "${SCRIPT_DIR}/configs/trial-12gb.json" "${SCRIPT_DIR}/configs/multidatabackend.trial.json" <<'PY'
import json
import sys
for path in sys.argv[1:]:
    with open(path, "r", encoding="utf-8") as fh:
        json.load(fh)
PY

zimage_log "layout validation passed"
zimage_emit_runtime_summary

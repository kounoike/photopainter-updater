#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
# shellcheck source=lib/common.sh
source "${SCRIPT_DIR}/lib/common.sh"

usage() {
  cat <<'EOF'
Usage: train-trial.sh [--env-file PATH] [--character-id ID] [--build-only] [--prepare-only] [--skip-build]
EOF
}

TRIAL_ENV_FILE="${SCRIPT_DIR}/configs/trial-12gb.env"
CHARACTER_ID_OVERRIDE=""
BUILD_ONLY=0
PREPARE_ONLY=0
SKIP_BUILD=0

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
    --build-only)
      BUILD_ONLY=1
      shift
      ;;
    --prepare-only)
      PREPARE_ONLY=1
      shift
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
      zimage_die "unknown argument: $1"
      ;;
  esac
done

TRIAL_ENV_FILE="$(zimage_abspath "${TRIAL_ENV_FILE}")"
zimage_require_cmds bash python docker git
zimage_load_env_file "${TRIAL_ENV_FILE}"
zimage_apply_defaults
if [[ -n "${CHARACTER_ID_OVERRIDE}" ]]; then
  export CHARACTER_ID="${CHARACTER_ID_OVERRIDE}"
  export TRAINING_NAME="${CHARACTER_ID}-trial"
fi
zimage_prepare_dirs
zimage_resolve_training_paths

mkdir -p "${ZLORA_RENDERED_RUNTIME_DIR}" "${ZLORA_RENDERED_OUTPUT_DIR}"
zimage_render_prompt_library "${SCRIPT_DIR}/templates/validation-prompts.txt" "${ZLORA_RENDERED_PROMPT_LIBRARY}"
zimage_render_json_template "${SCRIPT_DIR}/configs/multidatabackend.trial.json" "${ZLORA_RENDERED_DATALOADER_CONFIG}"
zimage_render_json_template "${SCRIPT_DIR}/configs/trial-12gb.json" "${ZLORA_RENDERED_TRAINER_CONFIG}"
cp "${TRIAL_ENV_FILE}" "${ZLORA_RENDERED_RUNTIME_DIR}/trial.env.snapshot"

if [[ "${PREPARE_ONLY}" -eq 1 ]]; then
  zimage_log "prepared runtime files only"
  zimage_emit_runtime_summary
  exit 0
fi

if [[ "${SKIP_BUILD}" -eq 0 ]]; then
  zimage_compose build simpletuner
fi

if [[ "${BUILD_ONLY}" -eq 1 ]]; then
  zimage_log "docker image build completed"
  exit 0
fi

zimage_log "starting trial training"
zimage_emit_runtime_summary
zimage_compose run --rm simpletuner \
  bash -lc "simpletuner train --config '${ZLORA_RENDERED_TRAINER_CONFIG}'"

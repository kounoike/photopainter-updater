#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
# shellcheck source=lib/common.sh
source "${SCRIPT_DIR}/lib/common.sh"

usage() {
  cat <<'EOF'
Usage: validate-reuse.sh --artifact PATH [--env-file PATH] [--prompt-file PATH] [--output-dir PATH]
EOF
}

TRIAL_ENV_FILE="${SCRIPT_DIR}/configs/trial-12gb.env"
ARTIFACT_PATH=""
PROMPT_FILE="${SCRIPT_DIR}/templates/validation-prompts.txt"
OUTPUT_DIR=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --env-file)
      TRIAL_ENV_FILE="$2"
      shift 2
      ;;
    --artifact)
      ARTIFACT_PATH="$2"
      shift 2
      ;;
    --prompt-file)
      PROMPT_FILE="$2"
      shift 2
      ;;
    --output-dir)
      OUTPUT_DIR="$2"
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

[[ -n "${ARTIFACT_PATH}" ]] || zimage_die "--artifact is required"
TRIAL_ENV_FILE="$(zimage_abspath "${TRIAL_ENV_FILE}")"
ARTIFACT_PATH="$(zimage_abspath "${ARTIFACT_PATH}")"
PROMPT_FILE="$(zimage_abspath "${PROMPT_FILE}")"

zimage_require_cmds bash python
zimage_load_env_file "${TRIAL_ENV_FILE}"
zimage_apply_defaults
zimage_prepare_dirs
zimage_resolve_training_paths

[[ -f "${ARTIFACT_PATH}" ]] || zimage_die "artifact not found: ${ARTIFACT_PATH}"
[[ -f "${PROMPT_FILE}" ]] || zimage_die "prompt file not found: ${PROMPT_FILE}"

RUN_DIR="${ZLORA_REUSE_ROOT}/${TRAINING_NAME}"
OUTPUT_DIR="${OUTPUT_DIR:-${RUN_DIR}/output}"
mkdir -p "${RUN_DIR}" "${OUTPUT_DIR}"
export ARTIFACT_PATH OUTPUT_DIR RUN_DIR

export ZLORA_RENDERED_PROMPT_LIBRARY="${RUN_DIR}/user_prompt_library.json"
zimage_render_prompt_library "${PROMPT_FILE}" "${ZLORA_RENDERED_PROMPT_LIBRARY}"

MANIFEST_PATH="${RUN_DIR}/reuse-manifest.json"
python - "${MANIFEST_PATH}" <<'PY'
import json
import os
import sys

payload = {
    "training_name": os.environ["TRAINING_NAME"],
    "artifact_path": os.environ["ARTIFACT_PATH"],
    "prompt_library": os.environ["ZLORA_RENDERED_PROMPT_LIBRARY"],
    "output_dir": os.environ["OUTPUT_DIR"],
    "validate_reuse_command": os.environ.get("VALIDATE_REUSE_COMMAND", ""),
}
with open(sys.argv[1], "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2, ensure_ascii=False)
    fh.write("\n")
PY

if [[ -z "${VALIDATE_REUSE_COMMAND}" ]]; then
  cat <<EOF >"${RUN_DIR}/run-command.txt"
Set VALIDATE_REUSE_COMMAND in your env file to run existing local Z-Image inference automatically.

Available placeholders:
  __ARTIFACT__
  __PROMPT_LIBRARY__
  __OUTPUT_DIR__
  __RUN_DIR__
  __TRAINING_NAME__
EOF
  zimage_log "reuse manifest prepared at ${MANIFEST_PATH}"
  zimage_warn "VALIDATE_REUSE_COMMAND is empty; skipping inference execution"
  exit 0
fi

RENDERED_COMMAND="$(python - <<'PY'
import os
cmd = os.environ["VALIDATE_REUSE_COMMAND"]
for old, new in {
    "__ARTIFACT__": os.environ["ARTIFACT_PATH"],
    "__PROMPT_LIBRARY__": os.environ["ZLORA_RENDERED_PROMPT_LIBRARY"],
    "__OUTPUT_DIR__": os.environ["OUTPUT_DIR"],
    "__RUN_DIR__": os.environ["RUN_DIR"],
    "__TRAINING_NAME__": os.environ["TRAINING_NAME"],
}.items():
    cmd = cmd.replace(old, new)
print(cmd)
PY
)"

zimage_log "executing reuse command"
bash -lc "${RENDERED_COMMAND}"

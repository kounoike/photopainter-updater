#!/usr/bin/env bash
set -euo pipefail

zimage_repo_root() {
  git -C "${BASH_SOURCE[0]%/*}" rev-parse --show-toplevel
}

zimage_script_dir() {
  local source_path
  source_path="${BASH_SOURCE[0]}"
  cd "$(dirname "${source_path}")/.." >/dev/null 2>&1 && pwd
}

zimage_log() {
  printf '[zimage-lora] %s\n' "$*"
}

zimage_warn() {
  printf '[zimage-lora][warn] %s\n' "$*" >&2
}

zimage_die() {
  printf '[zimage-lora][error] %s\n' "$*" >&2
  exit 1
}

zimage_require_cmds() {
  local missing=0
  local cmd
  for cmd in "$@"; do
    if ! command -v "${cmd}" >/dev/null 2>&1; then
      zimage_warn "missing command: ${cmd}"
      missing=1
    fi
  done
  if [[ "${missing}" -ne 0 ]]; then
    zimage_die "required commands are missing"
  fi
}

zimage_abspath() {
  python - "$1" <<'PY'
from pathlib import Path
import sys
print(Path(sys.argv[1]).expanduser().resolve())
PY
}

zimage_load_env_file() {
  local env_file="$1"
  if [[ ! -f "${env_file}" ]]; then
    zimage_die "env file not found: ${env_file}"
  fi
  set -a
  # shellcheck source=/dev/null
  source "${env_file}"
  set +a
}

zimage_apply_defaults() {
  export REPO_ROOT="${REPO_ROOT:-$(zimage_repo_root)}"
  export ZLORA_DIR="${ZLORA_DIR:-${REPO_ROOT}/scripts/zimage-lora}"
  export ZLORA_WORK_ROOT="${ZLORA_WORK_ROOT:-${ZLORA_DIR}/workspace}"
  export ZLORA_DATASET_ROOT="${ZLORA_DATASET_ROOT:-${ZLORA_WORK_ROOT}/datasets}"
  export ZLORA_RUNTIME_ROOT="${ZLORA_RUNTIME_ROOT:-${ZLORA_WORK_ROOT}/runtime}"
  export ZLORA_OUTPUT_ROOT="${ZLORA_OUTPUT_ROOT:-${ZLORA_WORK_ROOT}/output}"
  export ZLORA_CACHE_ROOT="${ZLORA_CACHE_ROOT:-${ZLORA_WORK_ROOT}/cache}"
  export ZLORA_HF_HOME="${ZLORA_HF_HOME:-${ZLORA_WORK_ROOT}/huggingface}"
  export ZLORA_REUSE_ROOT="${ZLORA_REUSE_ROOT:-${ZLORA_WORK_ROOT}/reuse}"
  export ZLORA_LOG_ROOT="${ZLORA_LOG_ROOT:-${ZLORA_WORK_ROOT}/logs}"
  export ZLORA_IMAGE_NAME="${ZLORA_IMAGE_NAME:-photopainter-simpletuner:local}"
  export ZLORA_PYTHON_VERSION="${ZLORA_PYTHON_VERSION:-3.12}"
  export SIMPLETUNER_BRANCH="${SIMPLETUNER_BRANCH:-release}"
  export CHARACTER_ID="${CHARACTER_ID:-sample-character}"
  export TRAINING_NAME="${TRAINING_NAME:-${CHARACTER_ID}-trial}"
  export ZLORA_COMPOSE_PROJECT_NAME="${ZLORA_COMPOSE_PROJECT_NAME:-photopainter-zlora}"
  export ZLORA_SHM_SIZE="${ZLORA_SHM_SIZE:-16gb}"
  export ZLORA_DATASET_REPEATS="${ZLORA_DATASET_REPEATS:-20}"
  export ZLORA_MIN_IMAGE_COUNT="${ZLORA_MIN_IMAGE_COUNT:-8}"
  export ZLORA_PRIMARY_VALIDATION_PROMPT="${ZLORA_PRIMARY_VALIDATION_PROMPT:-portrait of ${CHARACTER_ID}, same character identity, clean anime illustration}"
  export ZLORA_FORCE_INT8="${ZLORA_FORCE_INT8:-0}"
  export ZLORA_FORCE_FP16="${ZLORA_FORCE_FP16:-0}"
  export ZLORA_DISABLE_GROUP_OFFLOAD="${ZLORA_DISABLE_GROUP_OFFLOAD:-0}"
  export ZLORA_MAX_TRAIN_STEPS="${ZLORA_MAX_TRAIN_STEPS:-400}"
  export VALIDATE_REUSE_COMMAND="${VALIDATE_REUSE_COMMAND:-}"
  zimage_normalize_paths
}

zimage_normalize_paths() {
  local key value
  for key in \
    ZLORA_DIR \
    ZLORA_WORK_ROOT \
    ZLORA_DATASET_ROOT \
    ZLORA_RUNTIME_ROOT \
    ZLORA_OUTPUT_ROOT \
    ZLORA_CACHE_ROOT \
    ZLORA_HF_HOME \
    ZLORA_REUSE_ROOT \
    ZLORA_LOG_ROOT; do
    value="${!key}"
    case "${value}" in
      /*) ;;
      *)
        export "${key}=${REPO_ROOT}/${value#./}"
        ;;
    esac
  done
}

zimage_prepare_dirs() {
  mkdir -p \
    "${ZLORA_WORK_ROOT}" \
    "${ZLORA_DATASET_ROOT}" \
    "${ZLORA_RUNTIME_ROOT}" \
    "${ZLORA_OUTPUT_ROOT}" \
    "${ZLORA_CACHE_ROOT}" \
    "${ZLORA_HF_HOME}" \
    "${ZLORA_REUSE_ROOT}" \
    "${ZLORA_LOG_ROOT}"
}

zimage_compose_file() {
  printf '%s\n' "${ZLORA_DIR}/docker-compose.yml"
}

zimage_compose() {
  docker compose \
    --project-name "${ZLORA_COMPOSE_PROJECT_NAME}" \
    -f "$(zimage_compose_file)" \
    "$@"
}

zimage_render_json_template() {
  local template_path="$1"
  local output_path="$2"
  python - "${template_path}" "${output_path}" <<'PY'
import json
import os
import sys
from pathlib import Path

template_path = Path(sys.argv[1])
output_path = Path(sys.argv[2])
payload = json.loads(template_path.read_text())

replacements = {
    "__DATA_BACKEND_CONFIG__": os.environ["ZLORA_RENDERED_DATALOADER_CONFIG"],
    "__OUTPUT_DIR__": os.environ["ZLORA_RENDERED_OUTPUT_DIR"],
    "__VALIDATION_PROMPT__": os.environ["ZLORA_PRIMARY_VALIDATION_PROMPT"],
    "__VALIDATION_PROMPT_LIBRARY__": os.environ["ZLORA_RENDERED_PROMPT_LIBRARY"],
    "__TRAINING_NAME__": os.environ["TRAINING_NAME"],
    "__DATASET_DIR__": os.environ["ZLORA_RENDERED_DATASET_DIR"],
    "__TEXT_EMBED_CACHE_DIR__": os.environ["ZLORA_RENDERED_TEXT_CACHE_DIR"],
    "__VAE_CACHE_DIR__": os.environ["ZLORA_RENDERED_VAE_CACHE_DIR"],
    "__DATASET_REPEATS__": os.environ["ZLORA_DATASET_REPEATS"],
    "__CHARACTER_ID__": os.environ["CHARACTER_ID"],
    "__MAX_TRAIN_STEPS__": os.environ["ZLORA_MAX_TRAIN_STEPS"],
}

def replace(obj):
    if isinstance(obj, dict):
      return {key: replace(value) for key, value in obj.items()}
    if isinstance(obj, list):
      return [replace(value) for value in obj]
    if isinstance(obj, str):
      for old, new in replacements.items():
        obj = obj.replace(old, new)
      if obj.isdigit():
        return int(obj)
      return obj
    return obj

rendered = replace(payload)

if os.environ.get("ZLORA_FORCE_INT8") == "1":
    rendered["base_model_precision"] = "int8-torchao"
if os.environ.get("ZLORA_FORCE_FP16") == "1":
    rendered["mixed_precision"] = "fp16"
if os.environ.get("ZLORA_DISABLE_GROUP_OFFLOAD") == "1":
    rendered["enable_group_offload"] = False
    rendered["group_offload_use_stream"] = False

output_path.parent.mkdir(parents=True, exist_ok=True)
output_path.write_text(json.dumps(rendered, indent=2, ensure_ascii=False) + "\n")
PY
}

zimage_render_prompt_library() {
  local template_path="$1"
  local output_path="$2"
  python - "${template_path}" "${output_path}" <<'PY'
import json
import os
import sys
from pathlib import Path

template_path = Path(sys.argv[1])
output_path = Path(sys.argv[2])
character_id = os.environ["CHARACTER_ID"]
prompts = {}
for raw_line in template_path.read_text().splitlines():
    line = raw_line.strip()
    if not line or line.startswith("#"):
        continue
    name, prompt = line.split("\t", 1)
    prompts[name] = prompt.replace("__CHARACTER_ID__", character_id)

output_path.parent.mkdir(parents=True, exist_ok=True)
output_path.write_text(json.dumps(prompts, indent=2, ensure_ascii=False) + "\n")
PY
}

zimage_resolve_training_paths() {
  export ZLORA_RENDERED_DATASET_DIR="${ZLORA_DATASET_ROOT}/${CHARACTER_ID}/train"
  export ZLORA_RENDERED_RUNTIME_DIR="${ZLORA_RUNTIME_ROOT}/${TRAINING_NAME}"
  export ZLORA_RENDERED_OUTPUT_DIR="${ZLORA_OUTPUT_ROOT}/${TRAINING_NAME}"
  export ZLORA_RENDERED_TEXT_CACHE_DIR="${ZLORA_CACHE_ROOT}/text/z_image/${CHARACTER_ID}"
  export ZLORA_RENDERED_VAE_CACHE_DIR="${ZLORA_CACHE_ROOT}/vae/z_image/${CHARACTER_ID}"
  export ZLORA_RENDERED_PROMPT_LIBRARY="${ZLORA_RENDERED_RUNTIME_DIR}/user_prompt_library.json"
  export ZLORA_RENDERED_DATALOADER_CONFIG="${ZLORA_RENDERED_RUNTIME_DIR}/multidatabackend.json"
  export ZLORA_RENDERED_TRAINER_CONFIG="${ZLORA_RENDERED_RUNTIME_DIR}/config.json"
}

zimage_emit_runtime_summary() {
  cat <<EOF
env_file: ${TRIAL_ENV_FILE}
character_id: ${CHARACTER_ID}
training_name: ${TRAINING_NAME}
dataset_dir: ${ZLORA_RENDERED_DATASET_DIR}
runtime_dir: ${ZLORA_RENDERED_RUNTIME_DIR}
output_dir: ${ZLORA_RENDERED_OUTPUT_DIR}
EOF
}

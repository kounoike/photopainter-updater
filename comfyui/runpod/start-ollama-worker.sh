#!/usr/bin/env bash
set -euo pipefail

readonly DEFAULT_OLLAMA_HOST="127.0.0.1:11434"
readonly DEFAULT_PERSISTENT_MODELS_DIR="/runpod-volume/ollama/models"
readonly DEFAULT_EPHEMERAL_MODELS_DIR="/tmp/ollama/models"
readonly DEFAULT_START_SCRIPT="/start.sh"
readonly DEFAULT_HEALTHCHECK_URL="http://127.0.0.1:11434/api/version"
readonly DEFAULT_MAX_WAIT_SECONDS="60"

trim() {
  local value="$1"
  value="${value#"${value%%[![:space:]]*}"}"
  value="${value%"${value##*[![:space:]]}"}"
  printf '%s' "$value"
}

log_info() {
  printf '[runpod-ollama] %s\n' "$*"
}

log_warn() {
  printf '[runpod-ollama][warn] %s\n' "$*" >&2
}

resolve_models_dir() {
  local persistent_root="${RUNPOD_OLLAMA_MODELS_DIR:-$DEFAULT_PERSISTENT_MODELS_DIR}"
  local ephemeral_root="${EPHEMERAL_OLLAMA_MODELS_DIR:-$DEFAULT_EPHEMERAL_MODELS_DIR}"

  if [ -d "/runpod-volume" ] && [ -w "/runpod-volume" ]; then
    mkdir -p "$persistent_root"
    RUNTIME_MODE="persistent"
    OLLAMA_MODELS="$persistent_root"
  else
    mkdir -p "$ephemeral_root"
    RUNTIME_MODE="ephemeral"
    OLLAMA_MODELS="$ephemeral_root"
  fi

  export OLLAMA_MODELS
  log_info "runtime_mode=${RUNTIME_MODE} models_dir=${OLLAMA_MODELS}"

  if [ "$RUNTIME_MODE" = "ephemeral" ]; then
    log_warn "RunPod Network Volume is unavailable; using ephemeral model storage"
  fi
}

wait_for_ollama() {
  local healthcheck_url="${OLLAMA_HEALTHCHECK_URL:-$DEFAULT_HEALTHCHECK_URL}"
  local max_wait="${OLLAMA_START_TIMEOUT_SECONDS:-$DEFAULT_MAX_WAIT_SECONDS}"
  local elapsed=0

  until curl -fsS "$healthcheck_url" >/dev/null 2>&1; do
    elapsed=$((elapsed + 1))
    if [ "$elapsed" -ge "$max_wait" ]; then
      log_warn "Ollama API did not become ready within ${max_wait}s"
      return 1
    fi
    sleep 1
  done

  log_info "ollama_api_ready url=${healthcheck_url}"
}

pull_models() {
  local raw_models="${OLLAMA_PULL_MODELS:-}"
  local normalized=""
  local model

  if [ -z "$(trim "$raw_models")" ]; then
    log_info "no pull models configured"
    return 0
  fi

  IFS=',' read -r -a models <<< "$raw_models"
  for model in "${models[@]}"; do
    model="$(trim "$model")"
    if [ -z "$model" ]; then
      continue
    fi
    case ",${normalized}," in
      *,"${model}",*)
        continue
        ;;
      *)
        normalized="${normalized:+${normalized},}${model}"
        ;;
    esac
  done

  if [ -z "$normalized" ]; then
    log_info "pull model list resolved to empty after trimming"
    return 0
  fi

  IFS=',' read -r -a deduped_models <<< "$normalized"
  for model in "${deduped_models[@]}"; do
    if ollama show "$model" >/dev/null 2>&1; then
      log_info "model_result model=${model} result=reused storage_mode=${RUNTIME_MODE}"
      continue
    fi

    if ollama pull "$model"; then
      log_info "model_result model=${model} result=pulled storage_mode=${RUNTIME_MODE}"
    else
      log_warn "model_result model=${model} result=failed storage_mode=${RUNTIME_MODE}"
    fi
  done
}

main() {
  local start_script="${RUNPOD_START_SCRIPT:-$DEFAULT_START_SCRIPT}"
  local ollama_host="${OLLAMA_HOST:-$DEFAULT_OLLAMA_HOST}"

  if [ ! -x "$start_script" ]; then
    log_warn "upstream start script is not executable: ${start_script}"
    return 1
  fi

  export OLLAMA_HOST="$ollama_host"
  resolve_models_dir

  ollama serve >/tmp/ollama.log 2>&1 &
  OLLAMA_PID=$!
  trap 'kill "$OLLAMA_PID" >/dev/null 2>&1 || true' EXIT

  wait_for_ollama
  pull_models

  log_info "delegating to upstream start script: ${start_script}"
  trap - EXIT
  exec "$start_script"
}

main "$@"

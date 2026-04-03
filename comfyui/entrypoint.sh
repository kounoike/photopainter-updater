#!/usr/bin/env bash
set -euo pipefail

cd /root/ComfyUI

COMFYUI_MODELS_DIR="/root/ComfyUI/models"
MODEL_ROOT_VALUE="${MODEL_ROOT:-$COMFYUI_MODELS_DIR}"

mkdir -p \
  /root/.cache \
  /root/.local \
  /root/ComfyUI/custom_nodes \
  /root/ComfyUI/input \
  "$COMFYUI_MODELS_DIR" \
  /root/ComfyUI/output \
  /root/ComfyUI/user

if [ "$MODEL_ROOT_VALUE" != "$COMFYUI_MODELS_DIR" ]; then
  mkdir -p "$MODEL_ROOT_VALUE"

  # RunPod では /runpod-volume/models の実体を ComfyUI 既定パスへ見せる。
  find "$MODEL_ROOT_VALUE" -mindepth 1 -maxdepth 1 | while read -r source_path; do
    target_path="$COMFYUI_MODELS_DIR/$(basename "$source_path")"
    if [ -e "$target_path" ] || [ -L "$target_path" ]; then
      continue
    fi
    ln -s "$source_path" "$target_path"
  done
fi

CLI_ARGS_DEFAULT="--listen 0.0.0.0 --fast --enable-manager"
CLI_ARGS_VALUE="${CLI_ARGS:-$CLI_ARGS_DEFAULT}"

# shellcheck disable=SC2086
exec python main.py ${CLI_ARGS_VALUE}

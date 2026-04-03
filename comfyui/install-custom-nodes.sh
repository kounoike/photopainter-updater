#!/usr/bin/env bash
set -euo pipefail

: "${COMFYUI_HOME:?COMFYUI_HOME is required}"
: "${COMFYUI_MANAGER_REF:?COMFYUI_MANAGER_REF is required}"
: "${COMFYUI_EASY_USE_REF:?COMFYUI_EASY_USE_REF is required}"
: "${COMFYUI_OLLAMA_NODE_REF:?COMFYUI_OLLAMA_NODE_REF is required}"
: "${COMFYUI_XZ3R0_NODES_REF:?COMFYUI_XZ3R0_NODES_REF is required}"

CUSTOM_NODES_DIR="${COMFYUI_HOME}/custom_nodes"

mkdir -p "${CUSTOM_NODES_DIR}"

download_node_archive() {
  local repo_slug="$1"
  local ref_type="$2"
  local ref="$3"
  local target_dir="$4"
  local archive_url

  case "${ref_type}" in
    tag)
      archive_url="https://codeload.github.com/${repo_slug}/tar.gz/refs/tags/${ref}"
      ;;
    commit)
      archive_url="https://codeload.github.com/${repo_slug}/tar.gz/${ref}"
      ;;
    *)
      echo "Unsupported ref type: ${ref_type}" >&2
      exit 1
      ;;
  esac

  rm -rf "${target_dir}"
  mkdir -p "${target_dir}"

  echo "Installing ${repo_slug}@${ref_type}:${ref}"
  curl -fsSL "${archive_url}" | tar -xzf - --strip-components=1 -C "${target_dir}"
}

install_requirements_if_present() {
  local node_dir="$1"

  if [ -f "${node_dir}/requirements.txt" ]; then
    echo "Installing Python dependencies from ${node_dir}/requirements.txt"
    uv pip install --system -r "${node_dir}/requirements.txt"
  fi
}

# download_node_archive "Comfy-Org/ComfyUI-Manager" "tag" "${COMFYUI_MANAGER_REF}" "${CUSTOM_NODES_DIR}/comfyui-manager"
# install_requirements_if_present "${CUSTOM_NODES_DIR}/comfyui-manager"

download_node_archive "yolain/ComfyUI-Easy-Use" "tag" "${COMFYUI_EASY_USE_REF}" "${CUSTOM_NODES_DIR}/ComfyUI-Easy-Use"
install_requirements_if_present "${CUSTOM_NODES_DIR}/ComfyUI-Easy-Use"

download_node_archive "stavsap/comfyui-ollama" "commit" "${COMFYUI_OLLAMA_NODE_REF}" "${CUSTOM_NODES_DIR}/comfyui-ollama"
install_requirements_if_present "${CUSTOM_NODES_DIR}/comfyui-ollama"

download_node_archive "Xz3r0-M/ComfyUI-Xz3r0-Nodes" "tag" "${COMFYUI_XZ3R0_NODES_REF}" "${CUSTOM_NODES_DIR}/ComfyUI-Xz3r0-Nodes"
install_requirements_if_present "${CUSTOM_NODES_DIR}/ComfyUI-Xz3r0-Nodes"

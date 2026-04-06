# syntax=docker/dockerfile:1.7

ARG RUNPOD_WORKER_COMFYUI_IMAGE=runpod/worker-comfyui:5.8.5-base-cuda12.8.1
FROM ${RUNPOD_WORKER_COMFYUI_IMAGE}

ENV COMFYUI_HOME=/comfyui \
    OLLAMA_HOST=127.0.0.1:11434

USER root

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      bash \
      ca-certificates \
      curl \
      ffmpeg \
      zstd && \
    rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://ollama.com/install.sh | sh

ARG COMFYUI_CUSTOM_NODES="https://github.com/yolain/ComfyUI-Easy-Use,https://github.com/stavsap/comfyui-ollama,https://github.com/StableLlama/ComfyUI-basic_data_handling"

COPY runpod/start-ollama-worker.sh /usr/local/bin/start-ollama-worker
COPY custom_node/comfyui-photopainter-custom /comfyui/custom_nodes/comfyui-photopainter-custom

RUN mkdir -p /comfyui/custom_nodes && \
    if [ -n "${COMFYUI_CUSTOM_NODES}" ]; then \
      # customization.md に合わせて、追加 custom node は comfy-node-install へ渡す
      CUSTOM_NODES_ARGS="$(printf '%s' "${COMFYUI_CUSTOM_NODES}" | tr ',' ' ')"; \
      # shellcheck disable=SC2086
      comfy-node-install ${CUSTOM_NODES_ARGS}; \
    fi && \
    chmod +x /usr/local/bin/start-ollama-worker

ENTRYPOINT ["/usr/local/bin/start-ollama-worker"]

# syntax=docker/dockerfile:1.7

ARG RUNPOD_WORKER_COMFYUI_IMAGE=runpod/worker-comfyui:5.7.1-base-cuda12.8.1
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

COPY runpod/start-ollama-worker.sh /usr/local/bin/start-ollama-worker
COPY custom_node/comfyui-photopainter-custom /comfyui/custom_nodes/comfyui-photopainter-custom

RUN comfy-node-install comfyui-ollama && \
    chmod +x /usr/local/bin/start-ollama-worker

CMD ["/usr/local/bin/start-ollama-worker"]

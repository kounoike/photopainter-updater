# syntax=docker/dockerfile:1.7

FROM python:3.13-slim-bookworm

ARG COMFYUI_REPO=https://github.com/Comfy-Org/ComfyUI.git
ARG COMFYUI_REF=v0.18.2
ARG UV_VERSION=0.11.3
ARG UV_TORCH_BACKEND=cu128
ARG COMFYUI_MANAGER_REF=4.1
ARG COMFYUI_EASY_USE_REF=v1.3.6
ARG COMFYUI_OLLAMA_NODE_REF=6db7560576e5a59488708e6be13e07b5aba2432a
ARG COMFYUI_XZ3R0_NODES_REF=v1.7.0

ENV PYTHONUNBUFFERED=1 \
    PIP_NO_CACHE_DIR=1 \
    UV_SYSTEM_PYTHON=1 \
    COMFYUI_HOME=/root/ComfyUI \
    UV_TORCH_BACKEND=${UV_TORCH_BACKEND} \
    COMFYUI_MANAGER_REF=${COMFYUI_MANAGER_REF} \
    COMFYUI_EASY_USE_REF=${COMFYUI_EASY_USE_REF} \
    COMFYUI_OLLAMA_NODE_REF=${COMFYUI_OLLAMA_NODE_REF} \
    COMFYUI_XZ3R0_NODES_REF=${COMFYUI_XZ3R0_NODES_REF}

WORKDIR /root

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      build-essential \
      ca-certificates \
      cmake \
      curl \
      ffmpeg \
      git \
      libglib2.0-0 \
      libgl1 \
      libgomp1 && \
    rm -rf /var/lib/apt/lists/*

ADD https://astral.sh/uv/${UV_VERSION}/install.sh /tmp/uv-installer.sh

RUN sh /tmp/uv-installer.sh && \
    mv /root/.local/bin/uv /usr/local/bin/uv && \
    rm -rf /root/.local /tmp/uv-installer.sh

RUN git clone --branch "${COMFYUI_REF}" --depth 1 "${COMFYUI_REPO}" "${COMFYUI_HOME}"

WORKDIR ${COMFYUI_HOME}

COPY custom_node/comfyui-photopainter-custom ${COMFYUI_HOME}/custom_nodes/comfyui-photopainter-custom
COPY install-custom-nodes.sh /usr/local/bin/install-custom-nodes
COPY entrypoint.sh /usr/local/bin/photopainter-comfyui-entrypoint

RUN --mount=type=cache,target=/root/.cache/uv \
    uv pip install --system \
      torch \
      torchvision \
      torchaudio \
      --torch-backend "${UV_TORCH_BACKEND}" && \
    uv pip install --system -r requirements.txt && \
    uv pip install --system -r manager_requirements.txt && \
    uv pip install --system transformers jsonschema && \
    CMAKE_ARGS="-DGGML_CUDA=on" FORCE_CMAKE=1 uv pip install --system llama-cpp-python && \
    chmod +x /usr/local/bin/install-custom-nodes /usr/local/bin/photopainter-comfyui-entrypoint && \
    /usr/local/bin/install-custom-nodes

RUN mkdir -p \
      /root/.cache \
      /root/.local \
      ${COMFYUI_HOME}/custom_nodes \
      ${COMFYUI_HOME}/input \
      ${COMFYUI_HOME}/models \
      ${COMFYUI_HOME}/output \
      ${COMFYUI_HOME}/user

EXPOSE 8188

ENTRYPOINT ["photopainter-comfyui-entrypoint"]

# syntax=docker/dockerfile:1.7

FROM python:3.13-slim-bookworm

ARG COMFYUI_REPO=https://github.com/Comfy-Org/ComfyUI.git
ARG COMFYUI_REF=v0.18.2
ARG UV_VERSION=0.11.3
ARG UV_TORCH_BACKEND=cu128

ENV PYTHONUNBUFFERED=1 \
    PIP_NO_CACHE_DIR=1 \
    UV_SYSTEM_PYTHON=1 \
    COMFYUI_HOME=/root/ComfyUI \
    UV_TORCH_BACKEND=${UV_TORCH_BACKEND}

WORKDIR /root

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      ca-certificates \
      curl \
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

RUN --mount=type=cache,target=/root/.cache/uv \
    uv pip install --system \
      torch \
      torchvision \
      torchaudio \
      --torch-backend "${UV_TORCH_BACKEND}" && \
    uv pip install --system -r requirements.txt && \
    uv pip install --system -r manager_requirements.txt

COPY entrypoint.sh /usr/local/bin/photopainter-comfyui-entrypoint

RUN chmod +x /usr/local/bin/photopainter-comfyui-entrypoint && \
    mkdir -p \
      /root/.cache \
      /root/.local \
      ${COMFYUI_HOME}/custom_nodes \
      ${COMFYUI_HOME}/input \
      ${COMFYUI_HOME}/models \
      ${COMFYUI_HOME}/output \
      ${COMFYUI_HOME}/user

EXPOSE 8188

ENTRYPOINT ["photopainter-comfyui-entrypoint"]

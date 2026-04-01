FROM nvidia/cuda:12.8.1-cudnn-devel-ubuntu24.04

ARG PYTHON_VERSION=3.12
ARG SIMPLETUNER_BRANCH=release

ENV DEBIAN_FRONTEND=noninteractive
ENV PATH="/opt/venv/bin:${PATH}"
ENV HF_HOME=/workspace/huggingface

RUN apt-get update -y && apt-get install -y --no-install-recommends \
    build-essential \
    git \
    git-lfs \
    curl \
    python${PYTHON_VERSION} \
    python${PYTHON_VERSION}-dev \
    python${PYTHON_VERSION}-venv \
    ffmpeg \
    libsm6 \
    libxext6 \
    && rm -rf /var/lib/apt/lists/* \
    && git lfs install

RUN python${PYTHON_VERSION} -m venv /opt/venv \
    && pip install --upgrade pip setuptools wheel \
    && pip install --no-cache-dir "huggingface_hub[cli,hf_transfer]" wandb ninja mpi4py

RUN git clone https://github.com/bghira/SimpleTuner --branch "${SIMPLETUNER_BRANCH}" /opt/SimpleTuner \
    && pip install --no-cache-dir -e /opt/SimpleTuner[cuda]

WORKDIR /workspace
CMD ["bash"]

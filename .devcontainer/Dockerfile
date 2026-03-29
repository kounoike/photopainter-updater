FROM mcr.microsoft.com/devcontainers/base:ubuntu

ARG ESP_IDF_VERSION=v5.5.1
ARG ESP_IDF_TARGETS=esp32,esp32s3
ARG ESP_ROOT=/opt/esp
ARG NODE_MAJOR=22

ENV IDF_PATH=${ESP_ROOT}/idf
ENV IDF_TOOLS_PATH=${ESP_ROOT}
ENV ESP_IDF_VERSION=${ESP_IDF_VERSION}
ENV PATH=${IDF_PATH}/tools:${PATH}

# Install additional system packages needed by devcontainer features, ESP-IDF, and firmware builds.
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    build-essential \
    bison \
    ca-certificates \
    ccache \
    cmake \
    curl \
    dfu-util \
    flex \
    gnupg \
    gperf \
    libffi-dev \
    libncurses-dev \
    libssl-dev \
    libusb-1.0-0 \
    pkg-config \
    python3-venv \
    ripgrep \
    ninja-build \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js before devcontainer features so Codex and Claude are baked into the image.
RUN mkdir -p /etc/apt/keyrings \
    && curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key \
        | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg \
    && echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_${NODE_MAJOR}.x nodistro main" \
        >/etc/apt/sources.list.d/nodesource.list \
    && apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y nodejs \
    && npm install -g @anthropic-ai/claude-code @openai/codex \
    && npm cache clean --force \
    && rm -rf /var/lib/apt/lists/*

# Install ESP-IDF itself so idf.py and the Xtensa toolchains are available inside the devcontainer.
RUN mkdir -p ${ESP_ROOT} \
    && git clone --depth 1 --recursive --branch ${ESP_IDF_VERSION} https://github.com/espressif/esp-idf.git ${IDF_PATH} \
    && git config --system --add safe.directory ${IDF_PATH} \
    && ${IDF_PATH}/install.sh ${ESP_IDF_TARGETS} \
    && printf '%s\n' 'export PATH=/usr/bin:/bin:$PATH' 'source /opt/esp/idf/export.sh >/dev/null 2>&1' >/etc/profile.d/esp-idf.sh \
    && printf '%s\n' 'export PATH=/usr/bin:/bin:$PATH' 'source /opt/esp/idf/export.sh >/dev/null 2>&1' >>/etc/bash.bashrc \
    && chmod +x /etc/profile.d/esp-idf.sh \
    && mkdir -p /home/vscode/.codex /home/vscode/.claude \
    && chown -R vscode:vscode ${ESP_ROOT}

RUN chown -R vscode:vscode /home/vscode/.codex /home/vscode/.claude

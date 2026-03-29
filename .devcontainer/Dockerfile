FROM mcr.microsoft.com/devcontainers/base:ubuntu

# Install additional system packages needed by devcontainer features and tools
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

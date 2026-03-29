FROM espressif/idf:latest

# Install additional system packages needed by devcontainer features and tools
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    build-essential \
    libssl-dev \
    pkg-config \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# Allow the default user to use sudo (required by devcontainer features)
RUN echo "$(id -un 2>/dev/null || echo root) ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

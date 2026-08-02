FROM ubuntu:22.04

ARG RUST_VERSION=1.95.0
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        curl \
        git \
        gnupg \
        libssl-dev \
        pkg-config \
        rsync \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /etc/apt/keyrings \
    && curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key \
        | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg \
    && echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_22.x nodistro main" \
        > /etc/apt/sources.list.d/nodesource.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*

ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ \
        | sh -s -- --default-toolchain "${RUST_VERSION}" -y

WORKDIR /work
SHELL ["/bin/bash", "-c"]

CMD rsync -a --delete --exclude-from=/host_workspace/.gitignore /host_workspace/ /work/ \
    && npm --prefix /work/pwa ci \
    && npm --prefix /work/pwa run build \
    && cargo build --manifest-path /work/Cargo.toml --locked --release --bin asami \
    && install -D -m 0755 /work/target/release/asami /host_workspace/release/asami \
    && chown "${HOST_UID}:${HOST_GID}" /host_workspace/release/asami

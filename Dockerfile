# Pre-baked image for Codespaces / Distrobox / local Docker|Podman.
# Prefer the Nix image (containers/flake.nix → image-firmware-core) when publishing
# to a registry; this Dockerfile is the Codespaces-native build path (no Nix required).
#
# Includes: Rust host + riscv32imac-unknown-none-elf, xpack riscv-none-elf-gcc,
# nickel, cbindgen, just, cmake, ceedling. No LSPs / rust-analyzer.
#
#   docker build -t smart-keymap:devcontainer .
#   podman build -t smart-keymap:devcontainer .
#   distrobox create -i smart-keymap:devcontainer -n sk

ARG UBUNTU_TAG=24.04
ARG NICKEL_VERSION=1.16.0
ARG CBINDGEN_VERSION=0.28.0
ARG JUST_VERSION=1.40.0
ARG XPACK_GCC_VERSION=14.2.0-3
ARG RUST_TARGET=riscv32imac-unknown-none-elf

FROM ubuntu:${UBUNTU_TAG} AS bins
ARG NICKEL_VERSION CBINDGEN_VERSION JUST_VERSION XPACK_GCC_VERSION
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl wget xz-utils \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /opt/bins
RUN wget -q "https://github.com/nickel-lang/nickel/releases/download/${NICKEL_VERSION}/nickel-x86_64-linux" \
      -O nickel && chmod +x nickel \
 && wget -q "https://github.com/mozilla/cbindgen/releases/download/${CBINDGEN_VERSION}/cbindgen" \
      -O cbindgen && chmod +x cbindgen \
 && wget -q "https://github.com/casey/just/releases/download/${JUST_VERSION}/just-${JUST_VERSION}-x86_64-unknown-linux-musl.tar.gz" \
      -O just.tgz && tar -xzf just.tgz just && rm just.tgz \
 && wget -q "https://github.com/xpack-dev-tools/riscv-none-elf-gcc-xpack/releases/download/v${XPACK_GCC_VERSION}/xpack-riscv-none-elf-gcc-${XPACK_GCC_VERSION}-linux-x64.tar.gz" \
      -O xpack.tgz \
 && mkdir -p /opt/xpack && tar -xzf xpack.tgz -C /opt/xpack --strip-components=1 && rm xpack.tgz

FROM ubuntu:${UBUNTU_TAG}
ARG RUST_TARGET
ENV DEBIAN_FRONTEND=noninteractive \
    CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    PATH=/usr/local/cargo/bin:/opt/xpack/bin:/usr/local/bin:$PATH

RUN apt-get update && apt-get install -y --no-install-recommends \
      build-essential cmake make git ca-certificates curl \
      ruby ruby-dev \
      clang-format \
      pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=bins /opt/bins/nickel /opt/bins/cbindgen /opt/bins/just /usr/local/bin/
COPY --from=bins /opt/xpack /opt/xpack

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable \
      --profile minimal --component rustfmt,clippy \
 && rustup target add "${RUST_TARGET}" \
 && chmod -R a+rwX /usr/local/cargo /usr/local/rustup

RUN gem install --no-document ceedling

WORKDIR /workspace
CMD ["bash"]

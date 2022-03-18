FROM mcr.microsoft.com/vscode/devcontainers/universal:1-linux

USER root

ENV DENO_INSTALL=/deno \
    DENO_DIR=/deno/.cache/deno

RUN mkdir -p /deno \
    && curl -fsSL https://deno.land/x/install/install.sh | sh \
    && ln -s /deno/bin/deno /usr/bin/deno \
    && apt-get update && apt-get -y install ripgrep \
    && chown -R codespace /deno /usr/local

USER codespace

RUN rustup toolchain add stable nightly beta 1.59.0 \
    && rustup update                                 \
    && rustup component add clippy --toolchain stable \
    && rustup component add rustfmt --toolchain stable \
    && rustup component add rustfmt --toolchain nightly \
    && rustup target add wasm32-wasi wasm32-unknown-unknown

RUN curl https://wasmtime.dev/install.sh -sSf | bash

RUN cargo install baking cargo-edit cargo-expand cargo-format cargo-geiger cargo-udeps cargo-wasi git-uncommit https save tokio-console

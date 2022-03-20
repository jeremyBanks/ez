FROM mcr.microsoft.com/vscode/devcontainers/universal:1-linux

USER root

ENV DENO_INSTALL=/deno \
    DENO_DIR=/deno/.cache/deno

RUN mkdir -p /deno \
    && curl -fsSL https://deno.land/x/install/install.sh | sh \
    && ln -s /deno/bin/deno /usr/bin/deno \
    && chown -R codespace /deno /usr/local

USER codespace

RUN    rustup toolchain add             1.59.0 stable beta          \
    && rustup component add --toolchain 1.59.0  rustfmt clippy miri \
    && rustup component add --toolchain stable  rustfmt clippy miri \
    && rustup component add --toolchain beta    rustfmt clippy miri \
    && rustup target    add wasm32-wasi wasm32-unknown-unknown

RUN curl https://wasmtime.dev/install.sh -sSf | bash

RUN cargo install baking bindgen cargo-edit cargo-expand cargo-format cargo-geiger cargo-udeps cargo-wasi git-uncommit https ripgrep save tokio-console wasm-bindgen

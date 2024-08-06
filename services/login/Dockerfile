FROM ubuntu:latest

LABEL authors="Andrew Kozmin"

ARG config

ENV HOME_DIR="login"

WORKDIR /$HOME_DIR/

COPY ./services/$HOME_DIR/src/ /$HOME_DIR/src/
COPY ./services/$HOME_DIR/Cargo.toml /$HOME_DIR/

RUN apt update && \
    apt upgrade -y && \
    apt install -y build-essential cmake curl zlib1g-dev libssl-dev libsasl2-dev libzstd-dev && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile complete && \
    . "$HOME/.cargo/env" && \
    cargo build $config

CMD . "$HOME/.cargo/env" && cargo run $CONFIG

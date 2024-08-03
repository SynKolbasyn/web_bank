FROM ubuntu:latest

LABEL authors="Andrew Kozmin"

WORKDIR /front_end/

COPY ./front_end/src/ /front_end/src/
COPY ./front_end/Cargo.toml /front_end/

RUN apt update && \
    apt upgrade && \
    apt install -y build-essential cmake curl zlib1g-dev libssl-dev libsasl2-dev libzstd-dev && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile complete && \
    cargo build $CONFIG

CMD . "$HOME/.cargo/env" && cargo run $CONFIG

FROM ubuntu:24.04

LABEL authors="Andrew Kozmin"

ARG country_name
ARG state_of_province_name
ARG locality_name
ARG organization_name
ARG organizational_unit_name
ARG common_name

ARG config

ENV HOME_DIR="front_end"

WORKDIR /$HOME_DIR/

COPY ./services/$HOME_DIR/html/ /$HOME_DIR/html/
COPY ./services/$HOME_DIR/src/ /$HOME_DIR/src/
COPY ./services/$HOME_DIR/Cargo.toml /$HOME_DIR/

RUN apt update && \
    apt upgrade -y && \
    apt install -y build-essential cmake curl zlib1g-dev libssl-dev libsasl2-dev libzstd-dev && \
    openssl req -new -utf8 -x509 -newkey rsa:4096 -noenc -keyout key.pem -days 3653 -subj "/C=$country_name/ST=$state_of_province_name/L=$locality_name/O=$organization_name/OU=$organizational_unit_name/CN=$common_name" -out cert.crt && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile complete && \
    . "$HOME/.cargo/env" && \
    cargo build $config

CMD . "$HOME/.cargo/env" && cargo run $CONFIG

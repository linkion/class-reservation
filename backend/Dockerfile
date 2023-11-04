FROM rust:latest

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install cargo-watch

WORKDIR /backend
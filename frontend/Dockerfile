FROM rust:latest

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

ADD . /frontend
WORKDIR /frontend

CMD ["trunk", "serve"]
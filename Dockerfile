FROM rust:latest

WORKDIR /app

ENV CARGO_HTTP_TIMEOUT=120
ENV CARGO_NET_RETRY=5
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

COPY Cargo.toml Cargo.lock ./
COPY trust_rust_client ./trust_rust_client
COPY trust_rust_web ./trust_rust_web
COPY trust_rust_web/static ./static
COPY .env .env

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build -p trust_rust_web --release

EXPOSE 3000

CMD ["./target/release/trust_rust_web"]
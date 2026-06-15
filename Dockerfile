FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["cargo", "run", "-p", "trust_rust_web"]
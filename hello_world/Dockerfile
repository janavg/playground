FROM rust:1.84-slim AS builder

WORKDIR /app
COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/hello_world /usr/local/bin/hello_world

CMD ["hello_world"]

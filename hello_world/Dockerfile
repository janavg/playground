FROM rust:1.84-slim AS builder

WORKDIR /app
COPY Cargo.toml ./
COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    cp target/release/hello_world /usr/local/bin/hello_world

FROM debian:bookworm-slim

COPY --from=builder /usr/local/bin/hello_world /usr/local/bin/hello_world

CMD ["hello_world"]

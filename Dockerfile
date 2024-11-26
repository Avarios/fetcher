FROM rust:latest AS builder
WORKDIR /app

# Install SSL dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    openssl \
    ca-certificates

COPY . .
RUN RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-unknown-linux-musl

FROM debian:bullseye-slim

WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/fetcherRS /app/fetcherRS

ENTRYPOINT ["/app/fetcherRS"]
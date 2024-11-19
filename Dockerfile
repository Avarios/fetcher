FROM rust:latest AS builder
WORKDIR /app

# Install SSL dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    openssl \
    ca-certificates

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

# Install latest SSL libraries
RUN apt-get update && apt-get install -y \
    openssl \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/fetcherRS /app/fetcherRS

ENTRYPOINT ["/app/fetcherRS"]
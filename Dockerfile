FROM rust:slim AS builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y libssl-dev pkg-config
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/fetcherRS /app/fetcherRS
ENTRYPOINT ["/app/fetcherRS"]
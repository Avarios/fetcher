FROM rust:slim
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y libssl-dev pkg-config ca-certificates
RUN cargo build --release
ENTRYPOINT ["/app/target/release/fetcherRS"]
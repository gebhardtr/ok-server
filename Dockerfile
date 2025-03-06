FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release
FROM container-registry.oracle.com/os/oraclelinux:9-slim
ENV ADDRESS="0.0.0.0:8888"
WORKDIR /app
COPY --from=builder /app/target/release/ok-server .
CMD ["./ok-server"]

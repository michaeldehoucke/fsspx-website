# 1. Build
FROM rust:slim AS builder
# Install native dependencies needed by some crates
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

# 2. Runtime
FROM debian:stable-slim
RUN apt-get update && apt-get install -y libsqlite3-0 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/fsspx-website /usr/local/bin/
COPY .env .
COPY migrations ./migrations
EXPOSE 8080
CMD ["fsspx-website"]


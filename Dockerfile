FROM rust:slim AS builder

WORKDIR /src

COPY Cargo.toml Cargo.lock ./
# Pre-fetch dependencies so this layer is cached independently of source changes.
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo fetch --locked && \
    cargo build --release 2>/dev/null || true && \
    rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release --locked

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /src/target/release/netbox-mcp /usr/local/bin/netbox-mcp

ENTRYPOINT ["netbox-mcp"]

# Pin both the Rust version (matching rust-toolchain.toml) and the Debian
# release (matching the runtime stage below): a floating `rust:slim` moved to a
# newer Debian and produced a binary needing a glibc bookworm doesn't have.
FROM rust:1.100.0-slim-bookworm AS builder

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

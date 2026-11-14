BINARY     := netbox-mcp
IMAGE      := netbox-mcp
VERSION    := $(shell git describe --tags --always --dirty 2>/dev/null || echo dev)
# The declared minimum supported Rust version, from Cargo.toml.
MSRV       := $(shell sed -n 's/^rust-version = "\(.*\)"/\1/p' Cargo.toml)
# The binary `make smoke` exercises (override: make smoke BIN=path/to/netbox-mcp).
BIN        ?= target/release/$(BINARY)
# Container engine for docker-build / docker-smoke (docker, or podman locally).
ENGINE     ?= $(shell command -v docker >/dev/null 2>&1 && echo docker || echo podman)

# This Makefile is the single source of truth for every check: CI only calls
# these targets, so a green `make check` locally means the same as green CI.
#
# Clippy strictness (the pedantic, nursery, and cargo lint groups) is configured
# in Cargo.toml under [lints.clippy], so every `cargo clippy` — `make lint`,
# rust-analyzer, and the release gate — enforces it with no extra flags.

.PHONY: build release test test-real-gpg live-test lint secrets-scan fmt fmt-check \
	deny msrv smoke check install docker-build docker-smoke clean

build:
	cargo build --locked

# TARGET=<triple> cross-builds (the release workflow's matrix); the binary then
# lands in target/<triple>/release/.
release:
	cargo build --release --locked $(if $(TARGET),--target $(TARGET))

# Unit tests plus tests/launch_path.rs (the built binary over MCP stdio).
test:
	cargo test --all --locked

# Exercises the credentials code against the real `gpg` on PATH (the default
# tests use a fake): passphrase-less success, a locked key (code 85), and a
# foreign-key ciphertext (code 17). Catches gpg status-code drift. Keyrings are
# throwaway, under a short /tmp path (macOS caps gpg-agent socket paths at 104
# bytes).
test-real-gpg:
	REAL_GPG=1 cargo test --all --locked real_gpg -- --ignored --test-threads=1

# Live integration tests against a real, seeded NetBox (test/netbox-docker/up.sh
# brings one up). Requires NETBOX_URL and NETBOX_TOKEN; without them each test
# skips, unless LIVE_REQUIRED=1, which turns a skip into a failure. See
# docs/testing.md. Scoped to live:: because the config env-var tests remove the
# live credentials mid-run.
live-test:
	cargo test --locked --features live-tests live:: -- --test-threads=1

# Escalates every warning to an error across all targets (incl. tests) and every
# feature set; pedantic/nursery/cargo come from Cargo.toml.
lint: secrets-scan
	cargo clippy --locked --all-targets -- -D warnings
	cargo clippy --locked --all-targets --features live-tests -- -D warnings
	cargo fmt --check

# Fails if a NetBox v2 API token (nbt_<key>.<secret>) appears in a tracked or
# untracked-but-not-ignored file, so a real-looking test fixture is caught
# before it's committed. Test tokens must not use the nbt_ prefix. (Legacy v1
# tokens are bare 40-char hex, indistinguishable from other hex, so they can't
# be scanned for; the test instance's well-known all-digits token is fine.)
secrets-scan:
	@if git grep --untracked -nE 'nbt_[A-Za-z0-9]{8,}\.[A-Za-z0-9]{20,}' -- . ; then \
		echo "A NetBox API token appears to be committed"; exit 1; \
	fi

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

# Advisories, banned crates (aws-lc), sources, and licenses — see deny.toml.
deny:
	cargo deny --locked check

# Build and test on the declared rust-version (needs that toolchain installed:
# rustup toolchain install $(MSRV)). `+toolchain` overrides rust-toolchain.toml.
msrv:
	cargo +$(MSRV) test --all --locked

# Run the binary with a dummy config and no network: --version, then an MCP
# initialize + tools/list round-trip that must return every tool.
smoke:
	python3 scripts/smoke.py $(BIN)

# Full offline pre-release gate. Never includes the live suite (run
# `make live-test` against a seeded instance separately).
check: fmt-check lint test deny release smoke

install:
	cargo install --locked --path .

docker-build:
	$(ENGINE) build -t $(IMAGE):$(VERSION) -t $(IMAGE):latest .

# The image must start and report its version, and its Rust builder must be the
# toolchain pinned in rust-toolchain.toml (a drifted builder once shipped a
# binary needing a newer glibc than the runtime image had).
docker-smoke: docker-build
	@pin=$$(sed -n 's/^channel = "\(.*\)"/\1/p' rust-toolchain.toml); \
	if ! grep -q "^FROM rust:$$pin-" Dockerfile; then \
		echo "Dockerfile builder is not rust:$$pin (rust-toolchain.toml)"; exit 1; \
	fi
	$(ENGINE) run --rm $(IMAGE):$(VERSION) --version

clean:
	cargo clean

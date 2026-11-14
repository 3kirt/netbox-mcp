BINARY     := netbox-mcp
IMAGE      := netbox-mcp
VERSION    := $(shell git describe --tags --always --dirty 2>/dev/null || echo dev)

# Clippy strictness (the pedantic, nursery, and cargo lint groups) is configured
# in Cargo.toml under [lints.clippy], so every `cargo clippy` — `make lint`,
# rust-analyzer, and the release gate — enforces it with no extra flags.

.PHONY: build clean lint secrets-scan test test-live check install docker-build

build:
	cargo build --release

clean:
	cargo clean

# Escalates every warning to an error across all targets (incl. tests) and both
# feature sets; pedantic/nursery/cargo come from Cargo.toml.
lint: secrets-scan
	cargo clippy --all-targets -- -D warnings
	cargo clippy --features live-tests --tests -- -D warnings
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

# Full offline gate, mirroring the release process minus the live suite (which
# needs credentials — run `make test-live` with NETBOX_URL/NETBOX_TOKEN set).
check: lint test build

test:
	cargo test --all

# Live integration tests against a real, seeded NetBox. Requires NETBOX_URL and
# NETBOX_TOKEN; seed the instance first with scripts/seed_data.py. Skips (does
# not fail) when credentials are absent. See docs/testing.md.
# Scoped to the live:: module: the config env-var tests remove_var the live
# credentials mid-run, so an unfiltered single-threaded run silently skips
# every live test that executes after them.
test-live:
	cargo test --features live-tests live:: -- --test-threads=1

install:
	cargo install --path .

docker-build:
	docker build -t $(IMAGE):$(VERSION) -t $(IMAGE):latest .

BINARY     := netbox-mcp
IMAGE      := netbox-mcp
VERSION    := $(shell git describe --tags --always --dirty 2>/dev/null || echo dev)

.PHONY: build clean lint test install docker-build docker-run

build:
	cargo build --release

clean:
	cargo clean

lint:
	cargo clippy -- -D warnings
	cargo fmt --check

test:
	cargo test --all

install:
	cargo install --path .

docker-build:
	docker build -t $(IMAGE):$(VERSION) -t $(IMAGE):latest .

docker-run:
	docker run --rm -p 8080:8080 \
	  -e NETBOX_URL=$(NETBOX_URL) \
	  -e NETBOX_TOKEN=$(NETBOX_TOKEN) \
	  $(IMAGE):latest

BINARY  := netbox-mcp
VERSION := $(shell git describe --tags --always --dirty 2>/dev/null || echo dev)

.PHONY: build clean lint test install

build:
	go build -ldflags "-X main.version=$(VERSION)" -o $(BINARY) ./cmd/netbox-mcp

clean:
	rm -f $(BINARY)

lint:
	golangci-lint run ./...

test:
	go test ./...

install:
	go install ./cmd/netbox-mcp

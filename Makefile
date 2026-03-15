BINARY     := netbox-mcp
IMAGE      := netbox-mcp
VERSION    := $(shell git describe --tags --always --dirty 2>/dev/null || echo dev)

.PHONY: build clean lint test install docker-build docker-run

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

docker-build:
	docker build --build-arg VERSION=$(VERSION) -t $(IMAGE):$(VERSION) -t $(IMAGE):latest .

docker-run:
	docker run --rm -p 8080:8080 \
	  -e NETBOX_URL=$(NETBOX_URL) \
	  $(IMAGE):latest

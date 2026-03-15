FROM golang:1.24-alpine AS builder

WORKDIR /src

COPY go.mod go.sum ./
RUN go mod download

COPY . .
ARG VERSION=dev
RUN go build -ldflags "-X main.version=${VERSION}" -o /netbox-mcp ./cmd/netbox-mcp

FROM alpine:3.21

RUN apk add --no-cache ca-certificates

COPY --from=builder /netbox-mcp /usr/local/bin/netbox-mcp

EXPOSE 8080

ENTRYPOINT ["netbox-mcp"]
CMD ["--listen", ":8080"]

# netbox-mcp

A [Model Context Protocol](https://modelcontextprotocol.io) (MCP) server that connects Claude and other MCP-compatible AI clients to a live [NetBox](https://netbox.dev) instance.

Ask questions like *"Which devices in the NYC site are currently in maintenance?"* or *"Show me all IPs assigned to web01 and their VRFs"* — the server translates them into real-time NetBox API queries and returns structured results.

- **Read-only** — all 169 tools query NetBox but make no changes
- **Two transports** — stdio (local subprocess) or HTTP (remote/shared)
- **Token-efficient** — responses are slimmed before delivery, cutting typical NetBox payloads by 50–70%

---

## Table of contents

- [Requirements](#requirements)
- [Installation](#installation)
- [Configuration](#configuration)
- [Client setup](#client-setup)
  - [Claude Desktop](#claude-desktop)
  - [Claude Code](#claude-code)
- [Remote MCP (HTTP transport)](#remote-mcp-http-transport)
  - [Running with Docker](#running-with-docker)
  - [Running from the binary](#running-from-the-binary)
  - [Deploying to Kubernetes](#deploying-to-kubernetes)
  - [Operations](#operations)
- [Available tools](#available-tools)
- [Development](#development)

---

## Requirements

- A running NetBox instance with a valid API token
- To build from source: Rust stable toolchain (`rustup install stable`)
- To use a pre-built binary: nothing — grab it from the [releases page](https://github.com/3kirt/netbox-mcp/releases)

---

## Installation

### Pre-built binary (recommended)

Download the binary for your platform from the [releases page](https://github.com/3kirt/netbox-mcp/releases) and place it somewhere on your `$PATH`.

### From source

```sh
git clone https://github.com/3kirt/netbox-mcp
cd netbox-mcp
make install
```

Installs the `netbox-mcp` binary to `$CARGO_HOME/bin` (typically `~/.cargo/bin`).

---

## Configuration

netbox-mcp reads credentials from `~/.netbox_mcp.json`:

```json
{
  "url": "https://netbox.example.com",
  "token": "your-api-token"
}
```

Environment variables take precedence over the config file:

| Variable | Description |
|---|---|
| `NETBOX_URL` | NetBox base URL |
| `NETBOX_TOKEN` | NetBox API token |

A custom config file path can be specified with `--config`:

```sh
netbox-mcp --config /path/to/config.json
```

### Obtaining an API token

Tokens can be created in NetBox under **Admin → API Tokens**, or provisioned via the API:

```sh
curl -s -X POST https://netbox.example.com/api/users/tokens/provision/ \
  -H "Content-Type: application/json" \
  -d '{"username": "you", "password": "yourpassword"}' \
  | jq '.key'
```

Because netbox-mcp is read-only, create a **read-only token** (deselect "Write enabled" in the token settings). If the server starts returning 403 errors, check whether the token has expired.

---

## Client setup

### Claude Desktop

Add the following to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "netbox": {
      "command": "netbox-mcp",
      "env": {
        "NETBOX_URL": "https://netbox.example.com",
        "NETBOX_TOKEN": "your-api-token"
      }
    }
  }
}
```

Config file location:

| OS | Path |
|---|---|
| macOS | `~/Library/Application Support/Claude/claude_desktop_config.json` |
| Linux | `~/.config/Claude/claude_desktop_config.json` |
| Windows | `%APPDATA%\Claude\claude_desktop_config.json` |

### Claude Code

Register via the CLI:

```sh
claude mcp add --transport stdio \
  --env NETBOX_URL=https://netbox.example.com \
  --env NETBOX_TOKEN=your-api-token \
  netbox -- netbox-mcp
```

To share the configuration with your team (writes to `.mcp.json`):

```sh
claude mcp add --transport stdio --scope project \
  --env NETBOX_URL=https://netbox.example.com \
  netbox -- netbox-mcp
```

---

## Remote MCP (HTTP transport)

netbox-mcp can run as a remote MCP server over the Streamable HTTP transport. Each session authenticates with its own NetBox API token via an `Authorization: Bearer` header — no server-side token is configured.

### Running with Docker

```sh
docker build -t netbox-mcp .
docker run --rm -p 8080:8080 \
  -e NETBOX_URL=https://netbox.example.com \
  netbox-mcp
```

Or with `make`:

```sh
NETBOX_URL=https://netbox.example.com make docker-build docker-run
```

### Running from the binary

```sh
NETBOX_URL=https://netbox.example.com netbox-mcp --listen :8080
```

### Deploying to Kubernetes

Example manifests are provided in [deploy/kubernetes/](deploy/kubernetes/).

| File | Purpose |
|---|---|
| `configmap.yaml` | `NETBOX_URL` environment variable |
| `deployment.yaml` | Deployment with liveness/readiness probes, non-root security context |
| `service.yaml` | ClusterIP Service for MCP traffic |
| `ingress.yaml` | nginx Ingress with extended SSE proxy timeouts |
| `service-metrics.yaml` | Cluster-internal Service for Prometheus scraping |
| `service-monitor.yaml` | Prometheus Operator `ServiceMonitor` |

1. Edit `configmap.yaml` to set your NetBox URL.
2. Edit `deployment.yaml` to reference your image.
3. Edit `ingress.yaml` to set your hostname and TLS configuration.

```sh
kubectl apply -f deploy/kubernetes/
```

The Ingress routes only `/mcp` and is pre-configured for nginx with extended proxy timeouts to keep SSE streams alive.

### Operations

**Health endpoints**

| Endpoint | Purpose | Success response |
|---|---|---|
| `GET /healthz` | Liveness — server is running | `{"status":"ok","version":"v..."}` |
| `GET /readyz` | Readiness — NetBox hostname resolves | `{"status":"ok"}` |

`/readyz` returns `503` when the NetBox hostname cannot be resolved, preventing Kubernetes from routing traffic to a pod that cannot reach NetBox.

**Structured logging**

The server writes JSON log lines to stderr. Startup:

```json
{"time":"2026-01-15T10:00:00Z","level":"INFO","msg":"netbox-mcp starting","addr":":8080","netbox_url":"https://netbox.example.com","version":"v0.1.1"}
```

Per-request:

```json
{"time":"2026-01-15T10:00:01Z","level":"INFO","msg":"request","method":"POST","path":"/mcp","status":200,"duration_ms":42,"remote_addr":"10.0.0.1:54321"}
```

**Graceful shutdown**

On `SIGTERM` or `SIGINT` the server stops accepting new connections and gives in-flight requests up to 30 seconds to complete, matching the Kubernetes default `terminationGracePeriodSeconds`.

**Registering with Claude Code (HTTP)**

```sh
claude mcp add --transport http \
  --header "Authorization: Bearer your-netbox-token" \
  netbox https://netbox-mcp.example.com/mcp
```

> **TLS note:** The HTTP listener does not terminate TLS. In production, place it behind a reverse proxy (nginx, Caddy) or a platform like Fly.io or Railway that provides HTTPS.

---

## Available tools

169 read-only tools spanning ten NetBox API areas:

| Area | Coverage |
|---|---|
| **DCIM** | Sites, racks, devices, interfaces, cables, power, modules, inventory |
| **IPAM** | IP addresses, prefixes, VRFs, VLANs, aggregates, ranges, ASNs, FHRP |
| **Virtualization** | VMs, clusters, VM interfaces, virtual disks |
| **Circuits** | Circuits, providers, circuit types, terminations |
| **VPN** | Tunnels, L2VPNs, IKE/IPSec policies |
| **Wireless** | Wireless LANs, LAN groups, wireless links |
| **Tenancy** | Tenants, tenant groups, contacts, contact roles |
| **Extras** | Tags, config contexts, journal entries, custom fields, webhooks |
| **Core** | Data sources, background jobs, object change log |
| **Users** | Users, groups, API tokens |

Each area provides `list` (with filters) and `get` (by ID) tools. A meta-tool `netbox_lookup_host` searches both physical devices and VMs by name in a single call.

---

## Development

```sh
make build         # cargo build
make test          # cargo test --all
make lint          # cargo clippy -- -D warnings && cargo fmt --check
make clean         # remove build artifacts
make docker-build  # build Docker image
make docker-run    # run HTTP server on :8080 (requires NETBOX_URL=...)
```

---

## License

[GPL-3.0](LICENSE)

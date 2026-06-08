# netbox-mcp

A [Model Context Protocol](https://modelcontextprotocol.io) (MCP) server that connects Claude and other MCP-compatible AI clients to a live [NetBox](https://netbox.dev) instance.

Ask questions like *"Which devices in the NYC site are currently in maintenance?"* or *"Show me all IPs assigned to web01 and their VRFs"* — the server translates them into real-time NetBox API queries and returns structured results.

- **Read-only** — all 169 tools query NetBox but make no changes
- **stdio transport** — runs as a local subprocess managed by your MCP client
- **Token-efficient** — responses are slimmed before delivery, cutting typical NetBox payloads by 50–70%

---

## Table of contents

- [Requirements](#requirements)
- [Installation](#installation)
- [Configuration](#configuration)
- [Client setup](#client-setup)
  - [Claude Desktop](#claude-desktop)
  - [Claude Code](#claude-code)
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
make test          # cargo test --all  (unit tests, offline)
make test-live     # live integration tests against a seeded NetBox
make lint          # cargo clippy -- -D warnings && cargo fmt --check
make clean         # remove build artifacts
make docker-build  # build Docker image
```

### Testing

Two layers verify different risks — fast offline unit tests, and a feature-gated
live suite that checks fidelity to the real NetBox API. A self-contained,
auto-seeding NetBox stack for the live suite lives in
[`test/netbox-docker/`](test/netbox-docker/) (Podman on macOS):

```sh
cd test/netbox-docker && ./up.sh          # boot + seed a local NetBox
NETBOX_URL=http://localhost:8000 \
NETBOX_TOKEN=0123456789abcdef0123456789abcdef01234567 make test-live
```

See [`docs/testing.md`](docs/testing.md) for the full rationale and how to add
coverage.

---

## License

[GPL-3.0](LICENSE)

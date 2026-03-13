# netbox-mcp

A [Model Context Protocol](https://modelcontextprotocol.io) (MCP) server that exposes [NetBox](https://netbox.dev) infrastructure data as tools for Claude and other MCP-compatible clients.

The server runs as a local subprocess and communicates over stdio. It is read-only: all tools query NetBox but make no changes.

## Requirements

- Go 1.24 or later
- A running NetBox instance with a valid API token

## Installation

### From source

```sh
git clone https://github.com/3kirt/netbox-mcp
cd netbox-mcp
make install
```

This installs the `netbox-mcp` binary to `$GOPATH/bin`.

### Build only

```sh
make build
```

Produces a `netbox-mcp` binary in the current directory.

## Configuration

netbox-mcp reads its configuration from `~/.netbox_mcp.json`:

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

A custom config file path can be specified with the `--config` flag:

```sh
netbox-mcp --config /path/to/config.json
```

## Claude Desktop integration

Add the following to your Claude Desktop `claude_desktop_config.json`:

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

The config file is typically located at:
- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Linux: `~/.config/Claude/claude_desktop_config.json`
- Windows: `%APPDATA%\Claude\claude_desktop_config.json`

## Claude Code integration

Add the following to your project's `.mcp.json` or to `~/.claude.json` under `mcpServers`:

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

## Available tools

26 tools are currently implemented across five NetBox API areas.

### DCIM — [docs/dcim.md](docs/dcim.md)

| Tool | Description |
|---|---|
| `netbox_dcim_devices_list` | List devices (filter: site, role, status, rack) |
| `netbox_dcim_devices_get` | Get a device by ID |
| `netbox_dcim_sites_list` | List sites (filter: name, status, region) |
| `netbox_dcim_sites_get` | Get a site by ID |
| `netbox_dcim_racks_list` | List racks (filter: site, location, status) |
| `netbox_dcim_racks_get` | Get a rack by ID |
| `netbox_dcim_interfaces_list` | List device interfaces (filter: device, name, type) |
| `netbox_dcim_cables_list` | List cables (filter: site, status) |

### IPAM — [docs/ipam.md](docs/ipam.md)

| Tool | Description |
|---|---|
| `netbox_ipam_ip_addresses_list` | List IP addresses (filter: address, VRF, status, tenant, device) |
| `netbox_ipam_ip_addresses_get` | Get an IP address by ID |
| `netbox_ipam_prefixes_list` | List prefixes (filter: prefix, VRF, status, site, tenant) |
| `netbox_ipam_prefixes_get` | Get a prefix by ID |
| `netbox_ipam_vrfs_list` | List VRFs (filter: name, RD, tenant) |
| `netbox_ipam_vrfs_get` | Get a VRF by ID |
| `netbox_ipam_vlans_list` | List VLANs (filter: VID, name, site, group, status) |
| `netbox_ipam_vlans_get` | Get a VLAN by ID |

### Circuits — [docs/circuits.md](docs/circuits.md)

| Tool | Description |
|---|---|
| `netbox_circuits_circuits_list` | List circuits (filter: provider, status, type, site, tenant) |
| `netbox_circuits_circuits_get` | Get a circuit by ID |
| `netbox_circuits_providers_list` | List circuit providers (filter: name) |
| `netbox_circuits_providers_get` | Get a provider by ID |

### Tenancy — [docs/tenancy.md](docs/tenancy.md)

| Tool | Description |
|---|---|
| `netbox_tenancy_tenants_list` | List tenants (filter: name, group) |
| `netbox_tenancy_tenants_get` | Get a tenant by ID |

### Virtualization — [docs/virtualization.md](docs/virtualization.md)

| Tool | Description |
|---|---|
| `netbox_virtualization_vms_list` | List virtual machines (filter: cluster, site, status, role, tenant) |
| `netbox_virtualization_vms_get` | Get a virtual machine by ID |
| `netbox_virtualization_clusters_list` | List clusters (filter: name, type, site) |
| `netbox_virtualization_clusters_get` | Get a cluster by ID |

## Development

```sh
make build   # compile
make test    # run tests
make lint    # run golangci-lint
make clean   # remove compiled binary
```

## License

[MIT](LICENSE)

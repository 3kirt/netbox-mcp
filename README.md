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

84 tools are currently implemented across five NetBox API areas.

### DCIM — [docs/dcim.md](docs/dcim.md)

| Tool | Description |
|---|---|
| `netbox_dcim_devices_list` | List devices (filter: q, site, role, status, rack) |
| `netbox_dcim_devices_get` | Get a device by ID |
| `netbox_dcim_sites_list` | List sites (filter: q, name, status, region) |
| `netbox_dcim_sites_get` | Get a site by ID |
| `netbox_dcim_racks_list` | List racks (filter: q, site, location, status) |
| `netbox_dcim_racks_get` | Get a rack by ID |
| `netbox_dcim_interfaces_list` | List device interfaces (filter: q, device, name, type) |
| `netbox_dcim_interfaces_get` | Get a device interface by ID |
| `netbox_dcim_cables_list` | List cables (filter: q, site, status) |
| `netbox_dcim_cables_get` | Get a cable by ID |
| `netbox_dcim_regions_list` | List regions (filter: q, name, slug, parent) |
| `netbox_dcim_regions_get` | Get a region by ID |
| `netbox_dcim_locations_list` | List locations (filter: q, site, parent, status, tenant) |
| `netbox_dcim_locations_get` | Get a location by ID |
| `netbox_dcim_manufacturers_list` | List manufacturers (filter: q, name, slug) |
| `netbox_dcim_manufacturers_get` | Get a manufacturer by ID |
| `netbox_dcim_device_types_list` | List device types (filter: q, manufacturer, model) |
| `netbox_dcim_device_types_get` | Get a device type by ID |
| `netbox_dcim_device_roles_list` | List device roles (filter: q, name, slug, vm_role) |
| `netbox_dcim_device_roles_get` | Get a device role by ID |
| `netbox_dcim_platforms_list` | List platforms (filter: q, name, manufacturer) |
| `netbox_dcim_platforms_get` | Get a platform by ID |
| `netbox_dcim_power_panels_list` | List power panels (filter: q, site) |
| `netbox_dcim_power_panels_get` | Get a power panel by ID |
| `netbox_dcim_power_feeds_list` | List power feeds (filter: q, site, status, type) |
| `netbox_dcim_power_feeds_get` | Get a power feed by ID |
| `netbox_dcim_virtual_chassis_list` | List virtual chassis (filter: q, site, tenant) |
| `netbox_dcim_virtual_chassis_get` | Get a virtual chassis by ID |
| `netbox_dcim_inventory_items_list` | List inventory items (filter: q, device, manufacturer, discovered) |
| `netbox_dcim_inventory_items_get` | Get an inventory item by ID |

### IPAM — [docs/ipam.md](docs/ipam.md)

| Tool | Description |
|---|---|
| `netbox_ipam_ip_addresses_list` | List IP addresses (filter: q, address, VRF, status, tenant, device) |
| `netbox_ipam_ip_addresses_get` | Get an IP address by ID |
| `netbox_ipam_prefixes_list` | List prefixes (filter: q, prefix, VRF, status, site, tenant) |
| `netbox_ipam_prefixes_get` | Get a prefix by ID |
| `netbox_ipam_vrfs_list` | List VRFs (filter: q, name, RD, tenant) |
| `netbox_ipam_vrfs_get` | Get a VRF by ID |
| `netbox_ipam_vlans_list` | List VLANs (filter: q, VID, name, site, group, status) |
| `netbox_ipam_vlans_get` | Get a VLAN by ID |
| `netbox_ipam_aggregates_list` | List aggregates (filter: q, family, RIR, tenant) |
| `netbox_ipam_aggregates_get` | Get an aggregate by ID |
| `netbox_ipam_ip_ranges_list` | List IP ranges (filter: q, VRF, status, tenant) |
| `netbox_ipam_ip_ranges_get` | Get an IP range by ID |
| `netbox_ipam_route_targets_list` | List route targets (filter: q, name, tenant) |
| `netbox_ipam_route_targets_get` | Get a route target by ID |
| `netbox_ipam_rirs_list` | List RIRs (filter: q, name, slug) |
| `netbox_ipam_rirs_get` | Get a RIR by ID |
| `netbox_ipam_vlan_groups_list` | List VLAN groups (filter: q, name) |
| `netbox_ipam_vlan_groups_get` | Get a VLAN group by ID |
| `netbox_ipam_services_list` | List services (filter: q, device, virtual machine, protocol) |
| `netbox_ipam_services_get` | Get a service by ID |

### Circuits — [docs/circuits.md](docs/circuits.md)

| Tool | Description |
|---|---|
| `netbox_circuits_circuits_list` | List circuits (filter: q, provider, status, type, site, tenant) |
| `netbox_circuits_circuits_get` | Get a circuit by ID |
| `netbox_circuits_providers_list` | List circuit providers (filter: q, name) |
| `netbox_circuits_providers_get` | Get a provider by ID |
| `netbox_circuits_circuit_types_list` | List circuit types (filter: q, name, slug) |
| `netbox_circuits_circuit_types_get` | Get a circuit type by ID |
| `netbox_circuits_circuit_terminations_list` | List circuit terminations (filter: q, circuit, site) |
| `netbox_circuits_circuit_terminations_get` | Get a circuit termination by ID |
| `netbox_circuits_provider_accounts_list` | List provider accounts (filter: q, provider) |
| `netbox_circuits_provider_accounts_get` | Get a provider account by ID |
| `netbox_circuits_provider_networks_list` | List provider networks (filter: q, provider) |
| `netbox_circuits_provider_networks_get` | Get a provider network by ID |

### Tenancy — [docs/tenancy.md](docs/tenancy.md)

| Tool | Description |
|---|---|
| `netbox_tenancy_tenants_list` | List tenants (filter: q, name, group) |
| `netbox_tenancy_tenants_get` | Get a tenant by ID |
| `netbox_tenancy_tenant_groups_list` | List tenant groups (filter: q, name, parent) |
| `netbox_tenancy_tenant_groups_get` | Get a tenant group by ID |
| `netbox_tenancy_contacts_list` | List contacts (filter: q, name, group) |
| `netbox_tenancy_contacts_get` | Get a contact by ID |
| `netbox_tenancy_contact_groups_list` | List contact groups (filter: q, name, parent) |
| `netbox_tenancy_contact_groups_get` | Get a contact group by ID |
| `netbox_tenancy_contact_roles_list` | List contact roles (filter: q, name, slug) |
| `netbox_tenancy_contact_roles_get` | Get a contact role by ID |

### Virtualization — [docs/virtualization.md](docs/virtualization.md)

| Tool | Description |
|---|---|
| `netbox_virtualization_vms_list` | List virtual machines (filter: q, cluster, site, status, role, tenant) |
| `netbox_virtualization_vms_get` | Get a virtual machine by ID |
| `netbox_virtualization_clusters_list` | List clusters (filter: q, name, type, site) |
| `netbox_virtualization_clusters_get` | Get a cluster by ID |
| `netbox_virtualization_cluster_groups_list` | List cluster groups (filter: q, name) |
| `netbox_virtualization_cluster_groups_get` | Get a cluster group by ID |
| `netbox_virtualization_cluster_types_list` | List cluster types (filter: q, name) |
| `netbox_virtualization_cluster_types_get` | Get a cluster type by ID |
| `netbox_virtualization_interfaces_list` | List VM interfaces (filter: q, virtual machine, name, enabled) |
| `netbox_virtualization_interfaces_get` | Get a VM interface by ID |
| `netbox_virtualization_virtual_disks_list` | List virtual disks (filter: q, virtual machine, name) |
| `netbox_virtualization_virtual_disks_get` | Get a virtual disk by ID |

## Development

```sh
make build   # compile
make test    # run tests
make lint    # run golangci-lint
make clean   # remove compiled binary
```

## License

[GPL-3.0](LICENSE)

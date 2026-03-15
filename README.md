# netbox-mcp

A [Model Context Protocol](https://modelcontextprotocol.io) (MCP) server that exposes [NetBox](https://netbox.dev) infrastructure data as tools for Claude and other MCP-compatible clients.

The server communicates over stdio (local subprocess) or HTTP (remote MCP). It is read-only: all tools query NetBox but make no changes.

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

### Obtaining a NetBox API token

Tokens can be created from your NetBox profile under **Admin → API Tokens**, or provisioned programmatically via the REST API:

```sh
curl -s -X POST https://netbox.example.com/api/users/tokens/provision/ \
  -H "Content-Type: application/json" \
  -d '{"username": "you", "password": "yourpassword"}' \
  | jq '.key'
```

Because netbox-mcp is entirely read-only, it is recommended to create a **read-only token** (deselect "Write enabled" in the token settings). Tokens may optionally have an expiration date — if the server begins returning 403 errors, check whether the token has expired.

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

Register the server using the `claude mcp add` command:

```sh
claude mcp add --transport stdio netbox -- netbox-mcp
```

With credentials passed as environment variables:

```sh
claude mcp add --transport stdio \
  --env NETBOX_URL=https://netbox.example.com \
  --env NETBOX_TOKEN=your-api-token \
  netbox -- netbox-mcp
```

To share the configuration with your team, use `--scope project` (writes to `.mcp.json`):

```sh
claude mcp add --transport stdio --scope project \
  --env NETBOX_URL=https://netbox.example.com \
  netbox -- netbox-mcp
```

Alternatively, add the following directly to your project's `.mcp.json` or to `~/.claude.json` under `mcpServers`:

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

## Remote MCP (HTTP transport)

netbox-mcp can run as a remote MCP server over the Streamable HTTP transport.
Each session authenticates with its own NetBox API token via an
`Authorization: Bearer` header — no server-side token is required.

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

### Registering with Claude Code

```sh
claude mcp add --transport http \
  --header "Authorization: Bearer your-netbox-token" \
  netbox http://your-host:8080/mcp
```

> **TLS note:** The HTTP listener does not terminate TLS. In production, place
> it behind a reverse proxy (nginx, Caddy) or a platform like Fly.io or Railway
> that provides HTTPS. Never expose the listener without TLS — the bearer token
> would be sent in plaintext.

### Remote MCP for Claude.ai web and mobile

Appearing in Claude.ai Settings → Connectors requires a full OAuth 2.0
Authorization Code flow. NetBox does not provide this natively; a future phase
will add a token-provisioning proxy or support for a NetBox OAuth plugin. Until
then, use the HTTP transport with Claude Code as described above.

## Available tools

168 tools are currently implemented across ten NetBox API areas.

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
| `netbox_dcim_cable_terminations_list` | List cable terminations (filter: cable_id) |
| `netbox_dcim_cable_terminations_get` | Get a cable termination by ID |
| `netbox_dcim_console_ports_list` | List console ports (filter: q, name, device_id, site) |
| `netbox_dcim_console_ports_get` | Get a console port by ID |
| `netbox_dcim_console_server_ports_list` | List console server ports (filter: q, name, device_id, site) |
| `netbox_dcim_console_server_ports_get` | Get a console server port by ID |
| `netbox_dcim_device_bays_list` | List device bays (filter: q, name, device_id, site) |
| `netbox_dcim_device_bays_get` | Get a device bay by ID |
| `netbox_dcim_front_ports_list` | List front ports (filter: q, name) |
| `netbox_dcim_front_ports_get` | Get a front port by ID |
| `netbox_dcim_mac_addresses_list` | List MAC addresses (filter: q, device_id) |
| `netbox_dcim_mac_addresses_get` | Get a MAC address by ID |
| `netbox_dcim_modules_list` | List modules (filter: q, device_id, site, status) |
| `netbox_dcim_modules_get` | Get a module by ID |
| `netbox_dcim_module_bays_list` | List module bays (filter: q, device_id) |
| `netbox_dcim_module_bays_get` | Get a module bay by ID |
| `netbox_dcim_module_types_list` | List module types (filter: q, manufacturer) |
| `netbox_dcim_module_types_get` | Get a module type by ID |
| `netbox_dcim_power_outlets_list` | List power outlets (filter: q, name, device_id, site) |
| `netbox_dcim_power_outlets_get` | Get a power outlet by ID |
| `netbox_dcim_power_ports_list` | List power ports (filter: q, name, device_id, site) |
| `netbox_dcim_power_ports_get` | Get a power port by ID |
| `netbox_dcim_rack_reservations_list` | List rack reservations (filter: q, rack_id, site, tenant) |
| `netbox_dcim_rack_reservations_get` | Get a rack reservation by ID |
| `netbox_dcim_rack_roles_list` | List rack roles (filter: q, name, slug) |
| `netbox_dcim_rack_roles_get` | Get a rack role by ID |
| `netbox_dcim_rack_types_list` | List rack types (filter: q, slug, manufacturer) |
| `netbox_dcim_rack_types_get` | Get a rack type by ID |
| `netbox_dcim_rear_ports_list` | List rear ports (filter: q, name) |
| `netbox_dcim_rear_ports_get` | Get a rear port by ID |
| `netbox_dcim_site_groups_list` | List site groups (filter: q, name, slug) |
| `netbox_dcim_site_groups_get` | Get a site group by ID |
| `netbox_dcim_virtual_device_contexts_list` | List virtual device contexts (filter: q, device_id, tenant) |
| `netbox_dcim_virtual_device_contexts_get` | Get a virtual device context by ID |

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
| `netbox_ipam_asns_list` | List ASNs (filter: q, site, tenant) |
| `netbox_ipam_asns_get` | Get an ASN by ID |
| `netbox_ipam_fhrp_groups_list` | List FHRP groups (filter: q, name, protocol) |
| `netbox_ipam_fhrp_groups_get` | Get an FHRP group by ID |
| `netbox_ipam_fhrp_group_assignments_list` | List FHRP group assignments (filter: group_id, device_id) |
| `netbox_ipam_fhrp_group_assignments_get` | Get an FHRP group assignment by ID |
| `netbox_ipam_roles_list` | List IP roles (filter: q, name, slug) |
| `netbox_ipam_roles_get` | Get an IP role by ID |

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

### Extras — [docs/extras.md](docs/extras.md)

| Tool | Description |
|---|---|
| `netbox_extras_tags_list` | List tags (filter: q, name, slug) |
| `netbox_extras_tags_get` | Get a tag by ID |
| `netbox_extras_config_contexts_list` | List config contexts (filter: q, name, is_active, site, role) |
| `netbox_extras_config_contexts_get` | Get a config context by ID |
| `netbox_extras_journal_entries_list` | List journal entries (filter: q, assigned_object_type, assigned_object_id, kind, created_by) |
| `netbox_extras_journal_entries_get` | Get a journal entry by ID |
| `netbox_extras_custom_fields_list` | List custom fields (filter: q, name, type, object_type) |
| `netbox_extras_custom_fields_get` | Get a custom field by ID |
| `netbox_extras_export_templates_list` | List export templates (filter: q, name, object_type) |
| `netbox_extras_export_templates_get` | Get an export template by ID |
| `netbox_extras_webhooks_list` | List webhooks (filter: q, name) |
| `netbox_extras_webhooks_get` | Get a webhook by ID |

### VPN — [docs/vpn.md](docs/vpn.md)

| Tool | Description |
|---|---|
| `netbox_vpn_tunnels_list` | List VPN tunnels (filter: q, status, group, tenant) |
| `netbox_vpn_tunnels_get` | Get a VPN tunnel by ID |
| `netbox_vpn_tunnel_groups_list` | List VPN tunnel groups (filter: q, name, slug) |
| `netbox_vpn_tunnel_groups_get` | Get a VPN tunnel group by ID |
| `netbox_vpn_l2vpns_list` | List L2VPNs (filter: q, type, tenant) |
| `netbox_vpn_l2vpns_get` | Get an L2VPN by ID |
| `netbox_vpn_ike_policies_list` | List IKE policies (filter: q, name) |
| `netbox_vpn_ike_policies_get` | Get an IKE policy by ID |
| `netbox_vpn_ipsec_policies_list` | List IPSec policies (filter: q, name) |
| `netbox_vpn_ipsec_policies_get` | Get an IPSec policy by ID |
| `netbox_vpn_tunnel_terminations_list` | List VPN tunnel terminations (filter: q, tunnel_id, role) |
| `netbox_vpn_tunnel_terminations_get` | Get a VPN tunnel termination by ID |

### Wireless — [docs/wireless.md](docs/wireless.md)

| Tool | Description |
|---|---|
| `netbox_wireless_lans_list` | List wireless LANs (filter: q, ssid, group, status, tenant) |
| `netbox_wireless_lans_get` | Get a wireless LAN by ID |
| `netbox_wireless_lan_groups_list` | List wireless LAN groups (filter: q, name, parent) |
| `netbox_wireless_lan_groups_get` | Get a wireless LAN group by ID |
| `netbox_wireless_links_list` | List wireless links (filter: q, status, tenant) |
| `netbox_wireless_links_get` | Get a wireless link by ID |

### Core — [docs/core.md](docs/core.md)

| Tool | Description |
|---|---|
| `netbox_core_data_sources_list` | List data sources (filter: q, name, status) |
| `netbox_core_data_sources_get` | Get a data source by ID |
| `netbox_core_jobs_list` | List background jobs (filter: q, status) |
| `netbox_core_jobs_get` | Get a background job by ID |
| `netbox_core_object_changes_list` | List object changes / audit log (filter: q, user) |
| `netbox_core_object_changes_get` | Get an object change record by ID |

### Users — [docs/users.md](docs/users.md)

| Tool | Description |
|---|---|
| `netbox_users_users_list` | List users (filter: q, username, is_active) |
| `netbox_users_users_get` | Get a user by ID |
| `netbox_users_groups_list` | List user groups (filter: q, name) |
| `netbox_users_groups_get` | Get a user group by ID |
| `netbox_users_tokens_list` | List API tokens (filter: q, user_id) |
| `netbox_users_tokens_get` | Get an API token by ID |

## Development

```sh
make build         # compile
make test          # run tests
make lint          # run golangci-lint
make clean         # remove compiled binary
make docker-build  # build Docker image
make docker-run    # run HTTP server on :8080 (requires NETBOX_URL=...)
```

## License

[GPL-3.0](LICENSE)

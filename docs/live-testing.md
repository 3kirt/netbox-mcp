# Live Testing Guide

This document is a protocol for Claude to follow when testing `netbox-mcp` against a live NetBox instance. Work through it top to bottom. Record any tool that returns unexpected results.

---

## Prerequisites

Before starting, confirm:

1. `netbox-mcp` is built and on `$PATH` (`make install` or `make build`).
2. The MCP server is configured — either via `~/.netbox_mcp.json` or environment variables:
   ```sh
   export NETBOX_URL=https://netbox.example.com
   export NETBOX_TOKEN=your-token-here
   ```
3. The NetBox instance is reachable and the token has read access.

---

## What a passing tool call looks like

A healthy response is a JSON object (or paginated wrapper) with a non-empty `results` array for list tools, or a top-level object for get tools. For example:

```json
{
  "count": 12,
  "next": null,
  "previous": null,
  "results": [ ... ]
}
```

A failing tool call returns `isError: true` with a message such as:
```
listing devices: 401 Unauthorized
```

---

## Testing protocol

For each tool area below, follow the sequence: list with no filters → list with one filter → get by ID from a result.

---

### 1. DCIM

**Devices**
- Call `netbox_dcim_devices_list` with no arguments.
- Note a device name and its `id`.
- Call `netbox_dcim_devices_list` with `site` set to a known site slug from the results.
- Call `netbox_dcim_devices_get` with the noted `id`. Confirm the returned object matches.

**Sites**
- Call `netbox_dcim_sites_list` with no arguments.
- Note a site name and its `id`.
- Call `netbox_dcim_sites_list` with `status` set to `active`.
- Call `netbox_dcim_sites_get` with the noted `id`.

**Regions**
- Call `netbox_dcim_regions_list` with no arguments.
- If results exist, note a region name and `id`.
- Call `netbox_dcim_regions_list` with `name` set to that name.
- Call `netbox_dcim_regions_get` with the noted `id`.

**Site Groups**
- Call `netbox_dcim_site_groups_list` with no arguments.
- If results exist, call `netbox_dcim_site_groups_get` with a result `id`.

**Locations**
- Call `netbox_dcim_locations_list` with no arguments.
- If results exist, call `netbox_dcim_locations_list` with `site` set to a known site slug.
- Call `netbox_dcim_locations_get` with a result `id`.

**Racks**
- Call `netbox_dcim_racks_list` with no arguments.
- If results exist, call `netbox_dcim_racks_list` with `site` set to a known site slug.
- Call `netbox_dcim_racks_get` with a rack `id` from the results.

**Rack Roles**
- Call `netbox_dcim_rack_roles_list` with no arguments.
- If results exist, call `netbox_dcim_rack_roles_get` with a result `id`.

**Rack Types**
- Call `netbox_dcim_rack_types_list` with no arguments.
- If results exist, call `netbox_dcim_rack_types_get` with a result `id`.

**Rack Reservations**
- Call `netbox_dcim_rack_reservations_list` with no arguments.
- If results exist, call `netbox_dcim_rack_reservations_list` with `site` set to a known site slug.
- Call `netbox_dcim_rack_reservations_get` with a result `id`.

**Manufacturers**
- Call `netbox_dcim_manufacturers_list` with no arguments.
- If results exist, note a manufacturer name and `id`.
- Call `netbox_dcim_manufacturers_get` with the noted `id`.

**Device Types**
- Call `netbox_dcim_device_types_list` with no arguments.
- If results exist, call `netbox_dcim_device_types_list` with `manufacturer` set to a known manufacturer slug.
- Call `netbox_dcim_device_types_get` with a result `id`.

**Device Roles**
- Call `netbox_dcim_device_roles_list` with no arguments.
- If results exist, call `netbox_dcim_device_roles_get` with a result `id`.

**Platforms**
- Call `netbox_dcim_platforms_list` with no arguments.
- If results exist, call `netbox_dcim_platforms_get` with a result `id`.

**Interfaces**
- Call `netbox_dcim_interfaces_list` with `device_id` set to the device ID noted above.
- Confirm the results all belong to that device.

**Cables**
- Call `netbox_dcim_cables_list` with no arguments.
- If results exist, call with `status` set to `connected`.

**Cable Terminations**
- Call `netbox_dcim_cable_terminations_list` with no arguments.
- If results exist, call `netbox_dcim_cable_terminations_get` with a result `id`.

**Console Ports**
- Call `netbox_dcim_console_ports_list` with no arguments.
- If results exist, call `netbox_dcim_console_ports_list` with `device_id` from the noted device.
- Call `netbox_dcim_console_ports_get` with a result `id`.

**Console Server Ports**
- Call `netbox_dcim_console_server_ports_list` with no arguments.
- If results exist, call `netbox_dcim_console_server_ports_get` with a result `id`.

**Power Panels**
- Call `netbox_dcim_power_panels_list` with no arguments.
- If results exist, call `netbox_dcim_power_panels_list` with `site` set to a known site slug.
- Call `netbox_dcim_power_panels_get` with a result `id`.

**Power Feeds**
- Call `netbox_dcim_power_feeds_list` with no arguments.
- If results exist, call `netbox_dcim_power_feeds_list` with `status` set to `active`.
- Call `netbox_dcim_power_feeds_get` with a result `id`.

**Power Outlets**
- Call `netbox_dcim_power_outlets_list` with no arguments.
- If results exist, call `netbox_dcim_power_outlets_get` with a result `id`.

**Power Ports**
- Call `netbox_dcim_power_ports_list` with no arguments.
- If results exist, call `netbox_dcim_power_ports_get` with a result `id`.

**Front Ports**
- Call `netbox_dcim_front_ports_list` with no arguments.
- If results exist, call `netbox_dcim_front_ports_get` with a result `id`.

**Rear Ports**
- Call `netbox_dcim_rear_ports_list` with no arguments.
- If results exist, call `netbox_dcim_rear_ports_get` with a result `id`.

**Device Bays**
- Call `netbox_dcim_device_bays_list` with no arguments.
- If results exist, call `netbox_dcim_device_bays_get` with a result `id`.

**Modules**
- Call `netbox_dcim_modules_list` with no arguments.
- If results exist, call `netbox_dcim_modules_list` with `status` set to `active`.
- Call `netbox_dcim_modules_get` with a result `id`.

**Module Bays**
- Call `netbox_dcim_module_bays_list` with no arguments.
- If results exist, call `netbox_dcim_module_bays_get` with a result `id`.

**Module Types**
- Call `netbox_dcim_module_types_list` with no arguments.
- If results exist, call `netbox_dcim_module_types_get` with a result `id`.

**Inventory Items**
- Call `netbox_dcim_inventory_items_list` with no arguments.
- If results exist, call `netbox_dcim_inventory_items_list` with `device_id` from the noted device.
- Call `netbox_dcim_inventory_items_get` with a result `id`.

**MAC Addresses**
- Call `netbox_dcim_mac_addresses_list` with no arguments.
- If results exist, call `netbox_dcim_mac_addresses_get` with a result `id`.

**Virtual Chassis**
- Call `netbox_dcim_virtual_chassis_list` with no arguments.
- If results exist, call `netbox_dcim_virtual_chassis_get` with a result `id`.

**Virtual Device Contexts**
- Call `netbox_dcim_virtual_device_contexts_list` with no arguments.
- If results exist, call `netbox_dcim_virtual_device_contexts_get` with a result `id`.

---

### 2. IPAM

**IP Addresses**
- Call `netbox_ipam_ip_addresses_list` with no arguments.
- Note an IP address and its `id`.
- Call `netbox_ipam_ip_addresses_list` with `status` set to `active`.
- Call `netbox_ipam_ip_addresses_get` with the noted `id`.

**Prefixes**
- Call `netbox_ipam_prefixes_list` with no arguments.
- Note a prefix and its `id`.
- Call `netbox_ipam_prefixes_list` with `status` set to `active`.
- Call `netbox_ipam_prefixes_get` with the noted `id`.

**Aggregates**
- Call `netbox_ipam_aggregates_list` with no arguments.
- If results exist, call `netbox_ipam_aggregates_get` with a result `id`.

**IP Ranges**
- Call `netbox_ipam_ip_ranges_list` with no arguments.
- If results exist, call `netbox_ipam_ip_ranges_list` with `status` set to `active`.
- Call `netbox_ipam_ip_ranges_get` with a result `id`.

**VRFs**
- Call `netbox_ipam_vrfs_list` with no arguments.
- If results exist, note a VRF name and `id`.
- Call `netbox_ipam_vrfs_get` with that `id`.
- Call `netbox_ipam_ip_addresses_list` with `vrf` set to the VRF name and confirm results are scoped correctly.

**Route Targets**
- Call `netbox_ipam_route_targets_list` with no arguments.
- If results exist, call `netbox_ipam_route_targets_get` with a result `id`.

**RIRs**
- Call `netbox_ipam_rirs_list` with no arguments.
- If results exist, call `netbox_ipam_rirs_get` with a result `id`.

**VLANs**
- Call `netbox_ipam_vlans_list` with no arguments.
- If results exist, note a VLAN `vid` and `id`.
- Call `netbox_ipam_vlans_list` with `vid` set to that number.
- Call `netbox_ipam_vlans_get` with the noted `id`.

**VLAN Groups**
- Call `netbox_ipam_vlan_groups_list` with no arguments.
- If results exist, call `netbox_ipam_vlan_groups_get` with a result `id`.

**ASNs**
- Call `netbox_ipam_asns_list` with no arguments.
- If results exist, call `netbox_ipam_asns_get` with a result `id`.

**Services**
- Call `netbox_ipam_services_list` with no arguments.
- If results exist, call `netbox_ipam_services_list` with `protocol` set to `tcp`.
- Call `netbox_ipam_services_get` with a result `id`.

**FHRP Groups**
- Call `netbox_ipam_fhrp_groups_list` with no arguments.
- If results exist, note a group `id`.
- Call `netbox_ipam_fhrp_groups_get` with that `id`.
- Call `netbox_ipam_fhrp_group_assignments_list` with `group_id` set to that `id`.
- If assignments exist, call `netbox_ipam_fhrp_group_assignments_get` with an assignment `id`.

**IP Roles**
- Call `netbox_ipam_roles_list` with no arguments.
- If results exist, call `netbox_ipam_roles_get` with a result `id`.

---

### 3. Circuits

**Circuits**
- Call `netbox_circuits_circuits_list` with no arguments.
- If results exist, note a provider slug and circuit `id`.
- Call `netbox_circuits_circuits_list` with `provider` set to that slug.
- Call `netbox_circuits_circuits_get` with the noted `id`.

**Providers**
- Call `netbox_circuits_providers_list` with no arguments.
- If results exist, note a provider name and `id`.
- Call `netbox_circuits_providers_list` with `name` set to that name.
- Call `netbox_circuits_providers_get` with the noted `id`.

**Circuit Types**
- Call `netbox_circuits_circuit_types_list` with no arguments.
- If results exist, call `netbox_circuits_circuit_types_get` with a result `id`.

**Circuit Terminations**
- Call `netbox_circuits_circuit_terminations_list` with no arguments.
- If results exist, call `netbox_circuits_circuit_terminations_list` with `circuit` set to a known circuit `id`.
- Call `netbox_circuits_circuit_terminations_get` with a result `id`.

**Provider Accounts**
- Call `netbox_circuits_provider_accounts_list` with no arguments.
- If results exist, call `netbox_circuits_provider_accounts_get` with a result `id`.

**Provider Networks**
- Call `netbox_circuits_provider_networks_list` with no arguments.
- If results exist, call `netbox_circuits_provider_networks_list` with `provider` set to a known provider slug.
- Call `netbox_circuits_provider_networks_get` with a result `id`.

---

### 4. Tenancy

**Tenants**
- Call `netbox_tenancy_tenants_list` with no arguments.
- If results exist, note a tenant name and `id`.
- Call `netbox_tenancy_tenants_list` with `name` set to that name.
- Call `netbox_tenancy_tenants_get` with the noted `id`.
- If a tenant group is present in the results, call `netbox_tenancy_tenants_list` with `group` set to the group slug.

**Tenant Groups**
- Call `netbox_tenancy_tenant_groups_list` with no arguments.
- If results exist, call `netbox_tenancy_tenant_groups_get` with a result `id`.

**Contacts**
- Call `netbox_tenancy_contacts_list` with no arguments.
- If results exist, note a contact name and `id`.
- Call `netbox_tenancy_contacts_list` with `name` set to that name.
- Call `netbox_tenancy_contacts_get` with the noted `id`.

**Contact Groups**
- Call `netbox_tenancy_contact_groups_list` with no arguments.
- If results exist, call `netbox_tenancy_contact_groups_get` with a result `id`.

**Contact Roles**
- Call `netbox_tenancy_contact_roles_list` with no arguments.
- If results exist, call `netbox_tenancy_contact_roles_get` with a result `id`.

---

### 5. Virtualization

**Virtual Machines**
- Call `netbox_virtualization_vms_list` with no arguments.
- If results exist, note a cluster name and VM `id`.
- Call `netbox_virtualization_vms_list` with `cluster` set to that name.
- Call `netbox_virtualization_vms_list` with `status` set to `active`.
- Call `netbox_virtualization_vms_get` with the noted `id`.

**VM Interfaces**
- Call `netbox_virtualization_interfaces_list` with no arguments.
- If results exist, call `netbox_virtualization_interfaces_list` with `virtual_machine` set to a known VM name.
- Call `netbox_virtualization_interfaces_get` with a result `id`.

**Virtual Disks**
- Call `netbox_virtualization_virtual_disks_list` with no arguments.
- If results exist, call `netbox_virtualization_virtual_disks_get` with a result `id`.

**Clusters**
- Call `netbox_virtualization_clusters_list` with no arguments.
- If results exist, note a cluster name and `id`.
- Call `netbox_virtualization_clusters_list` with `name` set to that name.
- Call `netbox_virtualization_clusters_get` with the noted `id`.

**Cluster Groups**
- Call `netbox_virtualization_cluster_groups_list` with no arguments.
- If results exist, call `netbox_virtualization_cluster_groups_get` with a result `id`.

**Cluster Types**
- Call `netbox_virtualization_cluster_types_list` with no arguments.
- If results exist, call `netbox_virtualization_cluster_types_get` with a result `id`.

---

### 6. Extras

**Tags**
- Call `netbox_extras_tags_list` with no arguments.
- If results exist, note a tag name and `id`.
- Call `netbox_extras_tags_list` with `name` set to that name.
- Call `netbox_extras_tags_get` with the noted `id`.

**Config Contexts**
- Call `netbox_extras_config_contexts_list` with no arguments.
- If results exist, call `netbox_extras_config_contexts_list` with `is_active` set to `true`.
- Call `netbox_extras_config_contexts_get` with a result `id`.

**Custom Fields**
- Call `netbox_extras_custom_fields_list` with no arguments.
- If results exist, call `netbox_extras_custom_fields_get` with a result `id`.

**Export Templates**
- Call `netbox_extras_export_templates_list` with no arguments.
- If results exist, call `netbox_extras_export_templates_get` with a result `id`.

**Webhooks**
- Call `netbox_extras_webhooks_list` with no arguments.
- If results exist, call `netbox_extras_webhooks_get` with a result `id`.

**Journal Entries**
- Call `netbox_extras_journal_entries_list` with no arguments.
- If results exist, call `netbox_extras_journal_entries_get` with a result `id`.

---

### 7. VPN

**Tunnels**
- Call `netbox_vpn_tunnels_list` with no arguments.
- If results exist, call `netbox_vpn_tunnels_list` with `status` set to `active`.
- Call `netbox_vpn_tunnels_get` with a result `id`.

**Tunnel Groups**
- Call `netbox_vpn_tunnel_groups_list` with no arguments.
- If results exist, call `netbox_vpn_tunnel_groups_get` with a result `id`.

**Tunnel Terminations**
- Call `netbox_vpn_tunnel_terminations_list` with no arguments.
- If results exist, call `netbox_vpn_tunnel_terminations_list` with `tunnel_id` set to a known tunnel `id`.
- Call `netbox_vpn_tunnel_terminations_get` with a result `id`.

**L2VPNs**
- Call `netbox_vpn_l2vpns_list` with no arguments.
- If results exist, call `netbox_vpn_l2vpns_get` with a result `id`.

**IKE Policies**
- Call `netbox_vpn_ike_policies_list` with no arguments.
- If results exist, call `netbox_vpn_ike_policies_get` with a result `id`.

**IPSec Policies**
- Call `netbox_vpn_ipsec_policies_list` with no arguments.
- If results exist, call `netbox_vpn_ipsec_policies_get` with a result `id`.

---

### 8. Wireless

**Wireless LANs**
- Call `netbox_wireless_lans_list` with no arguments.
- If results exist, call `netbox_wireless_lans_list` with `status` set to `active`.
- Call `netbox_wireless_lans_get` with a result `id`.

**Wireless LAN Groups**
- Call `netbox_wireless_lan_groups_list` with no arguments.
- If results exist, call `netbox_wireless_lan_groups_get` with a result `id`.

**Wireless Links**
- Call `netbox_wireless_links_list` with no arguments.
- If results exist, call `netbox_wireless_links_list` with `status` set to `active`.
- Call `netbox_wireless_links_get` with a result `id`.

---

### 9. Core

**Data Sources**
- Call `netbox_core_data_sources_list` with no arguments.
- If results exist, call `netbox_core_data_sources_list` with `status` set to `completed`.
- Call `netbox_core_data_sources_get` with a result `id`.

**Background Jobs**
- Call `netbox_core_jobs_list` with no arguments.
- If results exist, call `netbox_core_jobs_get` with a result `id`.

**Object Changes (Audit Log)**
- Call `netbox_core_object_changes_list` with no arguments.
- Confirm the results contain change records.
- Call `netbox_core_object_changes_list` with `user` set to a username from the results.
- Call `netbox_core_object_changes_get` with a result `id`.

---

### 10. Users

**Users**
- Call `netbox_users_users_list` with no arguments.
- If results exist, note a username and `id`.
- Call `netbox_users_users_list` with `is_active` set to `true`.
- Call `netbox_users_users_get` with the noted `id`.

**Groups**
- Call `netbox_users_groups_list` with no arguments.
- If results exist, call `netbox_users_groups_get` with a result `id`.

**API Tokens**
- Call `netbox_users_tokens_list` with no arguments.
- If results exist, call `netbox_users_tokens_get` with a result `id`.

---

## Cross-tool checks

Once the per-tool checks pass, verify a few natural joins:

- Take a site from `netbox_dcim_sites_list`. Query `netbox_dcim_devices_list` and `netbox_ipam_prefixes_list` with that site and confirm the results are consistent.
- Take a tenant from `netbox_tenancy_tenants_list`. Query `netbox_ipam_ip_addresses_list` with that tenant and confirm the results reference it.
- Take a device from `netbox_dcim_devices_list`. Query `netbox_dcim_interfaces_list` with `device_id` and confirm all returned interfaces belong to that device.
- Take a VM from `netbox_virtualization_vms_list`. Query `netbox_virtualization_interfaces_list` with `virtual_machine` set to its name and confirm the results match.
- Take a circuit from `netbox_circuits_circuits_list`. Query `netbox_circuits_circuit_terminations_list` with `circuit` set to its `id` and confirm the terminations reference it.

---

## Pagination check

Pick any list tool that returns more than one result and call it with `limit=1`. Confirm:
- Exactly one result is returned.
- `count` reflects the true total, not 1.
- `next` is non-null (if there are more results).

Then call again with `limit=1, offset=1` and confirm a different result is returned.

---

## Remote MCP (HTTP transport)

These steps verify the `--listen` HTTP mode and bearer-token authentication.
Run them after the stdio checks above pass.

### Setup

Start the server in HTTP mode (no `NETBOX_TOKEN` needed):

```sh
NETBOX_URL=https://netbox.example.com netbox-mcp --listen :8080
```

Register it with Claude Code:

```sh
claude mcp add --transport http \
  --header "Authorization: Bearer your-netbox-token" \
  netbox-http http://localhost:8080/mcp
```

Confirm the server appears in `/mcp` and shows as connected.

### Health and readiness checks

**Liveness probe**

```sh
curl -s http://localhost:8080/healthz
```

Expected: HTTP 200 with body `{"status":"ok","version":"..."}`. Confirm the
`version` field is non-empty.

**Readiness probe**

```sh
curl -s http://localhost:8080/readyz
```

Expected: HTTP 200 with body `{"status":"ok"}` when the NetBox hostname is
resolvable. To verify the failure path, start the server with an unresolvable
hostname (e.g. `NETBOX_URL=https://does-not-exist.invalid netbox-mcp --listen :8080`)
and confirm `/readyz` returns HTTP 503 with `{"status":"error","error":"..."}`.

**Log format**

Confirm the server emits JSON lines to stderr on startup:

```sh
NETBOX_URL=https://netbox.example.com netbox-mcp --listen :8080 2>&1 | head -1 | python3 -m json.tool
```

The output should parse as valid JSON with at minimum `time`, `level`, `msg`,
`addr`, `netbox_url`, and `version` fields. Confirm no token values appear in
any log line.

**Graceful shutdown**

Start the server, register it with Claude Code, then send SIGTERM:

```sh
kill -TERM <pid>
```

Confirm:
- The server logs `{"level":"INFO","msg":"shutting down"}` followed by `{"level":"INFO","msg":"shutdown complete"}`.
- Any tool call in flight at the time completes normally (not abruptly disconnected).
- The process exits with code 0.

### Authentication checks

**Valid token**
- Confirm the server accepted the connection (no 401 in Claude Code or server logs).
- Call `netbox_dcim_sites_list` with no arguments. Confirm results are returned.

**Invalid token**
- Register a second server entry with a deliberately wrong token:
  ```sh
  claude mcp add --transport http \
    --header "Authorization: Bearer invalid-token-value" \
    netbox-http-bad http://localhost:8080/mcp
  ```
- Confirm the connection fails (Claude Code should report a 401 error).
- Remove the bad entry: `claude mcp remove netbox-http-bad`

**Missing token**
- Send a raw request with no Authorization header:
  ```sh
  curl -s -o /dev/null -w "%{http_code}" \
    -X POST http://localhost:8080/mcp \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}'
  ```
- Confirm the response is `401`.

### Tool behaviour

Run the following tool calls through the HTTP-registered server and confirm
results match what was returned during the stdio run above:

- `netbox_dcim_devices_list` with no arguments
- `netbox_dcim_sites_list` with `status` set to `active`
- `netbox_ipam_ip_addresses_list` with no arguments
- `netbox_dcim_devices_get` with an ID from the list result

### Session isolation

Open a second Claude Code window and register the same HTTP server with a
different (but valid) NetBox token belonging to a different user account. Run
`netbox_users_tokens_list` in both sessions and confirm each session returns
only the tokens for its own user, verifying that sessions do not share state.

### Pagination over HTTP

Call `netbox_dcim_devices_list` with `limit=1`. Confirm:
- Exactly one result is returned.
- `count` reflects the true total.
- `next` is non-null (if more results exist).

Call again with `limit=1, offset=1` and confirm a different device is returned.

---

## Things to note during testing

- Any tool that returns `isError: true` unexpectedly.
- Any filter that returns results that don't match the filter value.
- Any `get` result that doesn't match the corresponding object from the list.
- Empty results for a filter that should plausibly return something (may indicate a slug vs. name mismatch).
- Unexpectedly large response payloads that might warrant a lower default limit.

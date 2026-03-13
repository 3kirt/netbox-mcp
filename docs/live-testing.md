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

**Racks**
- Call `netbox_dcim_racks_list` with no arguments.
- If results exist, call `netbox_dcim_racks_list` with `site` set to a known site slug.
- Call `netbox_dcim_racks_get` with a rack `id` from the results.

**Interfaces**
- Call `netbox_dcim_interfaces_list` with `device_id` set to the device ID noted above.
- Confirm the results all belong to that device.

**Cables**
- Call `netbox_dcim_cables_list` with no arguments.
- If results exist, call with `status` set to `connected`.

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

**VRFs**
- Call `netbox_ipam_vrfs_list` with no arguments.
- If results exist, note a VRF name and `id`.
- Call `netbox_ipam_vrfs_get` with that `id`.
- Call `netbox_ipam_ip_addresses_list` with `vrf` set to the VRF name and confirm results are scoped correctly.

**VLANs**
- Call `netbox_ipam_vlans_list` with no arguments.
- If results exist, note a VLAN `vid` and `id`.
- Call `netbox_ipam_vlans_list` with `vid` set to that number.
- Call `netbox_ipam_vlans_get` with the noted `id`.

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

---

### 4. Tenancy

**Tenants**
- Call `netbox_tenancy_tenants_list` with no arguments.
- If results exist, note a tenant name and `id`.
- Call `netbox_tenancy_tenants_list` with `name` set to that name.
- Call `netbox_tenancy_tenants_get` with the noted `id`.
- If a tenant group is present in the results, call `netbox_tenancy_tenants_list` with `group` set to the group slug.

---

### 5. Virtualization

**Virtual Machines**
- Call `netbox_virtualization_vms_list` with no arguments.
- If results exist, note a cluster name and VM `id`.
- Call `netbox_virtualization_vms_list` with `cluster` set to that name.
- Call `netbox_virtualization_vms_list` with `status` set to `active`.
- Call `netbox_virtualization_vms_get` with the noted `id`.

**Clusters**
- Call `netbox_virtualization_clusters_list` with no arguments.
- If results exist, note a cluster name and `id`.
- Call `netbox_virtualization_clusters_list` with `name` set to that name.
- Call `netbox_virtualization_clusters_get` with the noted `id`.

---

## Cross-tool checks

Once the per-tool checks pass, verify a few natural joins:

- Take a site from `netbox_dcim_sites_list`. Query `netbox_dcim_devices_list` and `netbox_ipam_prefixes_list` with that site and confirm the results are consistent.
- Take a tenant from `netbox_tenancy_tenants_list`. Query `netbox_ipam_ip_addresses_list` with that tenant and confirm the results reference it.
- Take a device from `netbox_dcim_devices_list`. Query `netbox_dcim_interfaces_list` with `device_id` and confirm all returned interfaces belong to that device.

---

## Pagination check

Pick any list tool that returns more than one result and call it with `limit=1`. Confirm:
- Exactly one result is returned.
- `count` reflects the true total, not 1.
- `next` is non-null (if there are more results).

Then call again with `limit=1, offset=1` and confirm a different result is returned.

---

## Things to note during testing

- Any tool that returns `isError: true` unexpectedly.
- Any filter that returns results that don't match the filter value.
- Any `get` result that doesn't match the corresponding object from the list.
- Empty results for a filter that should plausibly return something (may indicate a slug vs. name mismatch).
- Unexpectedly large response payloads that might warrant a lower default limit.

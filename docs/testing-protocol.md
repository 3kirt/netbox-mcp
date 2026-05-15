# NetBox MCP Testing Protocol

This document describes how to use the MCP tools to verify that all functionality is working correctly against a local NetBox instance seeded with `scripts/seed_data.py`.

---

## Universal Invariants

Every response from every tool must satisfy these properties. Check them on every call.

| Property | What to verify |
|---|---|
| No `null` values | No field in the response should have a `null` value — `slim_value()` strips them |
| No `local_context_data` | This key must never appear, even on VM responses that have it set |
| No `primary_ip` alias | The `primary_ip` shorthand must never appear; only `primary_ip4`/`primary_ip6` |
| No `next`/`previous` URLs | List responses must not contain bare NetBox pagination URLs |
| Pagination shape | List responses must have `{ count, has_more, next_offset, results }` |
| `has_more` accuracy | `has_more: true` iff `next_offset < count`; `has_more: false` when on last page |

---

## Seed Data Reference

The seed script creates the following objects. Use these as ground truth for filter assertions.

### Sites
| Name | Slug | Region |
|---|---|---|
| London DC | london-dc | Europe |
| Frankfurt DC | frankfurt-dc | Europe |

### Devices (per site)
London DC has: `lon-core-sw-01` (Cisco Catalyst 9300, Core Switch), `lon-edge-fw-01` (Palo Alto PA-3220, Firewall), `lon-leaf-sw-01..04` (Cisco Nexus 93180YC-EX, Leaf Switch), `lon-spine-sw-01..02` (Cisco Nexus 9364C, Spine Switch), `lon-compute-01..04` (Dell PowerEdge R750, Server), `lon-mgmt-01` (Dell PowerEdge R640, Server)

Frankfurt DC has: `fra-core-sw-01`, `fra-edge-fw-01`, `fra-leaf-sw-01..02`, `fra-compute-01..02`, `fra-mgmt-01`

### Virtual Machines
Two VMware clusters: `lon-vmware-cluster-01` (London), `fra-vmware-cluster-01` (Frankfurt)

VMs: `lon-web-01..03`, `lon-db-01`, `lon-cache-01`, `fra-web-01`, `fra-db-01`

All VMs have `local_context_data` set — this must never appear in any response.

### Prefixes
| Prefix | VRF | Description |
|---|---|---|
| 10.0.0.0/8 | — | Global RFC1918 aggregate |
| 172.16.0.0/12 | — | Global RFC1918 aggregate |
| 10.10.0.0/16 | MGMT | London management |
| 10.20.0.0/16 | MGMT | Frankfurt management |
| 10.100.0.0/16 | PROD | London production |
| 10.200.0.0/16 | PROD | Frankfurt production |
| 10.10.1.0/24 | MGMT | London OOB management |
| 10.20.1.0/24 | MGMT | Frankfurt OOB management |
| 10.100.10.0/24 | PROD | London compute servers |
| 10.100.20.0/24 | PROD | London network devices |
| 10.200.10.0/24 | PROD | Frankfurt compute servers |
| 10.200.20.0/24 | PROD | Frankfurt network devices |

---

## Section 1: DCIM — Devices

### 1.1 List all devices
```
netbox_dcim_devices_list()
```
- `count` ≥ 20
- `has_more: false` (we have ~20 devices total)
- No `primary_ip` field in any result
- No `null` fields

### 1.2 Filter by site
```
netbox_dcim_devices_list(site="london-dc")
```
- All results have `site.slug == "london-dc"`
- Includes `lon-core-sw-01`, `lon-edge-fw-01`, etc.

### 1.3 Filter by role
```
netbox_dcim_devices_list(role="server")
```
- Returns only server-role devices
- Includes `lon-compute-01`, `fra-compute-01`, etc.

### 1.4 Filter by name (exact)
```
netbox_dcim_devices_list(name="lon-core-sw-01")
```
- Returns exactly 1 result
- `results[0].name == "lon-core-sw-01"`

### 1.5 Get device by ID
```
netbox_dcim_devices_get(id=<id from 1.4>)
```
- Same device; `primary_ip` absent; `primary_ip4` present if assigned

### 1.6 Check primary IP
Compute servers (`lon-compute-01..04`, `fra-compute-01..02`) have primary IPs assigned. Verify:
- `primary_ip4` present and contains an address string
- No `primary_ip` field

---

## Section 2: DCIM — Interfaces

### 2.1 List interfaces for a device
```
netbox_dcim_interfaces_list(device="lon-core-sw-01")
```
- Returns interfaces for that device only
- Should include `Management0`, `GigabitEthernet1/0/1`, etc.

### 2.2 Get interface by ID
```
netbox_dcim_interfaces_get(id=<id>)
```
- Single interface object; no nulls

---

## Section 3: DCIM — Sites, Racks, Regions

### 3.1 List sites
```
netbox_dcim_sites_list()
```
- Includes `london-dc` and `frankfurt-dc`
- Both have `status.value == "active"`

### 3.2 List regions
```
netbox_dcim_regions_list()
```
- Includes `Europe`; both sites roll up to it

### 3.3 List racks
```
netbox_dcim_racks_list(site="london-dc")
```
- Returns racks at London site

---

## Section 4: Virtualization — VMs

### 4.1 List all VMs
```
netbox_virtualization_vms_list()
```
- `count` ≥ 7
- No `local_context_data` field in any result — **critical invariant**
- No `primary_ip` field

### 4.2 Filter by cluster
```
netbox_virtualization_vms_list(cluster="lon-vmware-cluster-01")
```
- Returns only London VMs: `lon-web-01`, `lon-web-02`, `lon-web-03`, `lon-db-01`, `lon-cache-01`

### 4.3 Filter by site
```
netbox_virtualization_vms_list(site="frankfurt-dc")
```
- Returns `fra-web-01`, `fra-db-01`

### 4.4 Filter by status
```
netbox_virtualization_vms_list(status="active")
```
- Returns all active VMs

### 4.5 Get VM by ID
```
netbox_virtualization_vms_get(id=<id>)
```
- No `local_context_data`
- No `primary_ip`
- `primary_ip4` present

---

## Section 5: IPAM — IP Addresses

### 5.1 List all IPs
```
netbox_ipam_ip_addresses_list()
```
- `count` ≥ 30 (devices + VMs all have assigned IPs)
- Pagination shape correct

### 5.2 Filter by device name
```
netbox_ipam_ip_addresses_list(device="lon-compute-01")
```
- Returns only IPs assigned to `lon-compute-01`'s interfaces
- Result should include the management IP (10.10.1.x) and production IP (10.100.10.x)

### 5.3 Filter by device ID
```
netbox_ipam_ip_addresses_list(device_id=<id of lon-compute-01>)
```
- Same results as 5.2

### 5.4 Filter by virtual machine name
```
netbox_ipam_ip_addresses_list(virtual_machine="lon-web-01")
```
- Returns IPs assigned to `lon-web-01`'s VM interface
- Should include its 10.100.x.x production IP

### 5.5 Filter by virtual machine ID
```
netbox_ipam_ip_addresses_list(virtual_machine_id=<id of lon-web-01>)
```
- Same results as 5.4

### 5.6 Filter by parent prefix (containment)
```
netbox_ipam_ip_addresses_list(parent="10.100.10.0/24")
```
- Returns all IPs within the London compute subnet
- Should include IPs for `lon-compute-01..04`
- Should NOT include Frankfurt or management IPs

### 5.7 Filter by VRF
```
netbox_ipam_ip_addresses_list(vrf_id=<PROD VRF id>)
```
- Returns only IPs in the PROD VRF

### 5.8 Filter by status
```
netbox_ipam_ip_addresses_list(status="active")
```
- All results have `status.value == "active"`

### 5.9 Get IP by ID
```
netbox_ipam_ip_addresses_get(id=<id>)
```
- Single IP object; no nulls

---

## Section 6: IPAM — Prefixes

### 6.1 List all prefixes
```
netbox_ipam_prefixes_list()
```
- `count` ≥ 12 (see seed data reference)

### 6.2 Filter by VRF
```
netbox_ipam_prefixes_list(vrf_id=<PROD VRF id>)
```
- Returns only PROD prefixes

### 6.3 Filter by site
```
netbox_ipam_prefixes_list(site="london-dc")
```
- Returns London-scoped prefixes

### 6.4 Filter by prefix exact
```
netbox_ipam_prefixes_list(prefix="10.100.10.0/24")
```
- Returns exactly 1 result matching that prefix

### 6.5 Get prefix by ID
```
netbox_ipam_prefixes_get(id=<id>)
```
- Single prefix object; no nulls

---

## Section 7: IPAM — VRFs

### 7.1 List VRFs
```
netbox_ipam_vrfs_list()
```
- Includes `MGMT` and `PROD` VRFs

### 7.2 Filter by name
```
netbox_ipam_vrfs_list(name="PROD")
```
- Returns exactly 1 result

### 7.3 Get VRF by ID
```
netbox_ipam_vrfs_get(id=<id>)
```
- Single VRF object; no nulls

---

## Section 8: IPAM — Services

### 8.1 List all services
```
netbox_ipam_services_list()
```
- Includes services like `http`, `https`, `ssh`, `postgres`, `redis`

### 8.2 Filter by device (if supported)
```
netbox_ipam_services_list(device_id=<id>)
```
- Returns services attached to that device

### 8.3 Get service by ID
```
netbox_ipam_services_get(id=<id>)
```
- Single service object; no nulls

---

## Section 9: Virtualization — Clusters

### 9.1 List clusters
```
netbox_virtualization_clusters_list()
```
- Includes `lon-vmware-cluster-01` and `fra-vmware-cluster-01`

### 9.2 Filter by site
```
netbox_virtualization_clusters_list(site="london-dc")
```
- Returns only London cluster

### 9.3 Get cluster by ID
```
netbox_virtualization_clusters_get(id=<id>)
```
- Single cluster object; no nulls

---

## Section 10: Extras — Object Changes

### 10.1 List recent changes
```
netbox_core_object_changes_list(limit=5)
```
- Returns up to 5 changelog entries
- Each entry has `prechange_data` and `postchange_data` — both may be large

### 10.2 Filter by object type
```
netbox_core_object_changes_list(changed_object_type="dcim.device", limit=5)
```
- Returns only changes to device objects

### 10.3 Get change by ID
```
netbox_core_object_changes_get(id=<id>)
```
- Single change record

---

## Section 11: Meta-Tool — `netbox_lookup_host`

### 11.1 Lookup a device by name
```
netbox_lookup_host(name="lon-core-sw-01")
```
- Response shape: `{ devices: [...], virtual_machines: [...], total_matches: N }`
- `devices` has 1 result; `virtual_machines` is empty
- `total_matches == 1`

### 11.2 Lookup a VM by name
```
netbox_lookup_host(name="lon-web-01")
```
- `devices` is empty; `virtual_machines` has 1 result
- `total_matches == 1`
- No `local_context_data` in the VM result

### 11.3 Lookup with no match
```
netbox_lookup_host(name="nonexistent-host-xyz")
```
- `total_matches == 0`
- Both `devices` and `virtual_machines` are empty arrays

### 11.4 Lookup with ambiguous/partial name
```
netbox_lookup_host(name="lon-compute")
```
- May return multiple devices matching the prefix
- All returned objects must satisfy universal invariants

---

## Section 12: Pagination End-to-End

### 12.1 First page
```
netbox_ipam_ip_addresses_list(limit=5, offset=0)
```
- `results` has ≤ 5 items
- `has_more: true` (we have ≥ 30 IPs)
- `next_offset` == 5
- No `next`/`previous` URL fields

### 12.2 Walk pages
Continue fetching with `offset=5`, `offset=10`, etc. until `has_more: false`.
- Each page: `results` non-empty, correct `next_offset`
- Last page: `has_more: false`, `next_offset` == `count`
- Total items across all pages == `count` from first response

### 12.3 Fetch all at once
```
netbox_ipam_ip_addresses_list(fetch_all=true)
```
- `results` length == `count`
- `has_more: false`
- `next_offset` == `count`

---

## Cross-Tool Workflows

### Workflow A: Device IP discovery
1. `netbox_lookup_host(name="lon-compute-01")` — get device ID
2. `netbox_ipam_ip_addresses_list(device_id=<id>)` — get all IPs
3. `netbox_ipam_prefixes_list(prefix="10.100.10.0/24")` — verify subnet membership

### Workflow B: VM configuration audit
1. `netbox_virtualization_vms_list(cluster="lon-vmware-cluster-01")` — list cluster VMs
2. For each VM: `netbox_ipam_ip_addresses_list(virtual_machine_id=<id>)` — verify IPs present
3. Verify no `local_context_data` in any VM response

### Workflow C: Prefix-based IP inventory
1. `netbox_ipam_vrfs_list(name="PROD")` — get PROD VRF ID
2. `netbox_ipam_prefixes_list(vrf_id=<id>)` — list all PROD prefixes
3. `netbox_ipam_ip_addresses_list(parent="10.100.10.0/24")` — get IPs in compute subnet

### Workflow D: Change audit
1. `netbox_core_object_changes_list(limit=10)` — recent changes
2. For an interesting change: `netbox_core_object_changes_get(id=<id>)`
3. Verify `prechange_data` and `postchange_data` both present

---

## Error Handling Checks

| Scenario | Expected behavior |
|---|---|
| `netbox_dcim_devices_get(id=999999)` | Tool returns error or empty; no crash |
| `netbox_ipam_ip_addresses_list(device="nonexistent-device")` | `count: 0`, `results: []` |
| `netbox_lookup_host(name="")` | Error or empty result; no crash |
| Invalid filter value | NetBox 400 surfaced cleanly; no stack trace |

---

## Checklist Summary

Run through these in order for a complete regression pass:

- [ ] Universal invariants (no nulls, no `local_context_data`, no `primary_ip`, pagination shape)
- [ ] Section 1: Devices list, site filter, role filter, name filter, get by ID, primary IP check
- [ ] Section 2: Interfaces list by device, get by ID
- [ ] Section 3: Sites, regions, racks
- [ ] Section 4: VMs list, cluster filter, site filter, get by ID — confirm no `local_context_data`
- [ ] Section 5: IP list, device filter, device_id filter, VM filter, VM_id filter, parent filter
- [ ] Section 6: Prefixes list, VRF filter, prefix exact filter
- [ ] Section 7: VRFs list and get
- [ ] Section 8: Services list and get
- [ ] Section 9: Clusters list, site filter, get
- [ ] Section 10: Object changes list and get
- [ ] Section 11: `netbox_lookup_host` — device, VM, no match, ambiguous
- [ ] Section 12: Pagination — first page, walk pages, fetch_all
- [ ] Cross-tool workflows A, B, C, D
- [ ] Error handling scenarios

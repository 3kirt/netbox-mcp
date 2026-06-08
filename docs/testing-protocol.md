# NetBox MCP Testing Protocol

This document describes how to use the MCP tools to verify that all functionality is working correctly against a local NetBox instance seeded with `scripts/seed_data.py`.

The quickest way to get such an instance is the bundled test stack in [`../test/netbox-docker/`](../test/netbox-docker/), which boots NetBox and seeds it automatically (`cd test/netbox-docker && ./up.sh`). For the automated version of these checks, see the live test suite described in [`testing.md`](testing.md).

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
| `next_offset` always present | `next_offset` must always be present — equals `count` when `has_more: false` |

---

## Seed Data Reference

The seed script creates the following objects. Use these as ground truth for filter assertions.

### Regions and Sites
| Name | Slug | Region |
|---|---|---|
| New York DC | nyc-dc | North America |
| London DC | lon-dc | Europe |
| Frankfurt DC | fra-dc | Europe |

### Racks
| Name | Site |
|---|---|
| NYC-A01 | nyc-dc |
| NYC-A02 | nyc-dc |
| LON-A01 | lon-dc |
| FRA-A01 | fra-dc |

### Devices (12 total)
| Name | Type | Role | Site |
|---|---|---|---|
| nyc-spine-01 | Cisco Nexus 9300 | Spine Switch | nyc-dc |
| nyc-spine-02 | Cisco Nexus 9300 | Spine Switch | nyc-dc |
| nyc-leaf-01 | Juniper QFX5100 | Leaf Switch | nyc-dc |
| nyc-leaf-02 | Juniper QFX5100 | Leaf Switch | nyc-dc |
| nyc-router-01 | Cisco ASR 1001-X | Core Router | nyc-dc |
| nyc-server-01 | Dell PowerEdge R750 | Server | nyc-dc |
| nyc-server-02 | Dell PowerEdge R750 | Server | nyc-dc |
| lon-spine-01 | Cisco Nexus 9300 | Spine Switch | lon-dc |
| lon-leaf-01 | Juniper QFX5100 | Leaf Switch | lon-dc |
| lon-router-01 | Juniper MX204 | Core Router | lon-dc |
| fra-spine-01 | Cisco Nexus 9300 | Spine Switch | fra-dc |
| fra-server-01 | Dell PowerEdge R750 | Server | fra-dc |

### Device Interfaces (examples)
- **Spine switches** (`nyc-spine-01/02`, `lon-spine-01`, `fra-spine-01`): `mgmt0` (1000base-t, mgmt_only), `Loopback0`, `Ethernet1/1` – `Ethernet1/4`
- **Leaf switches** (`nyc-leaf-01/02`, `lon-leaf-01`): `em0` (1000base-t, mgmt_only), `lo0`, `xe-0/0/0` – `xe-0/0/3`
- **nyc-router-01** (IOS-XE): `GigabitEthernet0/0/0` (mgmt), `Loopback0`, `GigabitEthernet0/1/0`, `GigabitEthernet0/1/1`
- **lon-router-01** (Junos): `em0` (mgmt), `lo0`, `xe-0/0/0`, `xe-0/0/1`
- **Servers** (`nyc-server-01/02`, `fra-server-01`): `idrac` (mgmt_only), `bond0` (lag), `eth0`, `eth1`

### VRFs
| Name | Route Distinguisher |
|---|---|
| Global | 65000:0 |
| Management | 65000:100 |

### Prefixes (14 total)
| Prefix | VRF | Site | Description |
|---|---|---|---|
| 10.0.0.0/8 | — | — | Global private space (container) |
| 10.0.0.0/16 | Global | nyc-dc | NYC DC block |
| 10.1.0.0/16 | Global | lon-dc | LON DC block |
| 10.2.0.0/16 | Global | fra-dc | FRA DC block |
| 10.0.0.0/24 | Management | nyc-dc | NYC management |
| 10.0.1.0/24 | Global | nyc-dc | NYC servers |
| 10.0.2.0/24 | Global | nyc-dc | NYC VMs |
| 10.0.10.0/24 | Global | nyc-dc | NYC loopbacks |
| 10.1.0.0/24 | Management | lon-dc | LON management |
| 10.1.1.0/24 | Global | lon-dc | LON servers |
| 10.1.10.0/24 | Global | lon-dc | LON loopbacks |
| 10.2.0.0/24 | Management | fra-dc | FRA management |
| 10.2.1.0/24 | Global | fra-dc | FRA servers |
| 10.2.10.0/24 | Global | fra-dc | FRA loopbacks |

### IP Addresses (31 total)
Each device has 2 IPs (management + loopback/data). Servers have management (idrac) + data (bond0):

| Device/VM | Interface | Address | VRF |
|---|---|---|---|
| nyc-spine-01 | mgmt0 | 10.0.0.1/24 | Management |
| nyc-spine-01 | Loopback0 | 10.0.10.1/32 | Global |
| nyc-spine-02 | mgmt0 | 10.0.0.2/24 | Management |
| nyc-spine-02 | Loopback0 | 10.0.10.2/32 | Global |
| nyc-leaf-01 | em0 | 10.0.0.3/24 | Management |
| nyc-leaf-01 | lo0 | 10.0.10.3/32 | Global |
| nyc-leaf-02 | em0 | 10.0.0.4/24 | Management |
| nyc-leaf-02 | lo0 | 10.0.10.4/32 | Global |
| nyc-router-01 | GigabitEthernet0/0/0 | 10.0.0.5/24 | Management |
| nyc-router-01 | Loopback0 | 10.0.10.5/32 | Global |
| nyc-server-01 | idrac | 10.0.0.6/24 | Management |
| nyc-server-01 | bond0 | 10.0.1.10/24 | Global |
| nyc-server-02 | idrac | 10.0.0.7/24 | Management |
| nyc-server-02 | bond0 | 10.0.1.11/24 | Global |
| lon-spine-01 | mgmt0 | 10.1.0.1/24 | Management |
| lon-spine-01 | Loopback0 | 10.1.10.1/32 | Global |
| lon-leaf-01 | em0 | 10.1.0.2/24 | Management |
| lon-leaf-01 | lo0 | 10.1.10.2/32 | Global |
| lon-router-01 | em0 | 10.1.0.3/24 | Management |
| lon-router-01 | lo0 | 10.1.10.3/32 | Global |
| fra-spine-01 | mgmt0 | 10.2.0.1/24 | Management |
| fra-spine-01 | Loopback0 | 10.2.10.1/32 | Global |
| fra-server-01 | idrac | 10.2.0.2/24 | Management |
| fra-server-01 | bond0 | 10.2.1.10/24 | Global |
| web-prod-01 | eth0 | 10.0.2.10/24 | Global |
| web-prod-02 | eth0 | 10.0.2.11/24 | Global |
| db-prod-01 | eth0 | 10.0.2.20/24 | Global |
| cache-prod-01 | eth0 | 10.0.2.30/24 | Global |
| mon-prod-01 | eth0 | 10.0.2.40/24 | Global |
| web-lon-01 | eth0 | 10.1.1.10/24 | Global |
| web-lon-02 | eth0 | 10.1.1.11/24 | Global |

### Clusters and Virtual Machines
| Cluster | Site | VMs |
|---|---|---|
| NYC-PROD | nyc-dc | web-prod-01, web-prod-02, db-prod-01, cache-prod-01, mon-prod-01 |
| LON-PROD | lon-dc | web-lon-01, web-lon-02 |

All VMs have `local_context_data` set — this must never appear in any response.

### Services (11 total)
| Object | Service | Protocol | Port |
|---|---|---|---|
| nyc-server-01 | ssh | tcp | 22 |
| nyc-server-02 | ssh | tcp | 22 |
| web-prod-01 | https | tcp | 443 |
| web-prod-01 | http | tcp | 80 |
| web-prod-02 | https | tcp | 443 |
| web-prod-02 | http | tcp | 80 |
| db-prod-01 | postgresql | tcp | 5432 |
| cache-prod-01 | redis | tcp | 6379 |
| mon-prod-01 | prometheus | tcp | 9090 |
| web-lon-01 | https | tcp | 443 |
| web-lon-02 | https | tcp | 443 |

---

## Section 1: DCIM — Devices

### 1.1 List all devices
```
netbox_dcim_devices_list()
```
- `count == 12`
- `has_more: false`
- No `primary_ip` field in any result
- No `null` fields

### 1.2 Filter by site
```
netbox_dcim_devices_list(site="lon-dc")
```
- All results have `site.slug == "lon-dc"`
- Includes `lon-spine-01`, `lon-leaf-01`, `lon-router-01` (3 devices)

### 1.3 Filter by role
```
netbox_dcim_devices_list(role="server")
```
- Returns only server-role devices
- Includes `nyc-server-01`, `nyc-server-02`, `fra-server-01`

### 1.4 Filter by name (exact)
```
netbox_dcim_devices_list(name="nyc-spine-01")
```
- Returns exactly 1 result
- `results[0].name == "nyc-spine-01"`

### 1.5 Get device by ID
```
netbox_dcim_devices_get(id=<id from 1.4>)
```
- Same device; `primary_ip` absent; `primary_ip4` present (set to `10.0.0.1/24`)

### 1.6 Check primary IP
Servers (`nyc-server-01`, `nyc-server-02`, `fra-server-01`) and network devices all have `primary_ip4` assigned. Verify:
- `primary_ip4` present and contains an address string
- No `primary_ip` field

---

## Section 2: DCIM — Interfaces

### 2.1 List interfaces for a device
```
netbox_dcim_interfaces_list(device="lon-spine-01")
```
- Returns interfaces for `lon-spine-01` only
- Should include `mgmt0`, `Loopback0`, `Ethernet1/1`, `Ethernet1/2`, `Ethernet1/3`, `Ethernet1/4`

### 2.2 Filter management-only interfaces
```
netbox_dcim_interfaces_list(device="nyc-server-01", mgmt_only=true)
```
- Returns only the `idrac` interface

### 2.3 Get interface by ID
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
- Includes `nyc-dc`, `lon-dc`, and `fra-dc`
- All have `status.value == "active"`

### 3.2 List regions
```
netbox_dcim_regions_list()
```
- Includes `North America` and `Europe`
- London DC and Frankfurt DC roll up to `Europe`

### 3.3 List racks
```
netbox_dcim_racks_list(site="lon-dc")
```
- Returns `LON-A01`

---

## Section 4: Virtualization — VMs

### 4.1 List all VMs
```
netbox_virtualization_vms_list()
```
- `count == 7`
- `has_more: false`
- No `local_context_data` field in any result — **critical invariant**
- No `primary_ip` field

### 4.2 Filter by cluster
```
netbox_virtualization_vms_list(cluster="LON-PROD")
```
- Returns only London VMs: `web-lon-01`, `web-lon-02`

### 4.3 Filter by site
```
netbox_virtualization_vms_list(site="nyc-dc")
```
- Returns 5 VMs: `web-prod-01`, `web-prod-02`, `db-prod-01`, `cache-prod-01`, `mon-prod-01`

### 4.4 Filter by status
```
netbox_virtualization_vms_list(status="active")
```
- Returns all 7 VMs

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
- `count == 31` (24 device IPs + 7 VM IPs)
- `has_more: false` (31 < 50 default limit)
- Pagination shape correct

### 5.2 Filter by device name
```
netbox_ipam_ip_addresses_list(device="nyc-server-01")
```
- Returns 2 IPs: `10.0.0.6/24` (Management, idrac) and `10.0.1.10/24` (Global, bond0)

### 5.3 Filter by device ID
```
netbox_ipam_ip_addresses_list(device_id=<id of nyc-server-01>)
```
- Same 2 results as 5.2

### 5.4 Filter by virtual machine name
```
netbox_ipam_ip_addresses_list(virtual_machine="web-prod-01")
```
- Returns 1 IP: `10.0.2.10/24` (Global, eth0)

### 5.5 Filter by virtual machine ID
```
netbox_ipam_ip_addresses_list(virtual_machine_id=<id of web-prod-01>)
```
- Same result as 5.4

### 5.6 Filter by parent prefix (containment)
```
netbox_ipam_ip_addresses_list(parent="10.0.1.0/24")
```
- Returns IPs within NYC servers subnet: `10.0.1.10/24` (nyc-server-01) and `10.0.1.11/24` (nyc-server-02)
- Should NOT include Management or loopback IPs

### 5.7 Filter by VRF
```
netbox_ipam_ip_addresses_list(vrf_id=<Management VRF id>)
```
- Returns only management IPs (10.x.0.x/24 addresses across all sites)

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
- `count == 14`

### 6.2 Filter by VRF
```
netbox_ipam_prefixes_list(vrf_id=<Global VRF id>)
```
- Returns Global VRF prefixes (10 results: the three /16 site blocks plus all non-management /24s)

### 6.3 Filter by site
```
netbox_ipam_prefixes_list(site="lon-dc")
```
- Returns London-scoped prefixes: `10.1.0.0/16`, `10.1.0.0/24`, `10.1.1.0/24`, `10.1.10.0/24`

### 6.4 Filter by prefix exact
```
netbox_ipam_prefixes_list(prefix="10.1.0.0/24")
```
- Returns exactly 1 result: London management prefix

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
- `count == 2`
- Includes `Global` (rd: 65000:0) and `Management` (rd: 65000:100)

### 7.2 Filter by name
```
netbox_ipam_vrfs_list(name="Global")
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
- `count == 11`
- Includes services: `ssh`, `http`, `https`, `postgresql`, `redis`, `prometheus`

### 8.2 Filter by device ID
```
netbox_ipam_services_list(device_id=<id of nyc-server-01>)
```
- Returns `ssh` service on port 22

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
- `count == 2`
- Includes `NYC-PROD` and `LON-PROD`

### 9.2 Filter by site
```
netbox_virtualization_clusters_list(site="lon-dc")
```
- Returns only `LON-PROD`

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
netbox_lookup_host(name="lon-spine-01")
```
- Response shape: `{ devices: [...], virtual_machines: [...], total_matches: N, has_more: bool }`
- `devices` has 1 result; `virtual_machines` is empty
- `total_matches == 1`, `has_more: false`

### 11.2 Lookup a VM by name
```
netbox_lookup_host(name="web-prod-01")
```
- `devices` is empty; `virtual_machines` has 1 result
- `total_matches == 1`, `has_more: false`
- No `local_context_data` in the VM result

### 11.3 Lookup with no match
```
netbox_lookup_host(name="nonexistent-host-xyz")
```
- `total_matches == 0`, `has_more: false`
- Both `devices` and `virtual_machines` are empty arrays

### 11.4 Lookup with ambiguous/partial name
```
netbox_lookup_host(name="nyc-server")
```
- Returns 2 devices: `nyc-server-01` and `nyc-server-02`
- `total_matches == 2`, `has_more: false`
- All returned objects must satisfy universal invariants

---

## Section 12: Pagination End-to-End

### 12.1 First page
```
netbox_ipam_ip_addresses_list(limit=5, offset=0)
```
- `results` has exactly 5 items
- `has_more: true` (we have 31 IPs total)
- `next_offset == 5`
- No `next`/`previous` URL fields

### 12.2 Walk pages
Continue fetching with `offset=5`, `offset=10`, etc. until `has_more: false`.
- Each page: `results` non-empty, correct `next_offset`
- Last page: `has_more: false`, `next_offset == count` (== 31)
- Total items across all pages == `count` from first response

### 12.3 Fetch all at once
```
netbox_ipam_ip_addresses_list(fetch_all=true)
```
- `results` length == `count` == 31
- `has_more: false`
- `next_offset == 31`

---

## Cross-Tool Workflows

### Workflow A: Device IP discovery
1. `netbox_lookup_host(name="nyc-server-01")` — get device ID
2. `netbox_ipam_ip_addresses_list(device_id=<id>)` — get both IPs (management + data)
3. `netbox_ipam_prefixes_list(prefix="10.0.1.0/24")` — verify the data IP falls in the NYC servers subnet

### Workflow B: VM configuration audit
1. `netbox_virtualization_vms_list(cluster="LON-PROD")` — list cluster VMs (`web-lon-01`, `web-lon-02`)
2. For each VM: `netbox_ipam_ip_addresses_list(virtual_machine_id=<id>)` — verify IPs present
3. Verify no `local_context_data` in any VM response

### Workflow C: Prefix-based IP inventory
1. `netbox_ipam_vrfs_list(name="Global")` — get Global VRF ID
2. `netbox_ipam_prefixes_list(vrf_id=<id>)` — list all Global prefixes
3. `netbox_ipam_ip_addresses_list(parent="10.0.1.0/24")` — get IPs in NYC servers subnet

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
| `netbox_lookup_host(name="")` | Empty result or broad dump (no guard — see open issue); no crash |
| Invalid filter value | NetBox 400 surfaced cleanly; no stack trace |

---

## Checklist Summary

Run through these in order for a complete regression pass:

- [ ] Universal invariants (no nulls, no `local_context_data`, no `primary_ip`, pagination shape, `next_offset` always present)
- [ ] Section 1: Devices list, site filter, role filter, name filter, get by ID, primary IP check
- [ ] Section 2: Interfaces list by device, mgmt_only filter, get by ID
- [ ] Section 3: Sites, regions, racks
- [ ] Section 4: VMs list, cluster filter, site filter, get by ID — confirm no `local_context_data`
- [ ] Section 5: IP list, device filter, device_id filter, VM filter, VM_id filter, parent filter, VRF filter
- [ ] Section 6: Prefixes list, VRF filter, site filter, prefix exact filter
- [ ] Section 7: VRFs list and get
- [ ] Section 8: Services list, device_id filter, get
- [ ] Section 9: Clusters list, site filter, get
- [ ] Section 10: Object changes list and get
- [ ] Section 11: `netbox_lookup_host` — device, VM, no match, ambiguous; verify `has_more` field present
- [ ] Section 12: Pagination — first page, walk pages, fetch_all; verify `next_offset` on last page == `count`
- [ ] Cross-tool workflows A, B, C, D
- [ ] Error handling scenarios

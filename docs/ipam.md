# IPAM Tools

All tools are read-only. They return JSON-marshalled NetBox responses as text content. If NetBox returns an error, the tool result has `isError: true` and the content contains the error message.

---

### `netbox_ipam_ip_addresses_list`

List IP addresses, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `address` | string | no | IP address with prefix length (e.g. `192.0.2.1/24`) |
| `vrf` | string | no | VRF name |
| `status` | string | no | `active`, `reserved`, `deprecated`, `dhcp`, or `slaac` |
| `tenant` | string | no | Tenant name or slug |
| `device` | string | no | Device name |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `IPAddress` objects.

---

### `netbox_ipam_ip_addresses_get`

Get a single IP address by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox IP address ID |

Returns a single `IPAddress` object.

---

### `netbox_ipam_prefixes_list`

List IP prefixes, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `prefix` | string | no | Prefix in CIDR notation (e.g. `192.0.2.0/24`) |
| `vrf` | string | no | VRF name |
| `status` | string | no | `active`, `container`, `reserved`, or `deprecated` |
| `site` | string | no | Site name or slug |
| `tenant` | string | no | Tenant name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Prefix` objects.

---

### `netbox_ipam_prefixes_get`

Get a single IP prefix by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox prefix ID |

Returns a single `Prefix` object.

---

### `netbox_ipam_vrfs_list`

List VRFs, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | VRF name |
| `rd` | string | no | Route distinguisher |
| `tenant` | string | no | Tenant name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `VRF` objects.

---

### `netbox_ipam_vrfs_get`

Get a single VRF by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox VRF ID |

Returns a single `VRF` object.

---

### `netbox_ipam_vlans_list`

List VLANs, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `vid` | integer | no | VLAN ID number (1–4094) |
| `name` | string | no | VLAN name |
| `site` | string | no | Site name or slug |
| `group` | string | no | VLAN group name or slug |
| `status` | string | no | `active`, `reserved`, or `deprecated` |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `VLAN` objects.

---

### `netbox_ipam_vlans_get`

Get a single VLAN by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox VLAN ID |

Returns a single `VLAN` object.

---

### `netbox_ipam_aggregates_list`

List IP aggregates, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `family` | integer | no | Address family: `4` (IPv4) or `6` (IPv6) |
| `rir` | string | no | RIR name or slug |
| `tenant` | string | no | Tenant name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Aggregate` objects.

---

### `netbox_ipam_aggregates_get`

Get a single aggregate by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox aggregate ID |

Returns a single `Aggregate` object.

---

### `netbox_ipam_ip_ranges_list`

List IP ranges, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `vrf` | string | no | VRF name |
| `status` | string | no | `active`, `reserved`, or `deprecated` |
| `tenant` | string | no | Tenant name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `IPRange` objects.

---

### `netbox_ipam_ip_ranges_get`

Get a single IP range by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox IP range ID |

Returns a single `IPRange` object.

---

### `netbox_ipam_route_targets_list`

List route targets, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Route target name |
| `tenant` | string | no | Tenant name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `RouteTarget` objects.

---

### `netbox_ipam_route_targets_get`

Get a single route target by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox route target ID |

Returns a single `RouteTarget` object.

---

### `netbox_ipam_rirs_list`

List RIRs (Regional Internet Registries), with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | RIR name |
| `slug` | string | no | RIR slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `RIR` objects.

---

### `netbox_ipam_rirs_get`

Get a single RIR by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox RIR ID |

Returns a single `RIR` object.

---

### `netbox_ipam_vlan_groups_list`

List VLAN groups, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | VLAN group name |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `VLANGroup` objects.

---

### `netbox_ipam_vlan_groups_get`

Get a single VLAN group by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox VLAN group ID |

Returns a single `VLANGroup` object.

---

### `netbox_ipam_services_list`

List services, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `device_id` | integer | no | Device ID |
| `virtual_machine_id` | integer | no | Virtual machine ID |
| `protocol` | string | no | Protocol (e.g. `tcp`, `udp`) |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Service` objects.

---

### `netbox_ipam_services_get`

Get a single service by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox service ID |

Returns a single `Service` object.

---

### `netbox_ipam_asns_list`

List ASNs (Autonomous System Numbers), with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `site` | string[] | no | Site name or slug to filter by |
| `tenant` | string[] | no | Tenant name or slug to filter by |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `ASN` objects.

---

### `netbox_ipam_asns_get`

Get a single ASN by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox ASN ID |

Returns a single `ASN` object.

---

### `netbox_ipam_fhrp_groups_list`

List FHRP (First Hop Redundancy Protocol) groups, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Group name(s) to filter by |
| `protocol` | string[] | no | Protocol(s) to filter by (e.g. `vrrp2`, `vrrp3`, `carp`, `clusterxl`, `hsrp`, `glbp`) |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `FHRPGroup` objects.

---

### `netbox_ipam_fhrp_groups_get`

Get a single FHRP group by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox FHRP group ID |

Returns a single `FHRPGroup` object.

---

### `netbox_ipam_fhrp_group_assignments_list`

List FHRP group assignments, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `group_id` | integer | no | FHRP group ID to filter by |
| `device_id` | integer | no | Device ID to filter by |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `FHRPGroupAssignment` objects.

---

### `netbox_ipam_fhrp_group_assignments_get`

Get a single FHRP group assignment by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox FHRP group assignment ID |

Returns a single `FHRPGroupAssignment` object.

---

### `netbox_ipam_roles_list`

List IP roles, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Role name(s) to filter by |
| `slug` | string[] | no | Role slug(s) to filter by |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Role` objects.

---

### `netbox_ipam_roles_get`

Get a single IP role by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox IP role ID |

Returns a single `Role` object.

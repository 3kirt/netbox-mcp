# Tool Reference

All tools are read-only. They return JSON-marshalled NetBox responses as text content. If NetBox returns an error, the tool result has `isError: true` and the content contains the error message.

---

## DCIM

### `netbox_dcim_devices_list`

List devices, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `site` | string | no | Site name or slug |
| `role` | string | no | Device role name or slug |
| `status` | string | no | `active`, `planned`, `staged`, `failed`, `inventory`, or `decommissioning` |
| `rack_id` | integer | no | Rack ID |
| `limit` | integer | no | Maximum results (default 50) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `DeviceWithConfigContext` objects.

---

### `netbox_dcim_devices_get`

Get a single device by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox device ID |

Returns a single `DeviceWithConfigContext` object.

---

### `netbox_dcim_sites_list`

List sites, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `name` | string | no | Site name |
| `status` | string | no | `active`, `planned`, `retired`, or `staging` |
| `region` | string | no | Region name or slug |
| `limit` | integer | no | Maximum results (default 50) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Site` objects.

---

### `netbox_dcim_sites_get`

Get a single site by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox site ID |

Returns a single `Site` object.

---

### `netbox_dcim_racks_list`

List racks, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `site` | string | no | Site name or slug |
| `location` | string | no | Location name or slug |
| `status` | string | no | `active`, `planned`, `reserved`, `available`, or `deprecated` |
| `limit` | integer | no | Maximum results (default 50) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Rack` objects.

---

### `netbox_dcim_racks_get`

Get a single rack by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox rack ID |

Returns a single `Rack` object.

---

### `netbox_dcim_interfaces_list`

List device interfaces, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `device_id` | integer | no | Device ID |
| `name` | string | no | Interface name |
| `type` | string | no | Interface type (e.g. `1000base-t`, `10gbase-x-sfpp`) |
| `limit` | integer | no | Maximum results (default 50) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Interface` objects.

---

### `netbox_dcim_cables_list`

List cables, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `site` | string | no | Site name or slug |
| `status` | string | no | `connected`, `planned`, or `decommissioning` |
| `limit` | integer | no | Maximum results (default 50) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Cable` objects.

---

## IPAM

### `netbox_ipam_ip_addresses_list`

List IP addresses, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `address` | string | no | IP address with prefix length (e.g. `192.0.2.1/24`) |
| `vrf` | string | no | VRF name |
| `status` | string | no | `active`, `reserved`, `deprecated`, `dhcp`, or `slaac` |
| `tenant` | string | no | Tenant name or slug |
| `device` | string | no | Device name |
| `limit` | integer | no | Maximum results (default 50) |
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
| `prefix` | string | no | Prefix in CIDR notation (e.g. `192.0.2.0/24`) |
| `vrf` | string | no | VRF name |
| `status` | string | no | `active`, `container`, `reserved`, or `deprecated` |
| `site` | string | no | Site name or slug |
| `tenant` | string | no | Tenant name or slug |
| `limit` | integer | no | Maximum results (default 50) |
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
| `name` | string | no | VRF name |
| `rd` | string | no | Route distinguisher |
| `tenant` | string | no | Tenant name or slug |
| `limit` | integer | no | Maximum results (default 50) |
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
| `vid` | integer | no | VLAN ID number (1–4094) |
| `name` | string | no | VLAN name |
| `site` | string | no | Site name or slug |
| `group` | string | no | VLAN group name or slug |
| `status` | string | no | `active`, `reserved`, or `deprecated` |
| `limit` | integer | no | Maximum results (default 50) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `VLAN` objects.

---

### `netbox_ipam_vlans_get`

Get a single VLAN by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox VLAN ID |

Returns a single `VLAN` object.

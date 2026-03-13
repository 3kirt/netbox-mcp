# IPAM Tools

All tools are read-only. They return JSON-marshalled NetBox responses as text content. If NetBox returns an error, the tool result has `isError: true` and the content contains the error message.

---

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

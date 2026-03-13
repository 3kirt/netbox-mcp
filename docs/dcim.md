# DCIM Tools

All tools are read-only. They return JSON-marshalled NetBox responses as text content. If NetBox returns an error, the tool result has `isError: true` and the content contains the error message.

---

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

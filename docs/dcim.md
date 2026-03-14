# DCIM Tools

All tools are read-only. They return JSON-marshalled NetBox responses as text content. If NetBox returns an error, the tool result has `isError: true` and the content contains the error message.

---

### `netbox_dcim_devices_list`

List devices, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `site` | string | no | Site name or slug |
| `role` | string | no | Device role name or slug |
| `status` | string | no | `active`, `planned`, `staged`, `failed`, `inventory`, or `decommissioning` |
| `rack_id` | integer | no | Rack ID |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
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
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Site name |
| `status` | string | no | `active`, `planned`, `retired`, or `staging` |
| `region` | string | no | Region name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
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
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `site` | string | no | Site name or slug |
| `location` | string | no | Location name or slug |
| `status` | string | no | `active`, `planned`, `reserved`, `available`, or `deprecated` |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
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
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `device_id` | integer | no | Device ID |
| `name` | string | no | Interface name |
| `type` | string | no | Interface type (e.g. `1000base-t`, `10gbase-x-sfpp`) |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Interface` objects.

---

### `netbox_dcim_interfaces_get`

Get a single device interface by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox interface ID |

Returns a single `Interface` object.

---

### `netbox_dcim_cables_list`

List cables, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `site` | string | no | Site name or slug |
| `status` | string | no | `connected`, `planned`, or `decommissioning` |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Cable` objects.

---

### `netbox_dcim_cables_get`

Get a single cable by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox cable ID |

Returns a single `Cable` object.

---

### `netbox_dcim_regions_list`

List regions, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Region name |
| `slug` | string | no | Region slug |
| `parent` | string | no | Parent region slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Region` objects.

---

### `netbox_dcim_regions_get`

Get a single region by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox region ID |

Returns a single `Region` object.

---

### `netbox_dcim_locations_list`

List locations, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `site` | string | no | Site name or slug |
| `parent` | string | no | Parent location slug |
| `status` | string | no | Location status |
| `tenant` | string | no | Tenant name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Location` objects.

---

### `netbox_dcim_locations_get`

Get a single location by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox location ID |

Returns a single `Location` object.

---

### `netbox_dcim_manufacturers_list`

List manufacturers, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Manufacturer name |
| `slug` | string | no | Manufacturer slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Manufacturer` objects.

---

### `netbox_dcim_manufacturers_get`

Get a single manufacturer by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox manufacturer ID |

Returns a single `Manufacturer` object.

---

### `netbox_dcim_device_types_list`

List device types, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `manufacturer` | string | no | Manufacturer name or slug |
| `model` | string | no | Model name |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `DeviceType` objects.

---

### `netbox_dcim_device_types_get`

Get a single device type by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox device type ID |

Returns a single `DeviceType` object.

---

### `netbox_dcim_device_roles_list`

List device roles, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Role name |
| `slug` | string | no | Role slug |
| `vm_role` | boolean | no | Filter to VM-eligible roles only |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `DeviceRole` objects.

---

### `netbox_dcim_device_roles_get`

Get a single device role by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox device role ID |

Returns a single `DeviceRole` object.

---

### `netbox_dcim_platforms_list`

List platforms, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Platform name |
| `manufacturer` | string | no | Manufacturer name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Platform` objects.

---

### `netbox_dcim_platforms_get`

Get a single platform by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox platform ID |

Returns a single `Platform` object.

---

### `netbox_dcim_power_panels_list`

List power panels, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `site` | string | no | Site name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `PowerPanel` objects.

---

### `netbox_dcim_power_panels_get`

Get a single power panel by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox power panel ID |

Returns a single `PowerPanel` object.

---

### `netbox_dcim_power_feeds_list`

List power feeds, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `site` | string | no | Site name or slug |
| `status` | string | no | `active`, `offline`, or `planned` |
| `type` | string | no | `primary` or `redundant` |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `PowerFeed` objects.

---

### `netbox_dcim_power_feeds_get`

Get a single power feed by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox power feed ID |

Returns a single `PowerFeed` object.

---

### `netbox_dcim_virtual_chassis_list`

List virtual chassis, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `site` | string | no | Site name or slug |
| `tenant` | string | no | Tenant name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `VirtualChassis` objects.

---

### `netbox_dcim_virtual_chassis_get`

Get a single virtual chassis by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox virtual chassis ID |

Returns a single `VirtualChassis` object.

---

### `netbox_dcim_inventory_items_list`

List inventory items, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `device_id` | integer | no | Device ID |
| `manufacturer` | string | no | Manufacturer name or slug |
| `discovered` | boolean | no | Filter to auto-discovered items only |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `InventoryItem` objects.

---

### `netbox_dcim_inventory_items_get`

Get a single inventory item by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox inventory item ID |

Returns a single `InventoryItem` object.

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

---

### `netbox_dcim_cable_terminations_list`

List cable terminations, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `cable_id` | integer | no | Cable ID |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `CableTermination` objects.

---

### `netbox_dcim_cable_terminations_get`

Get a single cable termination by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox cable termination ID |

Returns a single `CableTermination` object.

---

### `netbox_dcim_console_ports_list`

List console ports, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Console port name(s) |
| `device_id` | integer | no | Device ID |
| `site` | string[] | no | Site name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `ConsolePort` objects.

---

### `netbox_dcim_console_ports_get`

Get a single console port by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox console port ID |

Returns a single `ConsolePort` object.

---

### `netbox_dcim_console_server_ports_list`

List console server ports, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Console server port name(s) |
| `device_id` | integer | no | Device ID |
| `site` | string[] | no | Site name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `ConsoleServerPort` objects.

---

### `netbox_dcim_console_server_ports_get`

Get a single console server port by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox console server port ID |

Returns a single `ConsoleServerPort` object.

---

### `netbox_dcim_device_bays_list`

List device bays, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Device bay name(s) |
| `device_id` | integer | no | Device ID |
| `site` | string[] | no | Site name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `DeviceBay` objects.

---

### `netbox_dcim_device_bays_get`

Get a single device bay by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox device bay ID |

Returns a single `DeviceBay` object.

---

### `netbox_dcim_front_ports_list`

List front ports, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Front port name(s) |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `FrontPort` objects.

---

### `netbox_dcim_front_ports_get`

Get a single front port by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox front port ID |

Returns a single `FrontPort` object.

---

### `netbox_dcim_mac_addresses_list`

List MAC addresses, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `device_id` | integer | no | Device ID |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `MACAddress` objects.

---

### `netbox_dcim_mac_addresses_get`

Get a single MAC address by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox MAC address ID |

Returns a single `MACAddress` object.

---

### `netbox_dcim_modules_list`

List modules, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `device_id` | integer | no | Device ID |
| `site` | string[] | no | Site name or slug |
| `status` | string[] | no | Module status |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Module` objects.

---

### `netbox_dcim_modules_get`

Get a single module by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox module ID |

Returns a single `Module` object.

---

### `netbox_dcim_module_bays_list`

List module bays, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `device_id` | integer | no | Device ID |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `ModuleBay` objects.

---

### `netbox_dcim_module_bays_get`

Get a single module bay by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox module bay ID |

Returns a single `ModuleBay` object.

---

### `netbox_dcim_module_types_list`

List module types, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `manufacturer` | string[] | no | Manufacturer name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `ModuleType` objects.

---

### `netbox_dcim_module_types_get`

Get a single module type by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox module type ID |

Returns a single `ModuleType` object.

---

### `netbox_dcim_power_outlets_list`

List power outlets, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Power outlet name(s) |
| `device_id` | integer | no | Device ID |
| `site` | string[] | no | Site name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `PowerOutlet` objects.

---

### `netbox_dcim_power_outlets_get`

Get a single power outlet by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox power outlet ID |

Returns a single `PowerOutlet` object.

---

### `netbox_dcim_power_ports_list`

List power ports, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Power port name(s) |
| `device_id` | integer | no | Device ID |
| `site` | string[] | no | Site name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `PowerPort` objects.

---

### `netbox_dcim_power_ports_get`

Get a single power port by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox power port ID |

Returns a single `PowerPort` object.

---

### `netbox_dcim_rack_reservations_list`

List rack reservations, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `rack_id` | integer | no | Rack ID |
| `site` | string[] | no | Site name or slug |
| `tenant` | string[] | no | Tenant name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `RackReservation` objects.

---

### `netbox_dcim_rack_reservations_get`

Get a single rack reservation by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox rack reservation ID |

Returns a single `RackReservation` object.

---

### `netbox_dcim_rack_roles_list`

List rack roles, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Rack role name(s) |
| `slug` | string[] | no | Rack role slug(s) |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `RackRole` objects.

---

### `netbox_dcim_rack_roles_get`

Get a single rack role by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox rack role ID |

Returns a single `RackRole` object.

---

### `netbox_dcim_rack_types_list`

List rack types, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `slug` | string[] | no | Rack type slug(s) |
| `manufacturer` | string[] | no | Manufacturer name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `RackType` objects.

---

### `netbox_dcim_rack_types_get`

Get a single rack type by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox rack type ID |

Returns a single `RackType` object.

---

### `netbox_dcim_rear_ports_list`

List rear ports, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Rear port name(s) |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `RearPort` objects.

---

### `netbox_dcim_rear_ports_get`

Get a single rear port by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox rear port ID |

Returns a single `RearPort` object.

---

### `netbox_dcim_site_groups_list`

List site groups, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Site group name(s) |
| `slug` | string[] | no | Site group slug(s) |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `SiteGroup` objects.

---

### `netbox_dcim_site_groups_get`

Get a single site group by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox site group ID |

Returns a single `SiteGroup` object.

---

### `netbox_dcim_virtual_device_contexts_list`

List virtual device contexts, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `device_id` | integer | no | Device ID |
| `tenant` | string[] | no | Tenant name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `VirtualDeviceContext` objects.

---

### `netbox_dcim_virtual_device_contexts_get`

Get a single virtual device context by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox virtual device context ID |

Returns a single `VirtualDeviceContext` object.

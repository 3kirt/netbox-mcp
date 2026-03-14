# Virtualization Tools

All tools are read-only. They return JSON-marshalled NetBox responses as text content. If NetBox returns an error, the tool result has `isError: true` and the content contains the error message.

---

### `netbox_virtualization_vms_list`

List virtual machines, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `cluster` | string | no | Cluster name |
| `site` | string | no | Site name or slug |
| `status` | string | no | `active`, `offline`, `staged`, `failed`, or `decommissioning` |
| `role` | string | no | Device role name or slug |
| `tenant` | string | no | Tenant name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `VirtualMachineWithConfigContext` objects.

---

### `netbox_virtualization_vms_get`

Get a single virtual machine by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox virtual machine ID |

Returns a single `VirtualMachineWithConfigContext` object.

---

### `netbox_virtualization_clusters_list`

List virtualization clusters, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Cluster name |
| `type` | string | no | Cluster type name or slug |
| `site` | string | no | Site name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Cluster` objects.

---

### `netbox_virtualization_clusters_get`

Get a single virtualization cluster by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox cluster ID |

Returns a single `Cluster` object.

---

### `netbox_virtualization_cluster_groups_list`

List cluster groups, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Cluster group name |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `ClusterGroup` objects.

---

### `netbox_virtualization_cluster_groups_get`

Get a single cluster group by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox cluster group ID |

Returns a single `ClusterGroup` object.

---

### `netbox_virtualization_cluster_types_list`

List cluster types, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Cluster type name |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `ClusterType` objects.

---

### `netbox_virtualization_cluster_types_get`

Get a single cluster type by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox cluster type ID |

Returns a single `ClusterType` object.

---

### `netbox_virtualization_interfaces_list`

List VM interfaces, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `virtual_machine_id` | integer | no | Virtual machine ID |
| `name` | string | no | Interface name |
| `enabled` | boolean | no | Filter to enabled interfaces only |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `VMInterface` objects.

---

### `netbox_virtualization_interfaces_get`

Get a single VM interface by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox VM interface ID |

Returns a single `VMInterface` object.

---

### `netbox_virtualization_virtual_disks_list`

List virtual disks, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `virtual_machine_id` | integer | no | Virtual machine ID |
| `name` | string | no | Virtual disk name |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `VirtualDisk` objects.

---

### `netbox_virtualization_virtual_disks_get`

Get a single virtual disk by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox virtual disk ID |

Returns a single `VirtualDisk` object.

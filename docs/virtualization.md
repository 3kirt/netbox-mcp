# Virtualization Tools

All tools are read-only. They return JSON-marshalled NetBox responses as text content. If NetBox returns an error, the tool result has `isError: true` and the content contains the error message.

---

### `netbox_virtualization_vms_list`

List virtual machines, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `cluster` | string | no | Cluster name |
| `site` | string | no | Site name or slug |
| `status` | string | no | `active`, `offline`, `staged`, `failed`, or `decommissioning` |
| `role` | string | no | Device role name or slug |
| `tenant` | string | no | Tenant name or slug |
| `limit` | integer | no | Maximum results (default 50) |
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
| `name` | string | no | Cluster name |
| `type` | string | no | Cluster type name or slug |
| `site` | string | no | Site name or slug |
| `limit` | integer | no | Maximum results (default 50) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Cluster` objects.

---

### `netbox_virtualization_clusters_get`

Get a single virtualization cluster by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox cluster ID |

Returns a single `Cluster` object.

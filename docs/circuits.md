# Circuits Tools

All tools are read-only. They return JSON-marshalled NetBox responses as text content. If NetBox returns an error, the tool result has `isError: true` and the content contains the error message.

---

### `netbox_circuits_circuits_list`

List circuits, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `provider` | string | no | Provider name or slug |
| `status` | string | no | `active`, `planned`, `provisioning`, `offline`, `deprovisioning`, or `decommissioned` |
| `type` | string | no | Circuit type name or slug |
| `site` | string | no | Site name or slug |
| `tenant` | string | no | Tenant name or slug |
| `limit` | integer | no | Maximum results (default 50) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Circuit` objects.

---

### `netbox_circuits_circuits_get`

Get a single circuit by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox circuit ID |

Returns a single `Circuit` object.

---

### `netbox_circuits_providers_list`

List circuit providers, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `name` | string | no | Provider name |
| `limit` | integer | no | Maximum results (default 50) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Provider` objects.

---

### `netbox_circuits_providers_get`

Get a single circuit provider by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox provider ID |

Returns a single `Provider` object.

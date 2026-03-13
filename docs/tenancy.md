# Tenancy Tools

All tools are read-only. They return JSON-marshalled NetBox responses as text content. If NetBox returns an error, the tool result has `isError: true` and the content contains the error message.

---

### `netbox_tenancy_tenants_list`

List tenants, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `name` | string | no | Tenant name |
| `group` | string | no | Tenant group name or slug |
| `limit` | integer | no | Maximum results (default 50) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Tenant` objects.

---

### `netbox_tenancy_tenants_get`

Get a single tenant by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox tenant ID |

Returns a single `Tenant` object.

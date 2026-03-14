# Tenancy Tools

All tools are read-only. They return JSON-marshalled NetBox responses as text content. If NetBox returns an error, the tool result has `isError: true` and the content contains the error message.

---

### `netbox_tenancy_tenants_list`

List tenants, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Tenant name |
| `group` | string | no | Tenant group name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Tenant` objects.

---

### `netbox_tenancy_tenants_get`

Get a single tenant by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox tenant ID |

Returns a single `Tenant` object.

---

### `netbox_tenancy_tenant_groups_list`

List tenant groups, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Tenant group name |
| `parent` | string | no | Parent group name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `TenantGroup` objects.

---

### `netbox_tenancy_tenant_groups_get`

Get a single tenant group by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox tenant group ID |

Returns a single `TenantGroup` object.

---

### `netbox_tenancy_contacts_list`

List contacts, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Contact name |
| `group` | string | no | Contact group name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Contact` objects.

---

### `netbox_tenancy_contacts_get`

Get a single contact by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox contact ID |

Returns a single `Contact` object.

---

### `netbox_tenancy_contact_groups_list`

List contact groups, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Contact group name |
| `parent` | string | no | Parent group name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `ContactGroup` objects.

---

### `netbox_tenancy_contact_groups_get`

Get a single contact group by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox contact group ID |

Returns a single `ContactGroup` object.

---

### `netbox_tenancy_contact_roles_list`

List contact roles, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Contact role name |
| `slug` | string | no | Contact role slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `ContactRole` objects.

---

### `netbox_tenancy_contact_roles_get`

Get a single contact role by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox contact role ID |

Returns a single `ContactRole` object.

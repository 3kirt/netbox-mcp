# Circuits Tools

All tools are read-only. They return JSON-marshalled NetBox responses as text content. If NetBox returns an error, the tool result has `isError: true` and the content contains the error message.

---

### `netbox_circuits_circuits_list`

List circuits, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `provider` | string | no | Provider name or slug |
| `status` | string | no | `active`, `planned`, `provisioning`, `offline`, `deprovisioning`, or `decommissioned` |
| `type` | string | no | Circuit type name or slug |
| `site` | string | no | Site name or slug |
| `tenant` | string | no | Tenant name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
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
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Provider name |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Provider` objects.

---

### `netbox_circuits_providers_get`

Get a single circuit provider by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox provider ID |

Returns a single `Provider` object.

---

### `netbox_circuits_circuit_types_list`

List circuit types, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string | no | Circuit type name |
| `slug` | string | no | Circuit type slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `CircuitType` objects.

---

### `netbox_circuits_circuit_types_get`

Get a single circuit type by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox circuit type ID |

Returns a single `CircuitType` object.

---

### `netbox_circuits_circuit_terminations_list`

List circuit terminations, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `circuit_id` | integer | no | Circuit ID |
| `site` | string | no | Site name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `CircuitTermination` objects.

---

### `netbox_circuits_circuit_terminations_get`

Get a single circuit termination by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox circuit termination ID |

Returns a single `CircuitTermination` object.

---

### `netbox_circuits_provider_accounts_list`

List provider accounts, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `provider` | string | no | Provider name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `ProviderAccount` objects.

---

### `netbox_circuits_provider_accounts_get`

Get a single provider account by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox provider account ID |

Returns a single `ProviderAccount` object.

---

### `netbox_circuits_provider_networks_list`

List provider networks, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `provider` | string | no | Provider name or slug |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `ProviderNetwork` objects.

---

### `netbox_circuits_provider_networks_get`

Get a single provider network by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox provider network ID |

Returns a single `ProviderNetwork` object.

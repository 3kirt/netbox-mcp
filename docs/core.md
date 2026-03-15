# Core Tools

All tools are read-only. They return JSON-marshalled NetBox responses as text content. If NetBox returns an error, the tool result has `isError: true` and the content contains the error message.

---

### `netbox_core_data_sources_list`

List data sources in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Data source name(s) to filter by |
| `status` | string[] | no | Data source status(es) to filter by (e.g. `new`, `synced`, `failed`) |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `DataSource` objects.

---

### `netbox_core_data_sources_get`

Get a single data source by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox data source ID |

Returns a single `DataSource` object.

---

### `netbox_core_jobs_list`

List background jobs in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `status` | string[] | no | Job status(es) to filter by (e.g. `pending`, `running`, `completed`, `failed`) |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Job` objects.

---

### `netbox_core_jobs_get`

Get a single background job by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox job ID |

Returns a single `Job` object.

---

### `netbox_core_object_changes_list`

List object changes (audit log) in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `user` | string[] | no | Username(s) to filter by |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `ObjectChange` objects.

---

### `netbox_core_object_changes_get`

Get a single object change record by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox object change ID |

Returns a single `ObjectChange` object.

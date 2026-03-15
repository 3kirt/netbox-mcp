# Extras Tools

All tools are read-only. They return JSON-marshalled NetBox responses as text content. If NetBox returns an error, the tool result has `isError: true` and the content contains the error message.

---

### `netbox_extras_tags_list`

List tags in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Tag name(s) to filter by |
| `slug` | string[] | no | Tag slug(s) to filter by |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Tag` objects.

---

### `netbox_extras_tags_get`

Get a single tag by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox tag ID |

Returns a single `Tag` object.

---

### `netbox_extras_config_contexts_list`

List config contexts in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Config context name(s) to filter by |
| `is_active` | boolean | no | Filter by active status |
| `site` | string[] | no | Site name or slug to filter by |
| `role` | string[] | no | Device role name or slug to filter by |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `ConfigContext` objects.

---

### `netbox_extras_config_contexts_get`

Get a single config context by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox config context ID |

Returns a single `ConfigContext` object.

---

### `netbox_extras_journal_entries_list`

List journal entries in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `assigned_object_type` | string[] | no | Assigned object type(s) to filter by (e.g. `dcim.device`) |
| `assigned_object_id` | integer | no | Assigned object ID to filter by |
| `kind` | string[] | no | Journal entry kind(s) to filter by (`info`, `success`, `warning`, `danger`) |
| `created_by` | string[] | no | Creator username(s) to filter by |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `JournalEntry` objects.

---

### `netbox_extras_journal_entries_get`

Get a single journal entry by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox journal entry ID |

Returns a single `JournalEntry` object.

---

### `netbox_extras_custom_fields_list`

List custom fields in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Custom field name(s) to filter by |
| `type` | string[] | no | Custom field type(s) to filter by (e.g. `text`, `integer`, `boolean`) |
| `object_type` | string | no | Object type to filter by (e.g. `dcim.device`) |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `CustomField` objects.

---

### `netbox_extras_custom_fields_get`

Get a single custom field by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox custom field ID |

Returns a single `CustomField` object.

---

### `netbox_extras_export_templates_list`

List export templates in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Export template name(s) to filter by |
| `object_type` | string | no | Object type to filter by (e.g. `dcim.device`) |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `ExportTemplate` objects.

---

### `netbox_extras_export_templates_get`

Get a single export template by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox export template ID |

Returns a single `ExportTemplate` object.

---

### `netbox_extras_webhooks_list`

List webhooks in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Webhook name(s) to filter by |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Webhook` objects.

---

### `netbox_extras_webhooks_get`

Get a single webhook by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox webhook ID |

Returns a single `Webhook` object.

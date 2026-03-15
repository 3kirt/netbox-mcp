# Wireless Tools

All tools are read-only. They return JSON-marshalled NetBox responses as text content. If NetBox returns an error, the tool result has `isError: true` and the content contains the error message.

---

### `netbox_wireless_lans_list`

List wireless LANs in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `ssid` | string[] | no | SSID(s) to filter by |
| `group` | string[] | no | Wireless LAN group name or slug to filter by |
| `status` | string[] | no | Wireless LAN status(es) to filter by (e.g. `active`, `disabled`, `reserved`) |
| `tenant` | string[] | no | Tenant name or slug to filter by |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `WirelessLAN` objects.

---

### `netbox_wireless_lans_get`

Get a single wireless LAN by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox wireless LAN ID |

Returns a single `WirelessLAN` object.

---

### `netbox_wireless_lan_groups_list`

List wireless LAN groups in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Wireless LAN group name(s) to filter by |
| `parent` | string[] | no | Parent group name or slug to filter by |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `WirelessLANGroup` objects.

---

### `netbox_wireless_lan_groups_get`

Get a single wireless LAN group by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox wireless LAN group ID |

Returns a single `WirelessLANGroup` object.

---

### `netbox_wireless_links_list`

List wireless links in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `status` | string[] | no | Wireless link status(es) to filter by (e.g. `planned`, `active`, `disabled`) |
| `tenant` | string[] | no | Tenant name or slug to filter by |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `WirelessLink` objects.

---

### `netbox_wireless_links_get`

Get a single wireless link by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox wireless link ID |

Returns a single `WirelessLink` object.

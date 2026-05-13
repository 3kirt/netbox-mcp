# VPN Tools

All tools are read-only. They return JSON-marshalled NetBox responses as text content. If NetBox returns an error, the tool result has `isError: true` and the content contains the error message.

---

### `netbox_vpn_tunnels_list`

List VPN tunnels in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Tunnel name(s) to filter by |
| `status` | string[] | no | Tunnel status(es) to filter by (`planned`, `active`, `disabled`) |
| `encapsulation` | string[] | no | Encapsulation type(s) (`ipsec-transport`, `ipsec-tunnel`, `ip-ip`, `gre`) |
| `tenant` | string[] | no | Tenant slug(s) to filter by |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `Tunnel` objects.

---

### `netbox_vpn_tunnels_get`

Get a single VPN tunnel by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox VPN tunnel ID |

Returns a single `Tunnel` object.

---

### `netbox_vpn_tunnel_groups_list`

List VPN tunnel groups in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | Tunnel group name(s) to filter by |
| `slug` | string[] | no | Tunnel group slug(s) to filter by |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `TunnelGroup` objects.

---

### `netbox_vpn_tunnel_groups_get`

Get a single VPN tunnel group by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox VPN tunnel group ID |

Returns a single `TunnelGroup` object.

---

### `netbox_vpn_l2vpns_list`

List L2VPNs in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | L2VPN name(s) to filter by |
| `type` | string[] | no | L2VPN type(s) to filter by (`vpws`, `vpls`, `vxlan`, `vxlan-evpn`, `mpls-evpn`, `pbb-evpn`, `epl`, `evpl`, `ep-lan`, `evp-lan`, `ep-tree`, `evp-tree`) |
| `tenant` | string[] | no | Tenant slug(s) to filter by |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `L2VPN` objects.

---

### `netbox_vpn_l2vpns_get`

Get a single L2VPN by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox L2VPN ID |

Returns a single `L2VPN` object.

---

### `netbox_vpn_ike_policies_list`

List IKE policies in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | IKE policy name(s) to filter by |
| `version` | integer | no | IKE version (`1` or `2`) |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `IKEPolicy` objects.

---

### `netbox_vpn_ike_policies_get`

Get a single IKE policy by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox IKE policy ID |

Returns a single `IKEPolicy` object.

---

### `netbox_vpn_ipsec_policies_list`

List IPSec policies in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `name` | string[] | no | IPSec policy name(s) to filter by |
| `pfs_group` | string[] | no | PFS group(s) to filter by |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `IPSecPolicy` objects.

---

### `netbox_vpn_ipsec_policies_get`

Get a single IPSec policy by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox IPSec policy ID |

Returns a single `IPSecPolicy` object.

---

### `netbox_vpn_tunnel_terminations_list`

List VPN tunnel terminations in NetBox, with optional filtering.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Free-text search |
| `ordering` | string | no | Field to order results by (prefix with `-` for descending) |
| `tunnel_id` | integer | no | Tunnel ID to filter by |
| `role` | string[] | no | Termination role(s) to filter by (`peer`, `hub`, `spoke`) |
| `limit` | integer | no | Maximum results (default 50, max 1000) |
| `offset` | integer | no | Pagination offset |

Returns a paginated list of `TunnelTermination` objects.

---

### `netbox_vpn_tunnel_terminations_get`

Get a single VPN tunnel termination by ID.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `id` | integer | yes | NetBox VPN tunnel termination ID |

Returns a single `TunnelTermination` object.

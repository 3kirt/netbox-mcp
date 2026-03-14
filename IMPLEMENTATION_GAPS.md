# Implementation Gaps

This document is a work plan for a future Claude session. It describes every gap between the current
`netbox-mcp` implementation and the full read/list surface of the `go-netbox/v4` API client, with
enough specificity to implement each item without additional research.

**Source of truth for the API:** `/Users/kirtis/source/repos/go-netbox/` — the generated OpenAPI
client. Refer to `api_*.go` files there for exact method names and available filter setters.

---

## Conventions (read before implementing anything)

All tools follow the established patterns in the existing `internal/tools/` files. Replicate them
exactly.

**Tool naming:** `netbox_{module}_{resource}_{action}` where action is `list` or `get`.

**Input struct:** local to the `add*` function, with `json` and `jsonschema` tags. Use `omitempty`
for optional fields, no `omitempty` for required ones (get `id`).

**List handler shape:**
```go
func addFooList(s *mcp.Server, client *netbox.APIClient) {
    type input struct {
        Q       string `json:"q,omitempty"       jsonschema:"Free-text search"`
        // ... domain filters ...
        Ordering string `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
        Limit    int32  `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
        Offset   int32  `json:"offset,omitempty"   jsonschema:"Pagination offset"`
    }
    mcp.AddTool(s, &mcp.Tool{
        Name:        "netbox_..._list",
        Description: "...",
    }, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
        r := client.XxxAPI.XxxYyyList(ctx)
        if in.Q != "" {
            r = r.Q(in.Q)
        }
        // ... domain filters ...
        if in.Ordering != "" {
            r = r.Ordering(in.Ordering)
        }
        limit := clampLimit(in.Limit)
        resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
        if err != nil {
            return toolError(fmt.Sprintf("listing ...: %v", err))
        }
        return jsonResult(resp)
    })
}
```

**Get handler shape:**
```go
func addFooGet(s *mcp.Server, client *netbox.APIClient) {
    type input struct {
        ID int32 `json:"id" jsonschema:"NetBox ID of the ... to retrieve"`
    }
    mcp.AddTool(s, &mcp.Tool{
        Name:        "netbox_..._get",
        Description: "Get a single ... by its NetBox ID.",
    }, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
        resp, _, err := client.XxxAPI.XxxYyyRetrieve(ctx, in.ID).Execute()
        if err != nil {
            return toolError(fmt.Sprintf("getting ... %d: %v", in.ID, err))
        }
        return jsonResult(resp)
    })
}
```

**Nullable foreign-key filters** (fields that can be null/unset in NetBox, like VRF) take
`[]*string` in the API rather than `[]string`. Use the existing `ptrOf` helper:
```go
r = r.Vrf([]*string{ptrOf(in.VRF)})
```

**New module files** must be registered in `internal/server/server.go`:
```go
tools.RegisterExtras(s, client)
```

---

## Priority order

1. ~~**Phase A** — Universal filter additions to all existing list tools (`q`, `ordering`)~~ ✅
2. ~~**Phase B** — Missing `_get` tools for existing list-only resources~~ ✅
3. ~~**Phase C** — Missing resources in existing modules (no new files)~~ ✅
4. **Phase D** — New modules (new files + server.go registration)

---

## ✅ Phase A: Add `q` and `ordering` to all existing list tools

Every list tool is missing two universally-available filters. Add them to each:

| File | Tool | API method to check |
|---|---|---|
| `circuits.go` | `netbox_circuits_circuits_list` | `CircuitsCircuitsList` |
| `circuits.go` | `netbox_circuits_providers_list` | `CircuitsProvidersList` |
| `dcim.go` | `netbox_dcim_devices_list` | `DcimDevicesList` |
| `dcim.go` | `netbox_dcim_sites_list` | `DcimSitesList` |
| `dcim.go` | `netbox_dcim_racks_list` | `DcimRacksList` |
| `dcim.go` | `netbox_dcim_interfaces_list` | `DcimInterfacesList` |
| `dcim.go` | `netbox_dcim_cables_list` | `DcimCablesList` |
| `ipam.go` | `netbox_ipam_ip_addresses_list` | `IpamIpAddressesList` |
| `ipam.go` | `netbox_ipam_prefixes_list` | `IpamPrefixesList` |
| `ipam.go` | `netbox_ipam_vrfs_list` | `IpamVrfsList` |
| `ipam.go` | `netbox_ipam_vlans_list` | `IpamVlansList` |
| `tenancy.go` | `netbox_tenancy_tenants_list` | `TenancyTenantsList` |
| `virtualization.go` | `netbox_virtualization_vms_list` | `VirtualizationVirtualMachinesList` |
| `virtualization.go` | `netbox_virtualization_clusters_list` | `VirtualizationClustersList` |

For each, add to the input struct:
```go
Q        string `json:"q,omitempty"        jsonschema:"Free-text search"`
Ordering string `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
```

And in the handler body (before `clampLimit`):
```go
if in.Q != "" {
    r = r.Q(in.Q)
}
// existing filters ...
if in.Ordering != "" {
    r = r.Ordering(in.Ordering)
}
```

---

## ✅ Phase B: Missing `_get` counterparts

Two existing list tools have no paired get tool.

### `netbox_dcim_interfaces_get`
```
File:    internal/tools/dcim.go
Add fn:  addDCIMInterfacesGet
API:     client.DcimAPI.DcimInterfacesRetrieve(ctx, in.ID).Execute()
Register in RegisterDCIM after addDCIMInterfacesList
```

### `netbox_dcim_cables_get`
```
File:    internal/tools/dcim.go
Add fn:  addDCIMCablesGet
API:     client.DcimAPI.DcimCablesRetrieve(ctx, in.ID).Execute()
Register in RegisterDCIM after addDCIMCablesList
```

---

## Phase C: Missing resources in existing modules

### ✅ C1: DCIM additions (`internal/tools/dcim.go`)

Add to `RegisterDCIM`. Each section lists the tool names, the go-netbox API methods, and the
filter fields worth exposing.

#### Regions
```
netbox_dcim_regions_list  →  DcimRegionsList
netbox_dcim_regions_get   →  DcimRegionsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Slug     string  — Slug([]string{})
  Parent   string  — Parent([]string{})  // parent region slug
  Ordering string
```

#### Locations
```
netbox_dcim_locations_list  →  DcimLocationsList
netbox_dcim_locations_get   →  DcimLocationsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Site     string  — Site([]string{})
  Parent   string  — Parent([]string{})
  Status   string  — Status([]string{})
  Tenant   string  — Tenant([]string{})
  Ordering string
```

#### Manufacturers
```
netbox_dcim_manufacturers_list  →  DcimManufacturersList
netbox_dcim_manufacturers_get   →  DcimManufacturersRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Slug     string  — Slug([]string{})
  Ordering string
```

#### Device types
```
netbox_dcim_device_types_list  →  DcimDeviceTypesList
netbox_dcim_device_types_get   →  DcimDeviceTypesRetrieve(ctx, id)

Filters for list:
  Q            string  — q
  Manufacturer string  — Manufacturer([]string{})
  Model        string  — Model([]string{})
  Ordering     string
```

#### Device roles
```
netbox_dcim_device_roles_list  →  DcimDeviceRolesList
netbox_dcim_device_roles_get   →  DcimDeviceRolesRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Slug     string  — Slug([]string{})
  VMRole   bool    — VmRole(in.VMRole)  // true = roles eligible for VMs
  Ordering string
```

#### Platforms
```
netbox_dcim_platforms_list  →  DcimPlatformsList
netbox_dcim_platforms_get   →  DcimPlatformsRetrieve(ctx, id)

Filters for list:
  Q            string  — q
  Name         string  — Name([]string{})
  Manufacturer string  — Manufacturer([]string{})
  Ordering     string
```

#### Power panels
```
netbox_dcim_power_panels_list  →  DcimPowerPanelsList
netbox_dcim_power_panels_get   →  DcimPowerPanelsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Site     string  — Site([]string{})
  Location string  — Location([]string{})
  Ordering string
```

#### Power feeds
```
netbox_dcim_power_feeds_list  →  DcimPowerFeedsList
netbox_dcim_power_feeds_get   →  DcimPowerFeedsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Site     string  — Site([]string{})    // via power panel
  Status   string  — Status([]string{})
  Type     string  — Type_([]string{})
  Ordering string
```

#### Virtual chassis
```
netbox_dcim_virtual_chassis_list  →  DcimVirtualChassisList
netbox_dcim_virtual_chassis_get   →  DcimVirtualChassisRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Site     string  — Site([]string{})
  Tenant   string  — Tenant([]string{})
  Ordering string
```

#### Inventory items
```
netbox_dcim_inventory_items_list  →  DcimInventoryItemsList
netbox_dcim_inventory_items_get   →  DcimInventoryItemsRetrieve(ctx, id)

Filters for list:
  Q           string  — q
  DeviceID    int32   — DeviceId([]int32{})
  Manufacturer string — Manufacturer([]string{})
  Discovered  bool    — Discovered(in.Discovered)
  Ordering    string
```

---

### ✅ C2: IPAM additions (`internal/tools/ipam.go`)

Add to `RegisterIPAM`.

#### Aggregates
```
netbox_ipam_aggregates_list  →  IpamAggregatesList
netbox_ipam_aggregates_get   →  IpamAggregatesRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Family   int32   — Family(in.Family)   // 4 or 6
  RIR      string  — Rir([]string{})
  Tenant   string  — Tenant([]string{})
  Ordering string
```

#### IP ranges
```
netbox_ipam_ip_ranges_list  →  IpamIpRangesList
netbox_ipam_ip_ranges_get   →  IpamIpRangesRetrieve(ctx, id)

Filters for list:
  Q          string  — q
  VRF        string  — Vrf([]*string{ptrOf(in.VRF)})  // nullable
  Status     string  — Status([]string{})
  Tenant     string  — Tenant([]string{})
  Ordering   string
```

#### Route targets
```
netbox_ipam_route_targets_list  →  IpamRouteTargetsList
netbox_ipam_route_targets_get   →  IpamRouteTargetsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Tenant   string  — Tenant([]string{})
  Ordering string
```

#### RIRs
```
netbox_ipam_rirs_list  →  IpamRirsList
netbox_ipam_rirs_get   →  IpamRirsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Slug     string  — Slug([]string{})
  Ordering string
```

#### VLAN groups
```
netbox_ipam_vlan_groups_list  →  IpamVlanGroupsList
netbox_ipam_vlan_groups_get   →  IpamVlanGroupsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Site     string  — Site([]string{})
  Ordering string
```

#### Services
```
netbox_ipam_services_list  →  IpamServicesList
netbox_ipam_services_get   →  IpamServicesRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  DeviceID int32   — DeviceId([]int32{})
  VirtualMachineID int32 — VirtualMachineId([]int32{})
  Protocol string  — Protocol([]string{})
  Ordering string
```

---

### ✅ C3: Circuits additions (`internal/tools/circuits.go`)

Add to `RegisterCircuits`.

#### Circuit types
```
netbox_circuits_circuit_types_list  →  CircuitsCircuitTypesList
netbox_circuits_circuit_types_get   →  CircuitsCircuitTypesRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Slug     string  — Slug([]string{})
  Ordering string
```

#### Circuit terminations
```
netbox_circuits_circuit_terminations_list  →  CircuitsCircuitTerminationsList
netbox_circuits_circuit_terminations_get   →  CircuitsCircuitTerminationsRetrieve(ctx, id)

Filters for list:
  Q         string  — q
  CircuitID int32   — CircuitId([]int32{})
  Site      string  — Site([]string{})
  Ordering  string
```

#### Provider accounts
```
netbox_circuits_provider_accounts_list  →  CircuitsProviderAccountsList
netbox_circuits_provider_accounts_get   →  CircuitsProviderAccountsRetrieve(ctx, id)

Filters for list:
  Q          string  — q
  Provider   string  — Provider([]string{})
  Ordering   string
```

#### Provider networks
```
netbox_circuits_provider_networks_list  →  CircuitsProviderNetworksList
netbox_circuits_provider_networks_get   →  CircuitsProviderNetworksRetrieve(ctx, id)

Filters for list:
  Q          string  — q
  Provider   string  — Provider([]string{})
  Ordering   string
```

---

### ✅ C4: Tenancy additions (`internal/tools/tenancy.go`)

Add to `RegisterTenancy`.

#### Tenant groups
```
netbox_tenancy_tenant_groups_list  →  TenancyTenantGroupsList
netbox_tenancy_tenant_groups_get   →  TenancyTenantGroupsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Parent   string  — Parent([]string{})
  Ordering string
```

#### Contacts
```
netbox_tenancy_contacts_list  →  TenancyContactsList
netbox_tenancy_contacts_get   →  TenancyContactsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Group    string  — Group([]string{})
  Ordering string
```

#### Contact groups
```
netbox_tenancy_contact_groups_list  →  TenancyContactGroupsList
netbox_tenancy_contact_groups_get   →  TenancyContactGroupsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Parent   string  — Parent([]string{})
  Ordering string
```

#### Contact roles
```
netbox_tenancy_contact_roles_list  →  TenancyContactRolesList
netbox_tenancy_contact_roles_get   →  TenancyContactRolesRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Slug     string  — Slug([]string{})
  Ordering string
```

---

### ✅ C5: Virtualization additions (`internal/tools/virtualization.go`)

Add to `RegisterVirtualization`.

#### Cluster groups
```
netbox_virtualization_cluster_groups_list  →  VirtualizationClusterGroupsList
netbox_virtualization_cluster_groups_get   →  VirtualizationClusterGroupsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Ordering string
```

#### Cluster types
```
netbox_virtualization_cluster_types_list  →  VirtualizationClusterTypesList
netbox_virtualization_cluster_types_get   →  VirtualizationClusterTypesRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Ordering string
```

#### VM interfaces
```
netbox_virtualization_interfaces_list  →  VirtualizationInterfacesList
netbox_virtualization_interfaces_get   →  VirtualizationInterfacesRetrieve(ctx, id)

Filters for list:
  Q               string  — q
  VirtualMachineID int32  — VirtualMachineId([]int32{})
  Name            string  — Name([]string{})
  Enabled         bool    — Enabled(in.Enabled)  // only apply if caller sets it
  Ordering        string

Note: Enabled is a bool that defaults to false in Go. Use a *bool pointer if you
want to distinguish "not set" from "filter on false". Check what the API setter
accepts (likely bool, not *bool).
```

#### Virtual disks
```
netbox_virtualization_virtual_disks_list  →  VirtualizationVirtualDisksList
netbox_virtualization_virtual_disks_get   →  VirtualizationVirtualDisksRetrieve(ctx, id)

Filters for list:
  Q               string  — q
  VirtualMachineID int32  — VirtualMachineId([]int32{})
  Name            string  — Name([]string{})
  Ordering        string
```

---

## Phase D: New module files

Each new file needs a `Register*` function and must be called from `internal/server/server.go`.

### D1: `internal/tools/extras.go`

```go
func RegisterExtras(s *mcp.Server, client *netbox.APIClient) {
    addExtrasTags...
    addExtrasConfigContexts...
    addExtrasObjectChanges...
    addExtrasJournalEntries...
    addExtrasCustomFields...
    addExtrasExportTemplates...
    addExtrasWebhooks...
}
```

#### Tags
```
netbox_extras_tags_list  →  ExtrasTagsList
netbox_extras_tags_get   →  ExtrasTagsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Slug     string  — Slug([]string{})
  Ordering string
```

#### Config contexts
```
netbox_extras_config_contexts_list  →  ExtrasConfigContextsList
netbox_extras_config_contexts_get   →  ExtrasConfigContextsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  IsActive bool    — IsActive(in.IsActive)
  Site     string  — Site([]string{})
  Role     string  — Role([]string{})
  Ordering string
```

#### Object changes (audit log)
```
netbox_extras_object_changes_list  →  ExtrasObjectChangesList
netbox_extras_object_changes_get   →  ExtrasObjectChangesRetrieve(ctx, id)

Filters for list:
  Q              string  — q
  User           string  — User([]string{})
  ChangedObjectType string — ChangedObjectType([]string{})  // e.g. "dcim.device"
  Action         string  — Action([]string{})  // create, update, delete
  Ordering       string
```

#### Journal entries
```
netbox_extras_journal_entries_list  →  ExtrasJournalEntriesList
netbox_extras_journal_entries_get   →  ExtrasJournalEntriesRetrieve(ctx, id)

Filters for list:
  Q              string  — q
  AssignedObjectType string — AssignedObjectType([]string{})
  AssignedObjectID   int32  — AssignedObjectId([]int32{})
  Kind           string  — Kind([]string{})  // info, success, warning, danger
  CreatedBy      string  — CreatedBy([]string{})
  Ordering       string
```

#### Custom fields
```
netbox_extras_custom_fields_list  →  ExtrasCustomFieldsList
netbox_extras_custom_fields_get   →  ExtrasCustomFieldsRetrieve(ctx, id)

Filters for list:
  Q          string  — q
  Name       string  — Name([]string{})
  Type       string  — Type_([]string{})
  ObjectType string  — ObjectType([]string{})
  Ordering   string
```

#### Export templates
```
netbox_extras_export_templates_list  →  ExtrasExportTemplatesList
netbox_extras_export_templates_get   →  ExtrasExportTemplatesRetrieve(ctx, id)

Filters for list:
  Q          string  — q
  Name       string  — Name([]string{})
  ObjectType string  — ObjectType([]string{})
  Ordering   string
```

#### Webhooks
```
netbox_extras_webhooks_list  →  ExtrasWebhooksList
netbox_extras_webhooks_get   →  ExtrasWebhooksRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Ordering string
```

---

### D2: `internal/tools/vpn.go`

```go
func RegisterVPN(s *mcp.Server, client *netbox.APIClient) {
    addVPNTunnels...
    addVPNL2VPNs...
    addVPNTunnelGroups...
    addVPNIKEPolicies...
    addVPNIPSecPolicies...
}
```

#### Tunnels
```
netbox_vpn_tunnels_list  →  VpnTunnelsList
netbox_vpn_tunnels_get   →  VpnTunnelsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Status   string  — Status([]string{})
  Group    string  — Group([]string{})
  Tenant   string  — Tenant([]string{})
  Ordering string
```

#### Tunnel groups
```
netbox_vpn_tunnel_groups_list  →  VpnTunnelGroupsList
netbox_vpn_tunnel_groups_get   →  VpnTunnelGroupsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Slug     string  — Slug([]string{})
  Ordering string
```

#### L2VPNs
```
netbox_vpn_l2vpns_list  →  VpnL2vpnsList
netbox_vpn_l2vpns_get   →  VpnL2vpnsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Type     string  — Type_([]string{})
  Tenant   string  — Tenant([]string{})
  Ordering string
```

#### IKE policies
```
netbox_vpn_ike_policies_list  →  VpnIkePoliciesList
netbox_vpn_ike_policies_get   →  VpnIkePoliciesRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Ordering string
```

#### IPSec policies
```
netbox_vpn_ipsec_policies_list  →  VpnIpsecPoliciesList
netbox_vpn_ipsec_policies_get   →  VpnIpsecPoliciesRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Ordering string
```

---

### D3: `internal/tools/wireless.go`

```go
func RegisterWireless(s *mcp.Server, client *netbox.APIClient) {
    addWirelessLANs...
    addWirelessLANGroups...
    addWirelessLinks...
}
```

#### Wireless LANs
```
netbox_wireless_lans_list  →  WirelessWirelessLansList
netbox_wireless_lans_get   →  WirelessWirelessLansRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  SSID     string  — Ssid([]string{})
  Group    string  — Group([]string{})
  Status   string  — Status([]string{})
  Tenant   string  — Tenant([]string{})
  Ordering string
```

#### Wireless LAN groups
```
netbox_wireless_lan_groups_list  →  WirelessWirelessLanGroupsList
netbox_wireless_lan_groups_get   →  WirelessWirelessLanGroupsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Parent   string  — Parent([]string{})
  Ordering string
```

#### Wireless links
```
netbox_wireless_links_list  →  WirelessWirelessLinksList
netbox_wireless_links_get   →  WirelessWirelessLinksRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Status   string  — Status([]string{})
  Tenant   string  — Tenant([]string{})
  Ordering string
```

---

### D4: `internal/tools/core.go`

```go
func RegisterCore(s *mcp.Server, client *netbox.APIClient) {
    addCoreObjectChanges...
    addCoreDataSources...
    addCoreJobs...
}
```

**Note:** `CoreObjectChangesList` duplicates `ExtrasObjectChangesList` — they are the same endpoint
in different API namespaces in older NetBox versions. Check `api_core.go` vs `api_extras.go` at
implementation time; only implement the one that actually exists in the go-netbox client.

#### Data sources
```
netbox_core_data_sources_list  →  CoreDataSourcesList
netbox_core_data_sources_get   →  CoreDataSourcesRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Status   string  — Status([]string{})
  Ordering string
```

#### Jobs
```
netbox_core_jobs_list  →  CoreJobsList
netbox_core_jobs_get   →  CoreJobsRetrieve(ctx, id)

Filters for list:
  Q          string  — q
  Status     string  — Status([]string{})
  ObjectType string  — ObjectType([]string{})
  Ordering   string
```

---

### D5: `internal/tools/users.go` (optional / lower priority)

Exposes read-only access to users, groups, and permissions. Useful for auditing but not core
infrastructure data. Implement last if needed.

```go
func RegisterUsers(s *mcp.Server, client *netbox.APIClient) {
    addUsersUsers...
    addUsersGroups...
    addUsersTokens...
}
```

#### Users
```
netbox_users_users_list  →  UsersUsersList
netbox_users_users_get   →  UsersUsersRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Username string  — Username([]string{})
  IsActive bool    — IsActive(in.IsActive)
  Ordering string
```

#### Groups
```
netbox_users_groups_list  →  UsersGroupsList
netbox_users_groups_get   →  UsersGroupsRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  Name     string  — Name([]string{})
  Ordering string
```

#### Tokens
```
netbox_users_tokens_list  →  UsersTokensList
netbox_users_tokens_get   →  UsersTokensRetrieve(ctx, id)

Filters for list:
  Q        string  — q
  UserID   int32   — UserId([]int32{})
  Ordering string
```

---

## server.go additions (Phase D)

When each new file is complete, add its `Register*` call in `internal/server/server.go`:

```go
func Register(s *mcp.Server, client *netbox.APIClient) {
    tools.RegisterCircuits(s, client)
    tools.RegisterCore(s, client)      // D4
    tools.RegisterDCIM(s, client)
    tools.RegisterExtras(s, client)    // D1
    tools.RegisterIPAM(s, client)
    tools.RegisterTenancy(s, client)
    tools.RegisterUsers(s, client)     // D5
    tools.RegisterVirtualization(s, client)
    tools.RegisterVPN(s, client)       // D2
    tools.RegisterWireless(s, client)  // D3
}
```

---

## Implementation notes

**Verify method signatures before writing.** The API builder method names in this document are
derived from the OpenAPI generator conventions but always confirm in the actual go-netbox source
(`/Users/kirtis/source/repos/go-netbox/api_*.go`) before coding. The generator occasionally
uses unexpected names (e.g., `Type_` instead of `Type` to avoid keyword collision).

**Bool filters need care.** Go bool zero-value is `false`, which is a valid filter value ("show
only disabled interfaces"). Use `*bool` in the input struct for any boolean filter where "not
provided" must be distinguishable from `false`, and only call the setter when the pointer is
non-nil.

**Nullable string filters** (fields that can be set to null in NetBox, e.g., VRF assignment)
take `[]*string` in the API. Use `ptrOf` from `helpers.go`. Examples already in `ipam.go`.

**`q` on resources that don't support it.** A small number of simpler resources (e.g., rack
roles, some template types) may not have a `q` setter. Check the builder methods in the API
file before adding it to the input struct; omit it if absent.

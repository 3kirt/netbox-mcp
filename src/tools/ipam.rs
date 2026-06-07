use crate::client::{NetboxClient, NetboxError};
use crate::tools::{PaginationParams, QueryBuilder};
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// IP Addresses
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct IpAddressesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by IP address (e.g. 192.0.2.1/24)")]
    pub address: Option<Vec<String>>,
    #[schemars(
        description = "Filter by VRF route distinguisher / rd (e.g. 65000:100) — preferred over vrf_id"
    )]
    pub vrf: Option<Vec<String>>,
    #[schemars(description = "Filter by VRF ID")]
    pub vrf_id: Option<i32>,
    #[schemars(description = "Filter by status (active, reserved, deprecated)")]
    pub status: Option<Vec<String>>,
    #[schemars(
        description = "Filter by role (loopback, secondary, anycast, vip, vrrp, hsrp, glbp, carp)"
    )]
    pub role: Option<Vec<String>>,
    #[schemars(description = "Return IPs within this prefix (e.g. 10.254.2.144/30)")]
    pub parent: Option<String>,
    #[schemars(description = "Filter by device name (multi-value)")]
    pub device: Option<Vec<String>>,
    #[schemars(description = "Filter by device ID")]
    pub device_id: Option<i32>,
    #[schemars(description = "Filter by virtual machine name (multi-value)")]
    pub virtual_machine: Option<Vec<String>>,
    #[schemars(description = "Filter by virtual machine ID")]
    pub virtual_machine_id: Option<i32>,
    #[schemars(description = "Filter by DNS name (exact match, e.g. host.example.com)")]
    pub dns_name: Option<String>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn ip_addresses_list(
    client: &NetboxClient,
    p: IpAddressesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("address", p.address)
        .many("vrf", p.vrf)
        .opt("vrf_id", p.vrf_id)
        .many("status", p.status)
        .many("role", p.role)
        .opt("parent", p.parent)
        .many("device", p.device)
        .opt("device_id", p.device_id)
        .many("virtual_machine", p.virtual_machine)
        .opt("virtual_machine_id", p.virtual_machine_id)
        .opt("dns_name", p.dns_name)
        .many("tenant", p.tenant)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/ip-addresses/", p.pagination)
        .await
}

// --------------------------------------------------------------------------
// Prefixes
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PrefixesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by prefix (e.g. 192.0.2.0/24)")]
    pub prefix: Option<Vec<String>>,
    #[schemars(
        description = "Filter by VRF route distinguisher / rd (e.g. 65000:100) — preferred over vrf_id"
    )]
    pub vrf: Option<Vec<String>>,
    #[schemars(description = "Filter by VRF ID")]
    pub vrf_id: Option<i32>,
    #[schemars(description = "Filter by status (active, container, reserved, deprecated)")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by role slug")]
    pub role: Option<Vec<String>>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by family (4 or 6)")]
    pub family: Option<i32>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn prefixes_list(
    client: &NetboxClient,
    p: PrefixesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("prefix", p.prefix)
        .many("vrf", p.vrf)
        .opt("vrf_id", p.vrf_id)
        .many("status", p.status)
        .many("role", p.role)
        .many("site", p.site)
        .many("tenant", p.tenant)
        .opt("family", p.family)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/prefixes/", p.pagination).await
}

// --------------------------------------------------------------------------
// VRFs
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VrfsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by VRF name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by route distinguisher")]
    pub rd: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn vrfs_list(client: &NetboxClient, p: VrfsListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("rd", p.rd)
        .many("tenant", p.tenant)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/vrfs/", p.pagination).await
}

// --------------------------------------------------------------------------
// VLANs
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VlansListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by VLAN ID (802.1Q VID)")]
    pub vid: Option<i32>,
    #[schemars(description = "Filter by VLAN name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by status (active, reserved, deprecated)")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by role slug")]
    pub role: Option<Vec<String>>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by VLAN group slug")]
    pub group: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn vlans_list(client: &NetboxClient, p: VlansListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .opt("vid", p.vid)
        .many("name", p.name)
        .many("status", p.status)
        .many("role", p.role)
        .many("site", p.site)
        .many("group", p.group)
        .many("tenant", p.tenant)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/vlans/", p.pagination).await
}

// --------------------------------------------------------------------------
// Aggregates
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AggregatesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by prefix")]
    pub prefix: Option<Vec<String>>,
    #[schemars(description = "Filter by RIR slug")]
    pub rir: Option<Vec<String>>,
    #[schemars(description = "Filter by family (4 or 6)")]
    pub family: Option<i32>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn aggregates_list(
    client: &NetboxClient,
    p: AggregatesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("prefix", p.prefix)
        .many("rir", p.rir)
        .opt("family", p.family)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/aggregates/", p.pagination).await
}

// --------------------------------------------------------------------------
// IP Ranges
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct IpRangesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by status (active, reserved, deprecated)")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by role slug")]
    pub role: Option<Vec<String>>,
    #[schemars(
        description = "Filter by VRF route distinguisher / rd (e.g. 65000:100) — preferred over vrf_id"
    )]
    pub vrf: Option<Vec<String>>,
    #[schemars(description = "Filter by VRF ID")]
    pub vrf_id: Option<i32>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn ip_ranges_list(
    client: &NetboxClient,
    p: IpRangesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("status", p.status)
        .many("role", p.role)
        .many("vrf", p.vrf)
        .opt("vrf_id", p.vrf_id)
        .many("tenant", p.tenant)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/ip-ranges/", p.pagination).await
}

// --------------------------------------------------------------------------
// Route Targets
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RouteTargetsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by route target name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn route_targets_list(
    client: &NetboxClient,
    p: RouteTargetsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("tenant", p.tenant)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/route-targets/", p.pagination)
        .await
}

// --------------------------------------------------------------------------
// RIRs
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RirsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by RIR name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn rirs_list(client: &NetboxClient, p: RirsListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("slug", p.slug)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/rirs/", p.pagination).await
}

// --------------------------------------------------------------------------
// VLAN Groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VlanGroupsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by VLAN group name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by site ID")]
    pub site_id: Option<i32>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn vlan_groups_list(
    client: &NetboxClient,
    p: VlanGroupsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .opt("site_id", p.site_id)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/vlan-groups/", p.pagination).await
}

// --------------------------------------------------------------------------
// Services
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ServicesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by service name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by device ID")]
    pub device_id: Option<i32>,
    #[schemars(description = "Filter by virtual machine ID")]
    pub virtual_machine_id: Option<i32>,
    #[schemars(description = "Filter by protocol (tcp, udp, sctp)")]
    pub protocol: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn services_list(
    client: &NetboxClient,
    p: ServicesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .opt("device_id", p.device_id)
        .opt("virtual_machine_id", p.virtual_machine_id)
        .many("protocol", p.protocol)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/services/", p.pagination).await
}

// --------------------------------------------------------------------------
// ASNs
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AsnsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by AS number")]
    pub asn: Option<i64>,
    #[schemars(description = "Filter by RIR slug")]
    pub rir: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn asns_list(client: &NetboxClient, p: AsnsListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .opt("asn", p.asn)
        .many("rir", p.rir)
        .many("tenant", p.tenant)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/asns/", p.pagination).await
}

// --------------------------------------------------------------------------
// FHRP Groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FhrpGroupsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(
        description = "Filter by protocol (vrrp2, vrrp3, carp, clusterxl, hsrp, glbp, other)"
    )]
    pub protocol: Option<Vec<String>>,
    #[schemars(description = "Filter by group ID number")]
    pub group_id: Option<i32>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn fhrp_groups_list(
    client: &NetboxClient,
    p: FhrpGroupsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("protocol", p.protocol)
        .opt("group_id", p.group_id)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/fhrp-groups/", p.pagination).await
}

// --------------------------------------------------------------------------
// FHRP Group Assignments
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FhrpGroupAssignmentsListParams {
    #[schemars(description = "Filter by FHRP group ID")]
    pub group_id: Option<i32>,
    #[schemars(description = "Filter by interface type (e.g. dcim.interface)")]
    pub interface_type: Option<String>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn fhrp_group_assignments_list(
    client: &NetboxClient,
    p: FhrpGroupAssignmentsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("group_id", p.group_id)
        .opt("interface_type", p.interface_type)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/fhrp-group-assignments/", p.pagination)
        .await
}

// --------------------------------------------------------------------------
// Roles (IP/VLAN roles)
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RolesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by role name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn roles_list(client: &NetboxClient, p: RolesListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("slug", p.slug)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/roles/", p.pagination).await
}

use crate::client::{NetboxClient, NetboxError};
use crate::tools::clamp_limit;
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
    #[schemars(description = "Filter by VRF ID")]
    pub vrf_id: Option<i32>,
    #[schemars(description = "Filter by status (active, reserved, deprecated)")]
    pub status: Option<Vec<String>>,
    #[schemars(
        description = "Filter by role (loopback, secondary, anycast, vip, vrrp, hsrp, glbp, carp)"
    )]
    pub role: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn ip_addresses_list(
    client: &NetboxClient,
    p: IpAddressesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.address.unwrap_or_default() {
        params.push(("address", v));
    }
    if let Some(v) = p.vrf_id {
        params.push(("vrf_id", v.to_string()));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    for v in p.role.unwrap_or_default() {
        params.push(("role", v));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/ipam/ip-addresses/", &params).await
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
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn prefixes_list(
    client: &NetboxClient,
    p: PrefixesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.prefix.unwrap_or_default() {
        params.push(("prefix", v));
    }
    if let Some(v) = p.vrf_id {
        params.push(("vrf_id", v.to_string()));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    for v in p.role.unwrap_or_default() {
        params.push(("role", v));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    if let Some(v) = p.family {
        params.push(("family", v.to_string()));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/ipam/prefixes/", &params).await
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
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn vrfs_list(client: &NetboxClient, p: VrfsListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.rd.unwrap_or_default() {
        params.push(("rd", v));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/ipam/vrfs/", &params).await
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
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn vlans_list(client: &NetboxClient, p: VlansListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    if let Some(v) = p.vid {
        params.push(("vid", v.to_string()));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    for v in p.role.unwrap_or_default() {
        params.push(("role", v));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    for v in p.group.unwrap_or_default() {
        params.push(("group", v));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/ipam/vlans/", &params).await
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
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn aggregates_list(
    client: &NetboxClient,
    p: AggregatesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.prefix.unwrap_or_default() {
        params.push(("prefix", v));
    }
    for v in p.rir.unwrap_or_default() {
        params.push(("rir", v));
    }
    if let Some(v) = p.family {
        params.push(("family", v.to_string()));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/ipam/aggregates/", &params).await
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
    #[schemars(description = "Filter by VRF ID")]
    pub vrf_id: Option<i32>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn ip_ranges_list(
    client: &NetboxClient,
    p: IpRangesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    for v in p.role.unwrap_or_default() {
        params.push(("role", v));
    }
    if let Some(v) = p.vrf_id {
        params.push(("vrf_id", v.to_string()));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/ipam/ip-ranges/", &params).await
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
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn route_targets_list(
    client: &NetboxClient,
    p: RouteTargetsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/ipam/route-targets/", &params).await
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
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn rirs_list(client: &NetboxClient, p: RirsListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.slug.unwrap_or_default() {
        params.push(("slug", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/ipam/rirs/", &params).await
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
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn vlan_groups_list(
    client: &NetboxClient,
    p: VlanGroupsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    if let Some(v) = p.site_id {
        params.push(("site_id", v.to_string()));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/ipam/vlan-groups/", &params).await
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
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn services_list(
    client: &NetboxClient,
    p: ServicesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    if let Some(v) = p.device_id {
        params.push(("device_id", v.to_string()));
    }
    if let Some(v) = p.virtual_machine_id {
        params.push(("virtual_machine_id", v.to_string()));
    }
    for v in p.protocol.unwrap_or_default() {
        params.push(("protocol", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/ipam/services/", &params).await
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
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn asns_list(client: &NetboxClient, p: AsnsListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    if let Some(v) = p.asn {
        params.push(("asn", v.to_string()));
    }
    for v in p.rir.unwrap_or_default() {
        params.push(("rir", v));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/ipam/asns/", &params).await
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
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn fhrp_groups_list(
    client: &NetboxClient,
    p: FhrpGroupsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.protocol.unwrap_or_default() {
        params.push(("protocol", v));
    }
    if let Some(v) = p.group_id {
        params.push(("group_id", v.to_string()));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/ipam/fhrp-groups/", &params).await
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
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn fhrp_group_assignments_list(
    client: &NetboxClient,
    p: FhrpGroupAssignmentsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(v) = p.group_id {
        params.push(("group_id", v.to_string()));
    }
    if let Some(v) = p.interface_type {
        params.push(("interface_type", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client
        .list("/api/ipam/fhrp-group-assignments/", &params)
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
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn roles_list(client: &NetboxClient, p: RolesListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.slug.unwrap_or_default() {
        params.push(("slug", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/ipam/roles/", &params).await
}

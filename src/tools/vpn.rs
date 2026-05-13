use crate::client::{NetboxClient, NetboxError};
use crate::tools::clamp_limit;
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Tunnels
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TunnelsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by tunnel name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by status (planned, active, disabled)")]
    pub status: Option<Vec<String>>,
    #[schemars(
        description = "Filter by encapsulation (ipsec-transport, ipsec-tunnel, ip-ip, gre)"
    )]
    pub encapsulation: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn tunnels_list(
    client: &NetboxClient,
    p: TunnelsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    for v in p.encapsulation.unwrap_or_default() {
        params.push(("encapsulation", v));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/vpn/tunnels/", &params).await
}

// --------------------------------------------------------------------------
// Tunnel Groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TunnelGroupsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by tunnel group name")]
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

pub async fn tunnel_groups_list(
    client: &NetboxClient,
    p: TunnelGroupsListParams,
) -> Result<Value, NetboxError> {
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
    client.list("/api/vpn/tunnel-groups/", &params).await
}

// --------------------------------------------------------------------------
// L2VPNs
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct L2vpnsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by L2VPN name")]
    pub name: Option<Vec<String>>,
    #[schemars(
        description = "Filter by type (vpws, vpls, vxlan, vxlan-evpn, mpls-evpn, pbb-evpn, epl, evpl, ep-lan, evp-lan, ep-tree, evp-tree)"
    )]
    pub r#type: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn l2vpns_list(client: &NetboxClient, p: L2vpnsListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.r#type.unwrap_or_default() {
        params.push(("type", v));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/vpn/l2vpns/", &params).await
}

// --------------------------------------------------------------------------
// IKE Policies
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct IkePoliciesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by IKE policy name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by IKE version (1 or 2)")]
    pub version: Option<i32>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn ike_policies_list(
    client: &NetboxClient,
    p: IkePoliciesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    if let Some(v) = p.version {
        params.push(("version", v.to_string()));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/vpn/ike-policies/", &params).await
}

// --------------------------------------------------------------------------
// IPSec Policies
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct IpsecPoliciesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by IPSec policy name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by PFS group")]
    pub pfs_group: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn ipsec_policies_list(
    client: &NetboxClient,
    p: IpsecPoliciesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.pfs_group.unwrap_or_default() {
        params.push(("pfs_group", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/vpn/ipsec-policies/", &params).await
}

// --------------------------------------------------------------------------
// Tunnel Terminations
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TunnelTerminationsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by tunnel ID")]
    pub tunnel_id: Option<i32>,
    #[schemars(description = "Filter by termination role (peer, hub, spoke)")]
    pub role: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn tunnel_terminations_list(
    client: &NetboxClient,
    p: TunnelTerminationsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    if let Some(v) = p.tunnel_id {
        params.push(("tunnel_id", v.to_string()));
    }
    for v in p.role.unwrap_or_default() {
        params.push(("role", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/vpn/tunnel-terminations/", &params).await
}

use crate::client::{NetboxClient, NetboxError};
use crate::tools::{CommonListParams, QueryBuilder};
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Tunnels
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TunnelsListParams {
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
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn tunnels_list(
    client: &NetboxClient,
    p: TunnelsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("status", p.status)
        .many("encapsulation", p.encapsulation)
        .many("tenant", p.tenant)
        .many("tag", p.tag);
    qb.run_common(client, "/api/vpn/tunnels/", p.common).await
}

// --------------------------------------------------------------------------
// Tunnel Groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TunnelGroupsListParams {
    #[schemars(description = "Filter by tunnel group name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn tunnel_groups_list(
    client: &NetboxClient,
    p: TunnelGroupsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("slug", p.slug);
    qb.run_common(client, "/api/vpn/tunnel-groups/", p.common)
        .await
}

// --------------------------------------------------------------------------
// L2VPNs
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct L2vpnsListParams {
    #[schemars(description = "Filter by L2VPN name")]
    pub name: Option<Vec<String>>,
    #[schemars(
        description = "Filter by type (vpws, vpls, vxlan, vxlan-evpn, mpls-evpn, pbb-evpn, epl, evpl, ep-lan, evp-lan, ep-tree, evp-tree)"
    )]
    pub r#type: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn l2vpns_list(client: &NetboxClient, p: L2vpnsListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("type", p.r#type)
        .many("tenant", p.tenant)
        .many("tag", p.tag);
    qb.run_common(client, "/api/vpn/l2vpns/", p.common).await
}

// --------------------------------------------------------------------------
// IKE Policies
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct IkePoliciesListParams {
    #[schemars(description = "Filter by IKE policy name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by IKE version (1 or 2)")]
    pub version: Option<i32>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn ike_policies_list(
    client: &NetboxClient,
    p: IkePoliciesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .opt("version", p.version);
    qb.run_common(client, "/api/vpn/ike-policies/", p.common)
        .await
}

// --------------------------------------------------------------------------
// IPSec Policies
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct IpsecPoliciesListParams {
    #[schemars(description = "Filter by IPSec policy name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by PFS group")]
    pub pfs_group: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn ipsec_policies_list(
    client: &NetboxClient,
    p: IpsecPoliciesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("pfs_group", p.pfs_group);
    qb.run_common(client, "/api/vpn/ipsec-policies/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Tunnel Terminations
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TunnelTerminationsListParams {
    #[schemars(description = "Filter by tunnel ID")]
    pub tunnel_id: Option<i32>,
    #[schemars(description = "Filter by termination role (peer, hub, spoke)")]
    pub role: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn tunnel_terminations_list(
    client: &NetboxClient,
    p: TunnelTerminationsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("tunnel_id", p.tunnel_id)
        .many("role", p.role);
    qb.run_common(client, "/api/vpn/tunnel-terminations/", p.common)
        .await
}

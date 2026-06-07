use crate::client::{NetboxClient, NetboxError};
use crate::tools::{PaginationParams, QueryBuilder};
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Wireless LANs
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct LansListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by SSID")]
    pub ssid: Option<Vec<String>>,
    #[schemars(description = "Filter by wireless LAN group slug")]
    pub group: Option<Vec<String>>,
    #[schemars(description = "Filter by status (active, reserved, disabled, deprecated)")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn lans_list(client: &NetboxClient, p: LansListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("ssid", p.ssid)
        .many("group", p.group)
        .many("status", p.status)
        .many("tenant", p.tenant)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/wireless/wireless-lans/", p.pagination)
        .await
}

// --------------------------------------------------------------------------
// Wireless LAN Groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct LanGroupsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by group name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by parent group slug")]
    pub parent: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn lan_groups_list(
    client: &NetboxClient,
    p: LanGroupsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("parent", p.parent)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/wireless/wireless-lan-groups/", p.pagination)
        .await
}

// --------------------------------------------------------------------------
// Wireless Links
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct LinksListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by status (active, planned, decommissioning)")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by SSID")]
    pub ssid: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn links_list(client: &NetboxClient, p: LinksListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("status", p.status)
        .many("tenant", p.tenant)
        .many("ssid", p.ssid)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/wireless/wireless-links/", p.pagination)
        .await
}

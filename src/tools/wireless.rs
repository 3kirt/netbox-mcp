use crate::client::{NetboxClient, NetboxError};
use crate::tools::{QueryBuilder, paginate};
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
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
    paginate(client, "/api/wireless/wireless-lans/", qb.into_params(), p.limit, p.offset, p.fetch_all)
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
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
    paginate(client, "/api/wireless/wireless-lan-groups/", qb.into_params(), p.limit, p.offset, p.fetch_all)
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn links_list(client: &NetboxClient, p: LinksListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("status", p.status)
        .many("tenant", p.tenant)
        .many("ssid", p.ssid)
        .opt("ordering", p.ordering);
    paginate(client, "/api/wireless/wireless-links/", qb.into_params(), p.limit, p.offset, p.fetch_all)
        .await
}

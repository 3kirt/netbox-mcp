use crate::client::{NetboxClient, NetboxError};
use crate::tools::clamp_limit;
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
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn lans_list(client: &NetboxClient, p: LansListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.ssid.unwrap_or_default() {
        params.push(("ssid", v));
    }
    for v in p.group.unwrap_or_default() {
        params.push(("group", v));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/wireless/wireless-lans/", &params).await
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
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn lan_groups_list(
    client: &NetboxClient,
    p: LanGroupsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.parent.unwrap_or_default() {
        params.push(("parent", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client
        .list("/api/wireless/wireless-lan-groups/", &params)
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
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn links_list(client: &NetboxClient, p: LinksListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    for v in p.ssid.unwrap_or_default() {
        params.push(("ssid", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/wireless/wireless-links/", &params).await
}

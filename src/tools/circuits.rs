use crate::client::{NetboxClient, NetboxError};
use crate::tools::clamp_limit;
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Circuits
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CircuitsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by provider slug")]
    pub provider: Option<Vec<String>>,
    #[schemars(
        description = "Filter by status (active, planned, provisioning, offline, deprovisioning, decommissioned)"
    )]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by circuit type slug")]
    pub r#type: Option<Vec<String>>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn circuits_list(
    client: &NetboxClient,
    p: CircuitsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.provider.unwrap_or_default() {
        params.push(("provider", v));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    for v in p.r#type.unwrap_or_default() {
        params.push(("type", v));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/circuits/circuits/", &params).await
}

// --------------------------------------------------------------------------
// Providers
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ProvidersListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by provider name")]
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

pub async fn providers_list(
    client: &NetboxClient,
    p: ProvidersListParams,
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
    client.list("/api/circuits/providers/", &params).await
}

// --------------------------------------------------------------------------
// Circuit Types
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CircuitTypesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by circuit type name")]
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

pub async fn circuit_types_list(
    client: &NetboxClient,
    p: CircuitTypesListParams,
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
    client.list("/api/circuits/circuit-types/", &params).await
}

// --------------------------------------------------------------------------
// Circuit Terminations
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CircuitTerminationsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by circuit ID")]
    pub circuit_id: Option<i32>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by termination side (A or Z)")]
    pub term_side: Option<String>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn circuit_terminations_list(
    client: &NetboxClient,
    p: CircuitTerminationsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    if let Some(v) = p.circuit_id {
        params.push(("circuit_id", v.to_string()));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    if let Some(v) = p.term_side {
        params.push(("term_side", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client
        .list("/api/circuits/circuit-terminations/", &params)
        .await
}

// --------------------------------------------------------------------------
// Provider Accounts
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ProviderAccountsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by provider slug")]
    pub provider: Option<Vec<String>>,
    #[schemars(description = "Filter by account name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn provider_accounts_list(
    client: &NetboxClient,
    p: ProviderAccountsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.provider.unwrap_or_default() {
        params.push(("provider", v));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client
        .list("/api/circuits/provider-accounts/", &params)
        .await
}

// --------------------------------------------------------------------------
// Provider Networks
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ProviderNetworksListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by provider slug")]
    pub provider: Option<Vec<String>>,
    #[schemars(description = "Filter by network name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn provider_networks_list(
    client: &NetboxClient,
    p: ProviderNetworksListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.provider.unwrap_or_default() {
        params.push(("provider", v));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client
        .list("/api/circuits/provider-networks/", &params)
        .await
}

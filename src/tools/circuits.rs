use crate::client::{NetboxClient, NetboxError};
use crate::tools::{PaginationParams, QueryBuilder};
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
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn circuits_list(
    client: &NetboxClient,
    p: CircuitsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("provider", p.provider)
        .many("status", p.status)
        .many("type", p.r#type)
        .many("site", p.site)
        .many("tenant", p.tenant)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/circuits/circuits/", p.pagination)
        .await
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
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn providers_list(
    client: &NetboxClient,
    p: ProvidersListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("slug", p.slug)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/circuits/providers/", p.pagination)
        .await
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
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn circuit_types_list(
    client: &NetboxClient,
    p: CircuitTypesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("slug", p.slug)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/circuits/circuit-types/", p.pagination)
        .await
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
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn circuit_terminations_list(
    client: &NetboxClient,
    p: CircuitTerminationsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .opt("circuit_id", p.circuit_id)
        .many("site", p.site)
        .opt("term_side", p.term_side)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/circuits/circuit-terminations/", p.pagination)
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
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn provider_accounts_list(
    client: &NetboxClient,
    p: ProviderAccountsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("provider", p.provider)
        .many("name", p.name)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/circuits/provider-accounts/", p.pagination)
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
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn provider_networks_list(
    client: &NetboxClient,
    p: ProviderNetworksListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("provider", p.provider)
        .many("name", p.name)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/circuits/provider-networks/", p.pagination)
        .await
}

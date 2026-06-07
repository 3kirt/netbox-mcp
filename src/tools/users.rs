use crate::client::{NetboxClient, NetboxError};
use crate::tools::{PaginationParams, QueryBuilder};
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Users
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UsersListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by username")]
    pub username: Option<Vec<String>>,
    #[schemars(description = "Filter by active state")]
    pub is_active: Option<bool>,
    #[schemars(description = "Filter by staff state")]
    pub is_staff: Option<bool>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn users_list(client: &NetboxClient, p: UsersListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("username", p.username)
        .opt("is_active", p.is_active)
        .opt("is_staff", p.is_staff)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/users/users/", p.pagination).await
}

// --------------------------------------------------------------------------
// Groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GroupsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by group name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn groups_list(client: &NetboxClient, p: GroupsListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/users/groups/", p.pagination).await
}

// --------------------------------------------------------------------------
// API Tokens
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TokensListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by user ID")]
    pub user_id: Option<i32>,
    #[schemars(description = "Filter by username")]
    pub user: Option<Vec<String>>,
    #[schemars(description = "Filter by active state")]
    pub is_active: Option<bool>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn tokens_list(client: &NetboxClient, p: TokensListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .opt("user_id", p.user_id)
        .many("user", p.user)
        .opt("is_active", p.is_active)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/users/tokens/", p.pagination).await
}

use crate::client::{NetboxClient, NetboxError};
use crate::tools::{CommonListParams, QueryBuilder};
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Users
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UsersListParams {
    #[schemars(description = "Filter by username")]
    pub username: Option<Vec<String>>,
    #[schemars(description = "Filter by active state")]
    pub is_active: Option<bool>,
    #[schemars(description = "Filter by staff state")]
    pub is_staff: Option<bool>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn users_list(client: &NetboxClient, p: UsersListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("username", p.username)
        .opt("is_active", p.is_active)
        .opt("is_staff", p.is_staff);
    qb.run_common(client, "/api/users/users/", p.common).await
}

// --------------------------------------------------------------------------
// Groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GroupsListParams {
    #[schemars(description = "Filter by group name")]
    pub name: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn groups_list(client: &NetboxClient, p: GroupsListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new().many("name", p.name);
    qb.run_common(client, "/api/users/groups/", p.common).await
}

// --------------------------------------------------------------------------
// API Tokens
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TokensListParams {
    #[schemars(description = "Filter by user ID")]
    pub user_id: Option<i32>,
    #[schemars(description = "Filter by username")]
    pub user: Option<Vec<String>>,
    #[schemars(description = "Filter by active state")]
    pub is_active: Option<bool>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn tokens_list(client: &NetboxClient, p: TokensListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("user_id", p.user_id)
        .many("user", p.user)
        .opt("is_active", p.is_active);
    qb.run_common(client, "/api/users/tokens/", p.common).await
}

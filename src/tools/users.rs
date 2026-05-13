use crate::client::{NetboxClient, NetboxError};
use crate::tools::clamp_limit;
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
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn users_list(client: &NetboxClient, p: UsersListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.username.unwrap_or_default() {
        params.push(("username", v));
    }
    if let Some(v) = p.is_active {
        params.push(("is_active", v.to_string()));
    }
    if let Some(v) = p.is_staff {
        params.push(("is_staff", v.to_string()));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/users/users/", &params).await
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
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn groups_list(client: &NetboxClient, p: GroupsListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/users/groups/", &params).await
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
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn tokens_list(client: &NetboxClient, p: TokensListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    if let Some(v) = p.user_id {
        params.push(("user_id", v.to_string()));
    }
    for v in p.user.unwrap_or_default() {
        params.push(("user", v));
    }
    if let Some(v) = p.is_active {
        params.push(("is_active", v.to_string()));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/users/tokens/", &params).await
}

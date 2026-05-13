use crate::client::{NetboxClient, NetboxError};
use crate::tools::clamp_limit;
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Data Sources
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DataSourcesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by data source name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by status (new, queued, syncing, completed, failed)")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn data_sources_list(
    client: &NetboxClient,
    p: DataSourcesListParams,
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
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/core/data-sources/", &params).await
}

// --------------------------------------------------------------------------
// Background Jobs
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JobsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(
        description = "Filter by job status (pending, running, completed, errored, failed)"
    )]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by object type (e.g. dcim.device)")]
    pub object_type: Option<String>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn jobs_list(client: &NetboxClient, p: JobsListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    if let Some(v) = p.object_type {
        params.push(("object_type", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/core/jobs/", &params).await
}

// --------------------------------------------------------------------------
// Object Changes (audit log)
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ObjectChangesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by username")]
    pub user: Option<Vec<String>>,
    #[schemars(description = "Filter by action (create, update, delete)")]
    pub action: Option<Vec<String>>,
    #[schemars(description = "Filter by changed object type (e.g. dcim.device)")]
    pub changed_object_type: Option<String>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn object_changes_list(
    client: &NetboxClient,
    p: ObjectChangesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.user.unwrap_or_default() {
        params.push(("user", v));
    }
    for v in p.action.unwrap_or_default() {
        params.push(("action", v));
    }
    if let Some(v) = p.changed_object_type {
        params.push(("changed_object_type", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/core/object-changes/", &params).await
}

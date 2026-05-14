use crate::client::{NetboxClient, NetboxError};
use crate::tools::{QueryBuilder, paginate};
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

pub async fn data_sources_list(
    client: &NetboxClient,
    p: DataSourcesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("status", p.status)
        .opt("ordering", p.ordering);
    paginate(client, "/api/core/data-sources/", qb.into_params(), p.limit, p.offset, p.fetch_all)
        .await
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

pub async fn jobs_list(client: &NetboxClient, p: JobsListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("status", p.status)
        .opt("object_type", p.object_type)
        .opt("ordering", p.ordering);
    paginate(client, "/api/core/jobs/", qb.into_params(), p.limit, p.offset, p.fetch_all)
        .await
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

pub async fn object_changes_list(
    client: &NetboxClient,
    p: ObjectChangesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("user", p.user)
        .many("action", p.action)
        .opt("changed_object_type", p.changed_object_type)
        .opt("ordering", p.ordering);
    paginate(client, "/api/core/object-changes/", qb.into_params(), p.limit, p.offset, p.fetch_all)
        .await
}

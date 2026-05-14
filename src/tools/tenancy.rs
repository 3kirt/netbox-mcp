use crate::client::{NetboxClient, NetboxError};
use crate::tools::{QueryBuilder, paginate};
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Tenants
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TenantsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by tenant name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant group slug")]
    pub group: Option<Vec<String>>,
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

pub async fn tenants_list(
    client: &NetboxClient,
    p: TenantsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("group", p.group)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    paginate(client, "/api/tenancy/tenants/", qb.into_params(), p.limit, p.offset, p.fetch_all)
        .await
}

// --------------------------------------------------------------------------
// Tenant Groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TenantGroupsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by tenant group name")]
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

pub async fn tenant_groups_list(
    client: &NetboxClient,
    p: TenantGroupsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("parent", p.parent)
        .opt("ordering", p.ordering);
    paginate(client, "/api/tenancy/tenant-groups/", qb.into_params(), p.limit, p.offset, p.fetch_all)
        .await
}

// --------------------------------------------------------------------------
// Contacts
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ContactsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by contact name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by contact group slug")]
    pub group: Option<Vec<String>>,
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

pub async fn contacts_list(
    client: &NetboxClient,
    p: ContactsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("group", p.group)
        .opt("ordering", p.ordering);
    paginate(client, "/api/tenancy/contacts/", qb.into_params(), p.limit, p.offset, p.fetch_all)
        .await
}

// --------------------------------------------------------------------------
// Contact Groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ContactGroupsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by contact group name")]
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

pub async fn contact_groups_list(
    client: &NetboxClient,
    p: ContactGroupsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("parent", p.parent)
        .opt("ordering", p.ordering);
    paginate(client, "/api/tenancy/contact-groups/", qb.into_params(), p.limit, p.offset, p.fetch_all)
        .await
}

// --------------------------------------------------------------------------
// Contact Roles
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ContactRolesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by role name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
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

pub async fn contact_roles_list(
    client: &NetboxClient,
    p: ContactRolesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("slug", p.slug)
        .opt("ordering", p.ordering);
    paginate(client, "/api/tenancy/contact-roles/", qb.into_params(), p.limit, p.offset, p.fetch_all)
        .await
}

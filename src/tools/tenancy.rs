use crate::client::{NetboxClient, NetboxError};
use crate::tools::{PaginationParams, QueryBuilder};
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
    #[serde(flatten)]
    pub pagination: PaginationParams,
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
    qb.run(client, "/api/tenancy/tenants/", p.pagination).await
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
    #[serde(flatten)]
    pub pagination: PaginationParams,
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
    qb.run(client, "/api/tenancy/tenant-groups/", p.pagination)
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
    #[serde(flatten)]
    pub pagination: PaginationParams,
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
    qb.run(client, "/api/tenancy/contacts/", p.pagination).await
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
    #[serde(flatten)]
    pub pagination: PaginationParams,
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
    qb.run(client, "/api/tenancy/contact-groups/", p.pagination)
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
    #[serde(flatten)]
    pub pagination: PaginationParams,
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
    qb.run(client, "/api/tenancy/contact-roles/", p.pagination)
        .await
}

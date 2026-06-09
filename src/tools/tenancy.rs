use crate::client::{NetboxClient, NetboxError};
use crate::tools::{CommonListParams, QueryBuilder};
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Tenants
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TenantsListParams {
    #[schemars(description = "Filter by tenant name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant group slug")]
    pub group: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn tenants_list(
    client: &NetboxClient,
    p: TenantsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("group", p.group)
        .many("tag", p.tag);
    qb.run_common(client, "/api/tenancy/tenants/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Tenant Groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TenantGroupsListParams {
    #[schemars(description = "Filter by tenant group name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by parent group slug")]
    pub parent: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn tenant_groups_list(
    client: &NetboxClient,
    p: TenantGroupsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("parent", p.parent);
    qb.run_common(client, "/api/tenancy/tenant-groups/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Contacts
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ContactsListParams {
    #[schemars(description = "Filter by contact name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by contact group slug")]
    pub group: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn contacts_list(
    client: &NetboxClient,
    p: ContactsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("group", p.group);
    qb.run_common(client, "/api/tenancy/contacts/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Contact Groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ContactGroupsListParams {
    #[schemars(description = "Filter by contact group name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by parent group slug")]
    pub parent: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn contact_groups_list(
    client: &NetboxClient,
    p: ContactGroupsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("parent", p.parent);
    qb.run_common(client, "/api/tenancy/contact-groups/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Contact Roles
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ContactRolesListParams {
    #[schemars(description = "Filter by role name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn contact_roles_list(
    client: &NetboxClient,
    p: ContactRolesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("slug", p.slug);
    qb.run_common(client, "/api/tenancy/contact-roles/", p.common)
        .await
}

use crate::client::{NetboxClient, NetboxError};
use crate::tools::{PaginationParams, QueryBuilder, resolve_vm_id_or};
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Virtual Machines
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VmsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by VM name")]
    pub name: Option<Vec<String>>,
    #[schemars(
        description = "Filter by status (active, planned, staged, failed, offline, decommissioning)"
    )]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by cluster name")]
    pub cluster: Option<Vec<String>>,
    #[schemars(description = "Filter by device role slug")]
    pub role: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by platform slug")]
    pub platform: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn vms_list(client: &NetboxClient, p: VmsListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("status", p.status)
        .many("site", p.site)
        .many("cluster", p.cluster)
        .many("role", p.role)
        .many("tenant", p.tenant)
        .many("platform", p.platform)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(
        client,
        "/api/virtualization/virtual-machines/",
        p.pagination,
    )
    .await
}

// --------------------------------------------------------------------------
// Clusters
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ClustersListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by cluster name")]
    pub name: Option<Vec<String>>,
    #[schemars(
        description = "Filter by status (planned, staging, active, decommissioning, offline)"
    )]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by cluster group slug")]
    pub group: Option<Vec<String>>,
    #[schemars(description = "Filter by cluster type slug")]
    pub r#type: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn clusters_list(
    client: &NetboxClient,
    p: ClustersListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("status", p.status)
        .many("site", p.site)
        .many("group", p.group)
        .many("type", p.r#type)
        .many("tenant", p.tenant)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/virtualization/clusters/", p.pagination)
        .await
}

// --------------------------------------------------------------------------
// Cluster Groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ClusterGroupsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by cluster group name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn cluster_groups_list(
    client: &NetboxClient,
    p: ClusterGroupsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("slug", p.slug)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/virtualization/cluster-groups/", p.pagination)
        .await
}

// --------------------------------------------------------------------------
// Cluster Types
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ClusterTypesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by cluster type name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn cluster_types_list(
    client: &NetboxClient,
    p: ClusterTypesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("slug", p.slug)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/virtualization/cluster-types/", p.pagination)
        .await
}

// --------------------------------------------------------------------------
// VM Interfaces
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct InterfacesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by interface name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by enabled state")]
    pub enabled: Option<bool>,
    #[schemars(description = "Filter by virtual machine name (preferred over virtual_machine_id)")]
    pub virtual_machine: Option<String>,
    #[schemars(description = "Filter by virtual machine ID")]
    pub virtual_machine_id: Option<i32>,
    #[schemars(description = "Filter by MAC address")]
    pub mac_address: Option<String>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn interfaces_list(
    client: &NetboxClient,
    p: InterfacesListParams,
) -> Result<Value, NetboxError> {
    let virtual_machine_id =
        resolve_vm_id_or(client, p.virtual_machine, p.virtual_machine_id).await?;
    let qb = QueryBuilder::new()
        .opt("virtual_machine_id", virtual_machine_id)
        .opt("q", p.q)
        .many("name", p.name)
        .opt("enabled", p.enabled)
        .opt("mac_address", p.mac_address)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/virtualization/interfaces/", p.pagination)
        .await
}

// --------------------------------------------------------------------------
// Virtual Disks
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VirtualDisksListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by disk name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by virtual machine name (preferred over virtual_machine_id)")]
    pub virtual_machine: Option<String>,
    #[schemars(description = "Filter by virtual machine ID")]
    pub virtual_machine_id: Option<i32>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn virtual_disks_list(
    client: &NetboxClient,
    p: VirtualDisksListParams,
) -> Result<Value, NetboxError> {
    let virtual_machine_id =
        resolve_vm_id_or(client, p.virtual_machine, p.virtual_machine_id).await?;
    let qb = QueryBuilder::new()
        .opt("virtual_machine_id", virtual_machine_id)
        .opt("q", p.q)
        .many("name", p.name)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/virtualization/virtual-disks/", p.pagination)
        .await
}

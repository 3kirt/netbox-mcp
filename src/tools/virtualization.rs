use crate::client::{NetboxClient, NetboxError};
use crate::tools::{PaginationParams, QueryBuilder, resolve_vm_id_or};
use serde::Deserialize;
use serde_json::{Map, Value};

const VMS_PATH: &str = "/api/virtualization/virtual-machines/";

/// Insert `(key, v)` into a JSON body map only when `v` is `Some`, so create
/// bodies carry only the fields the caller set and PATCH stays partial.
fn insert_opt<T: Into<Value>>(map: &mut Map<String, Value>, key: &str, v: Option<T>) {
    if let Some(v) = v {
        map.insert(key.to_string(), v.into());
    }
}

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
// Virtual Machines — write (create / update / delete)
//
// Foreign-key fields are taken as numeric NetBox IDs; use the corresponding
// *_list tools to find them. NetBox requires `name` plus exactly one of
// cluster/site/device on create — its 400 response spells out any violation.
// --------------------------------------------------------------------------

/// Writable VM attributes shared by create and update. Foreign keys are NetBox
/// IDs. Flattened into the create/update params so both expose the same fields.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VmFields {
    #[schemars(description = "Cluster ID (NetBox requires one of cluster/site/device)")]
    pub cluster: Option<i32>,
    #[schemars(description = "Site ID")]
    pub site: Option<i32>,
    #[schemars(description = "Device ID (host the VM runs on)")]
    pub device: Option<i32>,
    #[schemars(description = "Device role ID")]
    pub role: Option<i32>,
    #[schemars(description = "Tenant ID")]
    pub tenant: Option<i32>,
    #[schemars(description = "Platform ID")]
    pub platform: Option<i32>,
    #[schemars(
        description = "Status (active, planned, staged, failed, offline, decommissioning); defaults to active on create"
    )]
    pub status: Option<String>,
    #[schemars(description = "Number of vCPUs")]
    pub vcpus: Option<f64>,
    #[schemars(description = "Memory in MB")]
    pub memory: Option<i64>,
    #[schemars(description = "Disk in MB")]
    pub disk: Option<i64>,
    #[schemars(description = "Short description")]
    pub description: Option<String>,
    #[schemars(description = "Long-form comments")]
    pub comments: Option<String>,
    #[schemars(description = "Tag IDs (on update, replaces the existing set)")]
    pub tags: Option<Vec<i32>>,
}

impl VmFields {
    /// Insert every set field into a request body map, omitting `None`s so
    /// PATCH stays a partial update.
    fn insert_into(self, body: &mut Map<String, Value>) {
        insert_opt(body, "cluster", self.cluster);
        insert_opt(body, "site", self.site);
        insert_opt(body, "device", self.device);
        insert_opt(body, "role", self.role);
        insert_opt(body, "tenant", self.tenant);
        insert_opt(body, "platform", self.platform);
        insert_opt(body, "status", self.status);
        insert_opt(body, "vcpus", self.vcpus);
        insert_opt(body, "memory", self.memory);
        insert_opt(body, "disk", self.disk);
        insert_opt(body, "description", self.description);
        insert_opt(body, "comments", self.comments);
        insert_opt(body, "tags", self.tags);
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VmCreateParams {
    #[schemars(description = "VM name (required)")]
    pub name: String,
    #[serde(flatten)]
    pub fields: VmFields,
}

pub async fn vm_create(client: &NetboxClient, p: VmCreateParams) -> Result<Value, NetboxError> {
    let mut body = Map::new();
    body.insert("name".to_string(), Value::String(p.name));
    p.fields.insert_into(&mut body);
    client.post(VMS_PATH, &Value::Object(body)).await
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VmUpdateParams {
    #[schemars(description = "NetBox ID of the VM to update (required)")]
    pub id: i32,
    #[schemars(description = "New VM name")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub fields: VmFields,
}

pub async fn vm_update(client: &NetboxClient, p: VmUpdateParams) -> Result<Value, NetboxError> {
    let mut body = Map::new();
    insert_opt(&mut body, "name", p.name);
    p.fields.insert_into(&mut body);
    client.patch(VMS_PATH, p.id, &Value::Object(body)).await
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VmDeleteParams {
    #[schemars(description = "NetBox ID of the VM to delete (required)")]
    pub id: i32,
}

pub async fn vm_delete(client: &NetboxClient, p: VmDeleteParams) -> Result<(), NetboxError> {
    client.delete(VMS_PATH, p.id).await
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

// --------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::{
        VmCreateParams, VmDeleteParams, VmFields, VmUpdateParams, vm_create, vm_delete, vm_update,
    };
    use crate::client::NetboxClient;

    fn mock_client(server: &MockServer) -> NetboxClient {
        NetboxClient::new(server.uri(), "test-token").unwrap()
    }

    /// All-`None` field set; tests override only what they exercise.
    fn empty_fields() -> VmFields {
        VmFields {
            cluster: None,
            site: None,
            device: None,
            role: None,
            tenant: None,
            platform: None,
            status: None,
            vcpus: None,
            memory: None,
            disk: None,
            description: None,
            comments: None,
            tags: None,
        }
    }

    #[tokio::test]
    async fn vm_create_sends_only_set_fields() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/virtualization/virtual-machines/"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "id": 1 })))
            .mount(&server)
            .await;

        vm_create(
            &mock_client(&server),
            VmCreateParams {
                name: "vm-01".into(),
                fields: VmFields {
                    cluster: Some(3),
                    status: Some("active".into()),
                    memory: Some(2048),
                    tags: Some(vec![5, 6]),
                    ..empty_fields()
                },
            },
        )
        .await
        .unwrap();

        let reqs = server.received_requests().await.unwrap();
        let body = reqs
            .iter()
            .find(|r| r.method == wiremock::http::Method::POST)
            .and_then(|r| r.body_json::<serde_json::Value>().ok())
            .expect("POST body");
        assert_eq!(body["name"], "vm-01");
        assert_eq!(body["cluster"], 3);
        assert_eq!(body["status"], "active");
        assert_eq!(body["memory"], 2048);
        assert_eq!(body["tags"], json!([5, 6]));
        // Unset optional fields must be omitted, not sent as null.
        assert!(body.get("site").is_none());
        assert!(body.get("vcpus").is_none());
        assert!(body.get("description").is_none());
    }

    #[tokio::test]
    async fn vm_update_patches_id_with_partial_body() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/virtualization/virtual-machines/7/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "id": 7 })))
            .mount(&server)
            .await;

        vm_update(
            &mock_client(&server),
            VmUpdateParams {
                id: 7,
                name: None,
                fields: VmFields {
                    status: Some("offline".into()),
                    ..empty_fields()
                },
            },
        )
        .await
        .unwrap();

        let reqs = server.received_requests().await.unwrap();
        let body = reqs
            .iter()
            .find(|r| r.method == wiremock::http::Method::PATCH)
            .and_then(|r| r.body_json::<serde_json::Value>().ok())
            .expect("PATCH body");
        // Only the single field under change is sent.
        assert_eq!(body["status"], "offline");
        assert_eq!(body.as_object().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn vm_delete_hits_id_path() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/virtualization/virtual-machines/7/"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        vm_delete(&mock_client(&server), VmDeleteParams { id: 7 })
            .await
            .unwrap();
    }
}

use crate::client::{NetboxClient, NetboxError};
use crate::tools::{BodyBuilder, PaginationParams, QueryBuilder};
use serde::Deserialize;
use serde_json::Value;

const IP_ADDRESSES_PATH: &str = "/api/ipam/ip-addresses/";

// --------------------------------------------------------------------------
// IP Addresses
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct IpAddressesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by IP address (e.g. 192.0.2.1/24)")]
    pub address: Option<Vec<String>>,
    #[schemars(
        description = "Filter by VRF route distinguisher / rd (e.g. 65000:100) — preferred over vrf_id"
    )]
    pub vrf: Option<Vec<String>>,
    #[schemars(description = "Filter by VRF ID")]
    pub vrf_id: Option<i32>,
    #[schemars(description = "Filter by status (active, reserved, deprecated)")]
    pub status: Option<Vec<String>>,
    #[schemars(
        description = "Filter by role (loopback, secondary, anycast, vip, vrrp, hsrp, glbp, carp)"
    )]
    pub role: Option<Vec<String>>,
    #[schemars(description = "Return IPs within this prefix (e.g. 10.254.2.144/30)")]
    pub parent: Option<String>,
    #[schemars(description = "Filter by device name (multi-value)")]
    pub device: Option<Vec<String>>,
    #[schemars(description = "Filter by device ID")]
    pub device_id: Option<i32>,
    #[schemars(description = "Filter by virtual machine name (multi-value)")]
    pub virtual_machine: Option<Vec<String>>,
    #[schemars(description = "Filter by virtual machine ID")]
    pub virtual_machine_id: Option<i32>,
    #[schemars(description = "Filter by DNS name (exact match, e.g. host.example.com)")]
    pub dns_name: Option<String>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn ip_addresses_list(
    client: &NetboxClient,
    p: IpAddressesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("address", p.address)
        .many("vrf", p.vrf)
        .opt("vrf_id", p.vrf_id)
        .many("status", p.status)
        .many("role", p.role)
        .opt("parent", p.parent)
        .many("device", p.device)
        .opt("device_id", p.device_id)
        .many("virtual_machine", p.virtual_machine)
        .opt("virtual_machine_id", p.virtual_machine_id)
        .opt("dns_name", p.dns_name)
        .many("tenant", p.tenant)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, IP_ADDRESSES_PATH, p.pagination).await
}

// --------------------------------------------------------------------------
// IP Addresses — write (create / update / delete)
//
// Foreign-key fields are numeric NetBox IDs (use the *_list tools to find
// them). To attach an IP to an interface, set both assigned_object_type
// ("dcim.interface" or "virtualization.vminterface") and assigned_object_id.
// --------------------------------------------------------------------------

/// Writable IP-address attributes shared by create and update. Flattened into
/// the create/update params so both expose the same fields.
#[derive(Debug, Default, Deserialize, schemars::JsonSchema)]
pub struct IpAddressFields {
    #[schemars(
        description = "Status (active, reserved, deprecated, dhcp, slaac); defaults to active on create"
    )]
    pub status: Option<String>,
    #[schemars(description = "Role (loopback, secondary, anycast, vip, vrrp, hsrp, glbp, carp)")]
    pub role: Option<String>,
    #[schemars(description = "VRF ID")]
    pub vrf: Option<i32>,
    #[schemars(description = "Tenant ID")]
    pub tenant: Option<i32>,
    #[schemars(description = "DNS name (e.g. host.example.com)")]
    pub dns_name: Option<String>,
    #[schemars(description = "Short description")]
    pub description: Option<String>,
    #[schemars(description = "Long-form comments")]
    pub comments: Option<String>,
    #[schemars(description = "NAT inside IP-address ID this address maps to")]
    pub nat_inside: Option<i32>,
    #[schemars(
        description = "Assignment object type: 'dcim.interface' or 'virtualization.vminterface' (set together with assigned_object_id)"
    )]
    pub assigned_object_type: Option<String>,
    #[schemars(description = "ID of the interface this IP is assigned to")]
    pub assigned_object_id: Option<i32>,
    #[schemars(description = "Tag IDs (on update, replaces the existing set)")]
    pub tags: Option<Vec<i32>>,
}

impl IpAddressFields {
    /// Apply every set field to `b`, omitting `None`s so PATCH stays a partial
    /// update. Shared by create and update so both bodies carry the same fields.
    fn apply(self, b: BodyBuilder) -> BodyBuilder {
        b.opt("status", self.status)
            .opt("role", self.role)
            .opt("vrf", self.vrf)
            .opt("tenant", self.tenant)
            .opt("dns_name", self.dns_name)
            .opt("description", self.description)
            .opt("comments", self.comments)
            .opt("nat_inside", self.nat_inside)
            .opt("assigned_object_type", self.assigned_object_type)
            .opt("assigned_object_id", self.assigned_object_id)
            .opt("tags", self.tags)
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct IpAddressCreateParams {
    #[schemars(description = "IP address in CIDR notation, e.g. 192.0.2.10/24 (required)")]
    pub address: String,
    #[serde(flatten)]
    pub fields: IpAddressFields,
}

pub async fn ip_address_create(
    client: &NetboxClient,
    p: IpAddressCreateParams,
) -> Result<Value, NetboxError> {
    let body = p
        .fields
        .apply(BodyBuilder::new().req("address", p.address))
        .build();
    client.post(IP_ADDRESSES_PATH, &body).await
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct IpAddressUpdateParams {
    #[schemars(description = "NetBox ID of the IP address to update (required)")]
    pub id: i32,
    #[schemars(description = "New IP address in CIDR notation, e.g. 192.0.2.10/24")]
    pub address: Option<String>,
    #[serde(flatten)]
    pub fields: IpAddressFields,
}

pub async fn ip_address_update(
    client: &NetboxClient,
    p: IpAddressUpdateParams,
) -> Result<Value, NetboxError> {
    let body = p
        .fields
        .apply(BodyBuilder::new().opt("address", p.address))
        .build();
    client.patch(IP_ADDRESSES_PATH, p.id, &body).await
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct IpAddressDeleteParams {
    #[schemars(description = "NetBox ID of the IP address to delete (required)")]
    pub id: i32,
}

pub async fn ip_address_delete(
    client: &NetboxClient,
    p: IpAddressDeleteParams,
) -> Result<(), NetboxError> {
    client.delete(IP_ADDRESSES_PATH, p.id).await
}

// --------------------------------------------------------------------------
// Prefixes
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PrefixesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by prefix (e.g. 192.0.2.0/24)")]
    pub prefix: Option<Vec<String>>,
    #[schemars(
        description = "Filter by VRF route distinguisher / rd (e.g. 65000:100) — preferred over vrf_id"
    )]
    pub vrf: Option<Vec<String>>,
    #[schemars(description = "Filter by VRF ID")]
    pub vrf_id: Option<i32>,
    #[schemars(description = "Filter by status (active, container, reserved, deprecated)")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by role slug")]
    pub role: Option<Vec<String>>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by family (4 or 6)")]
    pub family: Option<i32>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn prefixes_list(
    client: &NetboxClient,
    p: PrefixesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("prefix", p.prefix)
        .many("vrf", p.vrf)
        .opt("vrf_id", p.vrf_id)
        .many("status", p.status)
        .many("role", p.role)
        .many("site", p.site)
        .many("tenant", p.tenant)
        .opt("family", p.family)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/prefixes/", p.pagination).await
}

// --------------------------------------------------------------------------
// VRFs
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VrfsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by VRF name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by route distinguisher")]
    pub rd: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn vrfs_list(client: &NetboxClient, p: VrfsListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("rd", p.rd)
        .many("tenant", p.tenant)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/vrfs/", p.pagination).await
}

// --------------------------------------------------------------------------
// VLANs
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VlansListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by VLAN ID (802.1Q VID)")]
    pub vid: Option<i32>,
    #[schemars(description = "Filter by VLAN name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by status (active, reserved, deprecated)")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by role slug")]
    pub role: Option<Vec<String>>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by VLAN group slug")]
    pub group: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn vlans_list(client: &NetboxClient, p: VlansListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .opt("vid", p.vid)
        .many("name", p.name)
        .many("status", p.status)
        .many("role", p.role)
        .many("site", p.site)
        .many("group", p.group)
        .many("tenant", p.tenant)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/vlans/", p.pagination).await
}

// --------------------------------------------------------------------------
// Aggregates
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AggregatesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by prefix")]
    pub prefix: Option<Vec<String>>,
    #[schemars(description = "Filter by RIR slug")]
    pub rir: Option<Vec<String>>,
    #[schemars(description = "Filter by family (4 or 6)")]
    pub family: Option<i32>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn aggregates_list(
    client: &NetboxClient,
    p: AggregatesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("prefix", p.prefix)
        .many("rir", p.rir)
        .opt("family", p.family)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/aggregates/", p.pagination).await
}

// --------------------------------------------------------------------------
// IP Ranges
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct IpRangesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by status (active, reserved, deprecated)")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by role slug")]
    pub role: Option<Vec<String>>,
    #[schemars(
        description = "Filter by VRF route distinguisher / rd (e.g. 65000:100) — preferred over vrf_id"
    )]
    pub vrf: Option<Vec<String>>,
    #[schemars(description = "Filter by VRF ID")]
    pub vrf_id: Option<i32>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn ip_ranges_list(
    client: &NetboxClient,
    p: IpRangesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("status", p.status)
        .many("role", p.role)
        .many("vrf", p.vrf)
        .opt("vrf_id", p.vrf_id)
        .many("tenant", p.tenant)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/ip-ranges/", p.pagination).await
}

// --------------------------------------------------------------------------
// Route Targets
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RouteTargetsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by route target name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn route_targets_list(
    client: &NetboxClient,
    p: RouteTargetsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("tenant", p.tenant)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/route-targets/", p.pagination)
        .await
}

// --------------------------------------------------------------------------
// RIRs
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RirsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by RIR name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn rirs_list(client: &NetboxClient, p: RirsListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("slug", p.slug)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/rirs/", p.pagination).await
}

// --------------------------------------------------------------------------
// VLAN Groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VlanGroupsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by VLAN group name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by site ID")]
    pub site_id: Option<i32>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn vlan_groups_list(
    client: &NetboxClient,
    p: VlanGroupsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .opt("site_id", p.site_id)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/vlan-groups/", p.pagination).await
}

// --------------------------------------------------------------------------
// Services
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ServicesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by service name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by device ID")]
    pub device_id: Option<i32>,
    #[schemars(description = "Filter by virtual machine ID")]
    pub virtual_machine_id: Option<i32>,
    #[schemars(description = "Filter by protocol (tcp, udp, sctp)")]
    pub protocol: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn services_list(
    client: &NetboxClient,
    p: ServicesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .opt("device_id", p.device_id)
        .opt("virtual_machine_id", p.virtual_machine_id)
        .many("protocol", p.protocol)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/services/", p.pagination).await
}

// --------------------------------------------------------------------------
// ASNs
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AsnsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by AS number")]
    pub asn: Option<i64>,
    #[schemars(description = "Filter by RIR slug")]
    pub rir: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn asns_list(client: &NetboxClient, p: AsnsListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .opt("asn", p.asn)
        .many("rir", p.rir)
        .many("tenant", p.tenant)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/asns/", p.pagination).await
}

// --------------------------------------------------------------------------
// FHRP Groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FhrpGroupsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(
        description = "Filter by protocol (vrrp2, vrrp3, carp, clusterxl, hsrp, glbp, other)"
    )]
    pub protocol: Option<Vec<String>>,
    #[schemars(description = "Filter by group ID number")]
    pub group_id: Option<i32>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn fhrp_groups_list(
    client: &NetboxClient,
    p: FhrpGroupsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("protocol", p.protocol)
        .opt("group_id", p.group_id)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/fhrp-groups/", p.pagination).await
}

// --------------------------------------------------------------------------
// FHRP Group Assignments
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FhrpGroupAssignmentsListParams {
    #[schemars(description = "Filter by FHRP group ID")]
    pub group_id: Option<i32>,
    #[schemars(description = "Filter by interface type (e.g. dcim.interface)")]
    pub interface_type: Option<String>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn fhrp_group_assignments_list(
    client: &NetboxClient,
    p: FhrpGroupAssignmentsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("group_id", p.group_id)
        .opt("interface_type", p.interface_type)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/fhrp-group-assignments/", p.pagination)
        .await
}

// --------------------------------------------------------------------------
// Roles (IP/VLAN roles)
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RolesListParams {
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

pub async fn roles_list(client: &NetboxClient, p: RolesListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("slug", p.slug)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/ipam/roles/", p.pagination).await
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
        IpAddressCreateParams, IpAddressDeleteParams, IpAddressFields, IpAddressUpdateParams,
        ip_address_create, ip_address_delete, ip_address_update,
    };
    use crate::client::NetboxClient;

    fn mock_client(server: &MockServer) -> NetboxClient {
        NetboxClient::new(server.uri(), "test-token").unwrap()
    }

    #[tokio::test]
    async fn ip_address_create_sends_address_and_only_set_fields() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/ipam/ip-addresses/"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "id": 1 })))
            .mount(&server)
            .await;

        ip_address_create(
            &mock_client(&server),
            IpAddressCreateParams {
                address: "192.0.2.10/24".into(),
                fields: IpAddressFields {
                    status: Some("reserved".into()),
                    dns_name: Some("host.example.com".into()),
                    assigned_object_type: Some("dcim.interface".into()),
                    assigned_object_id: Some(12),
                    ..Default::default()
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
        assert_eq!(body["address"], "192.0.2.10/24");
        assert_eq!(body["status"], "reserved");
        assert_eq!(body["dns_name"], "host.example.com");
        assert_eq!(body["assigned_object_type"], "dcim.interface");
        assert_eq!(body["assigned_object_id"], 12);
        // Unset optionals must be omitted, not sent as null.
        assert!(body.get("vrf").is_none());
        assert!(body.get("role").is_none());
        assert!(body.get("tenant").is_none());
    }

    #[tokio::test]
    async fn ip_address_update_patches_id_with_partial_body() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/ipam/ip-addresses/5/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "id": 5 })))
            .mount(&server)
            .await;

        ip_address_update(
            &mock_client(&server),
            IpAddressUpdateParams {
                id: 5,
                address: None,
                fields: IpAddressFields {
                    status: Some("deprecated".into()),
                    ..Default::default()
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
        assert_eq!(body["status"], "deprecated");
        assert_eq!(body.as_object().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn ip_address_delete_hits_id_path() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/ipam/ip-addresses/5/"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        ip_address_delete(&mock_client(&server), IpAddressDeleteParams { id: 5 })
            .await
            .unwrap();
    }
}

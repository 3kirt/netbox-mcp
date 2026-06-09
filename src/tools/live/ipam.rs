//! Live IPAM coverage: IP addresses (device/parent/vrf filters), prefixes
//! (exact + site filters), VRFs, services (protocol filter), and get-by-id.

use serde_json::json;

use super::harness::{
    assert_clean, assert_nonempty, assert_page_shape, first_id, params, results, skip_unless_live,
    slim, slim_get,
};
use crate::client::NetboxError;
use crate::tools::ipam::{
    self, AggregatesListParams, AsnsListParams, IpAddressesListParams, PrefixesListParams,
    RirsListParams, ServicesListParams, VlanGroupsListParams, VlansListParams, VrfsListParams,
};

const IP_ADDRESSES_PATH: &str = "/api/ipam/ip-addresses/";

/// An address unique to this run, in TEST-NET-3 (192.0.2.0/24, RFC 5737), so a
/// leaked address from a prior run never clashes and the value is obviously a
/// throwaway. The host octet is derived from the current nanosecond.
fn unique_test_address() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before epoch")
        .as_nanos();
    // 1..=254 keeps it a valid, non-zero host in the /24.
    let host = (nanos % 254) + 1;
    format!("192.0.2.{host}/24")
}

#[tokio::test]
async fn ip_addresses_filter_by_device_returns_rows() {
    let env = skip_unless_live!();
    let p: IpAddressesListParams = params(json!({ "device": ["nyc-server-01"] }));
    let resp = slim(ipam::ip_addresses_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "ip-addresses?device=nyc-server-01");
    assert_nonempty(&resp, "ip-addresses?device=nyc-server-01");
}

#[tokio::test]
async fn ip_addresses_parent_filter_is_containment_scoped() {
    let env = skip_unless_live!();
    let p: IpAddressesListParams = params(json!({ "parent": "10.0.1.0/24" }));
    let resp = slim(ipam::ip_addresses_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "ip-addresses?parent=10.0.1.0/24");
    assert_nonempty(&resp, "ip-addresses?parent=10.0.1.0/24");
    for ip in results(&resp) {
        let addr = ip["address"].as_str().expect("ip has no address");
        assert!(
            addr.starts_with("10.0.1."),
            "address {addr} is outside the parent prefix 10.0.1.0/24"
        );
    }
}

#[tokio::test]
async fn ip_addresses_filter_by_vrf_rd_scopes_to_that_vrf() {
    let env = skip_unless_live!();
    // 65000:100 is the Management VRF's route distinguisher in the seed.
    let p: IpAddressesListParams = params(json!({ "vrf": ["65000:100"] }));
    let resp = slim(ipam::ip_addresses_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "ip-addresses?vrf=65000:100");
    assert_nonempty(&resp, "ip-addresses?vrf=65000:100");
    for ip in results(&resp) {
        assert_eq!(
            ip["vrf"]["name"], "Management",
            "IP not scoped to the Management VRF: {ip}"
        );
    }
}

#[tokio::test]
async fn ip_address_get_by_id_is_clean() {
    let env = skip_unless_live!();
    let list = slim(
        ipam::ip_addresses_list(&env.client, params(json!({ "device": ["nyc-server-01"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&list, "ip-addresses?device=nyc-server-01");
    let id = first_id(&list);

    let ip = slim_get(&env.client, "/api/ipam/ip-addresses/", id).await;
    assert_clean(&ip, "ip-address.get");
    assert_eq!(ip["id"], id);
}

#[tokio::test]
async fn prefixes_filter_by_exact_prefix() {
    let env = skip_unless_live!();
    let p: PrefixesListParams = params(json!({ "prefix": ["10.1.0.0/24"] }));
    let resp = slim(ipam::prefixes_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "prefixes?prefix=10.1.0.0/24");
    assert_nonempty(&resp, "prefixes?prefix=10.1.0.0/24");
    for pfx in results(&resp) {
        assert_eq!(pfx["prefix"], "10.1.0.0/24");
    }
}

#[tokio::test]
async fn prefixes_filter_by_site_scopes_to_that_site() {
    let env = skip_unless_live!();
    let p: PrefixesListParams = params(json!({ "site": ["lon-dc"] }));
    let resp = slim(ipam::prefixes_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "prefixes?site=lon-dc");
    assert_nonempty(&resp, "prefixes?site=lon-dc");
    for pfx in results(&resp) {
        // NetBox 4.x exposes a prefix's site via the generic `scope` field.
        assert_eq!(
            pfx["scope_type"], "dcim.site",
            "prefix scope is not a site: {pfx}"
        );
        assert_eq!(
            pfx["scope"]["slug"], "lon-dc",
            "prefix not in filtered site: {pfx}"
        );
    }
}

#[tokio::test]
async fn vrfs_filter_by_name_is_exact() {
    let env = skip_unless_live!();
    let p: VrfsListParams = params(json!({ "name": ["Global"] }));
    let resp = slim(ipam::vrfs_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "vrfs?name=Global");
    assert_nonempty(&resp, "vrfs?name=Global");
    for vrf in results(&resp) {
        assert_eq!(vrf["name"], "Global");
    }
}

#[tokio::test]
async fn services_filter_by_protocol_scopes_to_that_protocol() {
    let env = skip_unless_live!();
    let p: ServicesListParams = params(json!({ "protocol": ["tcp"] }));
    let resp = slim(ipam::services_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "services?protocol=tcp");
    assert_nonempty(&resp, "services?protocol=tcp");
    for svc in results(&resp) {
        // Choice field: slim drops `label`, leaving the bare `value`.
        assert_eq!(
            svc["protocol"]["value"], "tcp",
            "service has wrong protocol: {svc}"
        );
    }
}

#[tokio::test]
async fn aggregates_filter_by_rir_scopes_to_that_rir() {
    let env = skip_unless_live!();
    let p: AggregatesListParams = params(json!({ "rir": ["rfc1918"] }));
    let resp = slim(ipam::aggregates_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "aggregates?rir=rfc1918");
    assert_nonempty(&resp, "aggregates?rir=rfc1918");
    for agg in results(&resp) {
        assert_eq!(
            agg["rir"]["slug"], "rfc1918",
            "aggregate not under filtered RIR: {agg}"
        );
    }
}

#[tokio::test]
async fn asns_filter_by_number_is_exact() {
    let env = skip_unless_live!();
    let p: AsnsListParams = params(json!({ "asn": 65001 }));
    let resp = slim(ipam::asns_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "asns?asn=65001");
    assert_nonempty(&resp, "asns?asn=65001");
    for a in results(&resp) {
        assert_eq!(a["asn"], 65001);
    }
}

#[tokio::test]
async fn rirs_filter_by_slug_is_exact() {
    let env = skip_unless_live!();
    let p: RirsListParams = params(json!({ "slug": ["arin"] }));
    let resp = slim(ipam::rirs_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "rirs?slug=arin");
    assert_nonempty(&resp, "rirs?slug=arin");
    for r in results(&resp) {
        assert_eq!(r["slug"], "arin");
    }
}

#[tokio::test]
async fn vlans_filter_by_vid_is_exact() {
    let env = skip_unless_live!();
    let p: VlansListParams = params(json!({ "vid": 10 }));
    let resp = slim(ipam::vlans_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "vlans?vid=10");
    assert_nonempty(&resp, "vlans?vid=10");
    for v in results(&resp) {
        assert_eq!(v["vid"], 10, "VLAN has wrong vid: {v}");
    }
}

#[tokio::test]
async fn vlan_groups_filter_by_name_is_exact() {
    let env = skip_unless_live!();
    let p: VlanGroupsListParams = params(json!({ "name": ["NYC VLANs"] }));
    let resp = slim(ipam::vlan_groups_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "vlan-groups?name=NYC VLANs");
    assert_nonempty(&resp, "vlan-groups?name=NYC VLANs");
    for g in results(&resp) {
        assert_eq!(g["name"], "NYC VLANs");
    }
}

/// Exercises the IP-address write path end-to-end against real NetBox: create an
/// address, change a field via PATCH, confirm it persisted through the get path,
/// then delete and confirm the follow-up get 404s. Self-cleaning — the address it
/// creates is the address it deletes — so the seeded instance is left untouched.
/// Requires a write-enabled token (a read-only token makes the create 403).
#[tokio::test]
async fn ip_address_create_update_delete_lifecycle() {
    let env = skip_unless_live!();
    let address = unique_test_address();

    // Create — response is slimmed exactly as the rmcp boundary would slim it.
    let created = slim(
        ipam::ip_address_create(
            &env.client,
            params(json!({ "address": address, "status": "reserved" })),
        )
        .await
        .expect("ip_address_create"),
    );
    assert_clean(&created, "ip.create");
    assert_eq!(created["address"], address);
    assert_eq!(created["status"]["value"], "reserved");
    let id = created["id"].as_i64().expect("created IP has an id") as i32;

    // Partial update: flip status, leave the address untouched.
    let updated = slim(
        ipam::ip_address_update(
            &env.client,
            params(json!({ "id": id, "status": "deprecated" })),
        )
        .await
        .expect("ip_address_update"),
    );
    assert_clean(&updated, "ip.update");
    assert_eq!(updated["id"], id);
    assert_eq!(updated["status"]["value"], "deprecated");
    assert_eq!(
        updated["address"], address,
        "partial update must not change the address"
    );

    // Read back through the get path: the change persisted server-side.
    let got = slim_get(&env.client, IP_ADDRESSES_PATH, id).await;
    assert_eq!(got["status"]["value"], "deprecated");

    // Delete, then a follow-up get must surface a 404.
    ipam::ip_address_delete(&env.client, params(json!({ "id": id })))
        .await
        .expect("ip_address_delete");

    let err = env
        .client
        .get(IP_ADDRESSES_PATH, id)
        .await
        .expect_err("get after delete must 404");
    assert!(
        matches!(err, NetboxError::Api { status, .. } if status.as_u16() == 404),
        "expected 404 after delete, got {err:?}"
    );
}

#[tokio::test]
async fn vrf_and_aggregate_get_by_id_are_clean() {
    let env = skip_unless_live!();

    let vrf_list = slim(
        ipam::vrfs_list(&env.client, params(json!({ "name": ["Global"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&vrf_list, "vrfs?name=Global");
    let vrf = slim_get(&env.client, "/api/ipam/vrfs/", first_id(&vrf_list)).await;
    assert_clean(&vrf, "vrf.get");

    let agg_list = slim(
        ipam::aggregates_list(&env.client, params(json!({ "rir": ["rfc1918"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&agg_list, "aggregates?rir=rfc1918");
    let agg = slim_get(&env.client, "/api/ipam/aggregates/", first_id(&agg_list)).await;
    assert_clean(&agg, "aggregate.get");
}

//! Live virtualization coverage. The headline fidelity check lives here: VMs
//! carry `local_context_data` in NetBox, so a live VM response is the only way
//! to prove `slim_value` actually strips it — a wiremock test only proves we
//! strip a field *we* put in the mock.

use serde_json::json;

use super::harness::{
    assert_clean, assert_nonempty, assert_page_shape, first_id, params, results, skip_unless_live,
    slim, slim_get,
};
use crate::tools::virtualization::{
    self, ClusterTypesListParams, ClustersListParams, InterfacesListParams, VmsListParams,
};

#[tokio::test]
async fn vms_list_strips_local_context_data() {
    let env = skip_unless_live!();
    // fetch_all so every seeded VM is inspected, not just the first page.
    let p: VmsListParams = params(json!({ "fetch_all": true }));
    let resp = slim(virtualization::vms_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "vms (fetch_all)");
    assert_nonempty(&resp, "vms (fetch_all)");
    // assert_clean (inside assert_page_shape) already forbids local_context_data
    // anywhere; this explicit per-VM check documents the critical invariant.
    for vm in results(&resp) {
        assert!(
            vm.get("local_context_data").is_none(),
            "VM leaked local_context_data: {vm}"
        );
    }
}

#[tokio::test]
async fn vms_filter_by_cluster_returns_only_that_cluster() {
    let env = skip_unless_live!();
    let p: VmsListParams = params(json!({ "cluster": ["LON-PROD"] }));
    let resp = slim(virtualization::vms_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "vms?cluster=LON-PROD");
    assert_nonempty(&resp, "vms?cluster=LON-PROD");
    for vm in results(&resp) {
        assert_eq!(
            vm["cluster"]["name"], "LON-PROD",
            "VM not in filtered cluster: {vm}"
        );
    }
}

#[tokio::test]
async fn vms_filter_by_site_returns_rows() {
    let env = skip_unless_live!();
    let p: VmsListParams = params(json!({ "site": ["nyc-dc"] }));
    let resp = slim(virtualization::vms_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "vms?site=nyc-dc");
    assert_nonempty(&resp, "vms?site=nyc-dc");
}

#[tokio::test]
async fn vm_get_by_id_strips_local_context_and_primary_ip_alias() {
    let env = skip_unless_live!();
    let list = slim(
        virtualization::vms_list(&env.client, params(json!({ "cluster": ["LON-PROD"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&list, "vms?cluster=LON-PROD");
    let id = first_id(&list);

    let vm = slim_get(&env.client, "/api/virtualization/virtual-machines/", id).await;
    assert_clean(&vm, "vm.get");
    assert_eq!(vm["id"], id);
    assert!(
        vm.get("local_context_data").is_none(),
        "vm.get leaked local_context_data"
    );
    assert!(
        vm.get("primary_ip").is_none(),
        "vm.get leaked the primary_ip alias"
    );
}

#[tokio::test]
async fn clusters_filter_by_site_returns_only_that_site() {
    let env = skip_unless_live!();
    let p: ClustersListParams = params(json!({ "site": ["lon-dc"] }));
    let resp = slim(virtualization::clusters_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "clusters?site=lon-dc");
    assert_nonempty(&resp, "clusters?site=lon-dc");
    for c in results(&resp) {
        // NetBox 4.x exposes a cluster's site via the generic `scope` field
        // (scope_type == dcim.site), not a `site` key.
        assert_eq!(
            c["scope_type"], "dcim.site",
            "cluster scope is not a site: {c}"
        );
        assert_eq!(
            c["scope"]["slug"], "lon-dc",
            "cluster not in filtered site: {c}"
        );
    }
}

#[tokio::test]
async fn cluster_types_filter_by_slug_is_exact() {
    let env = skip_unless_live!();
    let p: ClusterTypesListParams = params(json!({ "slug": ["vsphere"] }));
    let resp = slim(
        virtualization::cluster_types_list(&env.client, p)
            .await
            .unwrap(),
    );

    assert_page_shape(&resp, "cluster-types?slug=vsphere");
    assert_nonempty(&resp, "cluster-types?slug=vsphere");
    for t in results(&resp) {
        assert_eq!(t["slug"], "vsphere");
    }
}

#[tokio::test]
async fn vm_interfaces_filter_by_vm_returns_rows() {
    let env = skip_unless_live!();
    let p: InterfacesListParams = params(json!({ "virtual_machine": "web-prod-01" }));
    let resp = slim(
        virtualization::interfaces_list(&env.client, p)
            .await
            .unwrap(),
    );

    assert_page_shape(&resp, "vm-interfaces?virtual_machine=web-prod-01");
    assert_nonempty(&resp, "vm-interfaces?virtual_machine=web-prod-01");
}

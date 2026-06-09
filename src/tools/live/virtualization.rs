//! Live virtualization coverage. The headline fidelity check lives here: VMs
//! carry `local_context_data` in NetBox, so a live VM response is the only way
//! to prove `slim_value` actually strips it — a wiremock test only proves we
//! strip a field *we* put in the mock.

use serde_json::json;

use super::harness::{
    assert_clean, assert_nonempty, assert_page_shape, first_id, params, results, skip_unless_live,
    slim, slim_get,
};
use crate::client::NetboxError;
use crate::tools::virtualization::{
    self, ClusterTypesListParams, ClustersListParams, InterfacesListParams, VmsListParams,
};

const VMS_PATH: &str = "/api/virtualization/virtual-machines/";

/// A name unique to this run, so a leaked object from a prior run never clashes
/// and parallel test binaries don't collide.
fn unique_vm_name() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before epoch")
        .as_nanos();
    format!("mcp-live-vm-{nanos}")
}

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

/// Exercises the write path end-to-end against real NetBox: create a VM, flip a
/// field via PATCH, confirm it persisted through the get path, then delete and
/// confirm the follow-up get 404s. Self-cleaning — the VM it creates is the VM
/// it deletes — so it leaves the seeded instance as it found it. Requires a
/// write-enabled token; a read-only token makes `vm_create` fail with 403.
#[tokio::test]
async fn vm_create_update_delete_lifecycle() {
    let env = skip_unless_live!();

    // A VM needs a cluster (or site/device) — borrow one the seed created.
    let clusters = slim(
        virtualization::clusters_list(&env.client, params(json!({})))
            .await
            .unwrap(),
    );
    assert_nonempty(&clusters, "clusters");
    let cluster_id = first_id(&clusters);
    let name = unique_vm_name();

    // Create — response is slimmed exactly as the rmcp boundary would slim it.
    let created = slim(
        virtualization::vm_create(
            &env.client,
            params(json!({ "name": name, "cluster": cluster_id, "status": "active" })),
        )
        .await
        .expect("vm_create"),
    );
    assert_clean(&created, "vm.create");
    assert_eq!(created["name"], name);
    assert_eq!(created["status"]["value"], "active");
    let id = created["id"].as_i64().expect("created VM has an id") as i32;

    // Partial update: flip status, leave everything else untouched.
    let updated = slim(
        virtualization::vm_update(
            &env.client,
            params(json!({ "id": id, "status": "offline" })),
        )
        .await
        .expect("vm_update"),
    );
    assert_clean(&updated, "vm.update");
    assert_eq!(updated["id"], id);
    assert_eq!(updated["status"]["value"], "offline");
    assert_eq!(updated["name"], name, "partial update must not change name");

    // Read back through the get path: the change persisted server-side.
    let got = slim_get(&env.client, VMS_PATH, id).await;
    assert_eq!(got["status"]["value"], "offline");

    // Delete, then a follow-up get must surface a 404.
    virtualization::vm_delete(&env.client, params(json!({ "id": id })))
        .await
        .expect("vm_delete");

    let err = env
        .client
        .get(VMS_PATH, id)
        .await
        .expect_err("get after delete must 404");
    assert!(
        matches!(err, NetboxError::Api { status, .. } if status.as_u16() == 404),
        "expected 404 after delete, got {err:?}"
    );
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

//! Live core coverage: the object-change log. Seeding generates a large
//! changelog automatically, so this needs no extra seed data. Also exercises
//! the `diff_only` transform against real change records.

use serde_json::json;

use super::harness::{
    assert_clean, assert_nonempty, assert_page_shape, first_id, params, results, skip_unless_live,
    slim, slim_get,
};
use crate::tools::core::{self, ObjectChangesListParams};

#[tokio::test]
async fn object_changes_list_is_clean() {
    let env = skip_unless_live!();
    let p: ObjectChangesListParams = params(json!({}));
    let resp = slim(core::object_changes_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "object-changes");
    assert_nonempty(&resp, "object-changes");
}

#[tokio::test]
async fn object_changes_filter_by_type_scopes_to_that_type() {
    let env = skip_unless_live!();
    let p: ObjectChangesListParams = params(json!({ "changed_object_type": "dcim.device" }));
    let resp = slim(core::object_changes_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "object-changes?changed_object_type=dcim.device");
    assert_nonempty(&resp, "object-changes?changed_object_type=dcim.device");
    for c in results(&resp) {
        assert_eq!(
            c["changed_object_type"], "dcim.device",
            "change not scoped to the filtered type: {c}"
        );
    }
}

#[tokio::test]
async fn object_change_get_by_id_is_clean() {
    let env = skip_unless_live!();
    let list = slim(
        core::object_changes_list(&env.client, params(json!({})))
            .await
            .unwrap(),
    );
    assert_nonempty(&list, "object-changes");
    let id = first_id(&list);

    let change = slim_get(&env.client, "/api/core/object-changes/", id).await;
    assert_clean(&change, "object-change.get");
    assert_eq!(change["id"], id);
}

#[tokio::test]
async fn object_changes_diff_only_keeps_matching_keysets() {
    let env = skip_unless_live!();
    // diff_only rewrites update records so prechange_data/postchange_data hold
    // only the keys that differ — i.e. both sides end with identical key sets.
    let p: ObjectChangesListParams = params(json!({ "diff_only": true }));
    let resp = slim(core::object_changes_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "object-changes?diff_only=true");
    assert_nonempty(&resp, "object-changes?diff_only=true");
    for c in results(&resp) {
        // Genuine update records carry both snapshots as NON-EMPTY objects;
        // the diff then keeps only the differing keys, leaving identical key
        // sets on both sides. Create/delete records have one side as `{}` in
        // NetBox 4.x (not null) and are intentionally left asymmetric, so skip
        // any record where either snapshot is empty.
        if let (Some(pre), Some(post)) = (
            c.get("prechange_data").and_then(|v| v.as_object()),
            c.get("postchange_data").and_then(|v| v.as_object()),
        ) {
            if pre.is_empty() || post.is_empty() {
                continue;
            }
            let pre_keys: Vec<&String> = pre.keys().collect();
            let post_keys: Vec<&String> = post.keys().collect();
            assert_eq!(
                pre_keys, post_keys,
                "diff_only should leave identical key sets on both snapshots: {c}"
            );
        }
    }
}

//! Live extras coverage: tags (the only extras resource the seed populates).
//! Note the top-level tags-list endpoint returns full tag objects via `results`
//! — it is unaffected by the embedded-tags collapsing that `slim_value` applies
//! to `tags` arrays inside other objects.

use serde_json::json;

use super::harness::{
    assert_clean, assert_nonempty, assert_page_shape, first_id, params, results, skip_unless_live,
    slim, slim_get,
};
use crate::client::NetboxError;
use crate::tools::extras::{self, TagsListParams};

const TAGS_PATH: &str = "/api/extras/tags/";

/// A slug unique to this run, so a leaked tag from a prior run never clashes on
/// NetBox's unique-slug constraint.
fn unique_tag_slug() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before epoch")
        .as_nanos();
    format!("mcp-live-tag-{nanos}")
}

#[tokio::test]
async fn tags_filter_by_name_is_exact() {
    let env = skip_unless_live!();
    let p: TagsListParams = params(json!({ "name": ["production"] }));
    let resp = slim(extras::tags_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "tags?name=production");
    assert_nonempty(&resp, "tags?name=production");
    for t in results(&resp) {
        assert_eq!(t["name"], "production");
        assert_eq!(t["slug"], "production");
    }
}

/// Exercises the tag write path end-to-end against real NetBox: create a tag,
/// change a field via PATCH, confirm it persisted through the get path, then
/// delete and confirm the follow-up get 404s. Self-cleaning — the tag it creates
/// is the tag it deletes — so the seeded instance is left untouched. Requires a
/// write-enabled token (a read-only token makes the create 403).
#[tokio::test]
async fn tag_create_update_delete_lifecycle() {
    let env = skip_unless_live!();
    let slug = unique_tag_slug();

    // Create — response is slimmed exactly as the rmcp boundary would slim it.
    let created = slim(
        extras::tag_create(
            &env.client,
            params(json!({ "name": slug, "slug": slug, "color": "9e9e9e" })),
        )
        .await
        .expect("tag_create"),
    );
    assert_clean(&created, "tag.create");
    assert_eq!(created["slug"], slug);
    assert_eq!(created["color"], "9e9e9e");
    let id = created["id"].as_i64().expect("created tag has an id") as i32;

    // Partial update: change the color, leave name/slug untouched.
    let updated = slim(
        extras::tag_update(&env.client, params(json!({ "id": id, "color": "ff0000" })))
            .await
            .expect("tag_update"),
    );
    assert_clean(&updated, "tag.update");
    assert_eq!(updated["id"], id);
    assert_eq!(updated["color"], "ff0000");
    assert_eq!(updated["slug"], slug, "partial update must not change slug");

    // Read back through the get path: the change persisted server-side.
    let got = slim_get(&env.client, TAGS_PATH, id).await;
    assert_eq!(got["color"], "ff0000");

    // Delete, then a follow-up get must surface a 404.
    extras::tag_delete(&env.client, params(json!({ "id": id })))
        .await
        .expect("tag_delete");

    let err = env
        .client
        .get(TAGS_PATH, id)
        .await
        .expect_err("get after delete must 404");
    assert!(
        matches!(err, NetboxError::Api { status, .. } if status.as_u16() == 404),
        "expected 404 after delete, got {err:?}"
    );
}

#[tokio::test]
async fn tag_get_by_id_is_clean() {
    let env = skip_unless_live!();
    let list = slim(
        extras::tags_list(&env.client, params(json!({ "name": ["production"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&list, "tags?name=production");
    let tag = slim_get(&env.client, "/api/extras/tags/", first_id(&list)).await;
    assert_clean(&tag, "tag.get");
}

//! Live extras coverage: tags (the only extras resource the seed populates).
//! Note the top-level tags-list endpoint returns full tag objects via `results`
//! — it is unaffected by the embedded-tags collapsing that `slim_value` applies
//! to `tags` arrays inside other objects.

use serde_json::json;

use super::harness::{
    assert_clean, assert_nonempty, assert_page_shape, first_id, params, results, skip_unless_live,
    slim, slim_get,
};
use crate::tools::extras::{self, TagsListParams};

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

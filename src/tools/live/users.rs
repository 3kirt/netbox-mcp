//! Live users coverage: users and tokens. The `admin` superuser and the v1 API
//! token minted by the test stack's `netbox-token` service exist without extra
//! seeding. (Groups are empty until seeded, so they're not covered here.)

use serde_json::json;

use super::harness::{
    assert_clean, assert_nonempty, assert_page_shape, first_id, params, results, skip_unless_live,
    slim, slim_get,
};
use crate::tools::users::{self, TokensListParams, UsersListParams};

#[tokio::test]
async fn users_filter_by_username_is_exact() {
    let env = skip_unless_live!();
    let p: UsersListParams = params(json!({ "username": ["admin"] }));
    let resp = slim(users::users_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "users?username=admin");
    assert_nonempty(&resp, "users?username=admin");
    for u in results(&resp) {
        assert_eq!(u["username"], "admin");
    }
}

#[tokio::test]
async fn user_get_by_id_is_clean() {
    let env = skip_unless_live!();
    let list = slim(
        users::users_list(&env.client, params(json!({ "username": ["admin"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&list, "users?username=admin");
    let id = first_id(&list);

    let user = slim_get(&env.client, "/api/users/users/", id).await;
    assert_clean(&user, "user.get");
    assert_eq!(user["id"], id);
}

#[tokio::test]
async fn tokens_list_is_clean() {
    let env = skip_unless_live!();
    let p: TokensListParams = params(json!({}));
    let resp = slim(users::tokens_list(&env.client, p).await.unwrap());

    // At least the admin token used for this very request exists.
    assert_page_shape(&resp, "tokens");
    assert_nonempty(&resp, "tokens");
}

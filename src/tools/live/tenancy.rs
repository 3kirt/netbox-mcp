//! Live tenancy coverage: tenants, contacts, contact roles (list filters +
//! get-by-id). All seeded by scripts/seed_data.py.

use serde_json::json;

use super::harness::{
    assert_clean, assert_nonempty, assert_page_shape, first_id, params, results, skip_unless_live,
    slim, slim_get,
};
use crate::tools::tenancy::{self, ContactRolesListParams, ContactsListParams, TenantsListParams};

#[tokio::test]
async fn tenants_filter_by_name_is_exact() {
    let env = skip_unless_live!();
    let p: TenantsListParams = params(json!({ "name": ["Infrastructure"] }));
    let resp = slim(tenancy::tenants_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "tenants?name=Infrastructure");
    assert_nonempty(&resp, "tenants?name=Infrastructure");
    for t in results(&resp) {
        assert_eq!(t["name"], "Infrastructure");
    }
}

#[tokio::test]
async fn tenants_list_is_clean() {
    let env = skip_unless_live!();
    let p: TenantsListParams = params(json!({}));
    let resp = slim(tenancy::tenants_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "tenants");
    assert_nonempty(&resp, "tenants");
}

#[tokio::test]
async fn contacts_filter_by_name_is_exact() {
    let env = skip_unless_live!();
    let p: ContactsListParams = params(json!({ "name": ["Alice Smith"] }));
    let resp = slim(tenancy::contacts_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "contacts?name=Alice Smith");
    assert_nonempty(&resp, "contacts?name=Alice Smith");
    for c in results(&resp) {
        assert_eq!(c["name"], "Alice Smith");
    }
}

#[tokio::test]
async fn contact_roles_filter_by_slug_is_exact() {
    let env = skip_unless_live!();
    let p: ContactRolesListParams = params(json!({ "slug": ["noc"] }));
    let resp = slim(tenancy::contact_roles_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "contact-roles?slug=noc");
    assert_nonempty(&resp, "contact-roles?slug=noc");
    for r in results(&resp) {
        assert_eq!(r["slug"], "noc");
    }
}

#[tokio::test]
async fn tenant_contact_role_get_by_id_are_clean() {
    let env = skip_unless_live!();

    let tenant_list = slim(
        tenancy::tenants_list(&env.client, params(json!({ "name": ["Infrastructure"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&tenant_list, "tenants?name=Infrastructure");
    let tenant = slim_get(&env.client, "/api/tenancy/tenants/", first_id(&tenant_list)).await;
    assert_clean(&tenant, "tenant.get");

    let contact_list = slim(
        tenancy::contacts_list(&env.client, params(json!({ "name": ["Alice Smith"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&contact_list, "contacts?name=Alice Smith");
    let contact = slim_get(
        &env.client,
        "/api/tenancy/contacts/",
        first_id(&contact_list),
    )
    .await;
    assert_clean(&contact, "contact.get");

    let role_list = slim(
        tenancy::contact_roles_list(&env.client, params(json!({ "slug": ["noc"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&role_list, "contact-roles?slug=noc");
    let role = slim_get(
        &env.client,
        "/api/tenancy/contact-roles/",
        first_id(&role_list),
    )
    .await;
    assert_clean(&role, "contact-role.get");
}

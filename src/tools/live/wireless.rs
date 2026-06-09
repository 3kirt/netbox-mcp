//! Live wireless coverage: LAN groups, wireless LANs, and links. All seeded by
//! scripts/seed_data.py (the link joins two `ieee802.11ac` interfaces).

use serde_json::json;

use super::harness::{
    assert_clean, assert_nonempty, assert_page_shape, first_id, params, results, skip_unless_live,
    slim, slim_get,
};
use crate::tools::wireless::{self, LanGroupsListParams, LansListParams, LinksListParams};

#[tokio::test]
async fn lan_groups_filter_by_name_is_exact() {
    let env = skip_unless_live!();
    let p: LanGroupsListParams = params(json!({ "name": ["Corporate WLANs"] }));
    let resp = slim(wireless::lan_groups_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "wireless-lan-groups?name=Corporate WLANs");
    assert_nonempty(&resp, "wireless-lan-groups?name=Corporate WLANs");
    for g in results(&resp) {
        assert_eq!(g["name"], "Corporate WLANs");
    }
}

#[tokio::test]
async fn lans_filter_by_ssid_is_exact() {
    let env = skip_unless_live!();
    let p: LansListParams = params(json!({ "ssid": ["Corp-WiFi"] }));
    let resp = slim(wireless::lans_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "wireless-lans?ssid=Corp-WiFi");
    assert_nonempty(&resp, "wireless-lans?ssid=Corp-WiFi");
    for l in results(&resp) {
        assert_eq!(l["ssid"], "Corp-WiFi");
    }
}

#[tokio::test]
async fn lans_filter_by_group_returns_rows() {
    let env = skip_unless_live!();
    let p: LansListParams = params(json!({ "group": ["corporate-wlans"] }));
    let resp = slim(wireless::lans_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "wireless-lans?group=corporate-wlans");
    assert_nonempty(&resp, "wireless-lans?group=corporate-wlans");
}

#[tokio::test]
async fn links_filter_by_ssid_returns_rows() {
    let env = skip_unless_live!();
    let p: LinksListParams = params(json!({ "ssid": ["Corp-WiFi"] }));
    let resp = slim(wireless::links_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "wireless-links?ssid=Corp-WiFi");
    assert_nonempty(&resp, "wireless-links?ssid=Corp-WiFi");
    for l in results(&resp) {
        assert_eq!(l["ssid"], "Corp-WiFi");
    }
}

#[tokio::test]
async fn lan_and_link_get_by_id_are_clean() {
    let env = skip_unless_live!();

    let lans = slim(
        wireless::lans_list(&env.client, params(json!({ "ssid": ["Corp-WiFi"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&lans, "wireless-lans?ssid=Corp-WiFi");
    let lan = slim_get(&env.client, "/api/wireless/wireless-lans/", first_id(&lans)).await;
    assert_clean(&lan, "wireless-lan.get");

    let links = slim(
        wireless::links_list(&env.client, params(json!({})))
            .await
            .unwrap(),
    );
    assert_nonempty(&links, "wireless-links");
    let link = slim_get(
        &env.client,
        "/api/wireless/wireless-links/",
        first_id(&links),
    )
    .await;
    assert_clean(&link, "wireless-link.get");
}

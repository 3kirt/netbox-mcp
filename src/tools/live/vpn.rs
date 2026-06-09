//! Live VPN coverage: tunnel groups, tunnels, tunnel terminations, IKE/IPSec
//! policies, and L2VPNs. All seeded by scripts/seed_data.py.

use serde_json::json;

use super::harness::{
    assert_clean, assert_nonempty, assert_page_shape, first_id, params, results, skip_unless_live,
    slim, slim_get,
};
use crate::tools::vpn::{
    self, IkePoliciesListParams, IpsecPoliciesListParams, L2vpnsListParams, TunnelGroupsListParams,
    TunnelsListParams,
};

#[tokio::test]
async fn tunnel_groups_filter_by_slug_is_exact() {
    let env = skip_unless_live!();
    let p: TunnelGroupsListParams = params(json!({ "slug": ["site-to-site"] }));
    let resp = slim(vpn::tunnel_groups_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "tunnel-groups?slug=site-to-site");
    assert_nonempty(&resp, "tunnel-groups?slug=site-to-site");
    for g in results(&resp) {
        assert_eq!(g["slug"], "site-to-site");
    }
}

#[tokio::test]
async fn tunnels_filter_by_encapsulation_scopes_to_that_encapsulation() {
    let env = skip_unless_live!();
    let p: TunnelsListParams = params(json!({ "encapsulation": ["ipsec-tunnel"] }));
    let resp = slim(vpn::tunnels_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "tunnels?encapsulation=ipsec-tunnel");
    assert_nonempty(&resp, "tunnels?encapsulation=ipsec-tunnel");
    for t in results(&resp) {
        // Choice field: slim drops `label`, leaving the bare `value`.
        assert_eq!(
            t["encapsulation"]["value"], "ipsec-tunnel",
            "tunnel has wrong encapsulation: {t}"
        );
    }
}

#[tokio::test]
async fn l2vpns_filter_by_type_scopes_to_that_type() {
    let env = skip_unless_live!();
    let p: L2vpnsListParams = params(json!({ "type": ["vxlan"] }));
    let resp = slim(vpn::l2vpns_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "l2vpns?type=vxlan");
    assert_nonempty(&resp, "l2vpns?type=vxlan");
    for l in results(&resp) {
        assert_eq!(l["type"]["value"], "vxlan", "L2VPN has wrong type: {l}");
    }
}

#[tokio::test]
async fn ike_policies_filter_by_name_is_exact() {
    let env = skip_unless_live!();
    let p: IkePoliciesListParams = params(json!({ "name": ["ike-policy-1"] }));
    let resp = slim(vpn::ike_policies_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "ike-policies?name=ike-policy-1");
    assert_nonempty(&resp, "ike-policies?name=ike-policy-1");
    for k in results(&resp) {
        assert_eq!(k["name"], "ike-policy-1");
    }
}

#[tokio::test]
async fn ipsec_policies_filter_by_name_is_exact() {
    let env = skip_unless_live!();
    let p: IpsecPoliciesListParams = params(json!({ "name": ["ipsec-policy-1"] }));
    let resp = slim(vpn::ipsec_policies_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "ipsec-policies?name=ipsec-policy-1");
    assert_nonempty(&resp, "ipsec-policies?name=ipsec-policy-1");
    for k in results(&resp) {
        assert_eq!(k["name"], "ipsec-policy-1");
    }
}

#[tokio::test]
async fn tunnel_terminations_filter_by_tunnel() {
    let env = skip_unless_live!();
    let tunnels = slim(
        vpn::tunnels_list(&env.client, params(json!({ "name": ["NYC-LON IPsec"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&tunnels, "tunnels?name=NYC-LON IPsec");
    let tunnel_id = first_id(&tunnels);

    let terms = slim(
        vpn::tunnel_terminations_list(&env.client, params(json!({ "tunnel_id": tunnel_id })))
            .await
            .unwrap(),
    );
    assert_page_shape(&terms, "tunnel-terminations?tunnel_id=..");
    assert_nonempty(&terms, "tunnel-terminations?tunnel_id=..");
    let roles: Vec<&str> = results(&terms)
        .iter()
        .filter_map(|t| t["role"]["value"].as_str())
        .collect();
    assert!(
        roles.contains(&"hub") && roles.contains(&"spoke"),
        "expected hub and spoke roles, got {roles:?}"
    );
}

#[tokio::test]
async fn tunnel_and_l2vpn_get_by_id_are_clean() {
    let env = skip_unless_live!();

    let tunnels = slim(
        vpn::tunnels_list(&env.client, params(json!({ "name": ["NYC-LON IPsec"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&tunnels, "tunnels?name=NYC-LON IPsec");
    let tunnel = slim_get(&env.client, "/api/vpn/tunnels/", first_id(&tunnels)).await;
    assert_clean(&tunnel, "tunnel.get");

    let l2vpns = slim(
        vpn::l2vpns_list(&env.client, params(json!({ "type": ["vxlan"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&l2vpns, "l2vpns?type=vxlan");
    let l2vpn = slim_get(&env.client, "/api/vpn/l2vpns/", first_id(&l2vpns)).await;
    assert_clean(&l2vpn, "l2vpn.get");
}

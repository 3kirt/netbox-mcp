//! Live circuits coverage: providers, provider accounts/networks, circuit types,
//! circuits, and terminations. All seeded by scripts/seed_data.py.

use serde_json::json;

use super::harness::{
    assert_clean, assert_nonempty, assert_page_shape, first_id, params, results, skip_unless_live,
    slim, slim_get,
};
use crate::tools::circuits::{
    self, CircuitTypesListParams, CircuitsListParams, ProviderAccountsListParams,
    ProviderNetworksListParams, ProvidersListParams,
};

#[tokio::test]
async fn providers_filter_by_slug_is_exact() {
    let env = skip_unless_live!();
    let p: ProvidersListParams = params(json!({ "slug": ["lumen"] }));
    let resp = slim(circuits::providers_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "providers?slug=lumen");
    assert_nonempty(&resp, "providers?slug=lumen");
    for pr in results(&resp) {
        assert_eq!(pr["slug"], "lumen");
    }
}

#[tokio::test]
async fn circuit_types_filter_by_slug_is_exact() {
    let env = skip_unless_live!();
    let p: CircuitTypesListParams = params(json!({ "slug": ["internet-transit"] }));
    let resp = slim(circuits::circuit_types_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "circuit-types?slug=internet-transit");
    assert_nonempty(&resp, "circuit-types?slug=internet-transit");
    for t in results(&resp) {
        assert_eq!(t["slug"], "internet-transit");
    }
}

#[tokio::test]
async fn provider_accounts_filter_by_provider_returns_rows() {
    let env = skip_unless_live!();
    let p: ProviderAccountsListParams = params(json!({ "provider": ["lumen"] }));
    let resp = slim(
        circuits::provider_accounts_list(&env.client, p)
            .await
            .unwrap(),
    );

    assert_page_shape(&resp, "provider-accounts?provider=lumen");
    assert_nonempty(&resp, "provider-accounts?provider=lumen");
}

#[tokio::test]
async fn provider_networks_filter_by_provider_returns_rows() {
    let env = skip_unless_live!();
    let p: ProviderNetworksListParams = params(json!({ "provider": ["lumen"] }));
    let resp = slim(
        circuits::provider_networks_list(&env.client, p)
            .await
            .unwrap(),
    );

    assert_page_shape(&resp, "provider-networks?provider=lumen");
    assert_nonempty(&resp, "provider-networks?provider=lumen");
}

#[tokio::test]
async fn circuits_filter_by_provider_scopes_to_that_provider() {
    let env = skip_unless_live!();
    let p: CircuitsListParams = params(json!({ "provider": ["lumen"] }));
    let resp = slim(circuits::circuits_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "circuits?provider=lumen");
    assert_nonempty(&resp, "circuits?provider=lumen");
    for c in results(&resp) {
        assert_eq!(
            c["provider"]["slug"], "lumen",
            "circuit not from filtered provider: {c}"
        );
    }
}

#[tokio::test]
async fn circuit_terminations_filter_by_circuit() {
    let env = skip_unless_live!();
    // The Lumen circuit (LUMEN-NYC-LON-001) is the seeded one with terminations.
    let circuits_resp = slim(
        circuits::circuits_list(&env.client, params(json!({ "provider": ["lumen"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&circuits_resp, "circuits?provider=lumen");
    let circuit_id = first_id(&circuits_resp);

    let terms = slim(
        circuits::circuit_terminations_list(
            &env.client,
            params(json!({ "circuit_id": circuit_id })),
        )
        .await
        .unwrap(),
    );
    assert_page_shape(&terms, "circuit-terminations?circuit_id=..");
    assert_nonempty(&terms, "circuit-terminations?circuit_id=..");
    let sides: Vec<&str> = results(&terms)
        .iter()
        .filter_map(|t| t["term_side"].as_str())
        .collect();
    assert!(
        sides.contains(&"A") && sides.contains(&"Z"),
        "expected both A and Z terminations, got {sides:?}"
    );
}

#[tokio::test]
async fn provider_and_circuit_get_by_id_are_clean() {
    let env = skip_unless_live!();

    let providers = slim(
        circuits::providers_list(&env.client, params(json!({ "slug": ["lumen"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&providers, "providers?slug=lumen");
    let provider = slim_get(
        &env.client,
        "/api/circuits/providers/",
        first_id(&providers),
    )
    .await;
    assert_clean(&provider, "provider.get");

    let circuits_resp = slim(
        circuits::circuits_list(&env.client, params(json!({ "provider": ["lumen"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&circuits_resp, "circuits?provider=lumen");
    let circuit = slim_get(
        &env.client,
        "/api/circuits/circuits/",
        first_id(&circuits_resp),
    )
    .await;
    assert_clean(&circuit, "circuit.get");
}

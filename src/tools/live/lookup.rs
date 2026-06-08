//! Live coverage of the `netbox_lookup_host` meta-tool. Unlike the domain
//! functions, its logic lives in the shim, so these drive the real server
//! method end-to-end and parse the JSON out of the returned `CallToolResult`.

use rmcp::handler::server::wrapper::Parameters;
use serde_json::json;

use super::harness::{assert_clean, call_result_json, params, skip_unless_live};
use crate::tools::{LookupHostParams, NetboxMcpServer};

fn server(env: &super::harness::LiveEnv) -> NetboxMcpServer {
    NetboxMcpServer::new(env.url.clone(), env.token.clone()).expect("build server")
}

#[tokio::test]
async fn lookup_finds_a_device() {
    let env = skip_unless_live!();
    let srv = server(&env);
    let p: LookupHostParams = params(json!({ "name": "lon-spine-01" }));
    let result = srv.netbox_lookup_host(Parameters(p)).await.unwrap();
    let v = call_result_json(&result);

    assert_clean(&v, "lookup_host(lon-spine-01)");
    let devices = v["devices"].as_array().expect("devices array");
    assert!(
        devices.iter().any(|d| d["name"] == "lon-spine-01"),
        "expected lon-spine-01 among devices: {v}"
    );
    assert!(v["virtual_machines"].is_array());
    assert!(v["total_matches"].as_u64().unwrap() >= 1);
    assert!(v["has_more"].is_boolean());
}

#[tokio::test]
async fn lookup_finds_a_vm_and_strips_local_context() {
    let env = skip_unless_live!();
    let srv = server(&env);
    let p: LookupHostParams = params(json!({ "name": "web-prod-01" }));
    let result = srv.netbox_lookup_host(Parameters(p)).await.unwrap();
    let v = call_result_json(&result);

    // assert_clean forbids local_context_data anywhere in the VM payload.
    assert_clean(&v, "lookup_host(web-prod-01)");
    let vms = v["virtual_machines"].as_array().expect("vms array");
    assert!(
        vms.iter().any(|m| m["name"] == "web-prod-01"),
        "expected web-prod-01 among virtual_machines: {v}"
    );
    assert!(v["total_matches"].as_u64().unwrap() >= 1);
}

#[tokio::test]
async fn lookup_with_no_match_is_empty() {
    let env = skip_unless_live!();
    let srv = server(&env);
    let p: LookupHostParams = params(json!({ "name": "nonexistent-host-xyz" }));
    let result = srv.netbox_lookup_host(Parameters(p)).await.unwrap();
    let v = call_result_json(&result);

    assert_eq!(v["total_matches"], 0);
    assert_eq!(v["has_more"], false);
    assert!(v["devices"].as_array().unwrap().is_empty());
    assert!(v["virtual_machines"].as_array().unwrap().is_empty());
}

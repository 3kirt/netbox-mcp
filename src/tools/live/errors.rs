//! Live error-path coverage: confirm NetBox's 404 (unknown id) and 400
//! (invalid filter value) surface as `NetboxError::Api` with the right status —
//! the fidelity the wiremock tests can only assume.

use serde_json::json;

use super::harness::{params, skip_unless_live};
use crate::client::NetboxError;
use crate::tools::dcim::{self, DevicesListParams};

#[tokio::test]
async fn get_with_unknown_id_returns_404() {
    let env = skip_unless_live!();
    let err = env
        .client
        .get("/api/dcim/devices/", 999_999_999)
        .await
        .expect_err("get of a nonexistent id should error");

    match &err {
        NetboxError::Api { status, .. } => {
            assert_eq!(status.as_u16(), 404, "expected 404, got {err:?}");
        }
        other => panic!("expected an Api error, got {other:?}"),
    }
}

#[tokio::test]
async fn invalid_filter_value_surfaces_400() {
    let env = skip_unless_live!();
    // An out-of-range choice for the `status` enum is a 400 from NetBox
    // ("Select a valid choice"). (Note: NetBox is lenient with some bad
    // filters — e.g. a malformed `parent` prefix returns 200/empty — so an
    // enum choice is the reliable 400 trigger.)
    let p: DevicesListParams = params(json!({ "status": ["bogus-status"] }));
    let err = dcim::devices_list(&env.client, p)
        .await
        .expect_err("an invalid filter value should error");

    match &err {
        NetboxError::Api { status, .. } => {
            assert_eq!(status.as_u16(), 400, "expected 400, got {err:?}");
        }
        other => panic!("expected an Api error, got {other:?}"),
    }
}

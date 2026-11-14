//! Test-only helpers shared across the crate's unit tests.

use std::sync::{Mutex, MutexGuard};

use crate::client::NetboxClient;
use wiremock::MockServer;

// Cargo runs unit tests on multiple threads in one process. Every test that
// sets, removes, or depends on a process environment variable (NETBOX_*,
// HOME, PASSWORD_STORE_DIR, …) must hold this guard for its whole body, or
// one test's set_var leaks into another's lookup.
static ENV_LOCK: Mutex<()> = Mutex::new(());

pub(crate) fn lock_env() -> MutexGuard<'static, ()> {
    ENV_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// A `NetboxClient` pointed at a wiremock server, with a dummy token.
pub(crate) fn mock_client(server: &MockServer) -> NetboxClient {
    NetboxClient::new(server.uri(), "test-token").unwrap()
}

/// The JSON body of the first request `server` received with `method`.
/// Panics if no such request arrived or its body is not JSON.
pub(crate) async fn sent_body(
    server: &MockServer,
    method: wiremock::http::Method,
) -> serde_json::Value {
    server
        .received_requests()
        .await
        .unwrap()
        .iter()
        .find(|r| r.method == method)
        .and_then(|r| r.body_json().ok())
        .unwrap_or_else(|| panic!("no {method} request with a JSON body"))
}

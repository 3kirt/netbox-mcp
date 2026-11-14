use std::sync::Arc;
use std::time::Duration;

use reqwest::{Client, StatusCode, header};
use serde_json::Value;
use thiserror::Error;

use crate::credentials::PassEntry;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_PAGES: usize = 200;

#[derive(Debug, Error)]
pub enum NetboxError {
    #[error("NetBox API error {status}: {body}")]
    Api { status: StatusCode, body: String },
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("{kind} '{name}' not found")]
    NotFound { kind: &'static str, name: String },
    #[error(
        "ambiguous lookup: '{name}' matched {count} {kind} records; add site= or tenant= to narrow"
    )]
    Ambiguous {
        kind: &'static str,
        name: String,
        count: u64,
    },
    #[error(
        "fetch_all stopped after {max_pages} pages; dataset is very large — narrow your filter or page manually with offset="
    )]
    PageLimitExceeded { max_pages: usize },
    /// The token couldn't be obtained (e.g. its GPG key is locked). The
    /// message is written for the MCP client to relay to the user.
    #[error("{0}")]
    Credentials(String),
}

impl NetboxError {
    /// Returns a message safe to forward to the MCP client.
    /// Truncates the NetBox API error body to 300 bytes so filter values
    /// echoed in 4xx responses don't blow up the assistant's context window.
    pub fn to_tool_message(&self) -> String {
        match self {
            Self::Api { status, body } => {
                let cut = body.char_indices().nth(300).map_or(body.len(), |(i, _)| i);
                if cut < body.len() {
                    format!("NetBox API error {status}: {}… (truncated)", &body[..cut])
                } else {
                    format!("NetBox API error {status}: {body}")
                }
            }
            other => other.to_string(),
        }
    }
}

/// Thin HTTP client for the NetBox REST API.
///
/// All responses are returned as `serde_json::Value` — tools serialize them
/// directly to text, so typed structs provide no benefit.
#[derive(Clone)]
pub struct NetboxClient {
    http: Client,
    base_url: String,
    auth: Auth,
}

/// How the Authorization header is supplied.
#[derive(Clone)]
enum Auth {
    /// Known up front; baked into the client's default headers.
    Static,
    /// Decrypted from a pass entry on first use and added per request.
    Pass(Arc<PassToken>),
}

/// A token decrypted from a `pass` entry on demand. Failures are *not*
/// cached: while the GPG key is locked every request fails fast with a
/// [`NetboxError::Credentials`] explaining how to unlock, and the first
/// request after the user unlocks succeeds — no server restart. Once
/// decrypted the token is held in memory, so gpg-agent's cache TTL no longer
/// matters for this process.
struct PassToken {
    entry: PassEntry,
    // Held across the decrypt so concurrent requests run gpg once, not N times.
    state: tokio::sync::Mutex<TokenState>,
}

#[derive(Default)]
struct TokenState {
    value: Option<header::HeaderValue>,
    /// NetBox rejected `value`: re-read the entry before the next request, in
    /// case the token was rotated with `netbox-mcp login`.
    stale: bool,
}

impl PassToken {
    async fn header(&self) -> Result<header::HeaderValue, NetboxError> {
        let mut state = self.state.lock().await;
        if let Some(value) = state.value.as_ref()
            && !state.stale
        {
            return Ok(value.clone());
        }
        let read = self.entry.read_token().await;
        let result = match (read, state.value.as_ref()) {
            (Ok(token), _) => {
                let value = auth_header(&token).map_err(|e| {
                    NetboxError::Credentials(format!("{e} (from pass entry {})", self.entry.name()))
                })?;
                tracing::debug!(
                    entry = self.entry.name(),
                    "decrypted NetBox token from pass"
                );
                state.value = Some(value.clone());
                state.stale = false;
                Ok(value)
            }
            // The re-read after a rejection failed (typically: gpg-agent's
            // cache has expired since the first decrypt). Keep sending the
            // rejected token so the caller sees NetBox's real auth error
            // rather than a misleading "locked". Clear `stale`, so a failed
            // re-read costs one gpg run per *rejection*, not one per request:
            // a read-only token's 403 on a write would otherwise leave every
            // later call (each page of a fetch_all) re-running gpg behind
            // this mutex. The next rejection marks it stale again.
            (Err(e), Some(old)) => {
                tracing::debug!(error = %e, "re-reading rejected token failed; reusing it");
                let old = old.clone();
                state.stale = false;
                Ok(old)
            }
            (Err(e), None) => Err(NetboxError::Credentials(e.to_string())),
        };
        drop(state);
        result
    }

    /// NetBox rejected a request sent with `used`. Mark the token for
    /// re-reading — but only if it's still the current one, so a rejection
    /// of an already-replaced token can't discard its fresh successor.
    async fn mark_rejected(&self, used: &header::HeaderValue) {
        let mut state = self.state.lock().await;
        if state.value.as_ref() == Some(used) {
            state.stale = true;
        }
    }
}

/// Whether `status` means NetBox refused the token itself. NetBox answers an
/// invalid, expired, or revoked token with 403 ("Invalid token"), not 401, so
/// both count. A 403 can also be a plain permission denial for a valid token;
/// re-reading the entry then decrypts the same token again (one gpg run per
/// such 403, and the request isn't retried since the token didn't change).
fn is_auth_rejection(status: StatusCode) -> bool {
    status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN
}

/// Build the sensitive `Authorization: Token <t>` header value. NetBox uses
/// "Token <token>", not "Bearer <token>"; NetBox 4.5+ also accepts its v2
/// `nbt_…` tokens under the `Token` scheme.
fn auth_header(token: &str) -> anyhow::Result<header::HeaderValue> {
    let mut value = header::HeaderValue::from_str(&format!("Token {token}")).map_err(|_| {
        anyhow::anyhow!(
            "NetBox token contains characters not valid in HTTP headers (must be visible ASCII)"
        )
    })?;
    // Keeps the token out of `Debug` output and out of HTTP/2 HPACK
    // dynamic tables (where header values could otherwise be inferred).
    value.set_sensitive(true);
    Ok(value)
}

/// Make ring the process-wide rustls crypto provider. reqwest is built with
/// `rustls-no-provider` (so aws-lc-rs never enters the build) and panics on
/// `Client::build()` without one. Idempotent: a second install (another
/// client, or a test that already ran) just returns `Err`, which is fine.
fn install_crypto_provider() {
    let _ = rustls::crypto::ring::default_provider().install_default();
}

fn http_client(headers: header::HeaderMap) -> anyhow::Result<Client> {
    install_crypto_provider();
    Ok(Client::builder()
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .default_headers(headers)
        .build()?)
}

impl NetboxClient {
    pub fn new(base_url: impl Into<String>, token: impl AsRef<str>) -> anyhow::Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, auth_header(token.as_ref())?);
        Ok(Self {
            http: http_client(headers)?,
            base_url: base_url.into(),
            auth: Auth::Static,
        })
    }

    /// A client whose token is decrypted from `entry` on first use (see
    /// [`PassToken`]), so the server can start while the GPG key is locked.
    pub fn with_pass_entry(base_url: impl Into<String>, entry: PassEntry) -> anyhow::Result<Self> {
        Ok(Self {
            http: http_client(header::HeaderMap::new())?,
            base_url: base_url.into(),
            auth: Auth::Pass(Arc::new(PassToken {
                entry,
                state: tokio::sync::Mutex::new(TokenState::default()),
            })),
        })
    }

    /// Send a request, attaching a pass-backed token when there is one. The
    /// single choke point for every verb, so the lazy-auth rules (decrypt on
    /// first use, re-read after a rejection) apply uniformly.
    ///
    /// When NetBox rejects the pass token, the entry is re-read at once and,
    /// if it now holds a *different* token (rotated with `netbox-mcp login`),
    /// the request is retried once with it — so the first call after a
    /// rotation succeeds instead of failing. Retrying is safe even for writes:
    /// NetBox rejected the original before acting on it.
    async fn send(
        &self,
        builder: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, NetboxError> {
        let Auth::Pass(token) = &self.auth else {
            return Ok(builder.send().await?);
        };
        let sent = token.header().await?;
        // Our bodies are in-memory JSON, so this is `Some`; without a clone
        // there's simply no retry.
        let retry = builder.try_clone();
        let resp = builder
            .header(header::AUTHORIZATION, sent.clone())
            .send()
            .await?;
        if !is_auth_rejection(resp.status()) {
            return Ok(resp);
        }
        token.mark_rejected(&sent).await;
        let Some(retry) = retry else {
            return Ok(resp);
        };
        let fresh = token.header().await?;
        if fresh == sent {
            // Same token (a plain permission denial, or the re-read failed):
            // NetBox's answer stands.
            return Ok(resp);
        }
        tracing::debug!("NetBox rejected the token; retrying with the re-read one");
        let resp = retry
            .header(header::AUTHORIZATION, fresh.clone())
            .send()
            .await?;
        if is_auth_rejection(resp.status()) {
            token.mark_rejected(&fresh).await;
        }
        Ok(resp)
    }

    /// GET /api/{path}?{params} — returns the full paginated JSON response.
    pub async fn list(&self, path: &str, params: &[(&str, String)]) -> Result<Value, NetboxError> {
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path);
        let resp = self.send(self.http.get(&url).query(params)).await?;
        self.handle_response(resp).await
    }

    /// GET /api/{path}{id}/ — returns the single-object JSON response.
    pub async fn get(&self, path: &str, id: i32) -> Result<Value, NetboxError> {
        let url = format!("{}{}{}/", self.base_url.trim_end_matches('/'), path, id);
        let resp = self.send(self.http.get(&url)).await?;
        self.handle_response(resp).await
    }

    /// Repeatedly GET all pages at 1000 items each and merge into one response.
    /// Returns `{"count": N, "results": [...]}` with all results combined.
    /// Fails with `PageLimitExceeded` if more than `MAX_PAGES` pages are fetched.
    pub async fn list_all(
        &self,
        path: &str,
        base_params: &[(&str, String)],
    ) -> Result<Value, NetboxError> {
        let mut all_results: Vec<Value> = vec![];
        let mut offset = 0usize;
        let mut pages_fetched = 0usize;

        loop {
            if pages_fetched >= MAX_PAGES {
                return Err(NetboxError::PageLimitExceeded {
                    max_pages: MAX_PAGES,
                });
            }

            let mut params = base_params.to_vec();
            params.push(("limit", "1000".to_string()));
            params.push(("offset", offset.to_string()));

            let resp = self.list(path, &params).await?;
            pages_fetched += 1;
            let total = resp["count"].as_u64().unwrap_or(0) as usize;

            match resp["results"].as_array() {
                Some(page) if !page.is_empty() => {
                    let n = page.len();
                    all_results.extend(page.iter().cloned());
                    offset += n;
                    if offset >= total {
                        break;
                    }
                }
                _ => break,
            }
        }

        Ok(serde_json::json!({
            "count": all_results.len(),
            "results": all_results,
        }))
    }

    /// POST /api/{path} with a JSON body — returns the created object.
    pub async fn post(&self, path: &str, body: &Value) -> Result<Value, NetboxError> {
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path);
        let resp = self.send(self.http.post(&url).json(body)).await?;
        self.handle_response(resp).await
    }

    /// PATCH /api/{path}{id}/ with a JSON body — partial update, returns the
    /// updated object. NetBox treats PATCH as a partial update (unlike PUT,
    /// which requires every writable field).
    pub async fn patch(&self, path: &str, id: i32, body: &Value) -> Result<Value, NetboxError> {
        let url = format!("{}{}{}/", self.base_url.trim_end_matches('/'), path, id);
        let resp = self.send(self.http.patch(&url).json(body)).await?;
        self.handle_response(resp).await
    }

    /// DELETE /api/{path}{id}/ — returns () on success (NetBox replies 204).
    pub async fn delete(&self, path: &str, id: i32) -> Result<(), NetboxError> {
        let url = format!("{}{}{}/", self.base_url.trim_end_matches('/'), path, id);
        let resp = self.send(self.http.delete(&url)).await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(NetboxError::Api { status, body });
        }
        Ok(())
    }

    async fn handle_response(&self, resp: reqwest::Response) -> Result<Value, NetboxError> {
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(NetboxError::Api { status, body });
        }
        Ok(resp.json().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_tool_message_short_body_passes_through() {
        let e = NetboxError::Api {
            status: StatusCode::BAD_REQUEST,
            body: "short error".to_string(),
        };
        assert_eq!(
            e.to_tool_message(),
            "NetBox API error 400 Bad Request: short error"
        );
    }

    #[test]
    fn to_tool_message_long_ascii_body_is_truncated() {
        let e = NetboxError::Api {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: "x".repeat(500),
        };
        let msg = e.to_tool_message();
        assert!(
            msg.ends_with("… (truncated)"),
            "expected truncated suffix, got: {msg}"
        );
        assert!(msg.contains(&"x".repeat(300)));
        assert!(!msg.contains(&"x".repeat(301)));
    }

    #[test]
    fn to_tool_message_multibyte_at_boundary_does_not_panic() {
        // 299 ASCII bytes then multi-byte '€' (3 bytes each). A byte slice at
        // position 300 would land inside the '€' and panic; char_indices must
        // find the safe boundary instead.
        let body = "a".repeat(299) + &"€".repeat(100);
        let e = NetboxError::Api {
            status: StatusCode::BAD_REQUEST,
            body,
        };
        let msg = e.to_tool_message();
        assert!(msg.ends_with("… (truncated)"));
    }

    #[test]
    fn to_tool_message_non_api_errors_use_display() {
        let e = NetboxError::NotFound {
            kind: "device",
            name: "rtr-01".to_string(),
        };
        assert_eq!(e.to_tool_message(), "device 'rtr-01' not found");
    }

    // ----- write verbs -----

    use crate::test_support::mock_client;
    use serde_json::json;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn post_sends_json_body_and_returns_created_object() {
        let server = MockServer::start().await;
        let req_body = json!({ "name": "vm-01", "cluster": 3 });
        Mock::given(method("POST"))
            .and(path("/api/virtualization/virtual-machines/"))
            .and(body_json(&req_body))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(json!({ "id": 7, "name": "vm-01" })),
            )
            .mount(&server)
            .await;

        let resp = mock_client(&server)
            .post("/api/virtualization/virtual-machines/", &req_body)
            .await
            .unwrap();
        assert_eq!(resp["id"], 7);
        assert_eq!(resp["name"], "vm-01");
    }

    #[tokio::test]
    async fn post_error_surfaces_api_error_with_body() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/virtualization/virtual-machines/"))
            .respond_with(
                ResponseTemplate::new(400)
                    .set_body_json(json!({ "name": ["This field is required."] })),
            )
            .mount(&server)
            .await;

        let err = mock_client(&server)
            .post("/api/virtualization/virtual-machines/", &json!({}))
            .await
            .unwrap_err();
        match err {
            NetboxError::Api { status, body } => {
                assert_eq!(status, StatusCode::BAD_REQUEST);
                assert!(body.contains("This field is required."));
            }
            other => panic!("expected Api error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn patch_targets_id_path_and_returns_updated_object() {
        let server = MockServer::start().await;
        let req_body = json!({ "status": "offline" });
        Mock::given(method("PATCH"))
            .and(path("/api/virtualization/virtual-machines/7/"))
            .and(body_json(&req_body))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({ "id": 7, "status": { "value": "offline" } })),
            )
            .mount(&server)
            .await;

        let resp = mock_client(&server)
            .patch("/api/virtualization/virtual-machines/", 7, &req_body)
            .await
            .unwrap();
        assert_eq!(resp["status"]["value"], "offline");
    }

    #[tokio::test]
    async fn delete_targets_id_path_and_succeeds_on_204() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/virtualization/virtual-machines/7/"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        mock_client(&server)
            .delete("/api/virtualization/virtual-machines/", 7)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn delete_error_surfaces_api_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/virtualization/virtual-machines/99/"))
            .respond_with(ResponseTemplate::new(404).set_body_string("Not found."))
            .mount(&server)
            .await;

        let err = mock_client(&server)
            .delete("/api/virtualization/virtual-machines/", 99)
            .await
            .unwrap_err();
        assert!(matches!(err, NetboxError::Api { status, .. } if status == StatusCode::NOT_FOUND));
    }

    // ----- pass-backed (lazy) auth -----

    mod pass_auth {
        use super::*;
        use crate::credentials::PassStore;
        use crate::credentials::test_support::{LOCKED_SNIPPET, fake_gpg, ok_snippet};
        use wiremock::matchers::header;

        const STATUS: &str = "/api/status/";

        fn pass_client(
            server: &MockServer,
        ) -> (tempfile::TempDir, std::path::PathBuf, NetboxClient) {
            let dir = tempfile::tempdir().unwrap();
            let entry = PassStore::new(dir.path(), fake_gpg()).entry("t").unwrap();
            let file = entry.path();
            let client = NetboxClient::with_pass_entry(server.uri(), entry).unwrap();
            (dir, file, client)
        }

        #[tokio::test]
        async fn locked_key_fails_then_succeeds_after_unlock() {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path(STATUS))
                .and(header("authorization", "Token tok-from-pass"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
                .expect(2)
                .mount(&server)
                .await;
            let (_dir, file, client) = pass_client(&server);

            std::fs::write(&file, LOCKED_SNIPPET).unwrap();
            let err = client.list(STATUS, &[]).await.unwrap_err();
            assert!(matches!(err, NetboxError::Credentials(_)), "{err}");
            let msg = err.to_tool_message();
            assert!(msg.contains("netbox-mcp unlock"), "{msg}");

            // "Unlock": the next request decrypts successfully — no restart.
            std::fs::write(&file, ok_snippet("tok-from-pass")).unwrap();
            client.list(STATUS, &[]).await.unwrap();

            // Cached in memory: re-locking the key doesn't affect this process.
            std::fs::write(&file, LOCKED_SNIPPET).unwrap();
            client.list(STATUS, &[]).await.unwrap();
        }

        #[tokio::test]
        async fn rejected_token_is_reread_and_rotation_picked_up() {
            let server = MockServer::start().await;
            // NetBox answers an invalid token with 403, not 401.
            Mock::given(method("GET"))
                .and(path(STATUS))
                .and(header("authorization", "Token tok-old"))
                .respond_with(
                    ResponseTemplate::new(403).set_body_json(json!({"detail": "Invalid v1 token"})),
                )
                .mount(&server)
                .await;
            Mock::given(method("GET"))
                .and(path(STATUS))
                .and(header("authorization", "Token tok-rotated"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
                .mount(&server)
                .await;
            let (_dir, file, client) = pass_client(&server);

            std::fs::write(&file, ok_snippet("tok-old")).unwrap();
            assert!(client.list(STATUS, &[]).await.is_err());
            // Rotated in the store (e.g. via `netbox-mcp login`) — picked up.
            std::fs::write(&file, ok_snippet("tok-rotated")).unwrap();
            client.list(STATUS, &[]).await.unwrap();
        }

        #[tokio::test]
        async fn rejected_token_with_locked_key_reports_the_real_403() {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path(STATUS))
                .and(header("authorization", "Token tok-revoked"))
                .respond_with(
                    ResponseTemplate::new(403).set_body_json(json!({"detail": "Invalid v1 token"})),
                )
                .mount(&server)
                .await;
            let (_dir, file, client) = pass_client(&server);

            std::fs::write(&file, ok_snippet("tok-revoked")).unwrap();
            assert!(client.list(STATUS, &[]).await.is_err());
            // gpg-agent's cache expires; the 403 must not turn into "locked".
            std::fs::write(&file, LOCKED_SNIPPET).unwrap();
            let err = client.list(STATUS, &[]).await.unwrap_err();
            assert!(
                matches!(err, NetboxError::Api { status, .. } if status == StatusCode::FORBIDDEN),
                "{err}"
            );
        }

        /// A snippet that logs each decrypt to `counter`, then runs `inner`.
        fn counted(counter: &std::path::Path, inner: &str) -> String {
            format!("echo x >> '{}'\n{inner}", counter.display())
        }

        fn decrypts(counter: &std::path::Path) -> usize {
            std::fs::read_to_string(counter).map_or(0, |s| s.lines().count())
        }

        #[tokio::test]
        async fn permission_denial_with_expired_cache_does_not_rerun_gpg_per_request() {
            let server = MockServer::start().await;
            // A read-only token: reads work, the write is refused with 403.
            Mock::given(method("GET"))
                .and(path(STATUS))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
                .mount(&server)
                .await;
            Mock::given(method("POST"))
                .and(path(STATUS))
                .respond_with(ResponseTemplate::new(403).set_body_json(
                    json!({"detail": "You do not have permission to perform this action."}),
                ))
                .mount(&server)
                .await;
            let (dir, file, client) = pass_client(&server);
            let counter = dir.path().join("decrypts");

            std::fs::write(&file, counted(&counter, &ok_snippet("tok-ro"))).unwrap();
            client.list(STATUS, &[]).await.unwrap();
            assert_eq!(decrypts(&counter), 1);

            // gpg-agent's cache expires, then the write is denied: one re-read
            // (which fails), and the real 403 is reported.
            std::fs::write(&file, counted(&counter, LOCKED_SNIPPET)).unwrap();
            let err = client.post(STATUS, &json!({})).await.unwrap_err();
            assert!(
                matches!(err, NetboxError::Api { status, .. } if status == StatusCode::FORBIDDEN),
                "{err}"
            );
            assert_eq!(decrypts(&counter), 2);

            // Later reads reuse the in-memory token: no gpg per request.
            for _ in 0..5 {
                client.list(STATUS, &[]).await.unwrap();
            }
            assert_eq!(decrypts(&counter), 2, "gpg re-ran on every request");
        }

        #[tokio::test]
        async fn first_call_after_rotation_succeeds_by_retrying() {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path(STATUS))
                .and(header("authorization", "Token tok-old"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
                .up_to_n_times(1)
                .with_priority(1)
                .mount(&server)
                .await;
            Mock::given(method("GET"))
                .and(path(STATUS))
                .and(header("authorization", "Token tok-new"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
                .mount(&server)
                .await;
            // tok-old is revoked after its first use.
            Mock::given(method("GET"))
                .respond_with(
                    ResponseTemplate::new(403).set_body_json(json!({"detail": "Invalid v1 token"})),
                )
                .with_priority(5)
                .mount(&server)
                .await;
            let (_dir, file, client) = pass_client(&server);

            std::fs::write(&file, ok_snippet("tok-old")).unwrap();
            client.list(STATUS, &[]).await.unwrap();
            // `netbox-mcp login` rotates the token in the store.
            std::fs::write(&file, ok_snippet("tok-new")).unwrap();
            // The very next call: rejected with tok-old, re-read, retried.
            client.list(STATUS, &[]).await.unwrap();
        }

        #[tokio::test]
        async fn concurrent_first_requests_decrypt_once() {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path(STATUS))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
                .mount(&server)
                .await;
            let (dir, file, client) = pass_client(&server);
            // Each decrypt appends a line to a counter file next to the entry.
            let counter = dir.path().join("decrypts");
            std::fs::write(
                &file,
                format!(
                    "echo x >> '{}'\n{}",
                    counter.display(),
                    ok_snippet("tok-once")
                ),
            )
            .unwrap();
            let calls = (0..8).map(|_| {
                let c = client.clone();
                async move { c.list(STATUS, &[]).await }
            });
            for r in futures_join_all(calls).await {
                r.unwrap();
            }
            let decrypts = std::fs::read_to_string(&counter).unwrap().lines().count();
            assert_eq!(decrypts, 1, "gpg ran {decrypts} times");
        }

        /// Minimal join-all over tokio, to avoid a `futures` dev-dependency.
        async fn futures_join_all<F>(futs: impl Iterator<Item = F>) -> Vec<F::Output>
        where
            F: std::future::Future + Send + 'static,
            F::Output: Send + 'static,
        {
            let handles: Vec<_> = futs.map(tokio::spawn).collect();
            let mut out = Vec::with_capacity(handles.len());
            for h in handles {
                out.push(h.await.unwrap());
            }
            out
        }
    }
}

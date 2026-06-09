use std::time::Duration;

use reqwest::{Client, StatusCode, header};
use serde_json::Value;
use thiserror::Error;

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
}

impl NetboxError {
    /// Returns a message safe to forward to the MCP client.
    /// Truncates the NetBox API error body to 300 bytes so filter values
    /// echoed in 4xx responses don't blow up the assistant's context window.
    pub fn to_tool_message(&self) -> String {
        match self {
            NetboxError::Api { status, body } => {
                let cut = body
                    .char_indices()
                    .nth(300)
                    .map(|(i, _)| i)
                    .unwrap_or(body.len());
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
}

impl NetboxClient {
    pub fn new(base_url: impl Into<String>, token: impl AsRef<str>) -> anyhow::Result<Self> {
        let auth = format!("Token {}", token.as_ref());
        let auth_value = header::HeaderValue::from_str(&auth).map_err(|_| {
            anyhow::anyhow!(
                "NetBox token contains characters not valid in HTTP headers (must be visible ASCII)"
            )
        })?;

        let mut headers = header::HeaderMap::new();
        // NetBox uses "Token <token>", not "Bearer <token>".
        headers.insert(header::AUTHORIZATION, auth_value);

        let http = Client::builder()
            .connect_timeout(CONNECT_TIMEOUT)
            .timeout(REQUEST_TIMEOUT)
            .default_headers(headers)
            .build()?;

        Ok(NetboxClient {
            http,
            base_url: base_url.into(),
        })
    }

    /// GET /api/{path}?{params} — returns the full paginated JSON response.
    pub async fn list(&self, path: &str, params: &[(&str, String)]) -> Result<Value, NetboxError> {
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path);
        let resp = self.http.get(&url).query(params).send().await?;
        self.handle_response(resp).await
    }

    /// GET /api/{path}{id}/ — returns the single-object JSON response.
    pub async fn get(&self, path: &str, id: i32) -> Result<Value, NetboxError> {
        let url = format!("{}{}{}/", self.base_url.trim_end_matches('/'), path, id);
        let resp = self.http.get(&url).send().await?;
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
        let resp = self.http.post(&url).json(body).send().await?;
        self.handle_response(resp).await
    }

    /// PATCH /api/{path}{id}/ with a JSON body — partial update, returns the
    /// updated object. NetBox treats PATCH as a partial update (unlike PUT,
    /// which requires every writable field).
    pub async fn patch(&self, path: &str, id: i32, body: &Value) -> Result<Value, NetboxError> {
        let url = format!("{}{}{}/", self.base_url.trim_end_matches('/'), path, id);
        let resp = self.http.patch(&url).json(body).send().await?;
        self.handle_response(resp).await
    }

    /// DELETE /api/{path}{id}/ — returns () on success (NetBox replies 204).
    pub async fn delete(&self, path: &str, id: i32) -> Result<(), NetboxError> {
        let url = format!("{}{}{}/", self.base_url.trim_end_matches('/'), path, id);
        let resp = self.http.delete(&url).send().await?;
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

    use serde_json::json;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn mock_client(server: &MockServer) -> NetboxClient {
        NetboxClient::new(server.uri(), "test-token").unwrap()
    }

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
}

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
                let cut = body.char_indices().nth(300).map(|(i, _)| i).unwrap_or(body.len());
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

    async fn handle_response(&self, resp: reqwest::Response) -> Result<Value, NetboxError> {
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(NetboxError::Api { status, body });
        }
        Ok(resp.json().await?)
    }
}

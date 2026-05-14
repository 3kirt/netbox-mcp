use reqwest::{Client, StatusCode, header};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetboxError {
    #[error("NetBox API error {status}: {body}")]
    Api { status: StatusCode, body: String },
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("{0}")]
    Generic(String),
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
    pub fn new(base_url: impl Into<String>, token: impl AsRef<str>) -> Self {
        let mut headers = header::HeaderMap::new();
        // NetBox uses "Token <token>", not "Bearer <token>".
        let auth = format!("Token {}", token.as_ref());
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&auth).expect("token is valid header value"),
        );

        let http = Client::builder()
            .default_headers(headers)
            .build()
            .expect("failed to build reqwest client");

        NetboxClient {
            http,
            base_url: base_url.into(),
        }
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
    pub async fn list_all(
        &self,
        path: &str,
        base_params: &[(&str, String)],
    ) -> Result<Value, NetboxError> {
        let mut all_results: Vec<Value> = vec![];
        let mut offset = 0usize;

        loop {
            let mut params = base_params.to_vec();
            params.push(("limit", "1000".to_string()));
            params.push(("offset", offset.to_string()));

            let resp = self.list(path, &params).await?;
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

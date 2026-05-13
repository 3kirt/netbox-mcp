use reqwest::{Client, StatusCode, header};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetboxError {
    #[error("NetBox API error {status}: {body}")]
    Api { status: StatusCode, body: String },
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
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

    async fn handle_response(&self, resp: reqwest::Response) -> Result<Value, NetboxError> {
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(NetboxError::Api { status, body });
        }
        Ok(resp.json().await?)
    }
}

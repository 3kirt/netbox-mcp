use crate::client::{NetboxClient, NetboxError};
use crate::tools::clamp_limit;
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Tags
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TagsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by tag name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by color (hex, e.g. ff0000)")]
    pub color: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn tags_list(client: &NetboxClient, p: TagsListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.color.unwrap_or_default() {
        params.push(("color", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/extras/tags/", &params).await
}

// --------------------------------------------------------------------------
// Config Contexts
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ConfigContextsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by config context name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by active state")]
    pub is_active: Option<bool>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by device role slug")]
    pub role: Option<Vec<String>>,
    #[schemars(description = "Filter by platform slug")]
    pub platform: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn config_contexts_list(
    client: &NetboxClient,
    p: ConfigContextsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    if let Some(v) = p.is_active {
        params.push(("is_active", v.to_string()));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    for v in p.role.unwrap_or_default() {
        params.push(("role", v));
    }
    for v in p.platform.unwrap_or_default() {
        params.push(("platform", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/extras/config-contexts/", &params).await
}

// --------------------------------------------------------------------------
// Journal Entries
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JournalEntriesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by creating user (username)")]
    pub created_by: Option<Vec<String>>,
    #[schemars(description = "Filter by kind (info, success, warning, danger)")]
    pub kind: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn journal_entries_list(
    client: &NetboxClient,
    p: JournalEntriesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.created_by.unwrap_or_default() {
        params.push(("created_by", v));
    }
    for v in p.kind.unwrap_or_default() {
        params.push(("kind", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/extras/journal-entries/", &params).await
}

// --------------------------------------------------------------------------
// Custom Fields
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CustomFieldsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by custom field name")]
    pub name: Option<Vec<String>>,
    #[schemars(
        description = "Filter by field type (text, longtext, integer, decimal, boolean, date, datetime, url, json, select, multiselect, object, multiobject)"
    )]
    pub r#type: Option<Vec<String>>,
    #[schemars(description = "Filter by content type (e.g. dcim.device)")]
    pub content_types: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn custom_fields_list(
    client: &NetboxClient,
    p: CustomFieldsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.r#type.unwrap_or_default() {
        params.push(("type", v));
    }
    for v in p.content_types.unwrap_or_default() {
        params.push(("content_types", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/extras/custom-fields/", &params).await
}

// --------------------------------------------------------------------------
// Export Templates
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ExportTemplatesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by export template name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by content type (e.g. dcim.device)")]
    pub content_types: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn export_templates_list(
    client: &NetboxClient,
    p: ExportTemplatesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.content_types.unwrap_or_default() {
        params.push(("content_types", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/extras/export-templates/", &params).await
}

// --------------------------------------------------------------------------
// Webhooks
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct WebhooksListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by webhook name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by HTTP method (GET, POST, PUT, PATCH, DELETE)")]
    pub http_method: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn webhooks_list(
    client: &NetboxClient,
    p: WebhooksListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.http_method.unwrap_or_default() {
        params.push(("http_method", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/extras/webhooks/", &params).await
}

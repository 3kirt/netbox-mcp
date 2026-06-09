use crate::client::{NetboxClient, NetboxError};
use crate::tools::{CommonListParams, QueryBuilder};
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Tags
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TagsListParams {
    #[schemars(description = "Filter by tag name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by color (hex, e.g. ff0000)")]
    pub color: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn tags_list(client: &NetboxClient, p: TagsListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("color", p.color);
    qb.run_common(client, "/api/extras/tags/", p.common).await
}

// --------------------------------------------------------------------------
// Config Contexts
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ConfigContextsListParams {
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
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn config_contexts_list(
    client: &NetboxClient,
    p: ConfigContextsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .opt("is_active", p.is_active)
        .many("site", p.site)
        .many("role", p.role)
        .many("platform", p.platform);
    qb.run_common(client, "/api/extras/config-contexts/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Journal Entries
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JournalEntriesListParams {
    #[schemars(description = "Filter by creating user (username)")]
    pub created_by: Option<Vec<String>>,
    #[schemars(description = "Filter by kind (info, success, warning, danger)")]
    pub kind: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn journal_entries_list(
    client: &NetboxClient,
    p: JournalEntriesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("created_by", p.created_by)
        .many("kind", p.kind);
    qb.run_common(client, "/api/extras/journal-entries/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Custom Fields
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CustomFieldsListParams {
    #[schemars(description = "Filter by custom field name")]
    pub name: Option<Vec<String>>,
    #[schemars(
        description = "Filter by field type (text, longtext, integer, decimal, boolean, date, datetime, url, json, select, multiselect, object, multiobject)"
    )]
    pub r#type: Option<Vec<String>>,
    #[schemars(description = "Filter by content type (e.g. dcim.device)")]
    pub content_types: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn custom_fields_list(
    client: &NetboxClient,
    p: CustomFieldsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("type", p.r#type)
        .many("content_types", p.content_types);
    qb.run_common(client, "/api/extras/custom-fields/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Export Templates
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ExportTemplatesListParams {
    #[schemars(description = "Filter by export template name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by content type (e.g. dcim.device)")]
    pub content_types: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn export_templates_list(
    client: &NetboxClient,
    p: ExportTemplatesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("content_types", p.content_types);
    qb.run_common(client, "/api/extras/export-templates/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Webhooks
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct WebhooksListParams {
    #[schemars(description = "Filter by webhook name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by HTTP method (GET, POST, PUT, PATCH, DELETE)")]
    pub http_method: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn webhooks_list(
    client: &NetboxClient,
    p: WebhooksListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("http_method", p.http_method);
    qb.run_common(client, "/api/extras/webhooks/", p.common)
        .await
}

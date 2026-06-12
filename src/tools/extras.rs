use crate::client::{NetboxClient, NetboxError};
use crate::tools::{BodyBuilder, CommonListParams, QueryBuilder};
use serde::Deserialize;
use serde_json::Value;

const TAGS_PATH: &str = "/api/extras/tags/";

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
    qb.run_common(client, TAGS_PATH, p.common).await
}

// --------------------------------------------------------------------------
// Tags — write (create / update / delete)
// --------------------------------------------------------------------------

/// Writable tag attributes shared by create and update. Flattened into the
/// create/update params so both expose the same fields.
#[derive(Debug, Default, Deserialize, schemars::JsonSchema)]
pub struct TagFields {
    #[schemars(description = "Color as a 6-digit hex string, no leading '#' (e.g. 9e9e9e)")]
    pub color: Option<String>,
    #[schemars(description = "Short description")]
    pub description: Option<String>,
    #[schemars(
        description = "Object types this tag may be applied to, as content-type labels (e.g. [\"dcim.device\", \"virtualization.virtualmachine\"]); empty means all types"
    )]
    pub object_types: Option<Vec<String>>,
    #[schemars(description = "Ordering weight (default 1000)")]
    pub weight: Option<i32>,
}

impl TagFields {
    /// Apply every set field to `b`, omitting `None`s so PATCH stays a partial
    /// update. Shared by create and update so both bodies carry the same fields.
    fn apply(self, b: BodyBuilder) -> BodyBuilder {
        b.opt("color", self.color)
            .opt("description", self.description)
            .opt("object_types", self.object_types)
            .opt("weight", self.weight)
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TagCreateParams {
    #[schemars(description = "Tag name (required)")]
    pub name: String,
    #[schemars(description = "URL-safe slug (required, e.g. my-tag)")]
    pub slug: String,
    #[serde(flatten)]
    pub fields: TagFields,
}

pub async fn tag_create(client: &NetboxClient, p: TagCreateParams) -> Result<Value, NetboxError> {
    let body = p
        .fields
        .apply(BodyBuilder::new().req("name", p.name).req("slug", p.slug))
        .build();
    client.post(TAGS_PATH, &body).await
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TagUpdateParams {
    #[schemars(description = "NetBox ID of the tag to update (required)")]
    pub id: i32,
    #[schemars(description = "New tag name")]
    pub name: Option<String>,
    #[schemars(description = "New URL-safe slug")]
    pub slug: Option<String>,
    #[serde(flatten)]
    pub fields: TagFields,
}

pub async fn tag_update(client: &NetboxClient, p: TagUpdateParams) -> Result<Value, NetboxError> {
    let body = p
        .fields
        .apply(BodyBuilder::new().opt("name", p.name).opt("slug", p.slug))
        .build();
    client.patch(TAGS_PATH, p.id, &body).await
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TagDeleteParams {
    #[schemars(description = "NetBox ID of the tag to delete (required)")]
    pub id: i32,
}

pub async fn tag_delete(client: &NetboxClient, p: TagDeleteParams) -> Result<(), NetboxError> {
    client.delete(TAGS_PATH, p.id).await
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

// --------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::{
        TagCreateParams, TagDeleteParams, TagFields, TagUpdateParams, tag_create, tag_delete,
        tag_update,
    };
    use crate::test_support::{mock_client, sent_body};

    #[tokio::test]
    async fn tag_create_sends_name_slug_and_only_set_fields() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/extras/tags/"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "id": 1 })))
            .mount(&server)
            .await;

        tag_create(
            &mock_client(&server),
            TagCreateParams {
                name: "Production".into(),
                slug: "production".into(),
                fields: TagFields {
                    color: Some("9e9e9e".into()),
                    object_types: Some(vec!["dcim.device".into()]),
                    ..Default::default()
                },
            },
        )
        .await
        .unwrap();

        let body = sent_body(&server, wiremock::http::Method::POST).await;
        assert_eq!(body["name"], "Production");
        assert_eq!(body["slug"], "production");
        assert_eq!(body["color"], "9e9e9e");
        assert_eq!(body["object_types"], json!(["dcim.device"]));
        // Unset optional fields must be omitted, not sent as null.
        assert!(body.get("description").is_none());
        assert!(body.get("weight").is_none());
    }

    #[tokio::test]
    async fn tag_update_patches_id_with_partial_body() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/extras/tags/4/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "id": 4 })))
            .mount(&server)
            .await;

        tag_update(
            &mock_client(&server),
            TagUpdateParams {
                id: 4,
                name: None,
                slug: None,
                fields: TagFields {
                    description: Some("edge devices".into()),
                    ..Default::default()
                },
            },
        )
        .await
        .unwrap();

        let body = sent_body(&server, wiremock::http::Method::PATCH).await;
        assert_eq!(body["description"], "edge devices");
        assert_eq!(body.as_object().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn tag_delete_hits_id_path() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/extras/tags/4/"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        tag_delete(&mock_client(&server), TagDeleteParams { id: 4 })
            .await
            .unwrap();
    }
}

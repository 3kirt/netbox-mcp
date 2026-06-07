use crate::client::{NetboxClient, NetboxError};
use crate::tools::{PaginationParams, QueryBuilder};
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Data Sources
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DataSourcesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by data source name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by status (new, queued, syncing, completed, failed)")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn data_sources_list(
    client: &NetboxClient,
    p: DataSourcesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("status", p.status)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/core/data-sources/", p.pagination)
        .await
}

// --------------------------------------------------------------------------
// Background Jobs
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JobsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(
        description = "Filter by job status (pending, running, completed, errored, failed)"
    )]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by object type (e.g. dcim.device)")]
    pub object_type: Option<String>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn jobs_list(client: &NetboxClient, p: JobsListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("status", p.status)
        .opt("object_type", p.object_type)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/core/jobs/", p.pagination).await
}

// --------------------------------------------------------------------------
// Object Changes (audit log)
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ObjectChangesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by username")]
    pub user: Option<Vec<String>>,
    #[schemars(description = "Filter by action (create, update, delete)")]
    pub action: Option<Vec<String>>,
    #[schemars(description = "Filter by changed object type (e.g. dcim.device)")]
    pub changed_object_type: Option<String>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "When true, replace prechange_data/postchange_data with only the keys that differ between them. Has no effect on create/delete records (one side is null). Reduces response size significantly for update-heavy logs."
    )]
    pub diff_only: Option<bool>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn object_changes_list(
    client: &NetboxClient,
    p: ObjectChangesListParams,
) -> Result<Value, NetboxError> {
    let diff_only = p.diff_only.unwrap_or(false);
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("user", p.user)
        .many("action", p.action)
        .opt("changed_object_type", p.changed_object_type)
        .opt("ordering", p.ordering);
    let mut resp = qb
        .run(client, "/api/core/object-changes/", p.pagination)
        .await?;
    if diff_only && let Some(results) = resp.get_mut("results") {
        apply_change_diff(results);
    }
    Ok(resp)
}

/// Replace `prechange_data` / `postchange_data` on each update record with
/// only the keys whose values differ between the two snapshots.
/// Create/delete records (where one side is null) are left untouched.
pub(crate) fn apply_change_diff(results: &mut Value) {
    let arr = match results.as_array_mut() {
        Some(a) => a,
        None => return,
    };
    for item in arr.iter_mut() {
        let obj = match item.as_object_mut() {
            Some(o) => o,
            None => continue,
        };
        let pre = match obj.get("prechange_data").cloned() {
            Some(Value::Object(m)) => m,
            _ => continue,
        };
        let post = match obj.get("postchange_data").cloned() {
            Some(Value::Object(m)) => m,
            _ => continue,
        };
        let all_keys: std::collections::HashSet<&String> = pre.keys().chain(post.keys()).collect();
        let mut diff_pre = serde_json::Map::new();
        let mut diff_post = serde_json::Map::new();
        for k in all_keys {
            let pre_v = pre.get(k).unwrap_or(&Value::Null);
            let post_v = post.get(k).unwrap_or(&Value::Null);
            if pre_v != post_v {
                diff_pre.insert(k.clone(), pre_v.clone());
                diff_post.insert(k.clone(), post_v.clone());
            }
        }
        obj.insert("prechange_data".to_string(), Value::Object(diff_pre));
        obj.insert("postchange_data".to_string(), Value::Object(diff_post));
    }
}

#[cfg(test)]
mod tests {
    use super::apply_change_diff;
    use serde_json::json;

    #[test]
    fn diff_keeps_only_divergent_fields() {
        let mut results = json!([{
            "action": {"value": "update"},
            "prechange_data": {"name": "old", "status": "planned", "vcpus": 2},
            "postchange_data": {"name": "new", "status": "active", "vcpus": 2},
        }]);
        apply_change_diff(&mut results);
        assert_eq!(
            results[0]["prechange_data"],
            json!({"name": "old", "status": "planned"})
        );
        assert_eq!(
            results[0]["postchange_data"],
            json!({"name": "new", "status": "active"})
        );
    }

    #[test]
    fn diff_both_empty_when_nothing_changed() {
        let mut results = json!([{
            "action": {"value": "update"},
            "prechange_data": {"name": "same", "status": "active"},
            "postchange_data": {"name": "same", "status": "active"},
        }]);
        apply_change_diff(&mut results);
        assert_eq!(results[0]["prechange_data"], json!({}));
        assert_eq!(results[0]["postchange_data"], json!({}));
    }

    #[test]
    fn diff_skips_create_records() {
        let mut results = json!([{
            "action": {"value": "create"},
            "prechange_data": null,
            "postchange_data": {"name": "new-vm", "status": "active"},
        }]);
        apply_change_diff(&mut results);
        assert_eq!(results[0]["prechange_data"], json!(null));
        assert_eq!(
            results[0]["postchange_data"],
            json!({"name": "new-vm", "status": "active"})
        );
    }

    #[test]
    fn diff_skips_delete_records() {
        let mut results = json!([{
            "action": {"value": "delete"},
            "prechange_data": {"name": "old-vm", "status": "active"},
            "postchange_data": null,
        }]);
        apply_change_diff(&mut results);
        assert_eq!(
            results[0]["prechange_data"],
            json!({"name": "old-vm", "status": "active"})
        );
        assert_eq!(results[0]["postchange_data"], json!(null));
    }
}

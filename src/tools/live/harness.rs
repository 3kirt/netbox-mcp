//! Shared scaffolding for the live integration suite: credentials/env, the
//! `skip_unless_live!` guard, param construction, the real output-path helpers,
//! and the cross-domain invariants encoded as reusable assertions.

use rmcp::model::CallToolResult;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::client::NetboxClient;
use crate::tools::slim::{STRIP_KEYS, slim_value};

/// True for any key that must never appear in a response: the keys `slim_value`
/// strips from every object (sourced from `slim::STRIP_KEYS` so the two can't
/// drift), plus the bare pagination URLs `clean_page_response` removes.
fn is_stripped_key(k: &str) -> bool {
    STRIP_KEYS.contains(&k) || k == "next" || k == "previous"
}

/// Live credentials plus a ready client, sourced from the environment.
pub struct LiveEnv {
    pub url: String,
    pub token: String,
    pub client: NetboxClient,
}

/// Returns a live environment when `NETBOX_URL` and `NETBOX_TOKEN` are both set
/// and non-empty; otherwise `None` so the caller can skip.
pub fn live_env() -> Option<LiveEnv> {
    let url = std::env::var("NETBOX_URL").ok()?;
    let token = std::env::var("NETBOX_TOKEN").ok()?;
    if url.trim().is_empty() || token.trim().is_empty() {
        return None;
    }
    let client = NetboxClient::new(url.clone(), &token).expect("build live client");
    Some(LiveEnv { url, token, client })
}

/// Bind a `LiveEnv` or return early (with a notice) when credentials are
/// absent — so the feature is safe to compile/run in CI without secrets.
macro_rules! skip_unless_live {
    () => {
        match $crate::tools::live::harness::live_env() {
            Some(env) => env,
            None => {
                eprintln!(
                    "skipping live test: set NETBOX_URL and NETBOX_TOKEN to run \
                     (seed the instance first with scripts/seed_data.py)"
                );
                return;
            }
        }
    };
}
pub(crate) use skip_unless_live;

/// Build a typed params struct from a JSON literal. Mirrors how params arrive
/// over MCP and keeps tests free of the long `field: None` lists.
pub fn params<T: DeserializeOwned>(v: Value) -> T {
    serde_json::from_value(v).expect("deserialize params")
}

/// Apply the boundary transform a list/get response would receive at the rmcp
/// edge, so assertions see exactly what an MCP client gets.
pub fn slim(v: Value) -> Value {
    slim_value(v)
}

/// Fetch a single object by id through the real client and slim it.
pub async fn slim_get(client: &NetboxClient, path: &str, id: i32) -> Value {
    slim_value(client.get(path, id).await.expect("get by id failed"))
}

/// The `results` array of a list response.
pub fn results(resp: &Value) -> &Vec<Value> {
    resp["results"]
        .as_array()
        .expect("list response missing results array")
}

/// Pull a numeric id out of the first result of a list response.
pub fn first_id(resp: &Value) -> i32 {
    results(resp)
        .first()
        .and_then(|r| r["id"].as_i64())
        .expect("expected at least one result with an id") as i32
}

/// Recursively assert the universal cleanliness invariants on any response:
/// no null object values, no stripped keys, and no un-slimmed choice fields.
pub fn assert_clean(v: &Value, ctx: &str) {
    match v {
        Value::Object(map) => {
            // Mirror slim's choice-field rule exactly: it strips `label` only when
            // `value` is present and `label` is a string. A non-string `label`
            // would not be stripped, so don't flag it here.
            assert!(
                !(map.contains_key("value") && matches!(map.get("label"), Some(Value::String(_)))),
                "{ctx}: choice field still has `value` + string `label` — slim should drop `label`"
            );
            for (k, val) in map {
                assert!(
                    !val.is_null(),
                    "{ctx}: key `{k}` is null — slim_value should strip null fields"
                );
                assert!(!is_stripped_key(k), "{ctx}: stripped key `{k}` is present");
                assert_clean(val, &format!("{ctx}.{k}"));
            }
        }
        // Arrays may legitimately contain null elements (slim preserves them),
        // so only recurse — don't assert non-null here.
        Value::Array(arr) => {
            for (i, e) in arr.iter().enumerate() {
                assert_clean(e, &format!("{ctx}[{i}]"));
            }
        }
        _ => {}
    }
}

/// Assert the pagination contract on a list response and its cleanliness:
/// `{count, has_more, next_offset, results}`, `has_more == next_offset < count`,
/// and no bare `next`/`previous` URLs.
pub fn assert_page_shape(resp: &Value, ctx: &str) {
    let obj = resp
        .as_object()
        .unwrap_or_else(|| panic!("{ctx}: not an object"));
    let count = obj["count"]
        .as_u64()
        .unwrap_or_else(|| panic!("{ctx}: missing numeric `count`"));
    let next_offset = obj["next_offset"]
        .as_u64()
        .unwrap_or_else(|| panic!("{ctx}: missing numeric `next_offset`"));
    let has_more = obj["has_more"]
        .as_bool()
        .unwrap_or_else(|| panic!("{ctx}: missing boolean `has_more`"));
    assert!(obj.contains_key("results"), "{ctx}: missing `results`");
    assert!(!obj.contains_key("next"), "{ctx}: bare `next` URL present");
    assert!(
        !obj.contains_key("previous"),
        "{ctx}: bare `previous` URL present"
    );
    assert_eq!(
        has_more,
        next_offset < count,
        "{ctx}: has_more must equal next_offset < count (count={count}, next_offset={next_offset})"
    );
    assert_clean(resp, ctx);
}

/// Assert the seed produced rows for this query. Shape-only — we never check an
/// exact count — but an empty result here almost always means an unseeded
/// instance, so fail loudly with the fix.
pub fn assert_nonempty(resp: &Value, ctx: &str) {
    assert!(
        !results(resp).is_empty(),
        "{ctx}: no results — did you seed the instance with scripts/seed_data.py?"
    );
}

/// Serialize a `CallToolResult` and parse the JSON payload out of its first
/// text content block. Used to assert against tools (like `lookup_host`) whose
/// output is a `CallToolResult` rather than a raw `Value`.
pub fn call_result_json(r: &CallToolResult) -> Value {
    let v = serde_json::to_value(r).expect("serialize CallToolResult");
    let text = v["content"][0]["text"]
        .as_str()
        .expect("CallToolResult has no text content");
    serde_json::from_str(text).expect("parse tool JSON payload")
}

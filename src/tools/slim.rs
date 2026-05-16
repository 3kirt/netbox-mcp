use serde_json::Value;

/// Fields unconditionally removed from every object in a NetBox response.
pub const STRIP_KEYS: &[&str] = &[
    // Raw per-object config context; config_context is the resolved value the AI needs.
    "local_context_data",
    // Convenience alias for primary_ip4/primary_ip6; always duplicates one of them.
    "primary_ip",
    // Web UI deep-link — not useful to an AI.
    "display_url",
    // Tree-rendering depth hint used by the NetBox UI.
    "_depth",
];

/// Fields kept when collapsing a tag object embedded in another object's `tags` array.
pub const TAG_KEEP_KEYS: &[&str] = &["id", "name", "slug"];

/// Recursively remove null-valued fields and noise keys from a JSON value.
/// Cuts typical NetBox response sizes by 50–70%.
pub fn slim_value(v: Value) -> Value {
    match v {
        Value::Object(map) => {
            let mut map: serde_json::Map<_, _> = map
                .into_iter()
                .filter(|(k, v)| !v.is_null() && !STRIP_KEYS.contains(&k.as_str()))
                .map(|(k, v)| {
                    if k == "tags" {
                        (k, slim_tag_array(v))
                    } else {
                        (k, slim_value(v))
                    }
                })
                .collect();
            // Choice-field objects carry {"value": <any>, "label": <str>} where label
            // is always just a human-readable capitalisation of value.
            if is_choice_object(&map) {
                map.remove("label");
            }
            Value::Object(map)
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(slim_value).collect()),
        other => other,
    }
}

/// Returns true when `map` matches the NetBox choice-field pattern:
/// contains both a `value` key (any type) and a `label` key (string).
fn is_choice_object(map: &serde_json::Map<String, Value>) -> bool {
    map.contains_key("value") && matches!(map.get("label"), Some(Value::String(_)))
}

/// Collapses a `tags` array so each element retains only {id, name, slug}.
/// The top-level tags-list endpoint uses `results`, not `tags`, so it is unaffected.
fn slim_tag_array(v: Value) -> Value {
    match v {
        Value::Array(arr) => Value::Array(
            arr.into_iter()
                .map(|tag| match tag {
                    Value::Object(map) => Value::Object(
                        map.into_iter()
                            .filter(|(k, _)| TAG_KEEP_KEYS.contains(&k.as_str()))
                            .collect(),
                    ),
                    other => other,
                })
                .collect(),
        ),
        other => slim_value(other),
    }
}

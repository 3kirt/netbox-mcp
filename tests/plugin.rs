//! Invariants tying the Claude Code plugin to the crate and the release
//! workflow. The plugin's launcher (`plugin/bin/netbox-mcp.sh`) downloads the
//! release binary matching `plugin.json`'s version and the host platform, so
//! these drifting apart would ship a plugin that can't find its binary.

use std::collections::BTreeSet;

use serde_json::Value;

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn read(path: &str) -> String {
    std::fs::read_to_string(format!("{ROOT}/{path}")).unwrap_or_else(|e| panic!("{path}: {e}"))
}

fn json(path: &str) -> Value {
    serde_json::from_str(&read(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

#[test]
fn plugin_version_matches_cargo() {
    // The release workflow enforces tag == both; this catches the drift
    // before a tag is ever pushed.
    assert_eq!(
        json("plugin/.claude-plugin/plugin.json")["version"],
        env!("CARGO_PKG_VERSION"),
        "plugin/.claude-plugin/plugin.json version must equal Cargo.toml's"
    );
}

#[test]
fn marketplace_lists_the_plugin_from_this_repo() {
    let market = json(".claude-plugin/marketplace.json");
    let plugins = market["plugins"].as_array().expect("plugins array");
    assert_eq!(plugins.len(), 1, "one plugin in the self-marketplace");
    assert_eq!(plugins[0]["source"], "./plugin");
    assert_eq!(
        plugins[0]["name"],
        json("plugin/.claude-plugin/plugin.json")["name"]
    );
}

#[test]
fn plugin_launches_the_bundled_bootstrap_script() {
    let config = json("plugin/.mcp.json");
    let command = config["mcpServers"]["netbox"]["command"]
        .as_str()
        .expect("netbox server command");
    let rel = command
        .strip_prefix("${CLAUDE_PLUGIN_ROOT}/")
        .expect("command is relative to the plugin root");
    let script = std::path::Path::new(ROOT).join("plugin").join(rel);
    let meta = std::fs::metadata(&script).unwrap_or_else(|e| panic!("{}: {e}", script.display()));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_ne!(
            meta.permissions().mode() & 0o111,
            0,
            "{} must be executable",
            script.display()
        );
    }
    #[cfg(not(unix))]
    let _ = meta;
}

/// The launcher maps `uname` onto `netbox-mcp-<os>-<arch>`; every name it can
/// produce must be an artifact the release matrix builds, and vice versa.
#[test]
fn launcher_platforms_match_the_release_matrix() {
    let launcher = read("plugin/bin/netbox-mcp.sh");
    let between = |text: &str, start: &str| -> BTreeSet<String> {
        let block = text.split(start).nth(1).expect(start);
        let block = block.split("esac").next().unwrap();
        block
            .lines()
            .filter_map(|l| l.split("=\"").nth(1)?.split('"').next())
            .map(str::to_string)
            .collect()
    };
    let oses = between(&launcher, "case \"$(uname -s)\" in");
    let arches = between(&launcher, "case \"$(uname -m)\" in");
    assert!(launcher.contains("asset=\"netbox-mcp-$os-$arch\""));
    let from_launcher: BTreeSet<String> = oses
        .iter()
        .flat_map(|os| arches.iter().map(move |a| format!("netbox-mcp-{os}-{a}")))
        .collect();

    let release = read(".github/workflows/release.yml");
    let from_release: BTreeSet<String> = release
        .lines()
        .filter_map(|l| l.trim().strip_prefix("artifact: "))
        .map(str::to_string)
        .collect();

    assert_eq!(
        from_launcher, from_release,
        "plugin/bin/netbox-mcp.sh platforms and release.yml artifacts differ"
    );
}

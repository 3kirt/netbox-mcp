//! `login` / `logout` / `unlock` / `status`: managing the GPG-encrypted token
//! in a `pass`-compatible store. These run in the user's terminal, so (unlike
//! the server) they may prompt. All output goes to stderr, matching the
//! server's rule that stdout is reserved.

use std::io::{BufRead, IsTerminal};
use std::path::Path;

use anyhow::{Context, anyhow, bail};

use crate::client::{NetboxClient, NetboxError};
use crate::config::{self, Config, TokenSource};
use crate::credentials::PassError;

#[derive(clap::Args)]
pub struct LoginArgs {
    /// NetBox base URL (default: NETBOX_URL, then "url" in the config file)
    #[arg(long)]
    url: Option<String>,

    /// Password-store entry to write (default: "pass_entry" in the config
    /// file, else netbox-mcp/<host>)
    #[arg(long)]
    entry: Option<String>,

    /// GPG key to encrypt to when the store has no .gpg-id yet (writes it,
    /// like `pass init`)
    #[arg(long, value_name = "KEY_ID")]
    gpg_id: Option<String>,

    /// Read the token from stdin instead of prompting
    #[arg(long)]
    token_stdin: bool,
}

#[derive(clap::Args)]
pub struct EntryArgs {
    /// Password-store entry (default: "pass_entry" in the config file)
    #[arg(long)]
    entry: Option<String>,
}

/// Verify a token, encrypt it into the store, and point the config at it.
pub async fn login(config_path: Option<&Path>, args: LoginArgs) -> anyhow::Result<()> {
    let cfg = Config::load_or_empty(config_path)?;
    let url_from_env = args.url.is_none() && config::env_var("NETBOX_URL").is_some();
    let url = match args.url {
        Some(u) => {
            config::enforce_https(&u)?;
            u
        }
        None => cfg.resolve_url()?,
    };
    let entry_name = choose_entry_name(args.entry, &cfg, &url)?;
    let store = cfg.pass_store()?;
    let entry = store.entry(&entry_name)?;

    // Settle every local precondition before prompting for the secret or
    // calling NetBox, so a missing .gpg-id doesn't waste a token entry.
    let (recipients, init_id) = match (entry.recipients()?, args.gpg_id) {
        (Some(_), Some(_)) => bail!(
            "the password store at {} already has a .gpg-id; drop --gpg-id",
            store.root().display()
        ),
        (Some(ids), None) => (ids, None),
        (None, Some(id)) => {
            let id = id.trim().to_string();
            if id.is_empty() {
                bail!("--gpg-id is empty");
            }
            (vec![id.clone()], Some(id))
        }
        (None, None) => bail!(
            "no .gpg-id in the password store at {}; pass --gpg-id <KEY_ID> to choose \
             the GPG key to encrypt to (see `gpg --list-secret-keys`)",
            store.root().display()
        ),
    };

    let token = read_token_input(args.token_stdin)?;
    let username = verify_token(&url, &token).await?;

    entry.write_token(&token, &recipients).await?;
    // Only now, with encryption to that key proven, record it as the store's
    // recipient: a mistyped --gpg-id must not leave a .gpg-id behind that
    // blocks every later login.
    if let Some(id) = &init_id {
        store.init(id)?;
        eprintln!(
            "Initialised password store at {} for {id}",
            store.root().display()
        );
    }

    let path = config::config_path(config_path)?;
    let previous_url = cfg.file_url().map(str::to_owned);
    let mut removed_plaintext = false;
    config::update_file(&path, |obj| {
        obj.insert("url".into(), url.clone().into());
        obj.insert("pass_entry".into(), entry_name.clone().into());
        // Pin the store and keyring login used, so a server launched without
        // this shell's PASSWORD_STORE_DIR / GNUPGHOME reads the same ones.
        obj.insert(
            "password_store_dir".into(),
            store.root().display().to_string().into(),
        );
        if let Some(home) = store.homedir() {
            obj.insert("gnupg_home".into(), home.display().to_string().into());
        }
        removed_plaintext = obj.remove("token").is_some();
    })?;

    eprintln!(
        "Logged in to {url} as {username}. Token encrypted to {} at {}.",
        recipients.join(", "),
        entry.path().display()
    );
    eprintln!(
        "Updated {} (\"pass_entry\": \"{entry_name}\").",
        path.display()
    );
    if let Some(prev) = previous_url.filter(|p| !same_instance(p, &url)) {
        let why = if url_from_env {
            " (from NETBOX_URL)"
        } else {
            ""
        };
        eprintln!("Changed \"url\" from {prev} to {url}{why}.");
    }
    if removed_plaintext {
        eprintln!("Removed the plaintext \"token\" from the config file.");
    }
    note_env_token_precedence();
    Ok(())
}

/// The entry to write: `--entry`, else the config's `pass_entry` *if* the
/// config is for this same NetBox instance, else `netbox-mcp/<host>`.
/// Reusing another host's entry would overwrite that host's token.
fn choose_entry_name(explicit: Option<String>, cfg: &Config, url: &str) -> anyhow::Result<String> {
    if let Some(name) = explicit {
        return Ok(name);
    }
    match (cfg.pass_entry(), cfg.file_url()) {
        (Some(name), Some(file_url)) if same_instance(file_url, url) => Ok(name.to_owned()),
        _ => default_entry_name(url),
    }
}

fn same_instance(a: &str, b: &str) -> bool {
    a.trim().trim_end_matches('/') == b.trim().trim_end_matches('/')
}

fn note_env_token_precedence() {
    if config::env_var("NETBOX_TOKEN").is_some() {
        eprintln!(
            "Note: NETBOX_TOKEN is set in this shell and takes precedence over the pass \
             entry for servers started from it."
        );
    }
}

/// Delete the stored token. The config's `pass_entry` is left in place, so
/// the server then reports "run `netbox-mcp login`" rather than "not set".
pub fn logout(config_path: Option<&Path>, args: &EntryArgs) -> anyhow::Result<()> {
    let cfg = Config::load(config_path)?;
    let name = args
        .entry
        .as_deref()
        .or_else(|| cfg.pass_entry())
        .context("no pass entry: pass --entry or set \"pass_entry\" in the config file")?;
    let entry = cfg.pass_store()?.entry(name)?;
    entry.remove()?;
    eprintln!("Removed {}.", entry.path().display());
    Ok(())
}

/// Get the token's passphrase into gpg-agent's cache, prompting if needed.
pub async fn unlock(config_path: Option<&Path>) -> anyhow::Result<()> {
    // The configured entry, not `resolve_token()`: NETBOX_TOKEN in this shell
    // says nothing about the (GUI-launched) server asking to be unlocked.
    let Some(entry) = Config::load(config_path)?.configured_pass_entry()? else {
        bail!("no \"pass_entry\" in the config file, so there is nothing to unlock");
    };
    note_env_token_precedence();
    match entry.read_token().await {
        Ok(_) => {
            eprintln!("Already unlocked.");
            return Ok(());
        }
        Err(PassError::Locked { .. }) => {}
        Err(e) => return Err(e.into()),
    }
    entry.unlock_interactive().await?;
    // Confirm the way the server will read it: non-interactively.
    entry.read_token().await?;
    eprintln!(
        "Unlocked. The server can now read the token until gpg-agent's cache expires \
         (default-cache-ttl / max-cache-ttl in gpg-agent.conf); a running server keeps \
         it in memory after its first successful read."
    );
    Ok(())
}

/// Report where the token comes from and, for pass, whether it's unlocked —
/// without decrypting it.
pub async fn status(config_path: Option<&Path>) -> anyhow::Result<()> {
    let cfg = Config::load(config_path)?;
    match cfg.resolve_url() {
        Ok(url) => eprintln!("NetBox URL:   {url}"),
        Err(e) => eprintln!("NetBox URL:   error: {e}"),
    }
    let source = cfg.resolve_token();
    match &source {
        Ok(s) => eprintln!("Token source: {}", s.describe()),
        Err(e) => eprintln!("Token source: error: {e}"),
    }
    // Report the configured entry even when NETBOX_TOKEN shadows it here: a
    // server started without that variable still uses it.
    let entry = match cfg.configured_pass_entry() {
        Ok(Some(entry)) => entry,
        Ok(None) => return Ok(()),
        Err(e) => {
            eprintln!("Pass entry:   error: {e}");
            return Ok(());
        }
    };
    if !matches!(source, Ok(TokenSource::Pass(_))) {
        eprintln!(
            "Pass entry:   {} ({})",
            entry.name(),
            entry.path().display()
        );
    }
    if matches!(source, Ok(TokenSource::Env(_))) {
        eprintln!(
            "Note:         NETBOX_TOKEN in this shell takes precedence; servers started \
             without it use the pass entry."
        );
    }
    if !entry.path().exists() {
        eprintln!("Entry:        missing (run `netbox-mcp login`)");
        return Ok(());
    }
    let recipients = match entry.recipients() {
        Ok(Some(r)) => r,
        Ok(None) => {
            eprintln!("Recipients:   unknown (no .gpg-id in the store)");
            return Ok(());
        }
        Err(e) => {
            eprintln!("Recipients:   error: {e}");
            return Ok(());
        }
    };
    eprintln!("Recipients:   {}", recipients.join(", "));
    eprintln!(
        "gpg-agent:    {}",
        agent_state(entry.store(), &recipients).await
    );
    Ok(())
}

#[cfg(unix)]
async fn agent_state(store: &crate::credentials::PassStore, recipients: &[String]) -> String {
    use crate::credentials::agent;
    let result = async {
        let grips = agent::encryption_keygrips(store, recipients).await?;
        if grips.is_empty() {
            return Ok(agent::AgentState::NoSecretKey);
        }
        let socket = agent::agent_socket(store).await?;
        Ok::<_, String>(
            agent::query_socket(&socket, &grips)
                .await?
                .map_or(agent::AgentState::NotRunning, |infos| {
                    agent::summarize(&infos)
                }),
        )
    }
    .await;
    result.map_or_else(|e| format!("unknown ({e})"), |s| s.to_string())
}

#[cfg(not(unix))]
async fn agent_state(_: &crate::credentials::PassStore, _: &[String]) -> String {
    "unknown (gpg-agent query is only supported on Unix)".into()
}

/// `netbox-mcp/<host>` (with `_<port>` for a non-default port).
fn default_entry_name(url: &str) -> anyhow::Result<String> {
    let parsed = url::Url::parse(url).with_context(|| format!("invalid NetBox URL: {url}"))?;
    let host = parsed
        .host_str()
        .with_context(|| format!("NetBox URL has no host: {url}"))?
        .trim_matches(['[', ']'])
        .replace(':', "_");
    Ok(parsed.port().map_or_else(
        || format!("netbox-mcp/{host}"),
        |port| format!("netbox-mcp/{host}_{port}"),
    ))
}

fn read_token_input(from_stdin: bool) -> anyhow::Result<String> {
    let token = if from_stdin {
        let mut line = String::new();
        std::io::stdin().lock().read_line(&mut line)?;
        line
    } else if std::io::stdin().is_terminal() {
        rpassword::prompt_password("NetBox API token: ")?
    } else {
        bail!("stdin is not a terminal; pass --token-stdin to read the token from it");
    };
    let token = token.trim().to_string();
    if token.is_empty() {
        bail!("empty token");
    }
    Ok(token)
}

/// Check `token` against NetBox and return the username it belongs to.
/// `/api/authentication-check/` (NetBox 4.x) names the user; on an instance
/// without it (404), `/api/status/` still rejects a bad token with 403.
async fn verify_token(url: &str, token: &str) -> anyhow::Result<String> {
    let client = NetboxClient::new(url, token)?;
    let rejected = |e: &NetboxError| anyhow!("NetBox rejected the token: {}", e.to_tool_message());
    match client.list("/api/authentication-check/", &[]).await {
        Ok(user) => Ok(user["username"].as_str().unwrap_or("<unknown>").to_string()),
        Err(NetboxError::Api { status, .. }) if status == reqwest::StatusCode::NOT_FOUND => {
            client
                .list("/api/status/", &[])
                .await
                .map_err(|e| rejected(&e))?;
            Ok("<unknown>".into())
        }
        Err(e) => Err(rejected(&e)),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::{choose_entry_name, default_entry_name, verify_token};
    use crate::config::Config;

    #[tokio::test]
    async fn verify_token_returns_the_username() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/authentication-check/"))
            .and(header("authorization", "Token tok-good"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"username": "admin"})))
            .mount(&server)
            .await;
        assert_eq!(
            verify_token(&server.uri(), "tok-good").await.unwrap(),
            "admin"
        );
    }

    #[tokio::test]
    async fn verify_token_falls_back_to_status_on_older_netbox() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/authentication-check/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/status/"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({"netbox-version": "3.7"})),
            )
            .expect(1)
            .mount(&server)
            .await;
        assert_eq!(
            verify_token(&server.uri(), "tok-good").await.unwrap(),
            "<unknown>"
        );
    }

    #[tokio::test]
    async fn verify_token_reports_rejection() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(403).set_body_json(json!({"detail": "Invalid v1 token"})),
            )
            .mount(&server)
            .await;
        let err = verify_token(&server.uri(), "tok-bad").await.unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("rejected") && msg.contains("Invalid v1 token"),
            "{msg}"
        );
    }

    fn cfg(json: &str) -> (tempfile::TempDir, Config) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("c.json");
        std::fs::write(&path, json).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        let cfg = Config::load(Some(&path)).unwrap();
        (dir, cfg)
    }

    #[test]
    fn existing_entry_is_reused_only_for_the_same_instance() {
        let (_d, c) = cfg(r#"{"url":"https://netbox.example.com/","pass_entry":"mine/nb"}"#);
        assert_eq!(
            choose_entry_name(None, &c, "https://netbox.example.com").unwrap(),
            "mine/nb"
        );
        // A different host must not overwrite netbox.example.com's token.
        assert_eq!(
            choose_entry_name(None, &c, "https://netbox.work.com").unwrap(),
            "netbox-mcp/netbox.work.com"
        );
        assert_eq!(
            choose_entry_name(Some("x/y".into()), &c, "https://netbox.work.com").unwrap(),
            "x/y"
        );
    }

    #[test]
    fn default_entry_name_uses_host_and_port() {
        assert_eq!(
            default_entry_name("https://netbox.example.com").unwrap(),
            "netbox-mcp/netbox.example.com"
        );
        assert_eq!(
            default_entry_name("https://netbox.example.com:8443/sub").unwrap(),
            "netbox-mcp/netbox.example.com_8443"
        );
        assert_eq!(
            default_entry_name("http://[::1]:8080").unwrap(),
            "netbox-mcp/__1_8080"
        );
    }
}

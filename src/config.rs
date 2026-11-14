use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow, bail};
use serde::Deserialize;

use crate::credentials::{PassEntry, PassStore, StoreLocation};

/// Raw config as loaded from the JSON file. Deliberately not `Debug`: it holds
/// the token.
#[derive(Default, Deserialize)]
struct RawConfig {
    url: Option<String>,
    token: Option<String>,
    /// Name of a `pass`-compatible store entry holding the GPG-encrypted token.
    pass_entry: Option<String>,
    /// Path to the gpg binary (default: `gpg` from PATH).
    gpg: Option<String>,
    /// Password-store root (default: `$PASSWORD_STORE_DIR`, else `~/.password-store`).
    password_store_dir: Option<String>,
    /// GnuPG home (default: `$GNUPGHOME`, else gpg's own default).
    gnupg_home: Option<String>,
}

/// Resolved NetBox connection settings.
#[derive(Clone)]
#[expect(
    clippy::struct_field_names,
    reason = "`file_` marks values read from the config file, as opposed to the env vars that override them"
)]
pub struct Config {
    file_url: Option<String>,
    file_token: Option<String>,
    file_pass_entry: Option<String>,
    file_gpg: Option<String>,
    file_password_store_dir: Option<String>,
    file_gnupg_home: Option<String>,
}

/// Hand-written so a stray `?config` in a log line can never print the token.
impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("file_url", &self.file_url)
            .field(
                "file_token",
                &self.file_token.as_ref().map(|_| "<redacted>"),
            )
            .field("file_pass_entry", &self.file_pass_entry)
            .field("file_gpg", &self.file_gpg)
            .field("file_password_store_dir", &self.file_password_store_dir)
            .field("file_gnupg_home", &self.file_gnupg_home)
            .finish()
    }
}

/// Where the NetBox token comes from.
pub enum TokenSource {
    /// `NETBOX_TOKEN` environment variable.
    Env(String),
    /// Plaintext `token` in the config file.
    File(String),
    /// GPG-encrypted `pass` entry, decrypted lazily on first use.
    Pass(PassEntry),
}

impl TokenSource {
    /// Human description, never including the secret.
    pub fn describe(&self) -> String {
        match self {
            Self::Env(_) => "NETBOX_TOKEN environment variable".into(),
            Self::File(_) => "plaintext \"token\" in the config file".into(),
            Self::Pass(e) => format!("pass entry {} ({})", e.name(), e.path().display()),
        }
    }
}

impl Config {
    /// Load configuration from path (default: `~/.netbox_mcp.json`).
    ///
    /// A missing *default* file is not an error (env vars may supply
    /// everything), but a path given explicitly via `--config` must exist —
    /// otherwise a typo would surface later as a misleading "URL not set".
    pub fn load(path: Option<&Path>) -> anyhow::Result<Self> {
        let explicit = path.is_some();
        let resolved = config_path(path)?;

        // `try_exists` (unlike `exists`) surfaces I/O errors such as a
        // permission-denied parent directory instead of reporting "absent".
        let present = resolved
            .try_exists()
            .with_context(|| format!("checking config file {}", resolved.display()))?;
        if !present {
            if explicit {
                bail!("config file {} does not exist", resolved.display());
            }
            return Ok(Self::from_raw(RawConfig::default()));
        }

        check_file_permissions(&resolved)?;

        let contents = std::fs::read_to_string(&resolved)
            .with_context(|| format!("reading config file {}", resolved.display()))?;
        let raw: RawConfig = serde_json::from_str(&contents)
            .with_context(|| format!("parsing config file {}", resolved.display()))?;

        Ok(Self::from_raw(raw))
    }

    /// Like [`Self::load`], but a missing file — even an explicit `--config`
    /// path — is an empty config: for `netbox-mcp login`, which creates it.
    pub fn load_or_empty(path: Option<&Path>) -> anyhow::Result<Self> {
        let resolved = config_path(path)?;
        if resolved
            .try_exists()
            .with_context(|| format!("checking config file {}", resolved.display()))?
        {
            Self::load(Some(&resolved))
        } else {
            Ok(Self::from_raw(RawConfig::default()))
        }
    }

    fn from_raw(raw: RawConfig) -> Self {
        Self {
            file_url: raw.url,
            file_token: raw.token,
            file_pass_entry: non_blank(raw.pass_entry),
            file_gpg: non_blank(raw.gpg),
            file_password_store_dir: non_blank(raw.password_store_dir),
            file_gnupg_home: non_blank(raw.gnupg_home),
        }
    }

    /// The config file's own `url` (ignoring `NETBOX_URL`).
    pub fn file_url(&self) -> Option<&str> {
        self.file_url.as_deref().filter(|u| !is_unset_value(u))
    }

    /// Resolve the NetBox base URL: NETBOX_URL env var, then config file, then error.
    pub fn resolve_url(&self) -> anyhow::Result<String> {
        let url =
            first_non_empty(env_var("NETBOX_URL"), self.file_url.clone()).ok_or_else(|| {
                anyhow!("NetBox URL not set: provide NETBOX_URL or set \"url\" in config file")
            })?;
        enforce_https(&url)?;
        Ok(url)
    }

    /// Resolve the token source: NETBOX_TOKEN env var, then the config file's
    /// plaintext `token` or its `pass_entry` (setting both is an error, since
    /// either silently shadowing the other would be surprising), then error.
    pub fn resolve_token(&self) -> anyhow::Result<TokenSource> {
        if let Some(t) = env_var("NETBOX_TOKEN") {
            return Ok(TokenSource::Env(t));
        }
        match (non_blank(self.file_token.clone()), &self.file_pass_entry) {
            (Some(_), Some(_)) => bail!(
                "config file sets both \"token\" and \"pass_entry\"; remove the plaintext \
                 \"token\" (netbox-mcp login does this for you)"
            ),
            (Some(t), None) => Ok(TokenSource::File(t)),
            (None, Some(name)) => Ok(TokenSource::Pass(self.pass_store()?.entry(name)?)),
            (None, None) => bail!(
                "NetBox token not set: provide NETBOX_TOKEN, set \"token\" in config file, \
                 or run `netbox-mcp login` to store it GPG-encrypted"
            ),
        }
    }

    pub fn pass_entry(&self) -> Option<&str> {
        self.file_pass_entry.as_deref()
    }

    /// The config's `pass_entry` as an entry, regardless of `NETBOX_TOKEN`.
    /// `unlock`/`status` use this: a shell exporting NETBOX_TOKEN must not
    /// hide the entry a GUI-launched server, started without that variable,
    /// is trying to decrypt.
    pub fn configured_pass_entry(&self) -> anyhow::Result<Option<PassEntry>> {
        self.file_pass_entry
            .as_deref()
            .map(|name| Ok(self.pass_store()?.entry(name)?))
            .transpose()
    }

    /// The password store, honoring the config's `gpg`, `password_store_dir`
    /// and `gnupg_home` over the process environment.
    pub fn pass_store(&self) -> anyhow::Result<PassStore> {
        PassStore::resolve(&StoreLocation {
            gpg: self.file_gpg.as_deref(),
            store_dir: self.file_password_store_dir.as_deref(),
            gnupg_home: self.file_gnupg_home.as_deref(),
        })
    }
}

/// Apply `edit` to the JSON object in the config file at `path` (created if
/// absent) and write it back atomically with owner-only permissions. A
/// symlinked config is updated at its target rather than replaced.
pub fn update_file(
    path: &Path,
    edit: impl FnOnce(&mut serde_json::Map<String, serde_json::Value>),
) -> anyhow::Result<()> {
    let target = if path
        .try_exists()
        .with_context(|| format!("checking config file {}", path.display()))?
    {
        check_file_permissions(path)?;
        std::fs::canonicalize(path).with_context(|| format!("resolving {}", path.display()))?
    } else {
        path.to_path_buf()
    };
    let mut obj = if target.exists() {
        let contents = std::fs::read_to_string(&target)
            .with_context(|| format!("reading config file {}", target.display()))?;
        match serde_json::from_str(&contents)
            .with_context(|| format!("parsing config file {}", target.display()))?
        {
            serde_json::Value::Object(m) => m,
            _ => bail!("config file {} is not a JSON object", target.display()),
        }
    } else {
        serde_json::Map::new()
    };
    edit(&mut obj);

    let mut tmp_name = target.clone().into_os_string();
    tmp_name.push(format!(".tmp{}", std::process::id()));
    let tmp = PathBuf::from(tmp_name);
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    let write = || -> anyhow::Result<()> {
        let mut file = opts.open(&tmp)?;
        let mut body = serde_json::to_string_pretty(&serde_json::Value::Object(obj))?;
        body.push('\n');
        std::io::Write::write_all(&mut file, body.as_bytes())?;
        file.sync_all()?;
        std::fs::rename(&tmp, &target)?;
        Ok(())
    };
    write()
        .inspect_err(|_| {
            let _ = std::fs::remove_file(&tmp);
        })
        .with_context(|| format!("writing config file {}", target.display()))
}

/// The first value that is actually set wins (see [`non_blank`]).
fn first_non_empty(env_val: Option<String>, file_val: Option<String>) -> Option<String> {
    non_blank(env_val).or_else(|| non_blank(file_val))
}

/// The one "is this value set?" rule for every env var and config value:
/// blank counts as unset, and so does an unexpanded `${VAR}` placeholder.
/// MCP clients substitute `${NETBOX_TOKEN}` in a shared config from the
/// user's environment, and when the variable isn't exported Claude Code
/// passes the placeholder through *literally*. Taking that as a real token
/// would shadow a `pass_entry` and fail every call with a 403; treating it
/// as unset falls through to the config file, as an empty value does.
fn non_blank(v: Option<String>) -> Option<String> {
    v.filter(|s| !is_unset_value(s))
}

fn is_unset_value(s: &str) -> bool {
    let t = s.trim();
    t.is_empty() || is_unexpanded_placeholder(t)
}

/// `${NAME}` or `${NAME:-…}` exactly, as left behind by a client that didn't
/// substitute it. No real token or URL has this shape.
fn is_unexpanded_placeholder(t: &str) -> bool {
    t.strip_prefix("${")
        .and_then(|rest| rest.strip_suffix('}'))
        .is_some_and(|inner| {
            let name = inner.split_once(":-").map_or(inner, |(n, _)| n);
            !name.is_empty() && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
        })
}

/// An environment variable, treating blank values and unexpanded `${…}`
/// placeholders as unset.
pub fn env_var(name: &str) -> Option<String> {
    let raw = std::env::var(name).ok()?;
    if is_unexpanded_placeholder(raw.trim()) {
        tracing::warn!(
            variable = name,
            "ignoring unexpanded placeholder (the MCP client did not substitute it; \
             the variable is not set in its environment)"
        );
        return None;
    }
    non_blank(Some(raw))
}

/// The config file path: `path` if given, else `~/.netbox_mcp.json`.
pub fn config_path(path: Option<&Path>) -> anyhow::Result<PathBuf> {
    match path {
        Some(p) => Ok(p.to_path_buf()),
        None => Ok(home_dir()?.join(".netbox_mcp.json")),
    }
}

pub fn home_dir() -> anyhow::Result<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .context("cannot determine home directory")?;
    Ok(PathBuf::from(home))
}

/// Reject world-readable config files on Unix to avoid token exposure.
#[cfg_attr(not(unix), expect(unused_variables))]
fn check_file_permissions(path: &Path) -> anyhow::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let meta = std::fs::metadata(path)
            .with_context(|| format!("checking permissions of {}", path.display()))?;
        if meta.permissions().mode() & 0o004 != 0 {
            bail!(
                "config file {} is world-readable; run: chmod o-r {}",
                path.display(),
                path.display()
            );
        }
    }
    Ok(())
}

/// Require HTTPS, except plain HTTP to a loopback host. The URL is parsed
/// rather than prefix-matched so look-alike hosts (`http://localhost.evil.com`)
/// and userinfo tricks (`http://127.0.0.1@evil.com`) are judged by their real
/// host.
pub fn enforce_https(url: &str) -> anyhow::Result<()> {
    let parsed = url::Url::parse(url).with_context(|| format!("invalid NetBox URL: {url}"))?;
    let is_loopback = match parsed.host() {
        Some(url::Host::Domain(d)) => d == "localhost",
        Some(url::Host::Ipv4(ip)) => ip.is_loopback(),
        Some(url::Host::Ipv6(ip)) => ip.is_loopback(),
        None => false,
    };
    match parsed.scheme() {
        "https" => Ok(()),
        "http" if is_loopback => Ok(()),
        _ => bail!(
            "NetBox URL must use HTTPS, got: {url}  \
             (use https:// to prevent token from being sent in plaintext)"
        ),
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;
    use crate::test_support::lock_env;

    fn write_config(dir: &Path, content: &str) -> PathBuf {
        let path = dir.join("config.json");
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(content.as_bytes()).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        path
    }

    fn raw(token: Option<&str>, pass_entry: Option<&str>) -> Config {
        Config::from_raw(RawConfig {
            url: Some("https://nb.example.com".into()),
            token: token.map(Into::into),
            pass_entry: pass_entry.map(Into::into),
            ..RawConfig::default()
        })
    }

    /// Run `f` with NETBOX_TOKEN removed, restoring it afterwards. Callers
    /// must already hold `lock_env()`.
    fn without_env_token<T>(f: impl FnOnce() -> T) -> T {
        let saved = std::env::var_os("NETBOX_TOKEN");
        // SAFETY: the caller holds ENV_LOCK, which serializes env-touching tests.
        unsafe {
            std::env::remove_var("NETBOX_TOKEN");
        }
        let out = f();
        // SAFETY: still holding ENV_LOCK (see above); restores the variable.
        unsafe {
            if let Some(v) = saved {
                std::env::set_var("NETBOX_TOKEN", v);
            }
        }
        out
    }

    #[test]
    fn missing_default_file_returns_empty() {
        let _guard = lock_env();
        let home = tempfile::tempdir().unwrap();
        let saved = std::env::var_os("HOME");
        // SAFETY: ENV_LOCK serializes all env-touching tests in this crate
        unsafe {
            std::env::set_var("HOME", home.path());
        }
        let cfg = Config::load(None);
        // SAFETY: still holding ENV_LOCK (see above); restores the variable.
        unsafe {
            match saved {
                Some(h) => std::env::set_var("HOME", h),
                None => std::env::remove_var("HOME"),
            }
        }
        let cfg = cfg.unwrap();
        assert!(cfg.file_url.is_none());
        assert!(cfg.file_token.is_none());
    }

    #[test]
    fn parses_valid_json() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_config(
            dir.path(),
            r#"{"url":"https://nb.example.com","token":"abc"}"#,
        );
        let cfg = Config::load(Some(&path)).unwrap();
        assert_eq!(cfg.file_url.as_deref(), Some("https://nb.example.com"));
        assert_eq!(cfg.file_token.as_deref(), Some("abc"));
    }

    #[test]
    fn rejects_malformed_json() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_config(dir.path(), "not json");
        assert!(Config::load(Some(&path)).is_err());
    }

    #[test]
    fn env_url_overrides_file() {
        let _guard = lock_env();
        let dir = tempfile::tempdir().unwrap();
        let path = write_config(
            dir.path(),
            r#"{"url":"https://file.example.com","token":"t"}"#,
        );
        let cfg = Config::load(Some(&path)).unwrap();
        let saved = std::env::var_os("NETBOX_URL");
        // SAFETY: ENV_LOCK serializes all env-touching tests in this crate
        unsafe {
            std::env::set_var("NETBOX_URL", "https://env.example.com");
        }
        let url = cfg.resolve_url().unwrap();
        // SAFETY: still holding ENV_LOCK (see above); restores the variable.
        unsafe {
            match saved {
                Some(v) => std::env::set_var("NETBOX_URL", v),
                None => std::env::remove_var("NETBOX_URL"),
            }
        }
        assert_eq!(url, "https://env.example.com");
    }

    #[test]
    fn env_token_overrides_file() {
        let _guard = lock_env();
        let dir = tempfile::tempdir().unwrap();
        let path = write_config(
            dir.path(),
            r#"{"url":"https://nb.example.com","token":"file-token"}"#,
        );
        let cfg = Config::load(Some(&path)).unwrap();
        let saved = std::env::var_os("NETBOX_TOKEN");
        // SAFETY: ENV_LOCK serializes all env-touching tests in this crate
        unsafe {
            std::env::set_var("NETBOX_TOKEN", "env-token");
        }
        let source = cfg.resolve_token();
        // SAFETY: still holding ENV_LOCK (see above); restores the variable.
        unsafe {
            match saved {
                Some(v) => std::env::set_var("NETBOX_TOKEN", v),
                None => std::env::remove_var("NETBOX_TOKEN"),
            }
        }
        assert!(
            matches!(source.unwrap(), TokenSource::Env(t) if t == "env-token"),
            "expected the env token"
        );
    }

    #[test]
    fn rejects_http_url() {
        let _guard = lock_env();
        let saved = std::env::var_os("NETBOX_URL");
        // SAFETY: ENV_LOCK serializes all env-touching tests in this crate
        unsafe {
            std::env::remove_var("NETBOX_URL");
        }
        let dir = tempfile::tempdir().unwrap();
        let path = write_config(dir.path(), r#"{"url":"http://nb.example.com","token":"t"}"#);
        let cfg = Config::load(Some(&path)).unwrap();
        let result = cfg.resolve_url();
        // SAFETY: still holding ENV_LOCK (see above); restores the variable.
        unsafe {
            if let Some(v) = saved {
                std::env::set_var("NETBOX_URL", v);
            }
        }
        assert!(result.is_err());
    }

    #[test]
    fn missing_url_is_error() {
        let _guard = lock_env();
        let cfg = Config {
            file_url: None,
            ..raw(Some("t"), None)
        };
        let saved = std::env::var_os("NETBOX_URL");
        // SAFETY: ENV_LOCK serializes all env-touching tests in this crate
        unsafe {
            std::env::remove_var("NETBOX_URL");
        }
        let result = cfg.resolve_url();
        // SAFETY: still holding ENV_LOCK (see above); restores the variable.
        unsafe {
            if let Some(v) = saved {
                std::env::set_var("NETBOX_URL", v);
            }
        }
        assert!(result.is_err());
    }

    #[test]
    fn missing_token_mentions_login() {
        let _guard = lock_env();
        let err = without_env_token(|| raw(None, None).resolve_token())
            .err()
            .unwrap();
        assert!(err.to_string().contains("netbox-mcp login"), "{err}");
    }

    #[test]
    fn pass_entry_resolves_to_pass_source() {
        let _guard = lock_env();
        let src =
            without_env_token(|| raw(None, Some("netbox-mcp/nb.example.com")).resolve_token())
                .unwrap();
        let TokenSource::Pass(entry) = src else {
            panic!("expected pass source");
        };
        assert_eq!(entry.name(), "netbox-mcp/nb.example.com");
    }

    #[test]
    fn token_and_pass_entry_together_is_an_error() {
        let _guard = lock_env();
        let err = without_env_token(|| raw(Some("tok-x"), Some("e")).resolve_token())
            .err()
            .unwrap();
        assert!(err.to_string().contains("both"), "{err}");
    }

    #[test]
    fn placeholder_in_config_file_counts_as_unset() {
        let _guard = lock_env();
        let err = without_env_token(|| raw(Some("${NETBOX_TOKEN}"), None).resolve_token())
            .err()
            .unwrap();
        assert!(err.to_string().contains("not set"), "{err}");
    }

    #[test]
    fn placeholder_env_token_falls_through_to_pass_entry() {
        // What Claude Code passes for `"NETBOX_TOKEN": "${NETBOX_TOKEN}"` when
        // the variable is unset: the literal placeholder must not shadow the
        // configured pass entry.
        let _guard = lock_env();
        let saved = std::env::var_os("NETBOX_TOKEN");
        // SAFETY: ENV_LOCK serializes all env-touching tests in this crate
        unsafe {
            std::env::set_var("NETBOX_TOKEN", "${NETBOX_TOKEN}");
        }
        let src = raw(None, Some("netbox-mcp/nb")).resolve_token();
        // SAFETY: still holding ENV_LOCK (see above); restores the variable.
        unsafe {
            match saved {
                Some(v) => std::env::set_var("NETBOX_TOKEN", v),
                None => std::env::remove_var("NETBOX_TOKEN"),
            }
        }
        assert!(matches!(src.unwrap(), TokenSource::Pass(_)));
    }

    #[test]
    fn explicit_missing_config_path_is_an_error() {
        let err = Config::load(Some(Path::new("/nonexistent/netbox_mcp.json"))).unwrap_err();
        assert!(err.to_string().contains("does not exist"), "{err}");
    }

    #[test]
    fn load_or_empty_tolerates_a_missing_explicit_path() {
        let cfg = Config::load_or_empty(Some(Path::new("/nonexistent/netbox_mcp.json"))).unwrap();
        assert!(cfg.file_url().is_none());
        assert!(cfg.pass_entry().is_none());
    }

    #[test]
    fn debug_redacts_token() {
        let cfg = raw(Some("nb-secret-token"), None);
        let shown = format!("{cfg:?}");
        assert!(!shown.contains("nb-secret-token"), "{shown}");
        assert!(shown.contains("<redacted>"), "{shown}");
    }

    #[test]
    fn empty_env_value_falls_back_to_file_value() {
        let s = |v: &str| Some(v.to_string());
        assert_eq!(first_non_empty(s("env"), s("file")), s("env"));
        assert_eq!(first_non_empty(s(""), s("file")), s("file"));
        assert_eq!(first_non_empty(s("  "), s("file")), s("file"));
        // What Claude Code passes for `"${NETBOX_TOKEN}"` when it's unset.
        assert_eq!(first_non_empty(s("${NETBOX_TOKEN}"), s("file")), s("file"));
        assert_eq!(first_non_empty(None, s("file")), s("file"));
        assert_eq!(first_non_empty(s(""), s("")), None);
        assert_eq!(first_non_empty(None, None), None);
    }

    #[test]
    // `${X:-}` etc. are shell-style placeholders under test, not format args.
    #[allow(clippy::literal_string_with_formatting_args)]
    fn unexpanded_placeholders_are_recognised() {
        for p in [
            "${NETBOX_TOKEN}",
            "${NETBOX_URL:-https://netbox.example.com}",
            "${A_1}",
            "${X:-}",
        ] {
            assert!(is_unexpanded_placeholder(p), "{p}");
        }
        for v in [
            "0123456789abcdef",
            "https://netbox.example.com",
            "${}",
            "$NETBOX_TOKEN",
            "${NETBOX_TOKEN",
            "x${NETBOX_TOKEN}",
            "${not valid}",
        ] {
            assert!(!is_unexpanded_placeholder(v), "{v}");
        }
    }

    #[test]
    fn update_file_edits_atomically_and_privately() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_config(
            dir.path(),
            r#"{"url":"https://a","token":"tok-x","extra":1}"#,
        );
        update_file(&path, |o| {
            o.remove("token");
            o.insert("pass_entry".into(), "e".into());
        })
        .unwrap();
        let v: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(
            v,
            serde_json::json!({"url":"https://a","extra":1,"pass_entry":"e"})
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&path).unwrap().permissions().mode();
            assert_eq!(mode & 0o077, 0, "{mode:o}");
        }
        // Only the config file remains: no stray temp file.
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[test]
    fn update_file_creates_missing_config() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("new.json");
        update_file(&path, |o| {
            o.insert("url".into(), "https://a".into());
        })
        .unwrap();
        assert!(
            std::fs::read_to_string(&path)
                .unwrap()
                .contains("https://a")
        );
    }

    /// Every MCP client config the README documents must give each `${VAR}`
    /// in `env` a `:-` default. Claude Code passes a bare `${VAR}` through
    /// *literally* when the variable is unset, which would shadow a
    /// `pass_entry` with a bogus token (gitlab-mcp shipped exactly that bug in
    /// v0.39.0). The server ignores such values too, but the documented
    /// configs shouldn't depend on that.
    #[test]
    fn documented_mcp_configs_default_every_placeholder() {
        let root = env!("CARGO_MANIFEST_DIR");
        let readme = std::fs::read_to_string(format!("{root}/README.md")).unwrap();
        let configs: Vec<(String, serde_json::Value)> = readme
            .split("```json\n")
            .skip(1)
            .enumerate()
            .filter_map(|(i, block)| {
                let body = block.split("```").next().unwrap();
                body.contains("mcpServers").then(|| {
                    (
                        format!("README.md json block {i}"),
                        serde_json::from_str(body).unwrap(),
                    )
                })
            })
            .collect();
        assert!(
            configs.len() >= 2,
            "expected the Claude Desktop and shared .mcp.json examples"
        );

        let mut placeholders = 0;
        for (source, config) in &configs {
            let servers = config["mcpServers"].as_object().expect("mcpServers object");
            for (server, def) in servers {
                let Some(env) = def["env"].as_object() else {
                    continue;
                };
                for (key, value) in env {
                    let value = value.as_str().unwrap_or_default();
                    for placeholder in value.split("${").skip(1) {
                        placeholders += 1;
                        let inner = placeholder.split('}').next().unwrap_or_default();
                        assert!(
                            inner.contains(":-"),
                            "{source}: {server}.env.{key} uses ${{{inner}}} without a `:-` default; \
                             write ${{{inner}:-}} so an unset variable expands to empty"
                        );
                    }
                }
            }
        }
        assert!(placeholders > 0, "no ${{VAR}} placeholders found to check");
    }

    #[test]
    fn enforce_https_accepts_https() {
        assert!(enforce_https("https://nb.example.com").is_ok());
        assert!(enforce_https("https://nb.example.com/").is_ok());
    }

    #[test]
    fn enforce_https_accepts_http_localhost_and_loopback() {
        assert!(enforce_https("http://localhost").is_ok());
        assert!(enforce_https("http://localhost:8080").is_ok());
        assert!(enforce_https("http://127.0.0.1").is_ok());
        assert!(enforce_https("http://127.0.0.1:8080").is_ok());
        assert!(enforce_https("http://[::1]:8080").is_ok());
        assert!(enforce_https("HTTP://LOCALHOST:8080").is_ok());
    }

    #[test]
    fn enforce_https_rejects_lookalike_local_hosts() {
        // Prefix matching once let these through, sending the token in
        // plaintext to a non-local host.
        assert!(enforce_https("http://localhost.evil.com").is_err());
        assert!(enforce_https("http://localhostevil.com").is_err());
        assert!(enforce_https("http://127.0.0.1.evil.com").is_err());
        assert!(enforce_https("http://127.0.0.1@evil.com/").is_err());
        assert!(enforce_https("http://localhost@evil.com/").is_err());
    }

    #[test]
    fn enforce_https_rejects_http_non_local() {
        assert!(enforce_https("http://nb.example.com").is_err());
        assert!(enforce_https("http://192.168.1.1").is_err());
        assert!(enforce_https("http://10.0.0.1:8080").is_err());
    }

    #[test]
    fn enforce_https_rejects_non_http_schemes() {
        assert!(enforce_https("ftp://nb.example.com").is_err());
        assert!(enforce_https("gopher://nb.example.com").is_err());
        assert!(enforce_https("").is_err());
    }
}

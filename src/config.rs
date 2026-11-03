use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow, bail};
use serde::Deserialize;

/// Raw config as loaded from the JSON file. Deliberately not `Debug`: it holds
/// the token.
#[derive(Default, Deserialize)]
struct RawConfig {
    url: Option<String>,
    token: Option<String>,
}

/// Resolved NetBox connection settings.
#[derive(Clone)]
pub struct Config {
    file_url: Option<String>,
    file_token: Option<String>,
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
            .finish()
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
        let resolved = match path {
            Some(p) => p.to_path_buf(),
            None => default_config_path()?,
        };

        // `try_exists` (unlike `exists`) surfaces I/O errors such as a
        // permission-denied parent directory instead of reporting "absent".
        let present = resolved
            .try_exists()
            .with_context(|| format!("checking config file {}", resolved.display()))?;
        if !present {
            if explicit {
                bail!("config file {} does not exist", resolved.display());
            }
            return Ok(Self {
                file_url: None,
                file_token: None,
            });
        }

        check_file_permissions(&resolved)?;

        let contents = std::fs::read_to_string(&resolved)
            .with_context(|| format!("reading config file {}", resolved.display()))?;
        let raw: RawConfig = serde_json::from_str(&contents)
            .with_context(|| format!("parsing config file {}", resolved.display()))?;

        Ok(Self {
            file_url: raw.url,
            file_token: raw.token,
        })
    }

    /// Resolve the NetBox base URL: NETBOX_URL env var, then config file, then error.
    pub fn resolve_url(&self) -> anyhow::Result<String> {
        let url = first_non_empty(std::env::var("NETBOX_URL").ok(), self.file_url.clone())
            .ok_or_else(|| {
                anyhow!("NetBox URL not set: provide NETBOX_URL or set \"url\" in config file")
            })?;
        enforce_https(&url)?;
        Ok(url)
    }

    /// Resolve the NetBox API token: NETBOX_TOKEN env var, then config file, then error.
    pub fn resolve_token(&self) -> anyhow::Result<String> {
        first_non_empty(std::env::var("NETBOX_TOKEN").ok(), self.file_token.clone()).ok_or_else(
            || {
                anyhow!(
                    "NetBox token not set: provide NETBOX_TOKEN or set \"token\" in config file"
                )
            },
        )
    }
}

/// The first non-blank value wins. A set-but-empty env var counts as unset:
/// MCP clients expanding `${NETBOX_TOKEN}` in a shared config pass "" when the
/// user hasn't exported the variable, and falling back to the file (or failing
/// with "not set") beats starting up and sending an empty token.
fn first_non_empty(env_val: Option<String>, file_val: Option<String>) -> Option<String> {
    let non_blank = |v: Option<String>| v.filter(|s| !s.trim().is_empty());
    non_blank(env_val).or_else(|| non_blank(file_val))
}

fn default_config_path() -> anyhow::Result<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .context("cannot determine home directory")?;
    Ok(PathBuf::from(home).join(".netbox_mcp.json"))
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
fn enforce_https(url: &str) -> anyhow::Result<()> {
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
    use std::sync::{Mutex, MutexGuard};

    use super::*;

    // Cargo runs unit tests on multiple threads in one process. Tests that touch
    // NETBOX_URL/NETBOX_TOKEN must serialize, or one test's set_var leaks into
    // another's resolve_url. Hold this guard for the whole test body.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn lock_env() -> MutexGuard<'static, ()> {
        ENV_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

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

    #[test]
    fn missing_default_file_returns_empty() {
        let _guard = lock_env();
        let home = tempfile::tempdir().unwrap();
        let saved = std::env::var_os("HOME");
        // SAFETY: ENV_LOCK serializes all env-touching tests in this module
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
        // SAFETY: ENV_LOCK serializes all env-touching tests in this module
        unsafe {
            std::env::set_var("NETBOX_URL", "https://env.example.com");
        }
        let url = cfg.resolve_url().unwrap();
        // SAFETY: still holding ENV_LOCK (see above); restores the variable.
        unsafe {
            std::env::remove_var("NETBOX_URL");
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
        // SAFETY: ENV_LOCK serializes all env-touching tests in this module
        unsafe {
            std::env::set_var("NETBOX_TOKEN", "env-token");
        }
        let token = cfg.resolve_token().unwrap();
        // SAFETY: still holding ENV_LOCK (see above); restores the variable.
        unsafe {
            std::env::remove_var("NETBOX_TOKEN");
        }
        assert_eq!(token, "env-token");
    }

    #[test]
    fn rejects_http_url() {
        let _guard = lock_env();
        // SAFETY: ENV_LOCK serializes all env-touching tests in this module
        unsafe {
            std::env::remove_var("NETBOX_URL");
        }
        let dir = tempfile::tempdir().unwrap();
        let path = write_config(dir.path(), r#"{"url":"http://nb.example.com","token":"t"}"#);
        let cfg = Config::load(Some(&path)).unwrap();
        assert!(cfg.resolve_url().is_err());
    }

    #[test]
    fn missing_url_is_error() {
        let _guard = lock_env();
        let cfg = Config {
            file_url: None,
            file_token: Some("t".into()),
        };
        // SAFETY: ENV_LOCK serializes all env-touching tests in this module
        unsafe {
            std::env::remove_var("NETBOX_URL");
        }
        assert!(cfg.resolve_url().is_err());
    }

    #[test]
    fn missing_token_is_error() {
        let _guard = lock_env();
        let cfg = Config {
            file_url: Some("https://nb.example.com".into()),
            file_token: None,
        };
        // SAFETY: ENV_LOCK serializes all env-touching tests in this module
        unsafe {
            std::env::remove_var("NETBOX_TOKEN");
        }
        assert!(cfg.resolve_token().is_err());
    }

    #[test]
    fn explicit_missing_config_path_is_an_error() {
        let err = Config::load(Some(Path::new("/nonexistent/netbox_mcp.json"))).unwrap_err();
        assert!(err.to_string().contains("does not exist"), "{err}");
    }

    #[test]
    fn debug_redacts_token() {
        let cfg = Config {
            file_url: Some("https://nb.example.com".into()),
            file_token: Some("nb-secret-token".into()),
        };
        let shown = format!("{cfg:?}");
        assert!(!shown.contains("nb-secret-token"), "{shown}");
        assert!(shown.contains("<redacted>"), "{shown}");
    }

    #[test]
    fn empty_env_value_falls_back_to_file_value() {
        assert_eq!(
            first_non_empty(Some(String::new()), Some("file".into())).as_deref(),
            Some("file")
        );
        assert_eq!(
            first_non_empty(Some("  ".into()), Some("file".into())).as_deref(),
            Some("file")
        );
        assert_eq!(
            first_non_empty(Some("env".into()), Some("file".into())).as_deref(),
            Some("env")
        );
        assert_eq!(
            first_non_empty(Some(String::new()), Some(String::new())),
            None
        );
        assert_eq!(first_non_empty(None, None), None);
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

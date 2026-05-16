use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow, bail};
use serde::Deserialize;

/// Raw config as loaded from the JSON file.
#[derive(Debug, Default, Deserialize)]
struct RawConfig {
    url: Option<String>,
    token: Option<String>,
}

/// Resolved NetBox connection settings.
#[derive(Debug, Clone)]
pub struct Config {
    file_url: Option<String>,
    file_token: Option<String>,
}

impl Config {
    /// Load configuration from path (default: `~/.netbox_mcp.json`).
    /// A missing file is not an error.
    pub fn load(path: Option<&Path>) -> anyhow::Result<Self> {
        let resolved = match path {
            Some(p) => p.to_path_buf(),
            None => default_config_path()?,
        };

        if !resolved.exists() {
            return Ok(Config {
                file_url: None,
                file_token: None,
            });
        }

        check_file_permissions(&resolved)?;

        let contents = std::fs::read_to_string(&resolved)
            .with_context(|| format!("reading config file {}", resolved.display()))?;
        let raw: RawConfig = serde_json::from_str(&contents)
            .with_context(|| format!("parsing config file {}", resolved.display()))?;

        Ok(Config {
            file_url: raw.url,
            file_token: raw.token,
        })
    }

    /// Resolve the NetBox base URL: NETBOX_URL env var, then config file, then error.
    pub fn resolve_url(&self) -> anyhow::Result<String> {
        let url = std::env::var("NETBOX_URL")
            .ok()
            .or_else(|| self.file_url.clone())
            .ok_or_else(|| {
                anyhow!("NetBox URL not set: provide NETBOX_URL or set \"url\" in config file")
            })?;
        enforce_https(&url)?;
        Ok(url)
    }

    /// Resolve the NetBox API token: NETBOX_TOKEN env var, then config file, then error.
    pub fn resolve_token(&self) -> anyhow::Result<String> {
        std::env::var("NETBOX_TOKEN")
            .ok()
            .or_else(|| self.file_token.clone())
            .ok_or_else(|| {
                anyhow!(
                    "NetBox token not set: provide NETBOX_TOKEN or set \"token\" in config file"
                )
            })
    }
}

fn default_config_path() -> anyhow::Result<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .context("cannot determine home directory")?;
    Ok(PathBuf::from(home).join(".netbox_mcp.json"))
}

/// Reject world-readable config files on Unix to avoid token exposure.
#[allow(unused_variables)]
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

fn enforce_https(url: &str) -> anyhow::Result<()> {
    if url.starts_with("https://") {
        return Ok(());
    }
    let is_local = url.starts_with("http://localhost") || url.starts_with("http://127.0.0.1");
    if is_local {
        return Ok(());
    }
    bail!(
        "NetBox URL must use HTTPS, got: {}  \
         (use https:// to prevent token from being sent in plaintext)",
        url
    );
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
        ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
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
    fn missing_file_returns_empty() {
        let cfg = Config::load(Some(Path::new("/nonexistent/path.json"))).unwrap();
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

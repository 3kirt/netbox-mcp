//! GPG-encrypted token storage in a `pass`-compatible password store.
//!
//! The token lives in `<store>/<entry>.gpg`, encrypted to the recipients listed
//! in the nearest `.gpg-id` (the same layout `pass` uses, so `pass show` reads
//! it too, but `pass` itself is not required). Decryption goes through the
//! `gpg` binary with `--pinentry-mode error`, so a passphrase that gpg-agent
//! hasn't cached fails immediately instead of prompting: the server has no
//! TTY, and a pinentry nobody is watching would hang the request. Failures
//! are classified from gpg's machine-readable `--status-fd` output (numeric
//! gpg-error codes), not its localized human messages, so a locked key
//! becomes a [`PassError::Locked`] the MCP client can act on ("ask the user
//! to unlock, then retry").

#[cfg(unix)]
pub mod agent;

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use tokio::io::AsyncWriteExt;
use tokio::process::Command;

/// How long a non-interactive decrypt may take. With `--pinentry-mode error`
/// gpg never waits on a human, so this only bounds a wedged gpg/agent.
const GPG_TIMEOUT: Duration = Duration::from_secs(10);

/// gpg-error codes (the low 16 bits of the `ERROR` status line's code; the
/// high bits are the error *source*, e.g. 4 = gpg-agent). Verified against
/// GnuPG 2.4: a locked key reports `ERROR pkdecrypt_failed 67108949`, i.e.
/// source 4, code 85.
const GPG_ERR_NO_SECKEY: u32 = 17;
const GPG_ERR_NO_PIN_ENTRY: u32 = 85;
const GPG_ERR_CANCELED: u32 = 99;

/// Most stderr text forwarded in an unclassified gpg failure.
const MAX_GPG_MESSAGE: usize = 300;

#[derive(Debug, thiserror::Error)]
pub enum PassError {
    /// `cli` is the command prefix that reaches this same config (see
    /// [`PassStore::cli`]), so the advice works for a non-default `--config`.
    #[error(
        "no NetBox token stored at pass entry '{entry}' ({} does not exist); \
         run `{cli} login` to store one",
        path.display()
    )]
    NotFound {
        entry: String,
        path: PathBuf,
        cli: String,
    },
    /// `cli` and `gpg_decrypt` are ready-to-paste commands for this store's
    /// config and keyring (`--homedir`), shell-quoted.
    #[error(
        "NetBox token is locked: gpg-agent has no cached passphrase for the key that \
         decrypts {}. Ask the user to unlock it by running `{cli} unlock` (or \
         `{gpg_decrypt} >/dev/null`) in a terminal, then retry.",
        path.display()
    )]
    Locked {
        path: PathBuf,
        cli: String,
        gpg_decrypt: String,
    },
    #[error("no secret key in this GnuPG keyring can decrypt {}", path.display())]
    NoSecretKey { path: PathBuf },
    #[error("gpg did not finish decrypting {} within {}s", path.display(), timeout.as_secs())]
    Timeout { path: PathBuf, timeout: Duration },
    #[error("{0}")]
    Other(String),
}

/// A `pass`-compatible password store: a directory of `.gpg` files plus
/// `.gpg-id` recipient lists, driven through the `gpg` binary.
#[derive(Debug, Clone)]
pub struct PassStore {
    root: PathBuf,
    gpg: PathBuf,
    /// GnuPG home passed as `--homedir` to gpg/gpgconf; `None` leaves gpg's
    /// own lookup (`$GNUPGHOME`, else `~/.gnupg`).
    homedir: Option<PathBuf>,
    /// The `--config` the server was started with, when not the default, so
    /// user-facing advice (`netbox-mcp unlock`) targets the same config.
    config_path: Option<PathBuf>,
    timeout: Duration,
}

/// Where the store and keyring live, as configured. Each `None` falls back
/// to the process environment, then the default. The config values exist
/// because MCP clients launched from a GUI (e.g. from the macOS Dock) don't
/// inherit shell-only `PASSWORD_STORE_DIR` / `GNUPGHOME` exports; `login`
/// records what it used so the server reads the same store and keyring.
#[derive(Debug, Default, Clone)]
pub struct StoreLocation<'a> {
    /// gpg binary (default: `gpg` from `PATH`, which a minimal client
    /// `PATH` may lack, e.g. `/opt/homebrew/bin`).
    pub gpg: Option<&'a str>,
    /// Store root (else `$PASSWORD_STORE_DIR`, else `~/.password-store`,
    /// the same lookup `pass` does).
    pub store_dir: Option<&'a str>,
    /// GnuPG home (else `$GNUPGHOME`, else gpg's default `~/.gnupg`).
    pub gnupg_home: Option<&'a str>,
}

impl PassStore {
    pub fn resolve(loc: &StoreLocation<'_>) -> anyhow::Result<Self> {
        let root = match loc.store_dir {
            Some(dir) => expand_tilde(dir)?,
            None => match crate::config::env_var("PASSWORD_STORE_DIR") {
                Some(dir) => PathBuf::from(dir),
                None => crate::config::home_dir()?.join(".password-store"),
            },
        };
        let homedir = match loc.gnupg_home {
            Some(dir) => Some(expand_tilde(dir)?),
            None => crate::config::env_var("GNUPGHOME").map(PathBuf::from),
        };
        let gpg = loc
            .gpg
            .map_or_else(|| Ok(PathBuf::from("gpg")), expand_tilde)?;
        Ok(Self::new(root, gpg).with_homedir(homedir))
    }

    pub fn new(root: impl Into<PathBuf>, gpg: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            gpg: gpg.into(),
            homedir: None,
            config_path: None,
            timeout: GPG_TIMEOUT,
        }
    }

    #[must_use]
    pub fn with_homedir(mut self, homedir: Option<PathBuf>) -> Self {
        self.homedir = homedir;
        self
    }

    /// Record a non-default `--config` path for user-facing advice.
    #[must_use]
    pub fn with_config_path(mut self, config_path: Option<PathBuf>) -> Self {
        self.config_path = config_path;
        self
    }

    /// `netbox-mcp`, plus `--config <path>` when the store came from a
    /// non-default config, as a shell-quoted command prefix.
    pub fn cli(&self) -> String {
        self.config_path.as_ref().map_or_else(
            || "netbox-mcp".to_string(),
            |p| {
                format!(
                    "netbox-mcp --config {}",
                    shell_quote(&p.display().to_string())
                )
            },
        )
    }

    /// A paste-ready `gpg [--homedir H] --decrypt <path>` for `path`.
    fn gpg_decrypt_command(&self, path: &Path) -> String {
        let mut cmd = shell_quote(&self.gpg.display().to_string());
        if let Some(home) = &self.homedir {
            cmd.push_str(" --homedir ");
            cmd.push_str(&shell_quote(&home.display().to_string()));
        }
        cmd.push_str(" --decrypt ");
        cmd.push_str(&shell_quote(&path.display().to_string()));
        cmd
    }

    /// The GnuPG home this store pins gpg to, if any.
    pub fn homedir(&self) -> Option<&Path> {
        self.homedir.as_deref()
    }

    /// A `gpg` command with `--homedir` applied.
    pub fn gpg_command(&self) -> Command {
        let mut cmd = Command::new(&self.gpg);
        if let Some(home) = &self.homedir {
            cmd.arg("--homedir").arg(home);
        }
        cmd
    }

    /// A `gpgconf` command with `--homedir` applied (so the agent socket it
    /// reports belongs to the same keyring).
    pub fn gpgconf_command(&self) -> Command {
        let mut cmd = Command::new(self.gpgconf());
        if let Some(home) = &self.homedir {
            cmd.arg("--homedir").arg(home);
        }
        cmd
    }

    #[cfg(test)]
    pub(crate) const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// `gpgconf` from the same directory as an explicitly located `gpg`
    /// (so a `gpg` override covers both), else `gpgconf` from `PATH`.
    fn gpgconf(&self) -> PathBuf {
        match self.gpg.parent() {
            Some(dir) if !dir.as_os_str().is_empty() => dir.join("gpgconf"),
            _ => PathBuf::from("gpgconf"),
        }
    }

    /// The entry named `name` (e.g. `netbox-mcp/netbox.example.com`).
    pub fn entry(&self, name: &str) -> Result<PassEntry, PassError> {
        validate_entry_name(name)?;
        Ok(PassEntry {
            store: self.clone(),
            name: name.to_string(),
        })
    }

    /// Initialise an empty store for `gpg_id` (writes `<root>/.gpg-id`, like
    /// `pass init`). Refuses to overwrite an existing root `.gpg-id`.
    pub fn init(&self, gpg_id: &str) -> Result<(), PassError> {
        let gpg_id = gpg_id.trim();
        if gpg_id.is_empty() || gpg_id.contains('\n') {
            return Err(PassError::Other(format!("invalid GPG key ID {gpg_id:?}")));
        }
        create_private_dir(&self.root)?;
        let id_file = self.root.join(".gpg-id");
        let mut opts = std::fs::OpenOptions::new();
        opts.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            opts.mode(0o600);
        }
        let mut file = opts
            .open(&id_file)
            .map_err(|e| io_err("creating", &id_file, &e))?;
        std::io::Write::write_all(&mut file, format!("{gpg_id}\n").as_bytes())
            .map_err(|e| io_err("writing", &id_file, &e))
    }
}

/// One token entry in a [`PassStore`].
#[derive(Debug, Clone)]
pub struct PassEntry {
    store: PassStore,
    name: String,
}

impl PassEntry {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn store(&self) -> &PassStore {
        &self.store
    }

    /// `<store>/<name>.gpg`.
    pub fn path(&self) -> PathBuf {
        self.store.root.join(format!("{}.gpg", self.name))
    }

    /// Decrypt the entry without ever prompting, returning its first line
    /// (the `pass` convention: line one is the secret, the rest is metadata).
    pub async fn read_token(&self) -> Result<String, PassError> {
        let path = self.path();
        if !tokio::fs::try_exists(&path)
            .await
            .map_err(|e| io_err("checking", &path, &e))?
        {
            return Err(PassError::NotFound {
                entry: self.name.clone(),
                path,
                cli: self.store.cli(),
            });
        }

        let child = self
            .store
            .gpg_command()
            .args([
                "--batch",
                "--quiet",
                "--no-tty",
                "--pinentry-mode",
                "error",
                "--status-fd",
                "2",
                "--decrypt",
                "--",
            ])
            .arg(&path)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            // A timed-out (dropped) wait kills gpg instead of leaking it.
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| spawn_err(&self.store.gpg, &e))?;
        let output = tokio::time::timeout(self.store.timeout, child.wait_with_output())
            .await
            .map_err(|_| PassError::Timeout {
                path: path.clone(),
                timeout: self.store.timeout,
            })?
            .map_err(|e| io_err("waiting for gpg on", &path, &e))?;

        let stderr = String::from_utf8_lossy(&output.stderr);
        match classify_decrypt(output.status.success(), &stderr) {
            DecryptOutcome::Ok => {}
            DecryptOutcome::Locked => {
                return Err(PassError::Locked {
                    gpg_decrypt: self.store.gpg_decrypt_command(&path),
                    cli: self.store.cli(),
                    path,
                });
            }
            DecryptOutcome::NoSecretKey => return Err(PassError::NoSecretKey { path }),
            DecryptOutcome::Failed => {
                return Err(PassError::Other(format!(
                    "gpg failed to decrypt {}: {}",
                    path.display(),
                    human_message(&stderr)
                )));
            }
        }

        let plaintext = String::from_utf8(output.stdout).map_err(|_| {
            PassError::Other(format!("{} does not decrypt to UTF-8 text", path.display()))
        })?;
        let token = plaintext.lines().next().unwrap_or("").trim();
        if token.is_empty() {
            return Err(PassError::Other(format!(
                "{} decrypts to an empty first line; run `netbox-mcp login` to store a token",
                path.display()
            )));
        }
        Ok(token.to_string())
    }

    /// Recipients from the nearest `.gpg-id`, walking up from the entry's
    /// directory to the store root (as `pass` does). `None` when the store
    /// has no `.gpg-id` anywhere on that path.
    pub fn recipients(&self) -> Result<Option<Vec<String>>, PassError> {
        let entry_path = self.path();
        let mut dir = entry_path.parent();
        while let Some(d) = dir {
            let id_file = d.join(".gpg-id");
            if id_file.is_file() {
                let contents = std::fs::read_to_string(&id_file)
                    .map_err(|e| io_err("reading", &id_file, &e))?;
                // Like pass 1.7.3+: everything from `#` on is a comment, so
                // `KEYID # work key` names KEYID.
                let ids: Vec<String> = contents
                    .lines()
                    .map(|l| l.split('#').next().unwrap_or("").trim())
                    .filter(|l| !l.is_empty())
                    .map(str::to_string)
                    .collect();
                if ids.is_empty() {
                    return Err(PassError::Other(format!(
                        "{} lists no GPG key IDs",
                        id_file.display()
                    )));
                }
                return Ok(Some(ids));
            }
            if d == self.store.root {
                break;
            }
            dir = d.parent();
        }
        Ok(None)
    }

    /// Encrypt `token` to `recipients` and atomically (temp file + rename)
    /// replace the entry. Encryption needs only public keys, so no pinentry.
    pub async fn write_token(&self, token: &str, recipients: &[String]) -> Result<(), PassError> {
        let path = self.path();
        let dir = path
            .parent()
            .ok_or_else(|| PassError::Other(format!("{} has no parent", path.display())))?;
        create_private_dir(dir)?;
        let file_name = path
            .file_name()
            .map_or_else(|| "entry".into(), |n| n.to_string_lossy().into_owned());
        let tmp = dir.join(format!(".{file_name}.tmp{}", std::process::id()));
        // Claim the temp path exclusively (owner-only) before gpg writes to
        // it, so a pre-planted file or symlink there fails the write rather
        // than being followed by gpg's `--yes --output`.
        create_private_file(&tmp)?;

        let outcome = self
            .encrypt_to(&tmp, token, recipients)
            .await
            .and_then(|()| {
                std::fs::rename(&tmp, &path).map_err(|e| io_err("replacing", &path, &e))
            });
        if outcome.is_err() {
            let _ = std::fs::remove_file(&tmp);
        }
        outcome
    }

    /// Run `gpg --encrypt` for `recipients` into `out`, the stdin write and
    /// the wait both under the timeout. A gpg that exits early (e.g. no
    /// public key for a recipient) closes its stdin; that broken pipe is
    /// ignored so the caller sees gpg's own error from stderr instead.
    async fn encrypt_to(
        &self,
        out: &Path,
        token: &str,
        recipients: &[String],
    ) -> Result<(), PassError> {
        let mut cmd = self.store.gpg_command();
        cmd.args(["--batch", "--quiet", "--yes", "--no-tty", "--encrypt"]);
        for r in recipients {
            cmd.arg("--recipient").arg(r);
        }
        cmd.arg("--output")
            .arg(out)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            // A timed-out (dropped) exchange kills gpg instead of leaking it.
            .kill_on_drop(true);
        let mut child = cmd.spawn().map_err(|e| spawn_err(&self.store.gpg, &e))?;
        let stdin = child.stdin.take();
        let exchange = async move {
            if let Some(mut stdin) = stdin {
                match stdin.write_all(format!("{token}\n").as_bytes()).await {
                    Err(e) if e.kind() != std::io::ErrorKind::BrokenPipe => {
                        return Err(io_err("writing to gpg for", out, &e));
                    }
                    _ => {}
                }
                // Close stdin so gpg sees EOF.
                drop(stdin);
            }
            child
                .wait_with_output()
                .await
                .map_err(|e| io_err("waiting for gpg on", out, &e))
        };
        let output = tokio::time::timeout(self.store.timeout, exchange)
            .await
            .map_err(|_| PassError::Timeout {
                path: out.to_path_buf(),
                timeout: self.store.timeout,
            })??;
        if output.status.success() {
            Ok(())
        } else {
            Err(PassError::Other(format!(
                "gpg failed to encrypt to {}: {}",
                recipients.join(", "),
                human_message(&String::from_utf8_lossy(&output.stderr))
            )))
        }
    }

    /// Delete the entry's `.gpg` file.
    pub fn remove(&self) -> Result<(), PassError> {
        let path = self.path();
        match std::fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(PassError::NotFound {
                entry: self.name.clone(),
                path,
                cli: self.store.cli(),
            }),
            Err(e) => Err(io_err("removing", &path, &e)),
        }
    }

    /// Decrypt interactively (normal pinentry, inherited terminal) and discard
    /// the plaintext — the point is the side effect of gpg-agent caching the
    /// passphrase so the server's non-interactive reads succeed.
    pub async fn unlock_interactive(&self) -> Result<(), PassError> {
        let path = self.path();
        let status = self
            .store
            .gpg_command()
            .args(["--quiet", "--decrypt", "--"])
            .arg(&path)
            .stdout(Stdio::null())
            .status()
            .await
            .map_err(|e| spawn_err(&self.store.gpg, &e))?;
        if status.success() {
            Ok(())
        } else {
            Err(PassError::Other(format!(
                "gpg could not decrypt {} (if no passphrase prompt appeared, \
                 try `export GPG_TTY=$(tty)` and run this again)",
                path.display()
            )))
        }
    }
}

/// Expand a leading `~/` (config values are often hand-written that way).
fn expand_tilde(path: &str) -> anyhow::Result<PathBuf> {
    match path.strip_prefix("~/") {
        Some(rest) => Ok(crate::config::home_dir()?.join(rest)),
        None if path == "~" => crate::config::home_dir(),
        None => Ok(PathBuf::from(path)),
    }
}

/// Reject names that would escape the store or address something other than
/// a file inside it.
fn validate_entry_name(name: &str) -> Result<(), PassError> {
    let bad = name.is_empty()
        || name.contains('\0')
        || name.contains('\\')
        || name
            .split('/')
            .any(|seg| seg.is_empty() || seg == "." || seg == "..");
    if bad {
        return Err(PassError::Other(format!(
            "invalid pass entry name {name:?}: use a relative path like netbox-mcp/netbox.example.com"
        )));
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum DecryptOutcome {
    Ok,
    Locked,
    NoSecretKey,
    Failed,
}

/// Classify a decrypt from its exit status and `--status-fd` lines.
/// Success additionally requires `DECRYPTION_OKAY`, which gpg emits only
/// once integrity (MDC/AEAD) has been verified.
fn classify_decrypt(success: bool, stderr: &str) -> DecryptOutcome {
    let status: Vec<&str> = stderr
        .lines()
        .filter_map(|l| l.strip_prefix("[GNUPG:] "))
        .collect();
    if success && status.contains(&"DECRYPTION_OKAY") {
        return DecryptOutcome::Ok;
    }
    // `ERROR <location> <code>`; with several recipients there can be one per
    // key, so prefer the actionable "locked" over "no secret key".
    let codes: Vec<u32> = status
        .iter()
        .filter_map(|s| s.strip_prefix("ERROR "))
        .filter_map(|rest| rest.split_whitespace().nth(1)?.parse::<u32>().ok())
        .map(|code| code & 0xFFFF)
        .collect();
    if codes
        .iter()
        .any(|c| matches!(*c, GPG_ERR_NO_PIN_ENTRY | GPG_ERR_CANCELED))
    {
        DecryptOutcome::Locked
    } else if codes.contains(&GPG_ERR_NO_SECKEY) {
        DecryptOutcome::NoSecretKey
    } else {
        DecryptOutcome::Failed
    }
}

/// gpg's human-readable stderr (status lines removed), truncated.
fn human_message(stderr: &str) -> String {
    let msg = stderr
        .lines()
        .filter(|l| !l.starts_with("[GNUPG:] ") && !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join("; ");
    if msg.is_empty() {
        return "no error output".into();
    }
    match msg.char_indices().nth(MAX_GPG_MESSAGE) {
        Some((cut, _)) => format!("{}…", &msg[..cut]),
        None => msg,
    }
}

fn create_private_dir(dir: &Path) -> Result<(), PassError> {
    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(dir).map_err(|e| io_err("creating", dir, &e))
}

/// Create `path` exclusively (fails if anything, including a symlink, is
/// already there), owner-only on Unix.
fn create_private_file(path: &Path) -> Result<(), PassError> {
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    opts.open(path)
        .map(drop)
        .map_err(|e| io_err("creating", path, &e))
}

/// Quote `s` for a POSIX shell, unless it's made only of characters that
/// never need quoting.
pub fn shell_quote(s: &str) -> String {
    let safe = !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"/._+-:@%=,".contains(&b));
    if safe {
        s.to_string()
    } else {
        format!("'{}'", s.replace('\'', "'\\''"))
    }
}

fn io_err(action: &str, path: &Path, e: &std::io::Error) -> PassError {
    PassError::Other(format!("{action} {}: {e}", path.display()))
}

fn spawn_err(gpg: &Path, e: &std::io::Error) -> PassError {
    if e.kind() == std::io::ErrorKind::NotFound {
        PassError::Other(format!(
            "gpg program {} not found; install GnuPG or set \"gpg\" in the config file \
             to its full path",
            gpg.display()
        ))
    } else {
        PassError::Other(format!("running {}: {e}", gpg.display()))
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    //! A fake `gpg` for tests. One shared script (written once, so no test
    //! ever executes a file another thread is still writing — the ETXTBSY
    //! race): in decrypt mode it *sources* the entry file, so each test's
    //! "ciphertext" is a shell snippet producing the canned stdout/stderr/exit
    //! it wants; in encrypt mode it copies stdin to `--output` and records its
    //! arguments next to it in `<output>.args`.

    use std::path::{Path, PathBuf};
    use std::sync::OnceLock;

    const FAKE_GPG: &str = r#"#!/bin/sh
mode=decrypt
out=
last=
while [ $# -gt 0 ]; do
  case "$1" in
    --encrypt) mode=encrypt ;;
    --output) shift; out="$1" ;;
  esac
  all="$all $1"
  last="$1"
  shift
done
if [ "$mode" = encrypt ]; then
  case "$all" in
    *" NOPUBKEY"*)
      # Like a real gpg with no public key: exit without reading stdin.
      echo "gpg: NOPUBKEY: skipped: No public key" >&2
      exit 2 ;;
  esac
  cat > "$out"
  echo "$all" > "$out.args"
  exit 0
fi
. "$last"
"#;

    pub(crate) fn fake_gpg() -> &'static Path {
        static PATH: OnceLock<PathBuf> = OnceLock::new();
        PATH.get_or_init(|| {
            let dir = tempfile::tempdir().expect("tempdir").keep();
            let path = dir.join("gpg");
            std::fs::write(&path, FAKE_GPG).expect("write fake gpg");
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
                    .expect("chmod fake gpg");
            }
            path
        })
    }

    /// Status lines a real GnuPG 2.4 emits for a successful decrypt.
    pub(crate) fn ok_snippet(plaintext: &str) -> String {
        format!(
            "printf '%s\\n' '{plaintext}'\n\
             echo '[GNUPG:] BEGIN_DECRYPTION' >&2\n\
             echo '[GNUPG:] DECRYPTION_OKAY' >&2\n\
             exit 0\n"
        )
    }

    /// What GnuPG 2.4 emits for a key whose passphrase isn't cached under
    /// `--pinentry-mode error` (captured from a real run).
    pub(crate) const LOCKED_SNIPPET: &str = "\
echo '[GNUPG:] ENC_TO D8936DA9BE6E9ED2 18 0' >&2
echo '[GNUPG:] NO_SECKEY D8936DA9BE6E9ED2' >&2
echo 'gpg: public key decryption failed: No pinentry' >&2
echo '[GNUPG:] ERROR pkdecrypt_failed 67108949' >&2
echo '[GNUPG:] DECRYPTION_FAILED' >&2
echo '[GNUPG:] FAILURE gpg-exit 33554433' >&2
exit 2
";
}

#[cfg(test)]
mod tests {
    use super::test_support::{LOCKED_SNIPPET, fake_gpg, ok_snippet};
    use super::*;

    fn store_with(name: &str, snippet: &str) -> (tempfile::TempDir, PassEntry) {
        let dir = tempfile::tempdir().unwrap();
        let store = PassStore::new(dir.path(), fake_gpg());
        let entry = store.entry(name).unwrap();
        std::fs::create_dir_all(entry.path().parent().unwrap()).unwrap();
        std::fs::write(entry.path(), snippet).unwrap();
        (dir, entry)
    }

    #[tokio::test]
    async fn read_token_returns_first_line() {
        let (_dir, entry) = store_with("netbox-mcp/netbox.example.com", &ok_snippet("tok-abc"));
        assert_eq!(entry.read_token().await.unwrap(), "tok-abc");
    }

    #[tokio::test]
    async fn read_token_ignores_metadata_lines() {
        let snippet = "printf 'tok-abc\\nuser: me\\n'\n\
                       echo '[GNUPG:] DECRYPTION_OKAY' >&2\nexit 0\n";
        let (_dir, entry) = store_with("t", snippet);
        assert_eq!(entry.read_token().await.unwrap(), "tok-abc");
    }

    #[tokio::test]
    async fn locked_key_is_reported_as_locked() {
        let (_dir, entry) = store_with("t", LOCKED_SNIPPET);
        let err = entry.read_token().await.unwrap_err();
        assert!(matches!(err, PassError::Locked { .. }), "{err}");
        let msg = err.to_string();
        assert!(msg.contains("netbox-mcp unlock"), "{msg}");
        assert!(msg.contains("retry"), "{msg}");
    }

    #[tokio::test]
    async fn missing_entry_is_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let entry = PassStore::new(dir.path(), fake_gpg())
            .entry("nope")
            .unwrap();
        let err = entry.read_token().await.unwrap_err();
        assert!(matches!(err, PassError::NotFound { .. }), "{err}");
        assert!(err.to_string().contains("netbox-mcp login"), "{err}");
    }

    #[tokio::test]
    async fn exit_zero_without_decryption_okay_is_rejected() {
        // E.g. an integrity failure gpg still exits 0 on: never trust stdout.
        let (_dir, entry) = store_with("t", "echo tok-abc\nexit 0\n");
        let err = entry.read_token().await.unwrap_err();
        assert!(matches!(err, PassError::Other(_)), "{err}");
    }

    #[tokio::test]
    async fn unclassified_failure_forwards_gpg_message() {
        let snippet = "echo 'gpg: no valid OpenPGP data found.' >&2\n\
                       echo '[GNUPG:] NODATA 1' >&2\nexit 2\n";
        let (_dir, entry) = store_with("t", snippet);
        let msg = entry.read_token().await.unwrap_err().to_string();
        assert!(msg.contains("no valid OpenPGP data found"), "{msg}");
        assert!(!msg.contains("[GNUPG:]"), "{msg}");
    }

    #[tokio::test]
    async fn empty_plaintext_is_rejected() {
        let (_dir, entry) = store_with("t", &ok_snippet(""));
        let err = entry.read_token().await.unwrap_err();
        assert!(err.to_string().contains("empty"), "{err}");
    }

    #[tokio::test]
    async fn hung_gpg_times_out() {
        let dir = tempfile::tempdir().unwrap();
        let store = PassStore::new(dir.path(), fake_gpg()).with_timeout(Duration::from_millis(200));
        let entry = store.entry("t").unwrap();
        // `exec`, so the kill on timeout ends the sleep itself. A plain `sleep`
        // child would outlive its killed shell and, on macOS (where spawning
        // isn't atomic with CLOEXEC), can inherit another concurrently
        // spawned test's pipe and stall that test until the sleep ends.
        std::fs::write(entry.path(), "exec sleep 10\n").unwrap();
        let err = entry.read_token().await.unwrap_err();
        assert!(matches!(err, PassError::Timeout { .. }), "{err}");
    }

    #[tokio::test]
    async fn missing_gpg_binary_names_the_config_key() {
        let dir = tempfile::tempdir().unwrap();
        let store = PassStore::new(dir.path(), dir.path().join("no-such-gpg"));
        let entry = store.entry("t").unwrap();
        std::fs::write(entry.path(), "").unwrap();
        let msg = entry.read_token().await.unwrap_err().to_string();
        assert!(msg.contains("\"gpg\""), "{msg}");
    }

    #[test]
    fn classify_prefers_locked_over_no_seckey() {
        let stderr = "[GNUPG:] ERROR pkdecrypt_failed 33554449\n\
                      [GNUPG:] ERROR pkdecrypt_failed 67108949\n";
        assert_eq!(classify_decrypt(false, stderr), DecryptOutcome::Locked);
    }

    #[test]
    fn classify_maps_codes() {
        let line = |code: u32| format!("[GNUPG:] ERROR pkdecrypt_failed {code}\n");
        // Cancelled pinentry (source 4, code 99), captured from GnuPG 2.4.
        assert_eq!(
            classify_decrypt(false, &line(67_108_963)),
            DecryptOutcome::Locked
        );
        // No secret key (source 2, code 17), captured from GnuPG 2.4.
        assert_eq!(
            classify_decrypt(false, &line(33_554_449)),
            DecryptOutcome::NoSecretKey
        );
        assert_eq!(classify_decrypt(false, ""), DecryptOutcome::Failed);
        // Human text alone never classifies — only numeric status codes do.
        assert_eq!(
            classify_decrypt(false, "gpg: decryption failed: No pinentry\n"),
            DecryptOutcome::Failed
        );
    }

    #[test]
    fn entry_names_cannot_escape_the_store() {
        let store = PassStore::new("/s", "gpg");
        for bad in ["", "/abs", "../x", "a/../b", "a//b", "a/", "./a", "a\\b"] {
            assert!(store.entry(bad).is_err(), "{bad:?} accepted");
        }
        for good in ["t", "netbox-mcp/netbox.example.com", "a/b/c.example_8080"] {
            assert!(store.entry(good).is_ok(), "{good:?} rejected");
        }
    }

    #[test]
    fn recipients_come_from_nearest_gpg_id() {
        let dir = tempfile::tempdir().unwrap();
        let store = PassStore::new(dir.path(), "gpg");
        let entry = store.entry("a/b/tok").unwrap();
        assert!(entry.recipients().unwrap().is_none());

        std::fs::write(dir.path().join(".gpg-id"), "ROOT\n").unwrap();
        assert_eq!(entry.recipients().unwrap().unwrap(), vec!["ROOT"]);

        std::fs::create_dir_all(dir.path().join("a")).unwrap();
        std::fs::write(
            dir.path().join("a/.gpg-id"),
            "# comment\nK1 # work key\n\nK2\n   # indented comment\n",
        )
        .unwrap();
        assert_eq!(entry.recipients().unwrap().unwrap(), vec!["K1", "K2"]);
    }

    #[test]
    fn init_writes_root_gpg_id_once() {
        let dir = tempfile::tempdir().unwrap();
        let store = PassStore::new(dir.path().join("store"), "gpg");
        store.init("ABCD1234").unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.path().join("store/.gpg-id")).unwrap(),
            "ABCD1234\n"
        );
        assert!(store.init("OTHER").is_err());
    }

    #[tokio::test]
    async fn write_token_encrypts_to_every_recipient_atomically() {
        let dir = tempfile::tempdir().unwrap();
        let store = PassStore::new(dir.path(), fake_gpg());
        let entry = store.entry("netbox-mcp/netbox.example.com").unwrap();
        entry
            .write_token("tok-new", &["K1".into(), "K2".into()])
            .await
            .unwrap();
        // The fake "encrypts" by copying stdin.
        assert_eq!(std::fs::read_to_string(entry.path()).unwrap(), "tok-new\n");
        let parent = entry.path().parent().unwrap().to_path_buf();
        let args_file = std::fs::read_dir(&parent)
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| p.to_string_lossy().ends_with(".args"))
            .unwrap();
        let args = std::fs::read_to_string(args_file).unwrap();
        assert!(args.contains("--recipient K1 --recipient K2"), "{args}");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(entry.path())
                .unwrap()
                .permissions()
                .mode();
            assert_eq!(mode & 0o077, 0, "entry is group/world accessible: {mode:o}");
        }
    }

    #[tokio::test]
    async fn locked_advice_names_the_config_and_keyring() {
        let dir = tempfile::tempdir().unwrap();
        let store = PassStore::new(dir.path(), fake_gpg())
            .with_homedir(Some(PathBuf::from("/keys/my gnupg")))
            .with_config_path(Some(PathBuf::from("/etc/netbox mcp.json")));
        let entry = store.entry("t").unwrap();
        std::fs::write(entry.path(), LOCKED_SNIPPET).unwrap();
        let msg = entry.read_token().await.unwrap_err().to_string();
        // The server's own --config, so `unlock` reads the same pass_entry.
        assert!(
            msg.contains("`netbox-mcp --config '/etc/netbox mcp.json' unlock`"),
            "{msg}"
        );
        // The keyring the server uses, quoted for pasting.
        assert!(
            msg.contains("--homedir '/keys/my gnupg' --decrypt"),
            "{msg}"
        );
    }

    #[tokio::test]
    async fn default_config_advice_is_plain_unlock() {
        let (_dir, entry) = store_with("t", LOCKED_SNIPPET);
        let msg = entry.read_token().await.unwrap_err().to_string();
        assert!(msg.contains("`netbox-mcp unlock`"), "{msg}");
        assert!(!msg.contains("--homedir"), "{msg}");
    }

    #[test]
    fn shell_quote_quotes_only_when_needed() {
        assert_eq!(shell_quote("/a/b-c_d.gpg"), "/a/b-c_d.gpg");
        assert_eq!(shell_quote("/a b"), "'/a b'");
        assert_eq!(shell_quote("it's"), "'it'\\''s'");
        assert_eq!(shell_quote("~/x"), "'~/x'");
        assert_eq!(shell_quote(""), "''");
    }

    #[tokio::test]
    async fn early_gpg_exit_reports_gpgs_error_not_a_broken_pipe() {
        let dir = tempfile::tempdir().unwrap();
        let entry = PassStore::new(dir.path(), fake_gpg()).entry("t").unwrap();
        // Bigger than a pipe buffer, so the write is still in flight when the
        // fake gpg exits without reading it.
        let token = "x".repeat(512 * 1024);
        let msg = entry
            .write_token(&token, &["NOPUBKEY".into()])
            .await
            .unwrap_err()
            .to_string();
        assert!(msg.contains("No public key"), "{msg}");
        assert!(!entry.path().exists());
        // No temp file left behind.
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn write_token_refuses_a_planted_temp_symlink() {
        let dir = tempfile::tempdir().unwrap();
        let entry = PassStore::new(dir.path(), fake_gpg()).entry("t").unwrap();
        let victim = dir.path().join("victim");
        std::fs::write(&victim, "precious").unwrap();
        let tmp = dir.path().join(format!(".t.gpg.tmp{}", std::process::id()));
        std::os::unix::fs::symlink(&victim, &tmp).unwrap();
        assert!(entry.write_token("tok-x", &["K1".into()]).await.is_err());
        assert_eq!(std::fs::read_to_string(&victim).unwrap(), "precious");
        // The planted link is not ours to delete.
        assert!(tmp.symlink_metadata().is_ok());
    }

    #[test]
    fn remove_missing_entry_is_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let entry = PassStore::new(dir.path(), "gpg").entry("t").unwrap();
        assert!(matches!(entry.remove(), Err(PassError::NotFound { .. })));
    }

    #[tokio::test]
    async fn homedir_is_passed_to_gpg() {
        let dir = tempfile::tempdir().unwrap();
        let store =
            PassStore::new(dir.path(), fake_gpg()).with_homedir(Some(dir.path().join("gnupg")));
        let entry = store.entry("t").unwrap();
        entry.write_token("tok-x", &["K1".into()]).await.unwrap();
        let args_file = std::fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| p.to_string_lossy().ends_with(".args"))
            .unwrap();
        let args = std::fs::read_to_string(args_file).unwrap();
        let want = format!("--homedir {}", dir.path().join("gnupg").display());
        assert!(args.contains(&want), "{args}");
    }

    #[test]
    fn configured_location_beats_environment() {
        let store = PassStore::resolve(&StoreLocation {
            gpg: Some("/opt/gpg/bin/gpg"),
            store_dir: Some("/stores/pass"),
            gnupg_home: Some("/keys/gnupg"),
        })
        .unwrap();
        assert_eq!(store.root(), Path::new("/stores/pass"));
        assert_eq!(store.homedir(), Some(Path::new("/keys/gnupg")));
        assert_eq!(store.gpgconf(), PathBuf::from("/opt/gpg/bin/gpgconf"));
    }

    #[test]
    fn tilde_expands_to_home() {
        // A config test points HOME at a tempdir under this lock.
        let _guard = crate::test_support::lock_env();
        let home = crate::config::home_dir().unwrap();
        assert_eq!(
            expand_tilde("~/.password-store").unwrap(),
            home.join(".password-store")
        );
        assert_eq!(expand_tilde("/abs").unwrap(), PathBuf::from("/abs"));
    }

    #[test]
    fn gpgconf_sits_next_to_an_explicit_gpg() {
        assert_eq!(
            PassStore::new("/s", "/opt/homebrew/bin/gpg").gpgconf(),
            PathBuf::from("/opt/homebrew/bin/gpgconf")
        );
        assert_eq!(
            PassStore::new("/s", "gpg").gpgconf(),
            PathBuf::from("gpgconf")
        );
    }

    /// Tests against the real `gpg` on PATH (`make test-real-gpg`). Ignored
    /// by default and skipped unless `REAL_GPG=1`, so `cargo test` never
    /// needs GnuPG. They pin the numeric status codes the classifier relies
    /// on, so a GnuPG release that changes them fails here, not in the field.
    mod real_gpg {
        use std::path::{Path, PathBuf};
        use std::process::Command;

        use super::super::{PassError, PassStore};

        const TOKEN: &str = "real-gpg-test-token";

        fn enabled() -> bool {
            if std::env::var_os("REAL_GPG").is_some_and(|v| v == "1") {
                return true;
            }
            eprintln!("skipping real-gpg test: set REAL_GPG=1 (make test-real-gpg)");
            false
        }

        /// A throwaway GnuPG home under /tmp: gpg-agent's socket lives in it,
        /// and macOS caps socket paths at 104 bytes, which a default temp
        /// dir there can exceed. The agent is stopped on drop.
        struct Keyring {
            dir: tempfile::TempDir,
        }

        impl Keyring {
            fn new() -> Self {
                let dir = tempfile::Builder::new()
                    .prefix("nbg")
                    .tempdir_in("/tmp")
                    .expect("create GnuPG home");
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700))
                        .unwrap();
                }
                Self { dir }
            }

            fn home(&self) -> &Path {
                self.dir.path()
            }

            fn gpg(&self, args: &[&str]) -> std::process::Output {
                Command::new("gpg")
                    .arg("--homedir")
                    .arg(self.home())
                    .args(["--batch", "--quiet", "--pinentry-mode", "loopback"])
                    .args(args)
                    .output()
                    .expect("run gpg")
            }

            /// Generate an encryption-capable key (passphrase "" = none) and
            /// return its fingerprint.
            fn gen_key(&self, uid: &str, passphrase: &str) -> String {
                let out = self.gpg(&[
                    "--passphrase",
                    passphrase,
                    "--quick-gen-key",
                    uid,
                    "default",
                    "default",
                    "never",
                ]);
                assert!(
                    out.status.success(),
                    "keygen: {}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let list = self.gpg(&["--with-colons", "--list-secret-keys", uid]);
                String::from_utf8_lossy(&list.stdout)
                    .lines()
                    .find_map(|l| {
                        l.strip_prefix("fpr:")
                            .map(|r| r.trim_matches(':').to_string())
                    })
                    .expect("fingerprint")
            }

            /// Forget cached passphrases (stop the agent; it restarts on demand).
            fn lock(&self) {
                let _ = Command::new("gpgconf")
                    .arg("--homedir")
                    .arg(self.home())
                    .args(["--kill", "gpg-agent"])
                    .status();
            }

            fn store(&self, root: &Path) -> PassStore {
                PassStore::new(root, "gpg").with_homedir(Some(self.home().to_path_buf()))
            }
        }

        impl Drop for Keyring {
            fn drop(&mut self) {
                self.lock();
            }
        }

        fn entry_path(store: &PassStore, name: &str) -> PathBuf {
            store.entry(name).unwrap().path()
        }

        #[tokio::test]
        #[ignore = "needs GnuPG: make test-real-gpg"]
        async fn real_gpg_unprotected_key_decrypts() {
            if !enabled() {
                return;
            }
            let ring = Keyring::new();
            let fpr = ring.gen_key("Free <free@example.invalid>", "");
            let root = tempfile::tempdir().unwrap();
            let entry = ring.store(root.path()).entry("netbox-mcp/free").unwrap();
            entry.write_token(TOKEN, &[fpr]).await.unwrap();
            ring.lock();
            assert_eq!(entry.read_token().await.unwrap(), TOKEN);
        }

        #[tokio::test]
        #[ignore = "needs GnuPG: make test-real-gpg"]
        async fn real_gpg_locked_key_is_classified_as_locked() {
            if !enabled() {
                return;
            }
            let ring = Keyring::new();
            let fpr = ring.gen_key("Locked <locked@example.invalid>", "correct horse");
            let root = tempfile::tempdir().unwrap();
            let entry = ring.store(root.path()).entry("netbox-mcp/locked").unwrap();
            entry.write_token(TOKEN, &[fpr]).await.unwrap();
            // Key generation cached the passphrase; drop it so the
            // non-interactive decrypt finds the key locked (gpg code 85).
            ring.lock();
            let err = entry.read_token().await.unwrap_err();
            assert!(matches!(err, PassError::Locked { .. }), "{err}");
        }

        #[tokio::test]
        #[ignore = "needs GnuPG: make test-real-gpg"]
        async fn real_gpg_foreign_key_is_no_secret_key() {
            if !enabled() {
                return;
            }
            // Encrypt in one keyring, decrypt in another that lacks the key.
            let theirs = Keyring::new();
            let their_fpr = theirs.gen_key("Theirs <theirs@example.invalid>", "");
            let ours = Keyring::new();
            ours.gen_key("Ours <ours@example.invalid>", "");

            let their_root = tempfile::tempdir().unwrap();
            let their_store = theirs.store(their_root.path());
            their_store
                .entry("netbox-mcp/foreign")
                .unwrap()
                .write_token(TOKEN, &[their_fpr])
                .await
                .unwrap();

            let our_root = tempfile::tempdir().unwrap();
            let our_store = ours.store(our_root.path());
            let target = entry_path(&our_store, "netbox-mcp/foreign");
            std::fs::create_dir_all(target.parent().unwrap()).unwrap();
            std::fs::copy(entry_path(&their_store, "netbox-mcp/foreign"), &target).unwrap();

            let err = our_store
                .entry("netbox-mcp/foreign")
                .unwrap()
                .read_token()
                .await
                .unwrap_err();
            assert!(matches!(err, PassError::NoSecretKey { .. }), "{err}");
        }
    }
}

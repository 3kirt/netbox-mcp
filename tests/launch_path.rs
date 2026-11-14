//! End-to-end tests of the *built binary*, launched the way an MCP client
//! launches it: a spawned process speaking MCP over stdio, configured only
//! through its environment and config file.
//!
//! Unit tests call domain functions directly and so can't see what a client
//! puts in the process environment. gitlab-mcp v0.39.0 shipped a bug that
//! lived exactly there: for `"GITLAB_TOKEN": "${GITLAB_TOKEN}"` with the
//! variable unset, Claude Code passes the placeholder text through literally,
//! the server took it as a real token, and a configured `pass_entry` was never
//! used. These tests drive each way a client can hand over `NETBOX_TOKEN` and
//! assert the request reaching NetBox carries the token decrypted from the
//! pass entry — and that a locked key still lets the server start.

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const PASS_TOKEN: &str = "token-from-pass-entry";
const STEP_TIMEOUT: Duration = Duration::from_secs(20);

/// What the fake `gpg --decrypt` does.
#[derive(Clone, Copy)]
enum Gpg {
    /// Prints the token with the status lines real GnuPG emits.
    Unlocked,
    /// Fails the way GnuPG 2.4 does under `--pinentry-mode error` when
    /// gpg-agent has no cached passphrase.
    Locked,
}

/// A stand-in for `gpg`, so the server's status-code classification is
/// exercised end to end.
fn write_fake_gpg(dir: &Path, mode: Gpg) -> PathBuf {
    let gpg = dir.join("gpg");
    let body = match mode {
        Gpg::Unlocked => format!(
            "printf '%s\\n' '{PASS_TOKEN}'\n\
             echo '[GNUPG:] BEGIN_DECRYPTION' >&2\n\
             echo '[GNUPG:] DECRYPTION_OKAY' >&2\n"
        ),
        Gpg::Locked => "echo 'gpg: public key decryption failed: No pinentry' >&2\n\
             echo '[GNUPG:] ERROR pkdecrypt_failed 67108949' >&2\n\
             echo '[GNUPG:] DECRYPTION_FAILED' >&2\n\
             exit 2\n"
            .to_string(),
    };
    std::fs::write(&gpg, format!("#!/bin/sh\n{body}")).unwrap();
    set_mode(&gpg, 0o755);
    gpg
}

fn set_mode(path: &Path, mode: u32) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).unwrap();
    }
    #[cfg(not(unix))]
    let _ = (path, mode);
}

/// How the client hands `NETBOX_TOKEN` to the server.
#[derive(Clone, Copy, Debug)]
enum TokenEnv {
    /// Not in the environment at all.
    Unset,
    /// `"${NETBOX_TOKEN:-}"` with the variable unset.
    Empty,
    /// `"${NETBOX_TOKEN}"` with the variable unset: Claude Code's literal
    /// passthrough.
    LiteralPlaceholder,
}

/// Spawn the server against `netbox` with a pass-entry config, call one tool
/// over MCP stdio, and return the `tools/call` result.
async fn call_through_binary(netbox: &MockServer, token_env: TokenEnv, gpg_mode: Gpg) -> Value {
    let dir = tempfile::tempdir().unwrap();
    let gpg = write_fake_gpg(dir.path(), gpg_mode);
    let store = dir.path().join("store");
    std::fs::create_dir_all(store.join("netbox-mcp")).unwrap();
    // Content is irrelevant: the fake gpg ignores its input.
    std::fs::write(store.join("netbox-mcp/test.gpg"), "ciphertext").unwrap();
    let config = dir.path().join("netbox_mcp.json");
    std::fs::write(
        &config,
        json!({
            "url": netbox.uri(),
            "pass_entry": "netbox-mcp/test",
            "password_store_dir": store,
            "gpg": gpg,
        })
        .to_string(),
    )
    .unwrap();
    set_mode(&config, 0o600);

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_netbox-mcp"));
    cmd.arg("--config")
        .arg(&config)
        // Isolate from the developer's own setup.
        .env("HOME", dir.path())
        .env_remove("NETBOX_URL")
        .env_remove("NETBOX_TOKEN")
        .env_remove("PASSWORD_STORE_DIR")
        .env_remove("GNUPGHOME")
        .env_remove("RUST_LOG")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true);
    match token_env {
        TokenEnv::Unset => {}
        TokenEnv::Empty => {
            cmd.env("NETBOX_TOKEN", "");
        }
        TokenEnv::LiteralPlaceholder => {
            cmd.env("NETBOX_TOKEN", "${NETBOX_TOKEN}");
        }
    }
    let mut child = cmd.spawn().expect("spawn netbox-mcp");
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap()).lines();

    let mut send = async |msg: Value| {
        stdin
            .write_all(format!("{msg}\n").as_bytes())
            .await
            .unwrap();
    };
    send(
        json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": {"name": "launch-path-test", "version": "0"}
        }}),
    )
    .await;
    read_response(&mut stdout, 1).await;
    send(json!({"jsonrpc": "2.0", "method": "notifications/initialized"})).await;
    send(
        json!({"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {
            "name": "netbox_dcim_sites_list", "arguments": {}
        }}),
    )
    .await;
    let response = read_response(&mut stdout, 2).await;
    response["result"].clone()
}

/// Read stdout lines until the JSON-RPC response with `id` arrives.
async fn read_response<R>(lines: &mut tokio::io::Lines<R>, id: u64) -> Value
where
    R: tokio::io::AsyncBufRead + Unpin,
{
    tokio::time::timeout(STEP_TIMEOUT, async {
        loop {
            let line = lines
                .next_line()
                .await
                .unwrap()
                .expect("server closed stdout before responding");
            let msg: Value = serde_json::from_str(&line).unwrap();
            if msg["id"] == id {
                return msg;
            }
        }
    })
    .await
    .unwrap_or_else(|_| panic!("no response to request {id} within {STEP_TIMEOUT:?}"))
}

/// A NetBox that lists one site for the pass token and, as NetBox does,
/// answers any other token (e.g. a placeholder sent as the token) with 403.
async fn netbox_accepting_pass_token() -> MockServer {
    let netbox = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/dcim/sites/"))
        .and(header("authorization", format!("Token {PASS_TOKEN}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "count": 1, "next": null, "previous": null,
            "results": [{"id": 1, "name": "site-from-pass"}]
        })))
        .expect(1)
        .mount(&netbox)
        .await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(403).set_body_json(json!({"detail": "Invalid v1 token"})),
        )
        .mount(&netbox)
        .await;
    netbox
}

async fn assert_uses_pass_token(token_env: TokenEnv) {
    let netbox = netbox_accepting_pass_token().await;
    let result = call_through_binary(&netbox, token_env, Gpg::Unlocked).await;
    assert_ne!(
        result["isError"],
        json!(true),
        "{token_env:?}: tool call failed: {result}"
    );
    assert!(
        result.to_string().contains("site-from-pass"),
        "{token_env:?}: unexpected result: {result}"
    );
}

#[tokio::test]
async fn pass_entry_is_used_when_token_env_is_unset() {
    assert_uses_pass_token(TokenEnv::Unset).await;
}

#[tokio::test]
async fn pass_entry_is_used_when_token_env_is_empty() {
    assert_uses_pass_token(TokenEnv::Empty).await;
}

#[tokio::test]
async fn pass_entry_is_used_when_client_passes_a_literal_placeholder() {
    assert_uses_pass_token(TokenEnv::LiteralPlaceholder).await;
}

#[tokio::test]
async fn locked_key_still_starts_and_asks_for_unlock() {
    let netbox = MockServer::start().await;
    // No request may reach NetBox without a token.
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(500))
        .expect(0)
        .mount(&netbox)
        .await;
    let result = call_through_binary(&netbox, TokenEnv::Unset, Gpg::Locked).await;
    assert_eq!(result["isError"], json!(true), "{result}");
    let text = result.to_string();
    assert!(text.contains("netbox-mcp unlock"), "{text}");
}

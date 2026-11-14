mod client;
mod commands;
mod config;
mod credentials;
mod tools;

/// Test-only helpers shared across the crate's unit tests.
#[cfg(test)]
pub(crate) mod test_support {
    use std::sync::{Mutex, MutexGuard};

    use crate::client::NetboxClient;
    use wiremock::MockServer;

    // Cargo runs unit tests on multiple threads in one process. Every test that
    // sets, removes, or depends on a process environment variable (NETBOX_*,
    // HOME, PASSWORD_STORE_DIR, …) must hold this guard for its whole body, or
    // one test's set_var leaks into another's lookup.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    pub(crate) fn lock_env() -> MutexGuard<'static, ()> {
        ENV_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// A `NetboxClient` pointed at a wiremock server, with a dummy token.
    pub(crate) fn mock_client(server: &MockServer) -> NetboxClient {
        NetboxClient::new(server.uri(), "test-token").unwrap()
    }

    /// The JSON body of the first request `server` received with `method`.
    /// Panics if no such request arrived or its body is not JSON.
    pub(crate) async fn sent_body(
        server: &MockServer,
        method: wiremock::http::Method,
    ) -> serde_json::Value {
        server
            .received_requests()
            .await
            .unwrap()
            .iter()
            .find(|r| r.method == method)
            .and_then(|r| r.body_json().ok())
            .unwrap_or_else(|| panic!("no {method} request with a JSON body"))
    }
}

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use rmcp::ServiceExt;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Parser)]
#[command(
    name = "netbox-mcp",
    about = "MCP server for NetBox infrastructure data (read + limited write)",
    long_about = "MCP server for NetBox infrastructure data (read + limited write).\n\
        \n\
        Without a subcommand it serves MCP over stdio; add it to your MCP client config and\n\
        the client launches it.\n\
        \n\
        Credentials come from NETBOX_URL / NETBOX_TOKEN, else ~/.netbox_mcp.json\n\
        (\"url\" plus either a plaintext \"token\" or a GPG-encrypted \"pass_entry\").\n\
        `netbox-mcp login` verifies a token, encrypts it into a pass-compatible store, and\n\
        writes the config. The server decrypts it on first use without ever prompting: while\n\
        gpg-agent has no cached passphrase, tool calls fail asking you to run\n\
        `netbox-mcp unlock`, and succeed once you have.",
    version
)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,

    /// Path to the configuration file (default: ~/.netbox_mcp.json)
    #[arg(long, global = true)]
    config: Option<PathBuf>,
}

/// Without a subcommand, the MCP server runs over stdio.
#[derive(Subcommand)]
enum Command {
    /// Verify a NetBox API token and store it GPG-encrypted in a pass store
    Login(commands::LoginArgs),
    /// Delete the stored token from the pass store
    Logout(commands::EntryArgs),
    /// Cache the token's GPG passphrase in gpg-agent (prompts if needed)
    Unlock,
    /// Show where the token comes from and whether its GPG key is unlocked
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // JSON logging to stderr; level controlled by RUST_LOG (default: info).
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer().json().with_writer(std::io::stderr))
        .init();

    let args = Args::parse();
    let config_path = args.config.as_deref();
    match args.command {
        Some(Command::Login(login)) => return commands::login(config_path, login).await,
        Some(Command::Logout(entry)) => return commands::logout(config_path, &entry),
        Some(Command::Unlock) => return commands::unlock(config_path).await,
        Some(Command::Status) => return commands::status(config_path).await,
        None => {}
    }

    let cfg = config::Config::load(config_path)?;
    let url = cfg.resolve_url()?;
    let source = cfg.resolve_token()?;

    info!(mode = "stdio", netbox_url = %url, token_source = %source.describe(), "starting netbox-mcp");
    let client = match source {
        config::TokenSource::Env(token) | config::TokenSource::File(token) => {
            client::NetboxClient::new(url, token)?
        }
        // Decrypted on first use, so a locked GPG key doesn't stop startup.
        config::TokenSource::Pass(entry) => client::NetboxClient::with_pass_entry(url, entry)?,
    };
    tools::NetboxMcpServer::new(client)
        .serve(rmcp::transport::io::stdio_pipes())
        .await?
        .waiting()
        .await?;

    Ok(())
}

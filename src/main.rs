mod client;
mod commands;
mod config;
mod credentials;
mod tools;

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

// Declared last: clippy's `items_after_test_module` (extended in Rust 1.101)
// rejects a test-only module placed before ordinary items.
#[cfg(test)]
mod test_support;

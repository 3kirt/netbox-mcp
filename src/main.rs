mod client;
mod config;
mod tools;

/// Test-only helpers shared across the crate's unit tests.
#[cfg(test)]
pub(crate) mod test_support {
    use crate::client::NetboxClient;
    use wiremock::MockServer;

    /// A `NetboxClient` pointed at a wiremock server, with a dummy token.
    pub(crate) fn mock_client(server: &MockServer) -> NetboxClient {
        NetboxClient::new(server.uri(), "test-token").unwrap()
    }
}

use std::path::PathBuf;

use clap::Parser;
use rmcp::ServiceExt;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Parser)]
#[command(
    name = "netbox-mcp",
    about = "MCP server for NetBox infrastructure data (read + limited write)"
)]
struct Args {
    /// Path to the configuration file (default: ~/.netbox_mcp.json)
    #[arg(long)]
    config: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // JSON logging to stderr; level controlled by RUST_LOG (default: info).
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer().json().with_writer(std::io::stderr))
        .init();

    let args = Args::parse();

    let cfg = config::Config::load(args.config.as_deref())?;
    let url = cfg.resolve_url()?;
    let token = cfg.resolve_token()?;

    info!(mode = "stdio", netbox_url = %url, "starting netbox-mcp");
    let server = tools::NetboxMcpServer::new(url, token)?;
    server
        .serve(rmcp::transport::io::stdio())
        .await?
        .waiting()
        .await?;

    Ok(())
}

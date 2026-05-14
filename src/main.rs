mod client;
mod config;
mod server;
mod tools;

use std::path::PathBuf;

use clap::Parser;
use rmcp::ServiceExt;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Parser)]
#[command(
    name = "netbox-mcp",
    about = "Read-only MCP server for NetBox infrastructure data"
)]
struct Args {
    /// Path to the configuration file (default: ~/.netbox_mcp.json)
    #[arg(long)]
    config: Option<PathBuf>,

    /// Address to listen on for HTTP transport (e.g. 0.0.0.0:8080).
    /// When omitted, the server runs in stdio transport mode.
    #[arg(long)]
    listen: Option<String>,
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

    if let Some(listen) = args.listen {
        info!(mode = "http", listen = %listen, netbox_url = %url, "starting netbox-mcp");
        server::http::run(&listen, url).await?;
    } else {
        let token = cfg.resolve_token()?;
        info!(mode = "stdio", netbox_url = %url, "starting netbox-mcp");
        let server = tools::NetboxMcpServer::new_stdio(url, token);
        server
            .serve(rmcp::transport::io::stdio())
            .await?
            .waiting()
            .await?;
    }

    Ok(())
}

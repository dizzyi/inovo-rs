use std::net::{IpAddr, Ipv4Addr};

use clap::{Parser, Subcommand};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod revive;
mod scan;
mod server;

use revive::*;
use scan::*;
use server::*;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = tracing::Level::INFO)]
    log_level: tracing::Level,
    /// output json
    #[arg(short, long)]
    json: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Clone, Subcommand)]
enum Commands {
    /// Revive a RSU
    Revive {
        /// Target PSU IP Address or Hostname
        host: String,
        /// user to login
        user: String,
        /// password to login
        password: String,
    },
    /// Scan Local Network for PSU
    Scan,
    /// Start server for web gui
    Server {
        /// local ip to start server
        #[arg(short, long)]
        ip: Option<IpAddr>,
        /// port to start server
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
        /// start server on localhost, if ip is not specified
        #[arg(short, long, default_value_t = false)]
        on_localhost: bool,
    },
}

pub async fn resolve_host_to_ip(host: impl Into<String>) -> Option<std::net::IpAddr> {
    let output = tokio::process::Command::new("tracert")
        .arg("-4")
        .arg("-w")
        .arg("1000")
        .arg(host.into())
        .output()
        .await
        .ok()?
        .stdout;

    let output_string = String::from_utf8_lossy(&output);

    output_string
        .split_ascii_whitespace()
        .find(|s| s.starts_with('[') && s.ends_with(']'))
        .map(|s| s.replace(&['[', ']'], ""))?
        .parse()
        .ok()
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    if !cli.json {
        let reg = tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .without_time()
                    .with_level(true)
                    .with_target(true),
            )
            .with(tracing_subscriber::filter::LevelFilter::from_level(
                cli.log_level,
            ));

        if let Some(Commands::Server { .. }) = cli.command {
            reg.init();
        } else {
            reg.with(EnvFilter::builder().parse_lossy(format!("inovo_cli")))
                .init();
        }
    }

    let Some(cmd) = cli.command else {
        return Ok(());
    };

    match cmd {
        Commands::Revive {
            host,
            user,
            password,
        } => revive(host, user, password).await,
        Commands::Scan => scan_command(cli.json).await,
        Commands::Server {
            ip,
            port,
            on_localhost,
        } => server(ip, port, on_localhost).await,
    }
}

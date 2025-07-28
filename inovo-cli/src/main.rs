use clap::{Parser, Subcommand};
use tracing::error;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod revive;
mod scan;

use revive::*;
use scan::*;

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
        /// Target PSU IP Address
        ip: String,
        /// user to login
        user: String,
        /// password to login
        password: String,
    },
    /// Scan Local Network for PSU
    Scan,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    if !cli.json {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .without_time()
                    .with_level(true)
                    .with_target(true),
            )
            .with(EnvFilter::builder().parse_lossy(format!("inovo_cli")))
            .with(tracing_subscriber::filter::LevelFilter::from_level(
                cli.log_level,
            ))
            .init();
    }

    let Some(cmd) = cli.command else {
        return Ok(());
    };

    match cmd {
        Commands::Revive { ip, user, password } => revive(ip, user, password).await,
        Commands::Scan => scan_command(cli.json).await,
    }
}

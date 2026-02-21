//! trw-daemon — a lightweight telemetry daemon for robotics fleets.
//!
//! Collects system metrics and ROS2 introspection data from the local
//! machine, then exposes everything via an HTTP/JSON API for a control
//! plane to poll.
//!
//! Usage:
//!   trw-daemon                         # uses ./config/default.toml
//!   trw-daemon --config /etc/trw.toml  # custom config path
//!   trw-daemon --port 8080             # override port

mod collectors;
mod config;
mod models;
mod server;

use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A lightweight telemetry daemon for robotics fleets.
#[derive(Parser, Debug)]
#[command(name = "trw-daemon", version, about)]
struct Cli {
    /// Path to TOML configuration file.
    #[arg(short, long, default_value = "config/default.toml")]
    config: PathBuf,

    /// Override the HTTP server port.
    #[arg(short, long)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse CLI args
    let cli = Cli::parse();

    // Load config
    let mut cfg = config::Config::load(&cli.config)?;
    if let Some(port) = cli.port {
        cfg.daemon.port = port;
    }

    tracing::info!(
        "trw-daemon v{} starting on port {}",
        env!("CARGO_PKG_VERSION"),
        cfg.daemon.port
    );

    // Initialize shared state
    let state: server::SharedState = Arc::new(RwLock::new(None));

    // TODO: Initialize enabled collectors based on config
    //   - Always create SystemCollector
    //   - If cfg.collectors.ros2 && feature "ros2": create Ros2Collector
    //   - If cfg.collectors.gpu && feature "gpu": create GpuCollector
    //
    // TODO: Spawn collector loop in a background tokio task
    //   - Loop every cfg.daemon.tick_interval_secs
    //   - Call collect() on each collector
    //   - Merge results into a MachineSnapshot
    //   - Write snapshot to shared state
    //
    // TODO: Detect hostname (cfg.daemon.hostname or gethostname())

    // Start HTTP server (this blocks)
    server::serve(state, cfg.daemon.port).await?;

    Ok(())
}

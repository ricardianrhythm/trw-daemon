//! Configuration loading and parsing.
//!
//! Reads a TOML config file and provides typed access to all
//! daemon settings. Falls back to sensible defaults if no
//! config file is found.

use serde::Deserialize;
use std::path::Path;

/// Top-level daemon configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub daemon: DaemonConfig,
    #[serde(default)]
    pub collectors: CollectorsConfig,
    #[serde(default)]
    pub ros2: Ros2Config,
}

/// Core daemon settings.
#[derive(Debug, Clone, Deserialize)]
pub struct DaemonConfig {
    /// HTTP server port. Default: 4040.
    #[serde(default = "default_port")]
    pub port: u16,
    /// Collection interval in seconds. Default: 2.
    #[serde(default = "default_tick")]
    pub tick_interval_secs: u64,
    /// Override hostname. Empty string means auto-detect.
    #[serde(default)]
    pub hostname: String,
}

/// Which collectors are enabled.
#[derive(Debug, Clone, Deserialize)]
pub struct CollectorsConfig {
    #[serde(default = "default_true")]
    pub system: bool,
    #[serde(default = "default_true")]
    pub ros2: bool,
    #[serde(default)]
    pub gpu: bool,
}

/// ROS2-specific collector settings.
#[derive(Debug, Clone, Deserialize)]
pub struct Ros2Config {
    /// Path to the ros2 CLI binary.
    #[serde(default = "default_ros2_cli")]
    pub cli_path: String,
    /// Timeout for each CLI call in seconds.
    #[serde(default = "default_ros2_timeout")]
    pub timeout_secs: u64,
}

fn default_port() -> u16 { 4040 }
fn default_tick() -> u64 { 2 }
fn default_true() -> bool { true }
fn default_ros2_cli() -> String { "ros2".to_string() }
fn default_ros2_timeout() -> u64 { 5 }

impl Default for DaemonConfig {
    fn default() -> Self {
        DaemonConfig {
            port: default_port(),
            tick_interval_secs: default_tick(),
            hostname: String::new(),
        }
    }
}

impl Default for CollectorsConfig {
    fn default() -> Self {
        CollectorsConfig {
            system: true,
            ros2: true,
            gpu: false,
        }
    }
}

impl Default for Ros2Config {
    fn default() -> Self {
        Ros2Config {
            cli_path: default_ros2_cli(),
            timeout_secs: default_ros2_timeout(),
        }
    }
}

impl Config {
    /// Load configuration from a TOML file.
    /// Returns default config if the file doesn't exist.
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if path.exists() {
            let contents = std::fs::read_to_string(path)?;
            let config: Config = toml::from_str(&contents)?;
            Ok(config)
        } else {
            tracing::warn!("Config file not found at {:?}, using defaults", path);
            Ok(Config {
                daemon: DaemonConfig::default(),
                collectors: CollectorsConfig::default(),
                ros2: Ros2Config::default(),
            })
        }
    }
}

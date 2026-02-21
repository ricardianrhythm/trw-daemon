//! ROS2 introspection collector.
//!
//! Discovers running ROS2 nodes and topics by shelling out to
//! the ros2 CLI. Requires ROS2 to be sourced in the environment.
//!
//! CLI commands used:
//! - `ros2 node list`       → list running nodes
//! - `ros2 topic list`      → list active topics
//! - `ros2 topic info <t>`  → get message type for a topic
//! - `ros2 topic hz <t>`    → measure publish rate
//!
//! If the ros2 CLI is not found at startup, this collector
//! gracefully disables itself and logs a warning.

use async_trait::async_trait;
use super::{Collector, CollectorError};

/// Collects ROS2 node and topic information via CLI subprocess calls.
pub struct Ros2Collector {
    /// Path to the ros2 CLI binary. Defaults to "ros2".
    cli_path: String,
    /// Timeout in seconds for each CLI call.
    timeout_secs: u64,
    /// Whether the ros2 CLI was found at startup.
    available: bool,
}

impl Ros2Collector {
    pub fn new(cli_path: String, timeout_secs: u64) -> Self {
        // TODO: Check if ros2 CLI is available by running `ros2 --version`
        let available = false; // placeholder
        Ros2Collector {
            cli_path,
            timeout_secs,
            available,
        }
    }
}

#[async_trait]
impl Collector for Ros2Collector {
    fn name(&self) -> &str {
        "ros2"
    }

    async fn collect(&self) -> Result<serde_json::Value, CollectorError> {
        if !self.available {
            return Err(CollectorError::NotAvailable(
                "ros2 CLI not found in PATH".to_string(),
            ));
        }

        // TODO: Run `ros2 node list` and parse stdout into Vec<String>
        // TODO: Run `ros2 topic list` and parse stdout into topic names
        // TODO: For each topic, run `ros2 topic info <name>`
        // TODO: For each topic, run `ros2 topic hz <name>` with timeout
        // TODO: Return Ros2State as serde_json::Value
        todo!("Implement ROS2 introspection via CLI subprocess calls")
    }
}

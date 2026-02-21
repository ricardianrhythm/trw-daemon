//! Collector trait and implementations for gathering telemetry data.
//!
//! Each collector is responsible for one domain of data collection
//! (system metrics, ROS2 introspection, GPU stats). Collectors run
//! on a configurable tick interval and return JSON values that are
//! merged into a MachineSnapshot.

pub mod system;

#[cfg(feature = "ros2")]
pub mod ros2;

#[cfg(feature = "gpu")]
pub mod gpu;

use async_trait::async_trait;
use std::fmt;

/// Error type for collector failures.
#[derive(Debug)]
pub enum CollectorError {
    /// The external tool (ros2 CLI, nvidia-smi) is not available.
    NotAvailable(String),
    /// The tool ran but returned unparseable output.
    ParseError(String),
    /// The tool timed out.
    Timeout(String),
    /// Any other error.
    Other(String),
}

impl fmt::Display for CollectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectorError::NotAvailable(msg) => write!(f, "not available: {}", msg),
            CollectorError::ParseError(msg) => write!(f, "parse error: {}", msg),
            CollectorError::Timeout(msg) => write!(f, "timeout: {}", msg),
            CollectorError::Other(msg) => write!(f, "error: {}", msg),
        }
    }
}

impl std::error::Error for CollectorError {}

/// Trait for telemetry data collectors.
///
/// Implement this trait to add a new data collection domain.
/// The daemon's main loop calls `collect()` on each enabled
/// collector every tick interval.
#[async_trait]
pub trait Collector: Send + Sync {
    /// Human-readable name for this collector (e.g., "system", "ros2").
    fn name(&self) -> &str;

    /// Collect telemetry data and return it as a JSON value.
    ///
    /// Returns Ok(value) on success, or Err if collection failed.
    /// A failed collection does not stop the daemon — the field
    /// is simply omitted from the snapshot.
    async fn collect(&self) -> Result<serde_json::Value, CollectorError>;
}

//! Data models for trw-daemon telemetry snapshots.
//!
//! These types represent the JSON structure served by the HTTP API.
//! See GET /status endpoint documentation.

use serde::{Deserialize, Serialize};

/// Complete snapshot of a machine's state at a point in time.
/// This is the top-level type returned by GET /status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineSnapshot {
    pub hostname: String,
    pub timestamp: String,
    pub system: SystemMetrics,
    pub gpu: Option<GpuMetrics>,
    pub ros2: Option<Ros2State>,
    pub processes: Vec<ProcessInfo>,
}

/// System-level resource metrics collected from /proc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub disk_percent: f64,
    pub load_avg: [f64; 3],
    pub uptime_secs: u64,
}

/// GPU metrics from nvidia-smi or rocm-smi.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuMetrics {
    pub name: String,
    pub utilization_percent: f64,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub temperature_c: f64,
}

/// ROS2 introspection state from ros2 CLI commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ros2State {
    pub nodes: Vec<String>,
    pub topics: Vec<TopicInfo>,
}

/// Information about a single ROS2 topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub hz: Option<f64>,
}

/// Per-process resource usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f64,
    pub mem_mb: f64,
}

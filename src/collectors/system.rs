//! System metrics collector.
//!
//! Reads CPU, memory, disk, and network statistics from /proc.
//! This collector is always enabled and has no external dependencies.
//!
//! Data sources:
//! - /proc/stat       → CPU usage
//! - /proc/meminfo    → Memory usage
//! - /proc/diskstats  → Disk usage
//! - /proc/net/dev    → Network stats
//! - /proc/{pid}/stat → Per-process CPU/memory

use async_trait::async_trait;
use super::{Collector, CollectorError};

/// Collects system-level resource metrics from /proc.
pub struct SystemCollector;

impl SystemCollector {
    pub fn new() -> Self {
        SystemCollector
    }
}

#[async_trait]
impl Collector for SystemCollector {
    fn name(&self) -> &str {
        "system"
    }

    async fn collect(&self) -> Result<serde_json::Value, CollectorError> {
        // TODO: Read /proc/stat and compute CPU usage percentage
        // TODO: Read /proc/meminfo for MemTotal and MemAvailable
        // TODO: Read /proc/diskstats for disk I/O
        // TODO: Read /proc/loadavg for load averages
        // TODO: Read /proc/uptime for uptime_secs
        // TODO: Enumerate /proc/{pid}/stat for per-process stats
        // TODO: Return SystemMetrics as serde_json::Value
        todo!("Implement system metrics collection from /proc")
    }
}

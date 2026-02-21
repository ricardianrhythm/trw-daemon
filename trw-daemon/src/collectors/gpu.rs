//! GPU metrics collector.
//!
//! Collects GPU utilization, memory, and temperature by shelling
//! out to `nvidia-smi` (NVIDIA) or `rocm-smi` (AMD).
//!
//! If neither tool is found at startup, this collector gracefully
//! disables itself and logs a warning.

use async_trait::async_trait;
use super::{Collector, CollectorError};

/// Collects GPU metrics via nvidia-smi or rocm-smi.
pub struct GpuCollector {
    /// Whether a supported GPU tool was found at startup.
    available: bool,
}

impl GpuCollector {
    pub fn new() -> Self {
        // TODO: Check for nvidia-smi, then rocm-smi
        let available = false; // placeholder
        GpuCollector { available }
    }
}

#[async_trait]
impl Collector for GpuCollector {
    fn name(&self) -> &str {
        "gpu"
    }

    async fn collect(&self) -> Result<serde_json::Value, CollectorError> {
        if !self.available {
            return Err(CollectorError::NotAvailable(
                "No supported GPU tool found (nvidia-smi, rocm-smi)".to_string(),
            ));
        }

        // TODO: Run nvidia-smi with CSV query
        // TODO: If nvidia-smi not found, try rocm-smi equivalent
        // TODO: Return GpuMetrics as serde_json::Value
        todo!("Implement GPU metrics collection via nvidia-smi/rocm-smi")
    }
}

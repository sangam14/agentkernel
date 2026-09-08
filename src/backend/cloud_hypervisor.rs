//! Cloud Hypervisor backend implementing the Sandbox trait.
//!
//! This is a modern, Rust-based VMM optimized for cloud workloads.
//! Currently a work-in-progress stub.

use anyhow::{Result, bail};
use async_trait::async_trait;

use super::{BackendType, ExecResult, Sandbox, SandboxConfig};

/// Cloud Hypervisor sandbox
pub struct CloudHypervisorSandbox {
    name: String,
    running: bool,
}

impl CloudHypervisorSandbox {
    /// Create a new Cloud Hypervisor sandbox
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            running: false,
        }
    }
}

#[async_trait]
impl Sandbox for CloudHypervisorSandbox {
    async fn start(&mut self, _config: &SandboxConfig) -> Result<()> {
        bail!("Cloud Hypervisor backend is not yet fully implemented.")
    }

    async fn exec(&mut self, _cmd: &[&str]) -> Result<ExecResult> {
        bail!("Cloud Hypervisor backend is not yet fully implemented.")
    }

    async fn stop(&mut self) -> Result<()> {
        self.running = false;
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn backend_type(&self) -> BackendType {
        BackendType::CloudHypervisor
    }

    fn is_running(&self) -> bool {
        self.running
    }

    async fn write_file_unchecked(&mut self, _path: &str, _content: &[u8]) -> anyhow::Result<()> {
        bail!("File operations not supported for Cloud Hypervisor yet")
    }

    async fn read_file_unchecked(&mut self, _path: &str) -> anyhow::Result<Vec<u8>> {
        bail!("File operations not supported for Cloud Hypervisor yet")
    }

    async fn remove_file_unchecked(&mut self, _path: &str) -> anyhow::Result<()> {
        bail!("File operations not supported for Cloud Hypervisor yet")
    }

    async fn mkdir_unchecked(&mut self, _path: &str, _recursive: bool) -> anyhow::Result<()> {
        bail!("File operations not supported for Cloud Hypervisor yet")
    }
}

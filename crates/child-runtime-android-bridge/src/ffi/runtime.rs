use std::{fmt, path::PathBuf};

use ocentra_child_runtime::service::{
    ChildAgentHealth, ChildAgentService, ChildAgentServiceError, ChildAgentServicePaths,
};
use tokio::runtime::Runtime;

/// Owns the real child service and the Tokio runtime that initialized it.
///
/// JNI uses this same boundary; the Rust integration test exercises it directly
/// without replacing filesystem custody or child-runtime behavior.
pub struct ChildRuntimeAndroidBridge {
    _runtime: Runtime,
    service: ChildAgentService,
}

impl ChildRuntimeAndroidBridge {
    pub fn start(durable_root: impl Into<PathBuf>) -> Result<Self, ChildRuntimeAndroidBridgeError> {
        let runtime = Runtime::new().map_err(ChildRuntimeAndroidBridgeError::Runtime)?;
        let service = runtime
            .block_on(ChildAgentService::initialize_with_paths(
                ChildAgentServicePaths::from_root(durable_root),
            ))
            .map_err(ChildRuntimeAndroidBridgeError::Service)?;
        Ok(Self {
            _runtime: runtime,
            service,
        })
    }

    pub fn health(&self) -> Result<ChildAgentHealth, ChildAgentServiceError> {
        self.service.health()
    }
}

#[derive(Debug)]
pub enum ChildRuntimeAndroidBridgeError {
    Runtime(std::io::Error),
    Service(ChildAgentServiceError),
}

impl fmt::Display for ChildRuntimeAndroidBridgeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Runtime(error) => error.fmt(formatter),
            Self::Service(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ChildRuntimeAndroidBridgeError {}

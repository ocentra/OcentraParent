use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiProviderPrivacyMode {
    #[serde(rename = "local-only")]
    LocalOnly,
}

impl LocalAiProviderPrivacyMode {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::LocalOnly => constants::local_ai_runtime::PRIVACY_MODE_LOCAL_ONLY,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiAdapterBoundary {
    #[serde(rename = "status-only")]
    StatusOnly,
    #[serde(rename = "local-adapter-unavailable")]
    LocalAdapterUnavailable,
    #[serde(rename = "local-adapter-ready")]
    LocalAdapterReady,
}

impl LocalAiAdapterBoundary {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::StatusOnly => constants::local_ai_runtime::ADAPTER_BOUNDARY_STATUS_ONLY,
            Self::LocalAdapterUnavailable => {
                constants::local_ai_runtime::ADAPTER_BOUNDARY_LOCAL_ADAPTER_UNAVAILABLE
            }
            Self::LocalAdapterReady => {
                constants::local_ai_runtime::ADAPTER_BOUNDARY_LOCAL_ADAPTER_READY
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiExecutionState {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "dry-run-ready")]
    DryRunReady,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "failed")]
    Failed,
}

impl LocalAiExecutionState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Disabled => constants::local_ai_runtime::EXECUTION_STATE_DISABLED,
            Self::DryRunReady => constants::local_ai_runtime::EXECUTION_STATE_DRY_RUN_READY,
            Self::Running => constants::local_ai_runtime::EXECUTION_STATE_RUNNING,
            Self::Failed => constants::local_ai_runtime::EXECUTION_STATE_FAILED,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiProviderSource {
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "local-config")]
    LocalConfig,
    #[serde(rename = "local-model-cache")]
    LocalModelCache,
    #[serde(rename = "os-capability-probe")]
    OsCapabilityProbe,
}

impl LocalAiProviderSource {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Unavailable => constants::local_ai_runtime::PROVIDER_SOURCE_UNAVAILABLE,
            Self::LocalConfig => constants::local_ai_runtime::PROVIDER_SOURCE_LOCAL_CONFIG,
            Self::LocalModelCache => constants::local_ai_runtime::PROVIDER_SOURCE_LOCAL_MODEL_CACHE,
            Self::OsCapabilityProbe => {
                constants::local_ai_runtime::PROVIDER_SOURCE_OS_CAPABILITY_PROBE
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiAdapterProbeState {
    #[serde(rename = "probe-unavailable")]
    ProbeUnavailable,
    #[serde(rename = "probe-ready")]
    ProbeReady,
    #[serde(rename = "probe-failed")]
    ProbeFailed,
}

impl LocalAiAdapterProbeState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ProbeUnavailable => constants::local_ai_runtime::ADAPTER_PROBE_STATE_UNAVAILABLE,
            Self::ProbeReady => constants::local_ai_runtime::ADAPTER_PROBE_STATE_READY,
            Self::ProbeFailed => constants::local_ai_runtime::ADAPTER_PROBE_STATE_FAILED,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiProviderConfigurationState {
    #[serde(rename = "local-provider-unconfigured")]
    LocalProviderUnconfigured,
    #[serde(rename = "local-provider-configured")]
    LocalProviderConfigured,
    #[serde(rename = "local-provider-config-invalid")]
    LocalProviderConfigInvalid,
}

impl LocalAiProviderConfigurationState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::LocalProviderUnconfigured => {
                constants::local_ai_runtime::PROVIDER_CONFIGURATION_UNCONFIGURED
            }
            Self::LocalProviderConfigured => {
                constants::local_ai_runtime::PROVIDER_CONFIGURATION_CONFIGURED
            }
            Self::LocalProviderConfigInvalid => {
                constants::local_ai_runtime::PROVIDER_CONFIGURATION_INVALID
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiAdapterReadinessState {
    #[serde(rename = "adapter-not-ready")]
    AdapterNotReady,
    #[serde(rename = "adapter-ready")]
    AdapterReady,
    #[serde(rename = "adapter-readiness-invalid")]
    AdapterReadinessInvalid,
}

impl LocalAiAdapterReadinessState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::AdapterNotReady => constants::local_ai_runtime::ADAPTER_READINESS_STATE_NOT_READY,
            Self::AdapterReady => constants::local_ai_runtime::ADAPTER_READINESS_STATE_READY,
            Self::AdapterReadinessInvalid => {
                constants::local_ai_runtime::ADAPTER_READINESS_STATE_INVALID
            }
        }
    }
}

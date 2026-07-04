use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiProviderPrivacyMode {
    #[serde(rename = "local-only")]
    LocalOnly,
}

impl LocalAiProviderPrivacyMode {
    pub fn as_protocol_str(&self) -> &'static str {
        constants::local_ai_runtime::PRIVACY_MODE_LOCAL_ONLY
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiAdapterBoundary {
    #[serde(rename = "status-only")]
    StatusOnly,
    #[serde(rename = "local-adapter-unavailable")]
    LocalAdapterUnavailable,
    #[serde(rename = "local-adapter-ready")]
    LocalAdapterReady,
}

impl LocalAiAdapterBoundary {
    const PROTOCOL_STRINGS: [&'static str; 3] = [
        constants::local_ai_runtime::ADAPTER_BOUNDARY_STATUS_ONLY,
        constants::local_ai_runtime::ADAPTER_BOUNDARY_LOCAL_ADAPTER_UNAVAILABLE,
        constants::local_ai_runtime::ADAPTER_BOUNDARY_LOCAL_ADAPTER_READY,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 4] = [
        constants::local_ai_runtime::EXECUTION_STATE_DISABLED,
        constants::local_ai_runtime::EXECUTION_STATE_DRY_RUN_READY,
        constants::local_ai_runtime::EXECUTION_STATE_RUNNING,
        constants::local_ai_runtime::EXECUTION_STATE_FAILED,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 4] = [
        constants::local_ai_runtime::PROVIDER_SOURCE_UNAVAILABLE,
        constants::local_ai_runtime::PROVIDER_SOURCE_LOCAL_CONFIG,
        constants::local_ai_runtime::PROVIDER_SOURCE_LOCAL_MODEL_CACHE,
        constants::local_ai_runtime::PROVIDER_SOURCE_OS_CAPABILITY_PROBE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiAdapterProbeState {
    #[serde(rename = "probe-unavailable")]
    ProbeUnavailable,
    #[serde(rename = "probe-ready")]
    ProbeReady,
    #[serde(rename = "probe-failed")]
    ProbeFailed,
}

impl LocalAiAdapterProbeState {
    const PROTOCOL_STRINGS: [&'static str; 3] = [
        constants::local_ai_runtime::ADAPTER_PROBE_STATE_UNAVAILABLE,
        constants::local_ai_runtime::ADAPTER_PROBE_STATE_READY,
        constants::local_ai_runtime::ADAPTER_PROBE_STATE_FAILED,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiProviderConfigurationState {
    #[serde(rename = "local-provider-unconfigured")]
    LocalProviderUnconfigured,
    #[serde(rename = "local-provider-configured")]
    LocalProviderConfigured,
    #[serde(rename = "local-provider-config-invalid")]
    LocalProviderConfigInvalid,
}

impl LocalAiProviderConfigurationState {
    const PROTOCOL_STRINGS: [&'static str; 3] = [
        constants::local_ai_runtime::PROVIDER_CONFIGURATION_UNCONFIGURED,
        constants::local_ai_runtime::PROVIDER_CONFIGURATION_CONFIGURED,
        constants::local_ai_runtime::PROVIDER_CONFIGURATION_INVALID,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiAdapterReadinessState {
    #[serde(rename = "adapter-not-ready")]
    AdapterNotReady,
    #[serde(rename = "adapter-ready")]
    AdapterReady,
    #[serde(rename = "adapter-readiness-invalid")]
    AdapterReadinessInvalid,
}

impl LocalAiAdapterReadinessState {
    const PROTOCOL_STRINGS: [&'static str; 3] = [
        constants::local_ai_runtime::ADAPTER_READINESS_STATE_NOT_READY,
        constants::local_ai_runtime::ADAPTER_READINESS_STATE_READY,
        constants::local_ai_runtime::ADAPTER_READINESS_STATE_INVALID,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

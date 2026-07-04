use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiModelLoadState {
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "loading")]
    Loading,
    #[serde(rename = "loaded")]
    Loaded,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "failed")]
    Failed,
}

impl LocalAiModelLoadState {
    const PROTOCOL_STRINGS: [&'static str; 5] = [
        constants::local_ai_runtime::LOAD_STATE_UNAVAILABLE,
        constants::local_ai_runtime::LOAD_STATE_LOADING,
        constants::local_ai_runtime::LOAD_STATE_LOADED,
        constants::local_ai_runtime::LOAD_STATE_DEGRADED,
        constants::local_ai_runtime::LOAD_STATE_FAILED,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiCapabilityFlag {
    #[serde(rename = "classification")]
    Classification,
    #[serde(rename = "summarization")]
    Summarization,
    #[serde(rename = "embedding")]
    Embedding,
    #[serde(rename = "safety-decision")]
    SafetyDecision,
    #[serde(rename = "chat-completion")]
    ChatCompletion,
}

impl LocalAiCapabilityFlag {
    const PROTOCOL_STRINGS: [&'static str; 5] = [
        constants::local_ai_runtime::CAPABILITY_CLASSIFICATION,
        constants::local_ai_runtime::CAPABILITY_SUMMARIZATION,
        constants::local_ai_runtime::CAPABILITY_EMBEDDING,
        constants::local_ai_runtime::CAPABILITY_SAFETY_DECISION,
        constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiGenerationState {
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "timed-out")]
    TimedOut,
}

impl LocalAiGenerationState {
    const PROTOCOL_STRINGS: [&'static str; 5] = [
        constants::local_ai_runtime::GENERATION_STATE_UNAVAILABLE,
        constants::local_ai_runtime::GENERATION_STATE_RUNNING,
        constants::local_ai_runtime::GENERATION_STATE_COMPLETE,
        constants::local_ai_runtime::GENERATION_STATE_FAILED,
        constants::local_ai_runtime::GENERATION_STATE_TIMED_OUT,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiResourceClass {
    #[serde(rename = "cpu")]
    Cpu,
    #[serde(rename = "gpu")]
    Gpu,
    #[serde(rename = "npu")]
    Npu,
    #[serde(rename = "remote-unavailable")]
    RemoteUnavailable,
}

impl LocalAiResourceClass {
    const PROTOCOL_STRINGS: [&'static str; 4] = [
        constants::local_ai_runtime::RESOURCE_CPU,
        constants::local_ai_runtime::RESOURCE_GPU,
        constants::local_ai_runtime::RESOURCE_NPU,
        constants::local_ai_runtime::RESOURCE_REMOTE_UNAVAILABLE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiDegradedState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "provider-unavailable")]
    ProviderUnavailable,
    #[serde(rename = "model-load-failed")]
    ModelLoadFailed,
    #[serde(rename = "overloaded")]
    Overloaded,
    #[serde(rename = "invalid-output")]
    InvalidOutput,
}

impl LocalAiDegradedState {
    const PROTOCOL_STRINGS: [&'static str; 5] = [
        constants::local_ai_runtime::DEGRADED_NONE,
        constants::local_ai_runtime::DEGRADED_PROVIDER_UNAVAILABLE,
        constants::local_ai_runtime::DEGRADED_MODEL_LOAD_FAILED,
        constants::local_ai_runtime::DEGRADED_OVERLOADED,
        constants::local_ai_runtime::DEGRADED_INVALID_OUTPUT,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

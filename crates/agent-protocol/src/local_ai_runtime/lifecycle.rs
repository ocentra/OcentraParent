use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Unavailable => constants::local_ai_runtime::LOAD_STATE_UNAVAILABLE,
            Self::Loading => constants::local_ai_runtime::LOAD_STATE_LOADING,
            Self::Loaded => constants::local_ai_runtime::LOAD_STATE_LOADED,
            Self::Degraded => constants::local_ai_runtime::LOAD_STATE_DEGRADED,
            Self::Failed => constants::local_ai_runtime::LOAD_STATE_FAILED,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Classification => constants::local_ai_runtime::CAPABILITY_CLASSIFICATION,
            Self::Summarization => constants::local_ai_runtime::CAPABILITY_SUMMARIZATION,
            Self::Embedding => constants::local_ai_runtime::CAPABILITY_EMBEDDING,
            Self::SafetyDecision => constants::local_ai_runtime::CAPABILITY_SAFETY_DECISION,
            Self::ChatCompletion => constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Unavailable => constants::local_ai_runtime::GENERATION_STATE_UNAVAILABLE,
            Self::Running => constants::local_ai_runtime::GENERATION_STATE_RUNNING,
            Self::Complete => constants::local_ai_runtime::GENERATION_STATE_COMPLETE,
            Self::Failed => constants::local_ai_runtime::GENERATION_STATE_FAILED,
            Self::TimedOut => constants::local_ai_runtime::GENERATION_STATE_TIMED_OUT,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Cpu => constants::local_ai_runtime::RESOURCE_CPU,
            Self::Gpu => constants::local_ai_runtime::RESOURCE_GPU,
            Self::Npu => constants::local_ai_runtime::RESOURCE_NPU,
            Self::RemoteUnavailable => constants::local_ai_runtime::RESOURCE_REMOTE_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::None => constants::local_ai_runtime::DEGRADED_NONE,
            Self::ProviderUnavailable => constants::local_ai_runtime::DEGRADED_PROVIDER_UNAVAILABLE,
            Self::ModelLoadFailed => constants::local_ai_runtime::DEGRADED_MODEL_LOAD_FAILED,
            Self::Overloaded => constants::local_ai_runtime::DEGRADED_OVERLOADED,
            Self::InvalidOutput => constants::local_ai_runtime::DEGRADED_INVALID_OUTPUT,
        }
    }
}

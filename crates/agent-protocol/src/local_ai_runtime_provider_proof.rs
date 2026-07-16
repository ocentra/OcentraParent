use serde::{Deserialize, Serialize};

use crate::{
    constants::local_ai_runtime_provider_proof as proof_constants, DeviceRuntimeRole,
    LocalAiDegradedState, LocalAiProviderSchedulerJobClass, LocalAiProviderSchedulerLifecycle,
    LocalAiProviderSchedulerQueue, LocalAiProviderSchedulerStatus, LocalAiProviderSingletonScope,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiRuntimeProviderProofRequirement {
    #[serde(rename = "one-ai-provider-role-per-physical-device")]
    OneAiProviderRolePerPhysicalDevice,
    #[serde(rename = "shared-parent-child-provider")]
    SharedParentChildProvider,
    #[serde(rename = "single-local-runtime-lane")]
    SingleLocalRuntimeLane,
    #[serde(rename = "child-safety-priority")]
    ChildSafetyPriority,
    #[serde(rename = "queued-degraded-unavailable-lifecycle")]
    QueuedDegradedUnavailableLifecycle,
    #[serde(rename = "parent-assistant-submits-when-allowed")]
    ParentAssistantSubmitsWhenAllowed,
    #[serde(rename = "no-duplicate-local-model-load")]
    NoDuplicateLocalModelLoad,
    #[serde(rename = "provider-status-contract-hardening")]
    ProviderStatusContractHardening,
}

impl LocalAiRuntimeProviderProofRequirement {
    const PROTOCOL_STRINGS: [&'static str; 8] = [
        proof_constants::REQUIREMENT_ONE_PROVIDER_ROLE,
        proof_constants::REQUIREMENT_SHARED_PARENT_CHILD_PROVIDER,
        proof_constants::REQUIREMENT_SINGLE_RUNTIME_LANE,
        proof_constants::REQUIREMENT_CHILD_SAFETY_PRIORITY,
        proof_constants::REQUIREMENT_LIFECYCLE,
        proof_constants::REQUIREMENT_PARENT_ASSISTANT_SUBMIT,
        proof_constants::REQUIREMENT_NO_DUPLICATE_MODEL_LOAD,
        proof_constants::REQUIREMENT_STATUS_CONTRACT_HARDENING,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiRuntimeProviderProofStatus {
    #[serde(rename = "proved")]
    Proved,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "not-claimed")]
    NotClaimed,
}

impl LocalAiRuntimeProviderProofStatus {
    const PROTOCOL_STRINGS: [&'static str; 4] = [
        proof_constants::PROOF_STATUS_PROVED,
        proof_constants::PROOF_STATUS_DEGRADED,
        proof_constants::PROOF_STATUS_UNAVAILABLE,
        proof_constants::PROOF_STATUS_NOT_CLAIMED,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiRuntimeProviderProofEntry {
    pub schema_version: String,
    pub proof_entry_id: String,
    pub requirement: LocalAiRuntimeProviderProofRequirement,
    pub proof_status: LocalAiRuntimeProviderProofStatus,
    pub physical_device_id: String,
    pub singleton_scope: LocalAiProviderSingletonScope,
    pub provider_id: String,
    pub runtime_reference_id: String,
    pub model_id: String,
    pub model_reference: String,
    pub participating_roles: Vec<DeviceRuntimeRole>,
    pub accepted_job_classes: Vec<LocalAiProviderSchedulerJobClass>,
    pub scheduler_lifecycle: LocalAiProviderSchedulerLifecycle,
    pub source_scheduler_status: LocalAiProviderSchedulerStatus,
    pub runtime_access_lane_count: u16,
    pub runtime_load_count: u16,
    pub duplicate_runtime_blocked: bool,
    pub child_safety_priority_proved: bool,
    pub parent_assistant_submission_allowed: bool,
    pub queue: LocalAiProviderSchedulerQueue,
    pub degraded_state: LocalAiDegradedState,
    pub unavailable_reason: Option<String>,
    pub evidence_label: String,
    pub capability_requirement: String,
    pub proof_requirement: String,
    pub claim_boundary: String,
    pub fallback_behavior: String,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiRuntimeProviderProofReadModel {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub entries: Vec<LocalAiRuntimeProviderProofEntry>,
}

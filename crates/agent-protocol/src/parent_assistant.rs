use serde::{Deserialize, Serialize};

use crate::{
    FamilyReference, LocalAiDegradedState, LocalAiProviderSchedulerJobStatus,
    ParentActionReference, ParentActorReference, ParentDeviceReference, ParentEvidenceReference,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAssistantProviderState {
    #[serde(rename = "configured")]
    Configured,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAssistantAnswerState {
    #[serde(rename = "answered")]
    Answered,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAssistantApiAuthorizationState {
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "not-authorized")]
    NotAuthorized,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAssistantActionPreviewKind {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "policy-suggestion")]
    PolicySuggestion,
    #[serde(rename = "schedule-change")]
    ScheduleChange,
    #[serde(rename = "time-limit-change")]
    TimeLimitChange,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAssistantScope {
    pub family: FamilyReference,
    pub device: Option<ParentDeviceReference>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAssistantEvidenceContext {
    pub evidence: ParentEvidenceReference,
    pub citation_label: String,
    pub allowed_summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAssistantActionPreview {
    pub preview_id: Option<String>,
    pub action_kind: ParentAssistantActionPreviewKind,
    pub summary: Option<String>,
    pub action_reference: Option<ParentActionReference>,
    pub requires_controller_lease: bool,
    pub child_agent_contract_required: bool,
    pub enforcement_applied: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAssistantGenerateRequest {
    pub schema_version: String,
    pub request_id: String,
    pub thread_id: String,
    pub message_id: String,
    pub asked_at: String,
    pub actor: ParentActorReference,
    pub scope: ParentAssistantScope,
    pub question: String,
    pub evidence_context: Vec<ParentAssistantEvidenceContext>,
    pub model_id: Option<String>,
    pub max_output_tokens: u32,
    pub timeout_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAssistantApiProviderBoundary {
    pub schema_version: String,
    pub provider_id: String,
    pub authorization_state: ParentAssistantApiAuthorizationState,
    pub custody_label: String,
    pub retention_policy: String,
    pub deletion_policy: String,
    pub citations: Vec<ParentAssistantEvidenceContext>,
    pub provider_state: ParentAssistantProviderState,
    pub unavailable_reason: Option<String>,
    pub child_safety_or_enforcement_use_allowed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAssistantAnswer {
    pub schema_version: String,
    pub request_id: String,
    pub thread_id: String,
    pub message_id: String,
    pub answered_at: String,
    pub provider_id: String,
    pub model_id: String,
    pub provider_state: ParentAssistantProviderState,
    pub answer_state: ParentAssistantAnswerState,
    pub scheduler_job_status: LocalAiProviderSchedulerJobStatus,
    pub degraded_state: LocalAiDegradedState,
    pub unavailable_reason: Option<String>,
    pub local_ai_result_id: Option<String>,
    pub answer_text: Option<String>,
    pub citations: Vec<ParentAssistantEvidenceContext>,
    pub action_preview: ParentAssistantActionPreview,
    pub api_provider_boundary: ParentAssistantApiProviderBoundary,
    pub prompt_version: String,
}

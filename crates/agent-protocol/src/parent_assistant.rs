use serde::{Deserialize, Serialize};

use crate::{
    FamilyReference, LocalAiDegradedState, LocalAiProviderSchedulerJobStatus,
    LocalAiProviderSchedulerStatus, ParentActionReference, ParentActorReference,
    ParentDeviceReference, ParentEvidenceReference,
};

pub mod provider_route;

use provider_route::ParentAssistantProviderRoute;

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
pub enum ParentAssistantRunState {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
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
pub enum ParentAssistantApiProviderAccessState {
    #[serde(rename = "not-authorized")]
    NotAuthorized,
    #[serde(rename = "authorized-unavailable")]
    AuthorizedUnavailable,
    #[serde(rename = "authorized-degraded")]
    AuthorizedDegraded,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAssistantActionPreviewState {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAssistantBackendState {
    #[serde(rename = "runtime-backed")]
    RuntimeBacked,
    #[serde(rename = "durable-local")]
    DurableLocal,
    #[serde(rename = "volatile-local")]
    VolatileLocal,
    #[serde(rename = "contract-required")]
    ContractRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAssistantThreadState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "archived")]
    Archived,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAssistantRunCancelState {
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "not-running")]
    NotRunning,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAssistantActionConfirmState {
    #[serde(rename = "contract-required")]
    ContractRequired,
    #[serde(rename = "not-applied")]
    NotApplied,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAssistantChildAgentValidationState {
    #[serde(rename = "child-agent-contract-required")]
    ChildAgentContractRequired,
    #[serde(rename = "child-agent-offline")]
    ChildAgentOffline,
    #[serde(rename = "child-agent-unavailable")]
    ChildAgentUnavailable,
    #[serde(rename = "child-agent-degraded")]
    ChildAgentDegraded,
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
    pub custody_label: String,
    pub source_label: String,
    pub raw_child_evidence_included: bool,
    pub direct_enforcement_allowed: bool,
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
pub struct ParentAssistantActionPreviewResult {
    pub schema_version: String,
    pub backend_state: ParentAssistantBackendState,
    pub action_intent_id: String,
    pub preview_state: ParentAssistantActionPreviewState,
    pub preview: ParentAssistantActionPreview,
    pub evidence_context: Vec<ParentAssistantEvidenceContext>,
    pub preview_required: bool,
    pub preview_satisfied: bool,
    pub raw_assistant_prose_accepted: bool,
    pub parent_confirmation_required: bool,
    pub parent_confirmation_recorded: bool,
    pub child_agent_validation_state: ParentAssistantChildAgentValidationState,
    pub source_refs: Vec<ParentEvidenceReference>,
    pub audit_reason: String,
    pub requires_controller_lease: bool,
    pub child_agent_contract_required: bool,
    pub enforcement_applied: bool,
    pub policy_written: bool,
    pub reason: String,
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
    pub access_state: ParentAssistantApiProviderAccessState,
    pub parent_authorization_required: bool,
    pub evidence_citation_required: bool,
    pub custody_label: String,
    pub custody_state: String,
    pub retention_policy: String,
    pub retention_state: String,
    pub deletion_policy: String,
    pub deletion_state: String,
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
    pub run_state: ParentAssistantRunState,
    pub scheduler_job_status: LocalAiProviderSchedulerJobStatus,
    pub degraded_state: LocalAiDegradedState,
    pub unavailable_reason: Option<String>,
    pub local_ai_result_id: Option<String>,
    pub answer_text: Option<String>,
    pub citations: Vec<ParentAssistantEvidenceContext>,
    pub action_preview: ParentAssistantActionPreview,
    pub api_provider_boundary: ParentAssistantApiProviderBoundary,
    pub provider_route: ParentAssistantProviderRoute,
    pub prompt_version: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAssistantThreadRecord {
    pub schema_version: String,
    pub thread_id: String,
    pub title: String,
    pub state: ParentAssistantThreadState,
    pub backend_state: ParentAssistantBackendState,
    pub created_at: String,
    pub updated_at: String,
    pub message_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAssistantThreadResponse {
    pub schema_version: String,
    pub backend_state: ParentAssistantBackendState,
    pub active_thread: Option<ParentAssistantThreadRecord>,
    pub threads: Vec<ParentAssistantThreadRecord>,
    pub reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAssistantProviderStatus {
    pub schema_version: String,
    pub backend_state: ParentAssistantBackendState,
    pub provider_id: String,
    pub model_id: String,
    pub provider_state: ParentAssistantProviderState,
    pub run_state: ParentAssistantRunState,
    pub scheduler_job_status: LocalAiProviderSchedulerJobStatus,
    pub scheduler_status: LocalAiProviderSchedulerStatus,
    pub degraded_state: LocalAiDegradedState,
    pub unavailable_reason: Option<String>,
    pub queue_depth: u16,
    pub busy: bool,
    pub api_provider_boundary: ParentAssistantApiProviderBoundary,
    pub provider_route: ParentAssistantProviderRoute,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAssistantRunCancelResult {
    pub schema_version: String,
    pub backend_state: ParentAssistantBackendState,
    pub thread_id: String,
    pub run_id: String,
    pub cancel_state: ParentAssistantRunCancelState,
    pub run_state: ParentAssistantRunState,
    pub provider_state: ParentAssistantProviderState,
    pub unavailable_reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAssistantActionConfirmResult {
    pub schema_version: String,
    pub backend_state: ParentAssistantBackendState,
    pub action_intent_id: String,
    pub preview_id: Option<String>,
    pub action_kind: ParentAssistantActionPreviewKind,
    pub confirm_state: ParentAssistantActionConfirmState,
    pub preview_required: bool,
    pub preview_satisfied: bool,
    pub raw_assistant_prose_accepted: bool,
    pub parent_confirmation_required: bool,
    pub parent_confirmation_recorded: bool,
    pub child_agent_validation_state: ParentAssistantChildAgentValidationState,
    pub source_refs: Vec<ParentEvidenceReference>,
    pub audit_reason: String,
    pub requires_controller_lease: bool,
    pub child_agent_contract_required: bool,
    pub enforcement_applied: bool,
    pub policy_written: bool,
    pub reason: String,
}

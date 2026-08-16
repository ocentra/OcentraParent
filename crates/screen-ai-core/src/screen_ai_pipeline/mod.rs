#![forbid(unsafe_code)]

//! Screen AI pipeline ownership.
//!
//! Browser/app/game/screen sources produce triggers and evidence refs. This
//! crate decides when a screen-analysis work item may be requested and keeps
//! model output advisory until policy consumes validated evidence.

use ocentra_eventing::{
    envelope::EventContract, error::EventingError, ids::EventType, ids::IdempotencyKey,
    ids::SchemaVersion,
};
use ocentra_evidence::EvidenceReferenceState;
use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-screen-ai-core";
pub(crate) const SCREEN_AI_SCHEMA_VERSION: u16 = 1;
pub(crate) const SCREEN_AI_PIPELINE_EVALUATION_REQUESTED_EVENT_TYPE: &str =
    "screen-ai.pipeline-evaluation.requested";
pub(crate) const SCREEN_AI_PIPELINE_DECISION_RECORDED_EVENT_TYPE: &str =
    "screen-ai.pipeline-decision.recorded";
pub(crate) const SCREEN_AI_IDEMPOTENCY_SEPARATOR: &str = ":";
pub(crate) const SCREEN_AI_DECISION_PREFIX: &str = "screen-ai-decision";

mod events;
mod identifiers;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenAiTriggerSource {
    #[serde(rename = "app")]
    App,
    #[serde(rename = "browser")]
    Browser,
    #[serde(rename = "app-game")]
    AppGame,
    #[serde(rename = "screen-capture")]
    ScreenCapture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RawPrivateFrameState {
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "blocked")]
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenAiPolicyNeedState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenAiAnalysisRequestState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenAiRawFrameInclusionState {
    #[serde(rename = "include")]
    Include,
    #[serde(rename = "exclude")]
    Exclude,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenAiPolicyAuthorityState {
    #[serde(rename = "evidence-only")]
    EvidenceOnly,
    #[serde(rename = "claims-authority")]
    ClaimsAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenAiPipelineInput {
    pub trigger_source: ScreenAiTriggerSource,
    pub evidence_reference_state: EvidenceReferenceState,
    pub raw_private_frame_state: RawPrivateFrameState,
    pub policy_need_state: ScreenAiPolicyNeedState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenAiPipelineDecision {
    pub analysis_request_state: ScreenAiAnalysisRequestState,
    pub raw_frame_inclusion_state: ScreenAiRawFrameInclusionState,
    pub policy_authority_state: ScreenAiPolicyAuthorityState,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ScreenAiPipelineEvaluationId(String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ScreenAiPipelineDecisionId(String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ScreenAiAggregateId(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenAiPipelineEvaluationRequestedEvent {
    pub aggregate_id: ScreenAiAggregateId,
    pub evaluation_id: ScreenAiPipelineEvaluationId,
    pub input: ScreenAiPipelineInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenAiPipelineDecisionRecordedEvent {
    pub aggregate_id: ScreenAiAggregateId,
    pub decision_id: ScreenAiPipelineDecisionId,
    pub source_evaluation_id: ScreenAiPipelineEvaluationId,
    pub decision: ScreenAiPipelineDecision,
}

pub fn evaluate_screen_ai_pipeline(input: ScreenAiPipelineInput) -> ScreenAiPipelineDecision {
    crate::screen_ai_pipeline_logic::evaluate_screen_ai_pipeline(input)
}

pub fn record_screen_ai_pipeline_decision(
    event: &ScreenAiPipelineEvaluationRequestedEvent,
) -> ScreenAiPipelineDecisionRecordedEvent {
    crate::screen_ai_pipeline_logic::record_screen_ai_pipeline_decision(event)
}

pub(crate) fn screen_ai_decision_id(value: impl Into<String>) -> ScreenAiPipelineDecisionId {
    ScreenAiPipelineDecisionId(value.into())
}

pub(crate) fn screen_ai_decision_ref(evaluation_id: &ScreenAiPipelineEvaluationId) -> String {
    format!("{SCREEN_AI_DECISION_PREFIX}:{}", evaluation_id.as_str())
}

pub(crate) fn screen_ai_event_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(SCREEN_AI_SCHEMA_VERSION)?,
    ))
}

pub(crate) fn screen_ai_idempotency_key(
    event_type: &str,
    unique_ref: impl AsRef<str>,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type,
        SCREEN_AI_IDEMPOTENCY_SEPARATOR,
        unique_ref.as_ref()
    ))
}

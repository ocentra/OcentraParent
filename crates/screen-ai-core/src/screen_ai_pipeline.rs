#![forbid(unsafe_code)]

//! Screen AI pipeline ownership.
//!
//! Browser/app/game/screen sources produce triggers and evidence refs. This
//! crate decides when a screen-analysis work item may be requested and keeps
//! model output advisory until policy consumes validated evidence.

use ocentra_eventing::{
    envelope::DomainEvent, envelope::EventContract, error::EventingError, ids::AggregateKey,
    ids::EventType, ids::IdempotencyKey, ids::SchemaVersion,
};
use ocentra_evidence::EvidenceReferenceState;
use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-screen-ai-core";
const SCREEN_AI_SCHEMA_VERSION: u16 = 1;
const SCREEN_AI_PIPELINE_EVALUATION_REQUESTED_EVENT_TYPE: &str =
    "screen-ai.pipeline-evaluation.requested";
const SCREEN_AI_PIPELINE_DECISION_RECORDED_EVENT_TYPE: &str =
    "screen-ai.pipeline-decision.recorded";
const SCREEN_AI_IDEMPOTENCY_SEPARATOR: &str = ":";
const SCREEN_AI_DECISION_PREFIX: &str = "screen-ai-decision:";

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

macro_rules! screen_ai_text_id {
    ($name:ident, $field:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(EventingError::EmptyValue { field: $field });
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

screen_ai_text_id!(ScreenAiPipelineEvaluationId, "screen_ai.evaluation_id");
screen_ai_text_id!(ScreenAiPipelineDecisionId, "screen_ai.decision_id");
screen_ai_text_id!(ScreenAiAggregateId, "screen_ai.aggregate_id");

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

impl DomainEvent for ScreenAiPipelineEvaluationRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        screen_ai_event_contract(SCREEN_AI_PIPELINE_EVALUATION_REQUESTED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        screen_ai_idempotency_key(
            SCREEN_AI_PIPELINE_EVALUATION_REQUESTED_EVENT_TYPE,
            &self.evaluation_id,
        )
    }
}

impl DomainEvent for ScreenAiPipelineDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        screen_ai_event_contract(SCREEN_AI_PIPELINE_DECISION_RECORDED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        screen_ai_idempotency_key(
            SCREEN_AI_PIPELINE_DECISION_RECORDED_EVENT_TYPE,
            &self.decision_id,
        )
    }
}

pub fn evaluate_screen_ai_pipeline(input: ScreenAiPipelineInput) -> ScreenAiPipelineDecision {
    let request_analysis = input.evidence_reference_state == EvidenceReferenceState::Stable
        && input.policy_need_state == ScreenAiPolicyNeedState::Required;

    ScreenAiPipelineDecision {
        analysis_request_state: if request_analysis {
            ScreenAiAnalysisRequestState::Required
        } else {
            ScreenAiAnalysisRequestState::NotRequired
        },
        raw_frame_inclusion_state: if request_analysis
            && input.raw_private_frame_state == RawPrivateFrameState::Allowed
        {
            ScreenAiRawFrameInclusionState::Include
        } else {
            ScreenAiRawFrameInclusionState::Exclude
        },
        policy_authority_state: ScreenAiPolicyAuthorityState::EvidenceOnly,
    }
}

pub fn record_screen_ai_pipeline_decision(
    event: &ScreenAiPipelineEvaluationRequestedEvent,
) -> ScreenAiPipelineDecisionRecordedEvent {
    ScreenAiPipelineDecisionRecordedEvent {
        aggregate_id: event.aggregate_id.clone(),
        decision_id: ScreenAiPipelineDecisionId(screen_ai_decision_ref(&event.evaluation_id)),
        source_evaluation_id: event.evaluation_id.clone(),
        decision: evaluate_screen_ai_pipeline(event.input),
    }
}

fn screen_ai_event_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(SCREEN_AI_SCHEMA_VERSION)?,
    ))
}

fn screen_ai_idempotency_key(
    event_type: &str,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type, SCREEN_AI_IDEMPOTENCY_SEPARATOR, unique_ref
    ))
}

fn screen_ai_decision_ref(evaluation_id: &ScreenAiPipelineEvaluationId) -> String {
    let mut value = String::from(SCREEN_AI_DECISION_PREFIX);
    value.push_str(evaluation_id.as_str());
    value
}

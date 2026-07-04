use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainObservedEvent;
use ocentra_parent_screen_capture_adapter::{
    trigger_scheduler::ScreenCaptureScheduleDecision, ScreenCaptureAttempt,
};
use serde::{Deserialize, Serialize};

use crate::{screen_observed_event, ScreenObservationIntent};
#[path = "runtime_decision_ids.rs"]
mod ids;
pub type ScreenAggregateId = ids::ScreenAggregateId;
pub type ScreenRuntimeDecisionId = ids::ScreenRuntimeDecisionId;

const SCREEN_SCHEMA_VERSION: u16 = 1;
const SCREEN_RUNTIME_DECISION_RECORDED_EVENT_TYPE: &str = "screen.runtime.decision-recorded";
const SCREEN_IDEMPOTENCY_SEPARATOR: &str = ":";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenCaptureScheduleState {
    #[serde(rename = "capture-enqueued")]
    CaptureEnqueued,
    #[serde(rename = "capture-suppressed")]
    CaptureSuppressed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenContentSignalState {
    #[serde(rename = "ambiguous-content")]
    AmbiguousContent,
    #[serde(rename = "known-policy-state")]
    KnownPolicyState,
    #[serde(rename = "observation-only")]
    ObservationOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenRuntimeActionState {
    #[serde(rename = "suppress-capture")]
    SuppressCapture,
    #[serde(rename = "record-degraded-capture")]
    RecordDegradedCapture,
    #[serde(rename = "record-captured-evidence")]
    RecordCapturedEvidence,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenAiHandoffState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenPolicyHandoffState {
    #[serde(rename = "publish")]
    Publish,
    #[serde(rename = "do-not-publish")]
    DoNotPublish,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenRuntimeInput {
    pub capture_schedule_state: ScreenCaptureScheduleState,
    pub capture_capability_status: ActivityCaptureCapabilityStatus,
    pub content_signal_state: ScreenContentSignalState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenRuntimeDecision {
    pub observation_intent: ScreenObservationIntent,
    pub runtime_action_state: ScreenRuntimeActionState,
    pub ai_handoff_state: ScreenAiHandoffState,
    pub policy_handoff_state: ScreenPolicyHandoffState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenRuntimeDecisionRecordedEvent {
    pub aggregate_id: ScreenAggregateId,
    pub decision_id: ScreenRuntimeDecisionId,
    pub input: ScreenRuntimeInput,
    pub decision: ScreenRuntimeDecision,
}

impl DomainEvent for ScreenRuntimeDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(SCREEN_RUNTIME_DECISION_RECORDED_EVENT_TYPE)?,
            SchemaVersion::new(SCREEN_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(format!(
            "{}{}{}",
            SCREEN_RUNTIME_DECISION_RECORDED_EVENT_TYPE,
            SCREEN_IDEMPOTENCY_SEPARATOR,
            self.decision_id
        ))
    }
}

pub fn screen_runtime_input_from_capture(
    schedule_decision: ScreenCaptureScheduleDecision,
    capture_attempt: Option<&ScreenCaptureAttempt>,
    content_signal_state: ScreenContentSignalState,
) -> ScreenRuntimeInput {
    ScreenRuntimeInput {
        capture_schedule_state: match schedule_decision {
            ScreenCaptureScheduleDecision::EnqueueCapture { .. } => {
                ScreenCaptureScheduleState::CaptureEnqueued
            }
            ScreenCaptureScheduleDecision::SuppressCapture { .. } => {
                ScreenCaptureScheduleState::CaptureSuppressed
            }
        },
        capture_capability_status: capture_attempt
            .map(ScreenCaptureAttempt::status)
            .unwrap_or(ActivityCaptureCapabilityStatus::Unavailable),
        content_signal_state,
    }
}

pub fn evaluate_screen_runtime(input: &ScreenRuntimeInput) -> ScreenRuntimeDecision {
    if input.capture_schedule_state == ScreenCaptureScheduleState::CaptureSuppressed {
        return ScreenRuntimeDecision {
            observation_intent: ScreenObservationIntent::IdleObservationOnly,
            runtime_action_state: ScreenRuntimeActionState::SuppressCapture,
            ai_handoff_state: ScreenAiHandoffState::NotRequired,
            policy_handoff_state: ScreenPolicyHandoffState::DoNotPublish,
        };
    }

    if input.capture_capability_status != ActivityCaptureCapabilityStatus::Available {
        return ScreenRuntimeDecision {
            observation_intent: ScreenObservationIntent::CaptureCapabilityRequiresPolicy,
            runtime_action_state: ScreenRuntimeActionState::RecordDegradedCapture,
            ai_handoff_state: ScreenAiHandoffState::NotRequired,
            policy_handoff_state: ScreenPolicyHandoffState::Publish,
        };
    }

    match input.content_signal_state {
        ScreenContentSignalState::AmbiguousContent => ScreenRuntimeDecision {
            observation_intent: ScreenObservationIntent::AmbiguousContentRequiresAi,
            runtime_action_state: ScreenRuntimeActionState::RecordCapturedEvidence,
            ai_handoff_state: ScreenAiHandoffState::Required,
            policy_handoff_state: ScreenPolicyHandoffState::DoNotPublish,
        },
        ScreenContentSignalState::KnownPolicyState => ScreenRuntimeDecision {
            observation_intent: ScreenObservationIntent::KnownPolicyStateRequiresPolicy,
            runtime_action_state: ScreenRuntimeActionState::RecordCapturedEvidence,
            ai_handoff_state: ScreenAiHandoffState::NotRequired,
            policy_handoff_state: ScreenPolicyHandoffState::Publish,
        },
        ScreenContentSignalState::ObservationOnly => ScreenRuntimeDecision {
            observation_intent: ScreenObservationIntent::IdleObservationOnly,
            runtime_action_state: ScreenRuntimeActionState::RecordCapturedEvidence,
            ai_handoff_state: ScreenAiHandoffState::NotRequired,
            policy_handoff_state: ScreenPolicyHandoffState::DoNotPublish,
        },
    }
}

pub fn screen_runtime_observed_event(input: &ScreenRuntimeInput) -> ChildDomainObservedEvent {
    screen_observed_event(evaluate_screen_runtime(input).observation_intent)
}

pub fn screen_runtime_decision_recorded_event(
    aggregate_id: ScreenAggregateId,
    decision_id: ScreenRuntimeDecisionId,
    input: &ScreenRuntimeInput,
) -> ScreenRuntimeDecisionRecordedEvent {
    ScreenRuntimeDecisionRecordedEvent {
        aggregate_id,
        decision_id,
        input: input.clone(),
        decision: evaluate_screen_runtime(input),
    }
}

#![forbid(unsafe_code)]

use ocentra_eventing::{
    AggregateKey, DomainEvent, EventContract, EventType, EventingError, IdempotencyKey,
    SchemaVersion,
};
use ocentra_parent_agent_protocol::{
    child_domain_ai_analysis_requested_event_if_required,
    child_domain_direct_policy_evaluation_requested_event_if_required,
    child_domain_evidence_recorded_event, child_domain_observed_event,
    ChildDomainAiAnalysisRequestedEvent, ChildDomainAiAnalysisRequirement,
    ChildDomainEvidenceRecordedEvent, ChildDomainObservedEvent, ChildDomainObservedEventProfile,
    ChildDomainObservedSignal, ChildDomainPolicyEvaluationRequestedEvent,
    ChildDomainPolicyEvaluationRequirement, ChildDomainRefSuffix, ChildRuntimeDomain,
};
use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-network-core";
const NETWORK_SCHEMA_VERSION: u16 = 1;
const NETWORK_RUNTIME_DECISION_RECORDED_EVENT_TYPE: &str = "network.runtime.decision-recorded";
const NETWORK_IDEMPOTENCY_SEPARATOR: &str = ":";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkObservationIntent {
    #[serde(rename = "flow-requires-policy")]
    FlowRequiresPolicy,
    #[serde(rename = "unknown-route-requires-ai")]
    UnknownRouteRequiresAi,
    #[serde(rename = "telemetry-observation-only")]
    TelemetryObservationOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAdapterState {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "missing")]
    Missing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkCapturePermissionState {
    #[serde(rename = "granted")]
    Granted,
    #[serde(rename = "missing")]
    Missing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkParserState {
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "drifted")]
    Drifted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRuntimeActionState {
    #[serde(rename = "capture-and-record")]
    CaptureAndRecord,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkPolicyHandoffState {
    #[serde(rename = "publish")]
    Publish,
    #[serde(rename = "do-not-publish")]
    DoNotPublish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRuntimeInput {
    pub adapter_state: NetworkAdapterState,
    pub capture_permission_state: NetworkCapturePermissionState,
    pub parser_state: NetworkParserState,
    pub observation_intent: NetworkObservationIntent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRuntimeDecision {
    pub runtime_action_state: NetworkRuntimeActionState,
    pub policy_handoff_state: NetworkPolicyHandoffState,
}

macro_rules! network_text_id {
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

network_text_id!(NetworkRuntimeDecisionId, "network.runtime_decision_id");
network_text_id!(NetworkAggregateId, "network.aggregate_id");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRuntimeDecisionRecordedEvent {
    pub aggregate_id: NetworkAggregateId,
    pub decision_id: NetworkRuntimeDecisionId,
    pub input: NetworkRuntimeInput,
    pub decision: NetworkRuntimeDecision,
}

impl DomainEvent for NetworkRuntimeDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(NETWORK_RUNTIME_DECISION_RECORDED_EVENT_TYPE)?,
            SchemaVersion::new(NETWORK_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(format!(
            "{}{}{}",
            NETWORK_RUNTIME_DECISION_RECORDED_EVENT_TYPE,
            NETWORK_IDEMPOTENCY_SEPARATOR,
            self.decision_id,
        ))
    }
}

pub fn default_network_observed_event() -> ChildDomainObservedEvent {
    network_observed_event(NetworkObservationIntent::FlowRequiresPolicy)
}

pub fn network_observed_event(intent: NetworkObservationIntent) -> ChildDomainObservedEvent {
    child_domain_observed_event(network_observed_profile(intent))
}

pub fn network_observed_profile(
    intent: NetworkObservationIntent,
) -> ChildDomainObservedEventProfile {
    let (observed_state, ai_analysis_requirement, policy_evaluation_requirement) = match intent {
        NetworkObservationIntent::FlowRequiresPolicy => (
            ChildDomainObservedSignal::RequiresPolicy,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::Required,
        ),
        NetworkObservationIntent::UnknownRouteRequiresAi => (
            ChildDomainObservedSignal::RequiresAi,
            ChildDomainAiAnalysisRequirement::Required,
            ChildDomainPolicyEvaluationRequirement::Required,
        ),
        NetworkObservationIntent::TelemetryObservationOnly => (
            ChildDomainObservedSignal::ObserveOnly,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::NotRequired,
        ),
    };

    ChildDomainObservedEventProfile {
        domain: ChildRuntimeDomain::Network,
        subject_ref_suffix: ChildDomainRefSuffix::NetworkSubject,
        observed_state,
        ai_analysis_requirement,
        policy_evaluation_requirement,
    }
}

pub fn network_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> ChildDomainEvidenceRecordedEvent {
    child_domain_evidence_recorded_event(event)
}

pub fn network_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainAiAnalysisRequestedEvent> {
    child_domain_ai_analysis_requested_event_if_required(event)
}

pub fn network_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    child_domain_direct_policy_evaluation_requested_event_if_required(event)
}

pub fn evaluate_network_runtime(input: NetworkRuntimeInput) -> NetworkRuntimeDecision {
    let can_capture = input.adapter_state == NetworkAdapterState::Available
        && input.capture_permission_state == NetworkCapturePermissionState::Granted
        && input.parser_state == NetworkParserState::Valid;
    let should_publish_policy =
        input.observation_intent == NetworkObservationIntent::FlowRequiresPolicy;

    NetworkRuntimeDecision {
        runtime_action_state: if can_capture {
            NetworkRuntimeActionState::CaptureAndRecord
        } else {
            NetworkRuntimeActionState::ManualRequired
        },
        policy_handoff_state: if can_capture && should_publish_policy {
            NetworkPolicyHandoffState::Publish
        } else {
            NetworkPolicyHandoffState::DoNotPublish
        },
    }
}

pub fn network_runtime_decision_recorded_event(
    aggregate_id: NetworkAggregateId,
    decision_id: NetworkRuntimeDecisionId,
    input: NetworkRuntimeInput,
) -> NetworkRuntimeDecisionRecordedEvent {
    NetworkRuntimeDecisionRecordedEvent {
        aggregate_id,
        decision_id,
        input,
        decision: evaluate_network_runtime(input),
    }
}

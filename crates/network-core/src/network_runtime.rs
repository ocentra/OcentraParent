use ocentra_parent_agent_protocol::child_domain_runtime::{
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
pub enum NetworkAiHandoffState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
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
    pub observation_intent: NetworkObservationIntent,
    pub runtime_action_state: NetworkRuntimeActionState,
    pub ai_handoff_state: NetworkAiHandoffState,
    pub policy_handoff_state: NetworkPolicyHandoffState,
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

    ChildRuntimeDomain::Network.observed_profile(
        ChildDomainRefSuffix::NetworkSubject,
        observed_state,
        ai_analysis_requirement,
        policy_evaluation_requirement,
    )
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
    let observation_intent = runtime_observation_intent(input);

    NetworkRuntimeDecision {
        observation_intent,
        runtime_action_state: if can_capture(input) {
            NetworkRuntimeActionState::CaptureAndRecord
        } else {
            NetworkRuntimeActionState::ManualRequired
        },
        ai_handoff_state: if observation_intent == NetworkObservationIntent::UnknownRouteRequiresAi
        {
            NetworkAiHandoffState::Required
        } else {
            NetworkAiHandoffState::NotRequired
        },
        policy_handoff_state: if observation_intent == NetworkObservationIntent::FlowRequiresPolicy
        {
            NetworkPolicyHandoffState::Publish
        } else {
            NetworkPolicyHandoffState::DoNotPublish
        },
    }
}

fn can_capture(input: NetworkRuntimeInput) -> bool {
    input.adapter_state == NetworkAdapterState::Available
        && input.capture_permission_state == NetworkCapturePermissionState::Granted
        && input.parser_state == NetworkParserState::Valid
}

fn runtime_observation_intent(input: NetworkRuntimeInput) -> NetworkObservationIntent {
    if can_capture(input) {
        input.observation_intent
    } else {
        NetworkObservationIntent::TelemetryObservationOnly
    }
}

#[path = "network_runtime_stream_event_values/capture.rs"]
mod capture;
#[path = "network_runtime_stream_event_values/classification.rs"]
mod classification;
#[path = "network_runtime_stream_event_values/common.rs"]
mod common;

use serde::Serialize;
use serde_json::Value;

use ocentra_parent_agent_protocol::network_flow::{
    NetworkAiAdvisoryState, NetworkClaimBoundary, NetworkDomainAttributionKind,
    NetworkEnforcementMode, NetworkEnforcementResultStatus, NetworkEvidenceGrade,
    NetworkPolicyDecisionAction, NetworkPortalUpdateKind, NetworkRuntimeEventPayload,
};

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub(crate) struct NetworkRuntimeStreamRef(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub(crate) struct NetworkRuntimeStreamText(pub(crate) String);

pub(crate) fn no_claim_boundary() -> NetworkClaimBoundary {
    common::no_claim_boundary()
}

pub(crate) fn custody(payload: &NetworkRuntimeEventPayload) -> NetworkRuntimeStreamText {
    capture::custody(payload)
}

pub(crate) fn evidence_grade(payload: &NetworkRuntimeEventPayload) -> NetworkEvidenceGrade {
    payload.evidence_grade_contract
}

pub(crate) fn domain_attribution(
    payload: &NetworkRuntimeEventPayload,
) -> NetworkDomainAttributionKind {
    classification::domain_attribution(payload)
}

pub(crate) fn ai_advisory_state(payload: &NetworkRuntimeEventPayload) -> NetworkAiAdvisoryState {
    classification::ai_advisory_state(payload)
}

pub(crate) fn policy_decision_action(
    payload: &NetworkRuntimeEventPayload,
) -> NetworkPolicyDecisionAction {
    payload.policy_action
}

pub(crate) fn enforcement_mode(payload: &NetworkRuntimeEventPayload) -> NetworkEnforcementMode {
    classification::enforcement_mode(payload)
}

pub(crate) fn enforcement_result_status(
    payload: &NetworkRuntimeEventPayload,
) -> NetworkEnforcementResultStatus {
    classification::enforcement_result_status(payload)
}

pub(crate) fn portal_update_kind(payload: &NetworkRuntimeEventPayload) -> NetworkPortalUpdateKind {
    classification::portal_update_kind(payload)
}

pub(crate) fn unavailable_reason_code(
    payload: &NetworkRuntimeEventPayload,
) -> Option<NetworkRuntimeStreamText> {
    classification::unavailable_reason_code(payload)
}

pub(crate) fn confidence(payload: &NetworkRuntimeEventPayload) -> f32 {
    capture::confidence(payload)
}

pub(crate) fn previous_event_ref(payload: &NetworkRuntimeEventPayload) -> NetworkRuntimeStreamRef {
    common::previous_event_ref(payload)
}

pub(crate) fn ref_or_current(
    value: &Option<NetworkRuntimeStreamRef>,
    event_ref: &NetworkRuntimeStreamRef,
) -> NetworkRuntimeStreamRef {
    common::ref_or_current(value, event_ref)
}

pub(crate) fn evidence_refs(payload: &NetworkRuntimeEventPayload) -> Vec<NetworkRuntimeStreamRef> {
    common::evidence_refs(payload)
}

pub(crate) fn parent_rule_refs() -> Vec<NetworkRuntimeStreamRef> {
    common::parent_rule_refs()
}

pub(crate) fn uncertainty_codes() -> Vec<NetworkRuntimeStreamText> {
    common::uncertainty_codes()
}

pub(crate) fn json_value<T>(value: T) -> Value
where
    T: Serialize,
{
    common::json_value(value)
}

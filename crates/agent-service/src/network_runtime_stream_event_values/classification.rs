#[path = "classification/advisory.rs"]
mod advisory;
#[path = "classification/domain.rs"]
mod domain;
#[path = "classification/enforcement.rs"]
mod enforcement;

use ocentra_parent_agent_protocol::network_flow::{
    NetworkAiAdvisoryState, NetworkDomainAttributionKind, NetworkEnforcementMode,
    NetworkEnforcementResultStatus, NetworkPortalUpdateKind, NetworkRuntimeEventPayload,
};

use super::NetworkRuntimeStreamText;

pub(crate) fn domain_attribution(
    payload: &NetworkRuntimeEventPayload,
) -> NetworkDomainAttributionKind {
    domain::domain_attribution(payload)
}

pub(crate) fn ai_advisory_state(payload: &NetworkRuntimeEventPayload) -> NetworkAiAdvisoryState {
    advisory::ai_advisory_state(payload)
}

pub(crate) fn enforcement_mode(payload: &NetworkRuntimeEventPayload) -> NetworkEnforcementMode {
    enforcement::enforcement_mode(payload)
}

pub(crate) fn enforcement_result_status(
    payload: &NetworkRuntimeEventPayload,
) -> NetworkEnforcementResultStatus {
    enforcement::enforcement_result_status(payload)
}

pub(crate) fn portal_update_kind(payload: &NetworkRuntimeEventPayload) -> NetworkPortalUpdateKind {
    enforcement::portal_update_kind(payload)
}

pub(crate) fn unavailable_reason_code(
    payload: &NetworkRuntimeEventPayload,
) -> Option<NetworkRuntimeStreamText> {
    enforcement::unavailable_reason_code(payload)
}

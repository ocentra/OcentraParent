use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{
    NetworkEnforcementMode, NetworkEnforcementResultStatus, NetworkInterventionState,
    NetworkPortalUpdateKind, NetworkRuntimeEventPayload,
};

use super::NetworkRuntimeStreamText;

pub(crate) fn enforcement_mode(payload: &NetworkRuntimeEventPayload) -> NetworkEnforcementMode {
    match payload.intervention_state {
        NetworkInterventionState::DryRunOnly => NetworkEnforcementMode::DryRun,
        NetworkInterventionState::ManualRequired => NetworkEnforcementMode::ManualRequired,
        NetworkInterventionState::Unavailable => NetworkEnforcementMode::Unavailable,
    }
}

pub(crate) fn enforcement_result_status(
    payload: &NetworkRuntimeEventPayload,
) -> NetworkEnforcementResultStatus {
    match payload.intervention_state {
        NetworkInterventionState::DryRunOnly => NetworkEnforcementResultStatus::DryRun,
        NetworkInterventionState::ManualRequired => NetworkEnforcementResultStatus::ManualRequired,
        NetworkInterventionState::Unavailable => NetworkEnforcementResultStatus::Unavailable,
    }
}

pub(crate) fn portal_update_kind(payload: &NetworkRuntimeEventPayload) -> NetworkPortalUpdateKind {
    match payload.intervention_state {
        NetworkInterventionState::DryRunOnly => NetworkPortalUpdateKind::NetworkReadModel,
        NetworkInterventionState::ManualRequired => NetworkPortalUpdateKind::ManualRequiredState,
        NetworkInterventionState::Unavailable => NetworkPortalUpdateKind::CapabilityState,
    }
}

pub(crate) fn unavailable_reason_code(
    payload: &NetworkRuntimeEventPayload,
) -> Option<NetworkRuntimeStreamText> {
    (payload.intervention_state != NetworkInterventionState::DryRunOnly).then(|| {
        NetworkRuntimeStreamText(
            constants::network_flow::UNAVAILABLE_REASON_MANUAL_REQUIRED.to_string(),
        )
    })
}

use ocentra_network_core::network_runtime::{
    evaluate_network_runtime, NetworkAdapterState, NetworkCapturePermissionState,
    NetworkObservationIntent, NetworkParserState, NetworkRuntimeDecision, NetworkRuntimeInput,
};
use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::network_flow::NetworkRuntimeEvidenceGrade;

use crate::NetworkObservation;

pub(super) fn network_runtime_decision_from_observation(
    observation: &NetworkObservation,
) -> NetworkRuntimeDecision {
    evaluate_network_runtime(NetworkRuntimeInput {
        adapter_state: adapter_state(observation.status),
        capture_permission_state: permission_state(observation.status),
        parser_state: NetworkParserState::Valid,
        observation_intent: observation_intent(observation),
    })
}

fn adapter_state(status: ActivityCaptureCapabilityStatus) -> NetworkAdapterState {
    match status {
        ActivityCaptureCapabilityStatus::Available => NetworkAdapterState::Available,
        _ => NetworkAdapterState::Missing,
    }
}

fn permission_state(status: ActivityCaptureCapabilityStatus) -> NetworkCapturePermissionState {
    match status {
        ActivityCaptureCapabilityStatus::Available => NetworkCapturePermissionState::Granted,
        _ => NetworkCapturePermissionState::Missing,
    }
}

fn observation_intent(observation: &NetworkObservation) -> NetworkObservationIntent {
    match evidence_grade(observation) {
        NetworkRuntimeEvidenceGrade::AdapterUnavailable => {
            NetworkObservationIntent::TelemetryObservationOnly
        }
        _ if observation.destination_domain.is_some() => {
            NetworkObservationIntent::FlowRequiresPolicy
        }
        _ => NetworkObservationIntent::UnknownRouteRequiresAi,
    }
}

fn evidence_grade(observation: &NetworkObservation) -> NetworkRuntimeEvidenceGrade {
    crate::network_event_runtime_state::evidence_grade(observation)
}

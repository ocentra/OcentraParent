use super::{
    NetworkLinuxAdapterCapabilityState, NetworkLinuxAdapterGateBoundaryReason,
    NetworkLinuxAdapterGateState,
};

pub(super) fn gate_state(
    research_only: bool,
    capability_state: NetworkLinuxAdapterCapabilityState,
    boundary_reasons: &[NetworkLinuxAdapterGateBoundaryReason],
) -> NetworkLinuxAdapterGateState {
    if research_only {
        return NetworkLinuxAdapterGateState::ResearchOnly;
    }
    if capability_state == NetworkLinuxAdapterCapabilityState::Unavailable {
        return NetworkLinuxAdapterGateState::Unavailable;
    }
    if boundary_reasons.is_empty() {
        NetworkLinuxAdapterGateState::DistroProofReady
    } else {
        NetworkLinuxAdapterGateState::ManualRequired
    }
}

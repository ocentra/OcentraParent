use ocentra_network_evidence::windows_wfp_gate::{
    NetworkWindowsWfpGateCapabilityState, NetworkWindowsWfpGateState,
};
use ocentra_parent_agent_protocol::network_windows_wfp_gate_status::{
    NetworkWindowsWfpGateCapabilityStatusState, NetworkWindowsWfpGateStatusState,
};

pub(super) fn protocol_capability_state(
    state: NetworkWindowsWfpGateCapabilityState,
) -> NetworkWindowsWfpGateCapabilityStatusState {
    match state {
        NetworkWindowsWfpGateCapabilityState::LabReady => {
            NetworkWindowsWfpGateCapabilityStatusState::LabReady
        }
        NetworkWindowsWfpGateCapabilityState::ManualRequired => {
            NetworkWindowsWfpGateCapabilityStatusState::ManualRequired
        }
        NetworkWindowsWfpGateCapabilityState::Unavailable => {
            NetworkWindowsWfpGateCapabilityStatusState::Unavailable
        }
    }
}

pub(super) fn protocol_gate_state(
    state: NetworkWindowsWfpGateState,
) -> NetworkWindowsWfpGateStatusState {
    match state {
        NetworkWindowsWfpGateState::ResearchOnly => NetworkWindowsWfpGateStatusState::ResearchOnly,
        NetworkWindowsWfpGateState::ManualRequired => {
            NetworkWindowsWfpGateStatusState::ManualRequired
        }
        NetworkWindowsWfpGateState::Unavailable => NetworkWindowsWfpGateStatusState::Unavailable,
        NetworkWindowsWfpGateState::LabProofReady => {
            NetworkWindowsWfpGateStatusState::LabProofReady
        }
    }
}

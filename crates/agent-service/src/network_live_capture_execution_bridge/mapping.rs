use ocentra_network_evidence::{
    live_capture::NetworkLiveCaptureProof, live_capture_execution::NetworkLiveCaptureExecutionState,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::NetworkLiveCaptureExecutionStatusState;

pub(super) fn protocol_execution_state(
    state: NetworkLiveCaptureExecutionState,
) -> NetworkLiveCaptureExecutionStatusState {
    match state {
        NetworkLiveCaptureExecutionState::BoundedExecuted => {
            NetworkLiveCaptureExecutionStatusState::BoundedExecuted
        }
        NetworkLiveCaptureExecutionState::ManualRequired => {
            NetworkLiveCaptureExecutionStatusState::ManualRequired
        }
        NetworkLiveCaptureExecutionState::Unavailable => {
            NetworkLiveCaptureExecutionStatusState::Unavailable
        }
        NetworkLiveCaptureExecutionState::Degraded => {
            NetworkLiveCaptureExecutionStatusState::Degraded
        }
    }
}

pub(super) fn execution_ref_index(proof: &NetworkLiveCaptureProof) -> usize {
    match proof.capture_proof_ref.as_str() {
        constants::network_flow::TEST_LIVE_CAPTURE_WINDOWS_PROOF_REF => 0,
        constants::network_flow::TEST_LIVE_CAPTURE_LINUX_PROOF_REF => 1,
        constants::network_flow::TEST_LIVE_CAPTURE_MACOS_PROOF_REF => 2,
        _ => 3,
    }
}

use ocentra_network_evidence::live_capture::NetworkLiveCaptureProofState;
use ocentra_parent_agent_protocol::network_flow::NetworkLiveCaptureProofStatusState;

pub(super) fn protocol_proof_state(
    state: NetworkLiveCaptureProofState,
) -> NetworkLiveCaptureProofStatusState {
    match state {
        NetworkLiveCaptureProofState::ProofReady => NetworkLiveCaptureProofStatusState::ProofReady,
        NetworkLiveCaptureProofState::ManualRequired => {
            NetworkLiveCaptureProofStatusState::ManualRequired
        }
        NetworkLiveCaptureProofState::Unavailable => {
            NetworkLiveCaptureProofStatusState::Unavailable
        }
        NetworkLiveCaptureProofState::Degraded => NetworkLiveCaptureProofStatusState::Degraded,
    }
}

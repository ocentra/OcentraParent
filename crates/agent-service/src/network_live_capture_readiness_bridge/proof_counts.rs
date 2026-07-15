use ocentra_parent_agent_protocol::network_flow::{
    NetworkLiveCaptureProofStatusState, NetworkLiveCaptureStatus,
};

pub(super) fn apply_proof_counts(
    status: &mut NetworkLiveCaptureStatus,
    proof_state: &NetworkLiveCaptureProofStatusState,
) {
    match proof_state {
        NetworkLiveCaptureProofStatusState::ProofReady => status.proof_ready_count += 1,
        NetworkLiveCaptureProofStatusState::ManualRequired => status.manual_required_count += 1,
        NetworkLiveCaptureProofStatusState::Unavailable => status.unavailable_count += 1,
        NetworkLiveCaptureProofStatusState::Degraded => status.degraded_count += 1,
    }
}

use super::{NetworkLiveCaptureProofInput, NetworkLiveCaptureProofState};

pub(super) fn proof_state(
    input: &NetworkLiveCaptureProofInput,
    missing_artifacts: &[super::NetworkLiveCaptureRequiredArtifact],
) -> NetworkLiveCaptureProofState {
    if !input.platform_available {
        NetworkLiveCaptureProofState::Unavailable
    } else if input.adapter_degraded {
        NetworkLiveCaptureProofState::Degraded
    } else if missing_artifacts.is_empty() {
        NetworkLiveCaptureProofState::ProofReady
    } else {
        NetworkLiveCaptureProofState::ManualRequired
    }
}

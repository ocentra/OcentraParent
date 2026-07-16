use super::*;

pub(super) fn push_state_reasons(
    input: &NetworkActionResultInput,
    reasons: &mut Vec<NetworkActionResultBoundaryReason>,
) {
    if input.dry_run {
        reasons.push(NetworkActionResultBoundaryReason::DryRunRequested);
    }
    match input.capability_state {
        NetworkActionResultCapabilityState::Supported => {}
        NetworkActionResultCapabilityState::ManualRequired => {
            reasons.push(NetworkActionResultBoundaryReason::CapabilityManualRequired);
        }
        NetworkActionResultCapabilityState::Unavailable => {
            reasons.push(NetworkActionResultBoundaryReason::CapabilityUnavailable);
        }
    }
    match input.adapter_proof_state {
        NetworkActionResultAdapterProofState::ApplyReady => {}
        NetworkActionResultAdapterProofState::DryRun => {
            reasons.push(NetworkActionResultBoundaryReason::AdapterProofDryRun);
        }
        NetworkActionResultAdapterProofState::ManualRequired => {
            reasons.push(NetworkActionResultBoundaryReason::AdapterProofManualRequired);
        }
        NetworkActionResultAdapterProofState::Unavailable => {
            reasons.push(NetworkActionResultBoundaryReason::AdapterProofUnavailable);
        }
    }
}

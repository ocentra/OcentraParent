use super::*;

pub(super) fn result_state(
    input: &NetworkActionResultInput,
    reasons: &[NetworkActionResultBoundaryReason],
) -> NetworkActionResultState {
    if input.dry_run || input.adapter_proof_state == NetworkActionResultAdapterProofState::DryRun {
        return NetworkActionResultState::DryRun;
    }
    if input.capability_state == NetworkActionResultCapabilityState::Unavailable
        || input.adapter_proof_state == NetworkActionResultAdapterProofState::Unavailable
    {
        return NetworkActionResultState::Unavailable;
    }
    if reasons.is_empty() {
        return match input.requested_action {
            NetworkActionResultRequestedAction::Block => NetworkActionResultState::Blocked,
            NetworkActionResultRequestedAction::TerminateProcess => {
                NetworkActionResultState::Terminated
            }
        };
    }
    NetworkActionResultState::ManualRequired
}

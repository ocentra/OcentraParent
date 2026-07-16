mod artifacts;
mod policy;
mod result;
mod state;

use super::*;

pub(super) fn boundary_reasons(
    input: &NetworkActionResultInput,
    has_required_artifacts: bool,
) -> Vec<NetworkActionResultBoundaryReason> {
    let mut reasons = Vec::new();
    state::push_state_reasons(input, &mut reasons);
    policy::push_policy_reasons(input, has_required_artifacts, &mut reasons);
    reasons
}

pub(super) fn missing_required_artifacts(
    artifacts: &NetworkActionResultArtifactRefs,
) -> Vec<NetworkActionResultRequiredArtifact> {
    artifacts::missing_required_artifacts(artifacts)
}

pub(super) fn result_state(
    input: &NetworkActionResultInput,
    reasons: &[NetworkActionResultBoundaryReason],
) -> NetworkActionResultState {
    result::result_state(input, reasons)
}

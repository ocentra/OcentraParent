mod artifacts;
mod claims;
mod input;
mod refs;

use super::*;

pub(super) fn reject_unsupported_claims(
    input: &NetworkActionResultInput,
) -> Result<(), NetworkActionResultError> {
    claims::reject_unsupported_claims(input)
}

pub(super) fn normalize_action_result_input(
    input: &NetworkActionResultInput,
) -> Result<NormalizedActionResultInput, NetworkActionResultError> {
    input::normalize_action_result_input(input)
}

pub(super) fn normalize_artifact_refs(
    input: &NetworkActionResultInput,
) -> Result<NetworkActionResultArtifactRefs, NetworkActionResultError> {
    artifacts::normalize_artifact_refs(input)
}

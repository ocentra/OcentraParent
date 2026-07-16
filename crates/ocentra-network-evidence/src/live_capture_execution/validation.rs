use crate::live_capture::NetworkLiveCapturePlatform;

use super::{
    NetworkLiveCaptureExecutionError, NetworkLiveCaptureExecutionInput,
    NetworkLiveCaptureExecutionRequiredArtifact, NetworkLiveCaptureExecutionSource,
};

mod artifacts;
mod claims;
mod shape;
mod source;

pub(super) fn validate_input(
    input: &NetworkLiveCaptureExecutionInput,
) -> Result<(), NetworkLiveCaptureExecutionError> {
    source::validate_input(input)?;
    shape::validate_input(input)?;
    claims::validate_input(input)
}

pub(super) fn missing_artifacts(
    input: &NetworkLiveCaptureExecutionInput,
) -> Vec<NetworkLiveCaptureExecutionRequiredArtifact> {
    artifacts::missing_artifacts(input)
}

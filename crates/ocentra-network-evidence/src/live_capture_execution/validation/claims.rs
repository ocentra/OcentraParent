use super::{NetworkLiveCaptureExecutionError, NetworkLiveCaptureExecutionInput};

#[path = "rejections.rs"]
mod rejections;

pub(super) fn validate_input(
    input: &NetworkLiveCaptureExecutionInput,
) -> Result<(), NetworkLiveCaptureExecutionError> {
    if input.raw_artifact_created {
        return Err(NetworkLiveCaptureExecutionError::RawArtifactCreationRejected);
    }
    if input.netstat_metadata_substitution_claimed {
        return Err(NetworkLiveCaptureExecutionError::NetstatSubstitutionClaimRejected);
    }
    if input.unbounded_capture_claimed {
        return Err(NetworkLiveCaptureExecutionError::UnboundedCaptureClaimRejected);
    }
    if input.raw_pcap_without_custody_claimed {
        return Err(NetworkLiveCaptureExecutionError::RawPcapWithoutCustodyClaimRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkLiveCaptureExecutionError::ExactUrlClaimRejected);
    }
    rejections::validate_input(input)
}

use crate::live_capture::{NetworkLiveCapturePlatform, NetworkLiveCaptureProofState};

use super::{
    NetworkLiveCaptureExecutionError, NetworkLiveCaptureExecutionInput,
    NetworkLiveCaptureExecutionRequiredArtifact, NetworkLiveCaptureExecutionSource,
};

pub(super) fn validate_input(
    input: &NetworkLiveCaptureExecutionInput,
) -> Result<(), NetworkLiveCaptureExecutionError> {
    if input.execution_ref.trim().is_empty() {
        return Err(NetworkLiveCaptureExecutionError::EmptyExecutionRef);
    }
    for artifact_ref in artifact_refs(input).into_iter().flatten() {
        if artifact_ref.trim().is_empty() {
            return Err(NetworkLiveCaptureExecutionError::EmptyArtifactRef);
        }
    }
    validate_source_platform(input)?;
    validate_execution_shape(input)?;
    validate_claims(input)
}

pub(super) fn missing_artifacts(
    input: &NetworkLiveCaptureExecutionInput,
) -> Vec<NetworkLiveCaptureExecutionRequiredArtifact> {
    let mut missing = Vec::new();
    require(
        input.live_capture_proof.proof_state == NetworkLiveCaptureProofState::ProofReady,
        NetworkLiveCaptureExecutionRequiredArtifact::ProofReadyLiveCapture,
        &mut missing,
    );
    require_execution_artifacts(input, &mut missing);
    require_custody_artifacts(input, &mut missing);
    missing
}

fn validate_source_platform(
    input: &NetworkLiveCaptureExecutionInput,
) -> Result<(), NetworkLiveCaptureExecutionError> {
    let matches_platform = match input.source {
        NetworkLiveCaptureExecutionSource::WindowsNpcapDriver => {
            input.live_capture_proof.platform == NetworkLiveCapturePlatform::WindowsNpcap
        }
        NetworkLiveCaptureExecutionSource::LinuxLibpcapDriver => {
            input.live_capture_proof.platform == NetworkLiveCapturePlatform::LinuxLibpcap
        }
        NetworkLiveCaptureExecutionSource::MacosBpfLibpcapDriver => {
            input.live_capture_proof.platform == NetworkLiveCapturePlatform::MacosBpfLibpcap
        }
        NetworkLiveCaptureExecutionSource::MetadataSnapshotOnly => true,
    };
    if matches_platform {
        Ok(())
    } else {
        Err(NetworkLiveCaptureExecutionError::SourcePlatformMismatch)
    }
}

fn validate_execution_shape(
    input: &NetworkLiveCaptureExecutionInput,
) -> Result<(), NetworkLiveCaptureExecutionError> {
    if input.source == NetworkLiveCaptureExecutionSource::MetadataSnapshotOnly
        && (input.driver_invoked || input.live_capture_executed)
    {
        return Err(NetworkLiveCaptureExecutionError::MetadataSnapshotCannotClaimDriverExecution);
    }
    if input.live_capture_executed && (!input.driver_invoked || input.captured_packet_count == 0) {
        return Err(NetworkLiveCaptureExecutionError::DriverExecutionRequiresPacketObservation);
    }
    Ok(())
}

fn validate_claims(
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
    validate_content_and_authority_claims(input)
}

fn validate_content_and_authority_claims(
    input: &NetworkLiveCaptureExecutionInput,
) -> Result<(), NetworkLiveCaptureExecutionError> {
    if input.decrypted_payload_claimed {
        return Err(NetworkLiveCaptureExecutionError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkLiveCaptureExecutionError::PageContentClaimRejected);
    }
    if input.private_message_claimed {
        return Err(NetworkLiveCaptureExecutionError::PrivateMessageClaimRejected);
    }
    if input.search_query_claimed {
        return Err(NetworkLiveCaptureExecutionError::SearchQueryClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkLiveCaptureExecutionError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkLiveCaptureExecutionError::AdapterAuthorityClaimRejected);
    }
    if input.host_filtering_claimed {
        return Err(NetworkLiveCaptureExecutionError::HostFilteringClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkLiveCaptureExecutionError::EnforcementCommandClaimRejected);
    }
    Ok(())
}

fn require_execution_artifacts(
    input: &NetworkLiveCaptureExecutionInput,
    missing: &mut Vec<NetworkLiveCaptureExecutionRequiredArtifact>,
) {
    require_artifact(
        input.driver_invoked,
        input.driver_invocation_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::DriverInvocation,
        missing,
    );
    require_artifact(
        input.live_capture_executed,
        input.interface_observation_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::InterfaceObservation,
        missing,
    );
    require_artifact(
        input.live_capture_executed,
        input.permission_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::Permission,
        missing,
    );
    require_artifact(
        input.live_capture_executed,
        input.bounded_window_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::BoundedWindow,
        missing,
    );
    require_artifact(
        input.live_capture_executed,
        input.clean_stop_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::CleanStop,
        missing,
    );
}

fn require_custody_artifacts(
    input: &NetworkLiveCaptureExecutionInput,
    missing: &mut Vec<NetworkLiveCaptureExecutionRequiredArtifact>,
) {
    require_artifact(
        input.live_capture_executed,
        input.custody_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::Custody,
        missing,
    );
    require_artifact(
        input.live_capture_executed,
        input.retention_delete_export_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::RetentionDeleteExport,
        missing,
    );
    require_artifact(
        input.live_capture_executed,
        input.metadata_only_sanitization_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::MetadataOnlySanitization,
        missing,
    );
    require_artifact(
        input.live_capture_executed,
        input.private_traffic_exclusion_ref.as_deref(),
        NetworkLiveCaptureExecutionRequiredArtifact::PrivateTrafficExclusion,
        missing,
    );
}

fn require_artifact(
    condition: bool,
    artifact_ref: Option<&str>,
    artifact: NetworkLiveCaptureExecutionRequiredArtifact,
    missing: &mut Vec<NetworkLiveCaptureExecutionRequiredArtifact>,
) {
    require(condition && artifact_ref.is_some(), artifact, missing);
}

fn require(
    condition: bool,
    artifact: NetworkLiveCaptureExecutionRequiredArtifact,
    missing: &mut Vec<NetworkLiveCaptureExecutionRequiredArtifact>,
) {
    if !condition {
        missing.push(artifact);
    }
}

fn artifact_refs(input: &NetworkLiveCaptureExecutionInput) -> [Option<&str>; 9] {
    [
        input.driver_invocation_ref.as_deref(),
        input.interface_observation_ref.as_deref(),
        input.permission_ref.as_deref(),
        input.bounded_window_ref.as_deref(),
        input.clean_stop_ref.as_deref(),
        input.custody_ref.as_deref(),
        input.retention_delete_export_ref.as_deref(),
        input.metadata_only_sanitization_ref.as_deref(),
        input.private_traffic_exclusion_ref.as_deref(),
    ]
}

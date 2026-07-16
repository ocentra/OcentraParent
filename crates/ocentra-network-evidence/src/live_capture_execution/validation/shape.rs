use super::{
    NetworkLiveCaptureExecutionError, NetworkLiveCaptureExecutionInput,
    NetworkLiveCaptureExecutionSource,
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
    Ok(())
}

fn validate_source_platform(
    input: &NetworkLiveCaptureExecutionInput,
) -> Result<(), NetworkLiveCaptureExecutionError> {
    let matches_platform = match input.source {
        NetworkLiveCaptureExecutionSource::WindowsNpcapDriver => {
            input.live_capture_proof.platform == super::NetworkLiveCapturePlatform::WindowsNpcap
        }
        NetworkLiveCaptureExecutionSource::LinuxLibpcapDriver => {
            input.live_capture_proof.platform == super::NetworkLiveCapturePlatform::LinuxLibpcap
        }
        NetworkLiveCaptureExecutionSource::MacosBpfLibpcapDriver => {
            input.live_capture_proof.platform == super::NetworkLiveCapturePlatform::MacosBpfLibpcap
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

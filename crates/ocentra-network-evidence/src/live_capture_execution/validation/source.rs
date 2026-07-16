use super::{
    NetworkLiveCaptureExecutionError, NetworkLiveCaptureExecutionInput,
    NetworkLiveCaptureExecutionSource,
};

pub(super) fn validate_input(
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

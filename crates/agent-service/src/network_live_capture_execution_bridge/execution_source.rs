use ocentra_network_evidence::{
    live_capture::NetworkLiveCapturePlatform,
    live_capture_execution::NetworkLiveCaptureExecutionSource,
};

pub(super) fn execution_source(
    platform: NetworkLiveCapturePlatform,
) -> NetworkLiveCaptureExecutionSource {
    match platform {
        NetworkLiveCapturePlatform::WindowsNpcap => {
            NetworkLiveCaptureExecutionSource::WindowsNpcapDriver
        }
        NetworkLiveCapturePlatform::LinuxLibpcap => {
            NetworkLiveCaptureExecutionSource::LinuxLibpcapDriver
        }
        NetworkLiveCapturePlatform::MacosBpfLibpcap => {
            NetworkLiveCaptureExecutionSource::MacosBpfLibpcapDriver
        }
    }
}

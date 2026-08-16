use ocentra_network_evidence::live_capture::NetworkLiveCapturePlatform;
use ocentra_parent_agent_protocol::network_flow::NetworkLiveCaptureStatusPlatform;

pub(super) fn protocol_platform(
    platform: NetworkLiveCapturePlatform,
) -> NetworkLiveCaptureStatusPlatform {
    match platform {
        NetworkLiveCapturePlatform::WindowsNpcap => NetworkLiveCaptureStatusPlatform::WindowsNpcap,
        NetworkLiveCapturePlatform::LinuxLibpcap => NetworkLiveCaptureStatusPlatform::LinuxLibpcap,
        NetworkLiveCapturePlatform::MacosBpfLibpcap => {
            NetworkLiveCaptureStatusPlatform::MacosBpfLibpcap
        }
    }
}

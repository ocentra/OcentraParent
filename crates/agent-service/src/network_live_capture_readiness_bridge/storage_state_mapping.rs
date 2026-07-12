use ocentra_network_evidence::raw_capture_storage::types::NetworkRawCaptureStorageState;
use ocentra_parent_agent_protocol::network_flow::NetworkRawCaptureStorageStatusState;

pub(super) fn protocol_storage_state(
    state: NetworkRawCaptureStorageState,
) -> NetworkRawCaptureStorageStatusState {
    match state {
        NetworkRawCaptureStorageState::CustodyReady => {
            NetworkRawCaptureStorageStatusState::CustodyReady
        }
        NetworkRawCaptureStorageState::ManualRequired => {
            NetworkRawCaptureStorageStatusState::ManualRequired
        }
        NetworkRawCaptureStorageState::Unavailable => {
            NetworkRawCaptureStorageStatusState::Unavailable
        }
        NetworkRawCaptureStorageState::Degraded => NetworkRawCaptureStorageStatusState::Degraded,
    }
}

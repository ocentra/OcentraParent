use ocentra_parent_agent_protocol::network_flow::{
    NetworkLiveCaptureStatus, NetworkRawCaptureStorageStatusState,
};

pub(super) fn apply_storage_counts(
    status: &mut NetworkLiveCaptureStatus,
    storage_state: &NetworkRawCaptureStorageStatusState,
) {
    match storage_state {
        NetworkRawCaptureStorageStatusState::CustodyReady => {
            status.storage_custody_ready_count += 1
        }
        NetworkRawCaptureStorageStatusState::ManualRequired => {
            status.storage_manual_required_count += 1
        }
        NetworkRawCaptureStorageStatusState::Unavailable => status.storage_unavailable_count += 1,
        NetworkRawCaptureStorageStatusState::Degraded => status.storage_degraded_count += 1,
    }
}

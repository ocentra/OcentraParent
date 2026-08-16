use crate::adapter_capability_status::{
    NetworkAdapterCapabilityStatusEntry, NetworkAdapterCapabilityStatusState,
};

pub(crate) struct NetworkAdapterCapabilityStatusCounts {
    pub supported: usize,
    pub dry_run: usize,
    pub lab_ready: usize,
    pub physical_device_ready: usize,
    pub apple_device_ready: usize,
    pub distro_ready: usize,
    pub research_only: usize,
    pub manual_required: usize,
    pub unavailable: usize,
}

pub(crate) fn status_counts(
    entries: &[NetworkAdapterCapabilityStatusEntry],
) -> NetworkAdapterCapabilityStatusCounts {
    NetworkAdapterCapabilityStatusCounts {
        supported: count_status(entries, NetworkAdapterCapabilityStatusState::Supported),
        dry_run: count_status(entries, NetworkAdapterCapabilityStatusState::DryRun),
        lab_ready: count_status(entries, NetworkAdapterCapabilityStatusState::LabReady),
        physical_device_ready: count_status(
            entries,
            NetworkAdapterCapabilityStatusState::PhysicalDeviceReady,
        ),
        apple_device_ready: count_status(
            entries,
            NetworkAdapterCapabilityStatusState::AppleDeviceReady,
        ),
        distro_ready: count_status(entries, NetworkAdapterCapabilityStatusState::DistroReady),
        research_only: count_status(entries, NetworkAdapterCapabilityStatusState::ResearchOnly),
        manual_required: count_status(entries, NetworkAdapterCapabilityStatusState::ManualRequired),
        unavailable: count_status(entries, NetworkAdapterCapabilityStatusState::Unavailable),
    }
}

fn count_status(
    entries: &[NetworkAdapterCapabilityStatusEntry],
    status: NetworkAdapterCapabilityStatusState,
) -> usize {
    entries
        .iter()
        .filter(|entry| entry.capability_status == status)
        .count()
}

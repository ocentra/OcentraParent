use crate::{
    adapter_capability_status::{
        NetworkAdapterCapabilityStatusEntry, NetworkAdapterCapabilityStatusError,
        NetworkAdapterCapabilityStatusState,
    },
    NetworkPlatformClaimEntry, NetworkPlatformClaimState, NetworkPlatformClaimTarget,
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

pub(crate) fn status_entry_from_platform_entry(
    entry: NetworkPlatformClaimEntry,
) -> Result<NetworkAdapterCapabilityStatusEntry, NetworkAdapterCapabilityStatusError> {
    validate_platform_entry(&entry)?;
    Ok(NetworkAdapterCapabilityStatusEntry {
        target: entry.target,
        capability_status: status_from_platform_entry(entry.target, entry.claim_state),
        adapter_capability_refs: entry.adapter_capability_refs,
        device_or_os_refs: entry.device_or_os_refs,
        permission_or_entitlement_refs: entry.permission_or_entitlement_refs,
        missing_required_artifacts: entry.missing_required_artifacts,
        audit_refs: entry.audit_refs,
        adapter_authorized_by_proof: entry.adapter_authorized_by_proof,
        enforcement_command_published: false,
    })
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

pub(crate) fn normalize_portal_ref(
    value: Option<&str>,
) -> Result<String, NetworkAdapterCapabilityStatusError> {
    match value {
        Some(raw) => {
            normalize_ref(raw).ok_or(NetworkAdapterCapabilityStatusError::EmptyPortalStatusProofRef)
        }
        None => Err(NetworkAdapterCapabilityStatusError::MissingPortalStatusProofRef),
    }
}

pub(crate) fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

fn validate_platform_entry(
    entry: &NetworkPlatformClaimEntry,
) -> Result<(), NetworkAdapterCapabilityStatusError> {
    if entry.enforcement_command_published {
        return Err(
            NetworkAdapterCapabilityStatusError::PlatformEntryPublishedEnforcementCommand(
                entry.target,
            ),
        );
    }
    if entry.device_or_os_refs.is_empty() {
        return Err(
            NetworkAdapterCapabilityStatusError::PlatformEntryMissingPlatformRef(entry.target),
        );
    }
    if entry.adapter_capability_refs.is_empty() && entry.missing_required_artifacts.is_empty() {
        return Err(
            NetworkAdapterCapabilityStatusError::PlatformEntryMissingCapabilityOrFollowup(
                entry.target,
            ),
        );
    }
    if entry.audit_refs.is_empty() {
        return Err(
            NetworkAdapterCapabilityStatusError::PlatformEntryMissingAuditRef(entry.target),
        );
    }
    Ok(())
}

fn status_from_platform_entry(
    target: NetworkPlatformClaimTarget,
    state: NetworkPlatformClaimState,
) -> NetworkAdapterCapabilityStatusState {
    match state {
        NetworkPlatformClaimState::Ready => ready_status_for_target(target),
        NetworkPlatformClaimState::DryRun => NetworkAdapterCapabilityStatusState::DryRun,
        NetworkPlatformClaimState::ResearchOnly => {
            NetworkAdapterCapabilityStatusState::ResearchOnly
        }
        NetworkPlatformClaimState::ManualRequired => {
            NetworkAdapterCapabilityStatusState::ManualRequired
        }
        NetworkPlatformClaimState::Unavailable => NetworkAdapterCapabilityStatusState::Unavailable,
    }
}

fn ready_status_for_target(
    target: NetworkPlatformClaimTarget,
) -> NetworkAdapterCapabilityStatusState {
    match target {
        NetworkPlatformClaimTarget::WindowsFirewall => {
            NetworkAdapterCapabilityStatusState::Supported
        }
        NetworkPlatformClaimTarget::WindowsWfp => NetworkAdapterCapabilityStatusState::LabReady,
        NetworkPlatformClaimTarget::AndroidVpnService => {
            NetworkAdapterCapabilityStatusState::PhysicalDeviceReady
        }
        NetworkPlatformClaimTarget::AppleNetworkExtensionMacOs
        | NetworkPlatformClaimTarget::AppleNetworkExtensionIos => {
            NetworkAdapterCapabilityStatusState::AppleDeviceReady
        }
        NetworkPlatformClaimTarget::LinuxNftables
        | NetworkPlatformClaimTarget::LinuxEbpf
        | NetworkPlatformClaimTarget::LinuxTun => NetworkAdapterCapabilityStatusState::DistroReady,
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

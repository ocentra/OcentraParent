use crate::adapter_capability_status::{
    NetworkAdapterCapabilityStatusEntry, NetworkAdapterCapabilityStatusError,
};
use crate::platform_claims::{NetworkPlatformClaimEntry, NetworkPlatformClaimState};

pub(crate) fn status_entry_from_platform_entry(
    entry: NetworkPlatformClaimEntry,
) -> Result<NetworkAdapterCapabilityStatusEntry, NetworkAdapterCapabilityStatusError> {
    validate_platform_entry(&entry)?;
    Ok(NetworkAdapterCapabilityStatusEntry {
        target: entry.target,
        capability_status: super::status_mapping::status_from_platform_entry(
            entry.target,
            entry.claim_state,
        ),
        adapter_capability_refs: entry.adapter_capability_refs,
        device_or_os_refs: entry.device_or_os_refs,
        permission_or_entitlement_refs: entry.permission_or_entitlement_refs,
        missing_required_artifacts: entry.missing_required_artifacts,
        audit_refs: entry.audit_refs,
        adapter_authorized_by_proof: entry.adapter_authorized_by_proof,
        enforcement_command_published: false,
    })
}

fn validate_platform_entry(
    entry: &NetworkPlatformClaimEntry,
) -> Result<(), NetworkAdapterCapabilityStatusError> {
    if entry.adapter_authorized_by_proof && entry.claim_state != NetworkPlatformClaimState::Ready {
        return Err(
            NetworkAdapterCapabilityStatusError::PlatformEntryAuthorizesNonReadyAdapter(
                entry.target,
            ),
        );
    }
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

use super::*;

pub(super) fn validate_platform_manifest_summary(
    manifest: &NetworkPlatformClaimManifestProof,
) -> Result<(), NetworkAdapterCapabilityStatusError> {
    if !manifest.every_claim_names_platform {
        return Err(NetworkAdapterCapabilityStatusError::PlatformManifestClaimsMissingPlatformRef);
    }
    if !manifest.every_claim_names_permission_or_manual_followup {
        return Err(
            NetworkAdapterCapabilityStatusError::PlatformManifestClaimsMissingPermissionOrManualFollowup,
        );
    }
    if !manifest.every_claim_names_audit_ref {
        return Err(NetworkAdapterCapabilityStatusError::PlatformManifestClaimsMissingAuditRef);
    }
    if manifest.ready_claims
        != count_claim_state(&manifest.entries, NetworkPlatformClaimState::Ready)
        || manifest.dry_run_claims
            != count_claim_state(&manifest.entries, NetworkPlatformClaimState::DryRun)
        || manifest.research_only_claims
            != count_claim_state(&manifest.entries, NetworkPlatformClaimState::ResearchOnly)
        || manifest.manual_required_claims
            != count_claim_state(&manifest.entries, NetworkPlatformClaimState::ManualRequired)
        || manifest.unavailable_claims
            != count_claim_state(&manifest.entries, NetworkPlatformClaimState::Unavailable)
    {
        return Err(NetworkAdapterCapabilityStatusError::PlatformManifestEntryCountsMismatch);
    }
    let expected_manual_followups = manifest
        .entries
        .iter()
        .filter(|entry| !entry.missing_required_artifacts.is_empty())
        .count();
    if manifest.manual_followups.len() != expected_manual_followups
        || manifest.manual_followups.iter().any(|followup| {
            !manifest.entries.iter().any(|entry| {
                entry.target == followup.target
                    && entry.missing_required_artifacts == followup.missing_required_artifacts
            })
        })
    {
        return Err(NetworkAdapterCapabilityStatusError::PlatformManifestManualFollowupMismatch);
    }
    Ok(())
}

fn count_claim_state(
    entries: &[NetworkPlatformClaimEntry],
    state: NetworkPlatformClaimState,
) -> usize {
    entries
        .iter()
        .filter(|entry| entry.claim_state == state)
        .count()
}

pub(super) fn every_status_names_platform(entries: &[NetworkAdapterCapabilityStatusEntry]) -> bool {
    entries
        .iter()
        .all(|entry| !entry.device_or_os_refs.is_empty())
}

pub(super) fn every_status_names_capability_or_followup(
    entries: &[NetworkAdapterCapabilityStatusEntry],
) -> bool {
    entries.iter().all(|entry| {
        !entry.adapter_capability_refs.is_empty() || !entry.missing_required_artifacts.is_empty()
    })
}

pub(super) fn every_status_names_audit_ref(
    entries: &[NetworkAdapterCapabilityStatusEntry],
) -> bool {
    entries.iter().all(|entry| !entry.audit_refs.is_empty())
}

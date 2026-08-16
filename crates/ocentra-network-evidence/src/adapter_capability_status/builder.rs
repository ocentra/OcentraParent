use super::*;

pub(super) fn build_network_adapter_capability_status(
    input: NetworkAdapterCapabilityStatusInput,
) -> Result<NetworkAdapterCapabilityStatusProof, NetworkAdapterCapabilityStatusError> {
    claims::reject_status_claims(&input)?;
    let status_ref = normalize_ref(&input.status_ref)
        .ok_or(NetworkAdapterCapabilityStatusError::EmptyStatusRef)?;
    let portal_status_proof_ref = normalize_portal_ref(input.portal_status_proof_ref.as_deref())?;
    let manifest = input.platform_manifest;
    let platform_manifest_ref = normalize_ref(&manifest.manifest_ref)
        .ok_or(NetworkAdapterCapabilityStatusError::EmptyPlatformManifestRef)?;
    if manifest.entries.is_empty() {
        return Err(NetworkAdapterCapabilityStatusError::EmptyPlatformManifest);
    }
    if !manifest.no_live_adapter_execution_claimed {
        return Err(
            NetworkAdapterCapabilityStatusError::PlatformManifestClaimsLiveAdapterExecution,
        );
    }
    if !manifest.no_enforcement_commands_published {
        return Err(
            NetworkAdapterCapabilityStatusError::PlatformManifestPublishesEnforcementCommand,
        );
    }
    if !manifest.ui_has_no_policy_authority {
        return Err(NetworkAdapterCapabilityStatusError::PlatformManifestAllowsUiPolicyAuthority);
    }
    manifest::validate_platform_manifest_summary(&manifest)?;

    let manual_followup_count = manifest.manual_followups.len();
    let entries = manifest
        .entries
        .into_iter()
        .map(status_entry_from_platform_entry)
        .collect::<Result<Vec<_>, _>>()?;
    let counts = status_counts(&entries);

    Ok(NetworkAdapterCapabilityStatusProof {
        status_ref,
        platform_manifest_ref,
        portal_status_proof_ref,
        supported_count: counts.supported,
        dry_run_count: counts.dry_run,
        lab_ready_count: counts.lab_ready,
        physical_device_ready_count: counts.physical_device_ready,
        apple_device_ready_count: counts.apple_device_ready,
        distro_ready_count: counts.distro_ready,
        research_only_count: counts.research_only,
        manual_required_count: counts.manual_required,
        unavailable_count: counts.unavailable,
        manual_followup_count,
        every_status_names_platform: manifest::every_status_names_platform(&entries),
        every_status_names_capability_or_followup:
            manifest::every_status_names_capability_or_followup(&entries),
        every_status_names_audit_ref: manifest::every_status_names_audit_ref(&entries),
        portal_status_from_service_read_model: true,
        broader_platform_capability_ux_claimed: false,
        no_live_adapter_execution_claimed: true,
        no_enforcement_commands_published: true,
        ui_has_no_policy_authority: true,
        entries,
    })
}

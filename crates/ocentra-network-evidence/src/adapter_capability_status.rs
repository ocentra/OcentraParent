use serde::{Deserialize, Serialize};

use crate::{
    adapter_capability_status_values::{
        normalize_portal_ref, normalize_ref, status_counts, status_entry_from_platform_entry,
    },
    NetworkPlatformClaimManifestProof, NetworkPlatformClaimState, NetworkPlatformClaimTarget,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAdapterCapabilityStatusState {
    Supported,
    DryRun,
    LabReady,
    PhysicalDeviceReady,
    AppleDeviceReady,
    DistroReady,
    ResearchOnly,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAdapterCapabilityStatusInput {
    pub status_ref: String,
    pub platform_manifest: NetworkPlatformClaimManifestProof,
    pub portal_status_proof_ref: Option<String>,
    pub generic_platform_support_claimed: bool,
    pub live_adapter_execution_claimed: bool,
    pub enforcement_command_claimed: bool,
    pub ui_policy_authority_claimed: bool,
    pub broader_platform_capability_ux_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAdapterCapabilityStatusEntry {
    pub target: NetworkPlatformClaimTarget,
    pub capability_status: NetworkAdapterCapabilityStatusState,
    pub adapter_capability_refs: Vec<String>,
    pub device_or_os_refs: Vec<String>,
    pub permission_or_entitlement_refs: Vec<String>,
    pub missing_required_artifacts: Vec<String>,
    pub audit_refs: Vec<String>,
    pub adapter_authorized_by_proof: bool,
    pub enforcement_command_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAdapterCapabilityStatusProof {
    pub status_ref: String,
    pub platform_manifest_ref: String,
    pub portal_status_proof_ref: String,
    pub entries: Vec<NetworkAdapterCapabilityStatusEntry>,
    pub supported_count: usize,
    pub dry_run_count: usize,
    pub lab_ready_count: usize,
    pub physical_device_ready_count: usize,
    pub apple_device_ready_count: usize,
    pub distro_ready_count: usize,
    pub research_only_count: usize,
    pub manual_required_count: usize,
    pub unavailable_count: usize,
    pub manual_followup_count: usize,
    pub every_status_names_platform: bool,
    pub every_status_names_capability_or_followup: bool,
    pub every_status_names_audit_ref: bool,
    pub portal_status_from_service_read_model: bool,
    pub broader_platform_capability_ux_claimed: bool,
    pub no_live_adapter_execution_claimed: bool,
    pub no_enforcement_commands_published: bool,
    pub ui_has_no_policy_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkAdapterCapabilityStatusError {
    EmptyStatusRef,
    EmptyPlatformManifestRef,
    EmptyPlatformManifest,
    MissingPortalStatusProofRef,
    EmptyPortalStatusProofRef,
    GenericPlatformSupportClaimRejected,
    LiveAdapterExecutionClaimRejected,
    EnforcementCommandClaimRejected,
    UiPolicyAuthorityClaimRejected,
    BroaderPlatformCapabilityUxClaimRejected,
    PlatformManifestClaimsLiveAdapterExecution,
    PlatformManifestPublishesEnforcementCommand,
    PlatformManifestAllowsUiPolicyAuthority,
    PlatformManifestClaimsMissingPlatformRef,
    PlatformManifestClaimsMissingPermissionOrManualFollowup,
    PlatformManifestClaimsMissingAuditRef,
    PlatformManifestEntryCountsMismatch,
    PlatformManifestManualFollowupMismatch,
    PlatformEntryMissingPlatformRef(NetworkPlatformClaimTarget),
    PlatformEntryMissingCapabilityOrFollowup(NetworkPlatformClaimTarget),
    PlatformEntryMissingAuditRef(NetworkPlatformClaimTarget),
    PlatformEntryAuthorizesNonReadyAdapter(NetworkPlatformClaimTarget),
    PlatformEntryPublishedEnforcementCommand(NetworkPlatformClaimTarget),
}

pub fn build_network_adapter_capability_status(
    input: NetworkAdapterCapabilityStatusInput,
) -> Result<NetworkAdapterCapabilityStatusProof, NetworkAdapterCapabilityStatusError> {
    reject_status_claims(&input)?;
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
    validate_platform_manifest_summary(&manifest)?;

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
        every_status_names_platform: every_status_names_platform(&entries),
        every_status_names_capability_or_followup: every_status_names_capability_or_followup(
            &entries,
        ),
        every_status_names_audit_ref: every_status_names_audit_ref(&entries),
        portal_status_from_service_read_model: true,
        broader_platform_capability_ux_claimed: false,
        no_live_adapter_execution_claimed: true,
        no_enforcement_commands_published: true,
        ui_has_no_policy_authority: true,
        entries,
    })
}

fn reject_status_claims(
    input: &NetworkAdapterCapabilityStatusInput,
) -> Result<(), NetworkAdapterCapabilityStatusError> {
    if input.generic_platform_support_claimed {
        return Err(NetworkAdapterCapabilityStatusError::GenericPlatformSupportClaimRejected);
    }
    if input.live_adapter_execution_claimed {
        return Err(NetworkAdapterCapabilityStatusError::LiveAdapterExecutionClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkAdapterCapabilityStatusError::EnforcementCommandClaimRejected);
    }
    if input.ui_policy_authority_claimed {
        return Err(NetworkAdapterCapabilityStatusError::UiPolicyAuthorityClaimRejected);
    }
    if input.broader_platform_capability_ux_claimed {
        return Err(NetworkAdapterCapabilityStatusError::BroaderPlatformCapabilityUxClaimRejected);
    }
    Ok(())
}

fn validate_platform_manifest_summary(
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
    entries: &[crate::NetworkPlatformClaimEntry],
    state: NetworkPlatformClaimState,
) -> usize {
    entries
        .iter()
        .filter(|entry| entry.claim_state == state)
        .count()
}

fn every_status_names_platform(entries: &[NetworkAdapterCapabilityStatusEntry]) -> bool {
    entries
        .iter()
        .all(|entry| !entry.device_or_os_refs.is_empty())
}

fn every_status_names_capability_or_followup(
    entries: &[NetworkAdapterCapabilityStatusEntry],
) -> bool {
    entries.iter().all(|entry| {
        !entry.adapter_capability_refs.is_empty() || !entry.missing_required_artifacts.is_empty()
    })
}

fn every_status_names_audit_ref(entries: &[NetworkAdapterCapabilityStatusEntry]) -> bool {
    entries.iter().all(|entry| !entry.audit_refs.is_empty())
}

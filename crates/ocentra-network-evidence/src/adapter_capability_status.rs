use serde::{Deserialize, Serialize};

use crate::{
    adapter_capability_status_values::{
        normalize_portal_ref, normalize_ref, status_counts, status_entry_from_platform_entry,
    },
    platform_claims::{
        NetworkPlatformClaimEntry, NetworkPlatformClaimManifestProof, NetworkPlatformClaimState,
        NetworkPlatformClaimTarget,
    },
};

mod builder;
mod claims;
mod manifest;

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
    builder::build_network_adapter_capability_status(input)
}

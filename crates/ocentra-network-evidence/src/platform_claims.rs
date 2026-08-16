mod entries;
mod summary;

use serde::{Deserialize, Serialize};

use self::{
    entries::entry_from_source,
    summary::{count_state, manual_followups},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkPlatformClaimTarget {
    WindowsFirewall,
    WindowsWfp,
    AndroidVpnService,
    AppleNetworkExtensionMacOs,
    AppleNetworkExtensionIos,
    LinuxNftables,
    LinuxEbpf,
    LinuxTun,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkPlatformClaimState {
    Ready,
    DryRun,
    ResearchOnly,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkPlatformUnsupportedClaims {
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub generic_platform_support_claimed: bool,
    pub live_adapter_execution_claimed: bool,
    pub enforcement_command_claimed: bool,
    pub ui_policy_authority_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkPlatformClaimManifestInput {
    pub manifest_ref: String,
    pub proof_sources: Vec<NetworkPlatformClaimProofSource>,
    pub unsupported_claims: NetworkPlatformUnsupportedClaims,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkPlatformClaimProofSource {
    WindowsFirewall(crate::NetworkWindowsFirewallAdapterProof),
    WindowsWfp(crate::NetworkWindowsWfpGateProof),
    AndroidVpnService(crate::NetworkAndroidVpnServiceGateProof),
    AppleNetworkExtension(crate::NetworkAppleNetworkExtensionGateProof),
    LinuxAdapter(crate::NetworkLinuxAdapterGateProof),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkPlatformClaimEntry {
    pub target: NetworkPlatformClaimTarget,
    pub claim_state: NetworkPlatformClaimState,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub device_or_os_refs: Vec<String>,
    pub permission_or_entitlement_refs: Vec<String>,
    pub adapter_capability_refs: Vec<String>,
    pub missing_required_artifacts: Vec<String>,
    pub audit_refs: Vec<String>,
    pub adapter_authorized_by_proof: bool,
    pub enforcement_command_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkPlatformClaimManualFollowup {
    pub target: NetworkPlatformClaimTarget,
    pub missing_required_artifacts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkPlatformClaimManifestProof {
    pub manifest_ref: String,
    pub entries: Vec<NetworkPlatformClaimEntry>,
    pub ready_claims: usize,
    pub dry_run_claims: usize,
    pub research_only_claims: usize,
    pub manual_required_claims: usize,
    pub unavailable_claims: usize,
    pub manual_followups: Vec<NetworkPlatformClaimManualFollowup>,
    pub every_claim_names_platform: bool,
    pub every_claim_names_permission_or_manual_followup: bool,
    pub every_claim_names_audit_ref: bool,
    pub no_enforcement_commands_published: bool,
    pub no_live_adapter_execution_claimed: bool,
    pub ui_has_no_policy_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkPlatformClaimManifestError {
    EmptyManifestRef,
    EmptyProofSources,
    UnsupportedExactUrlClaim,
    UnsupportedDecryptedPayloadClaim,
    UnsupportedPageContentClaim,
    GenericPlatformSupportClaimRejected,
    LiveAdapterExecutionClaimRejected,
    EnforcementCommandClaimRejected,
    UiPolicyAuthorityClaimRejected,
    ProofSourceAuthorizesNonReadyAdapter(NetworkPlatformClaimTarget),
    ProofSourcePublishedEnforcementCommand(NetworkPlatformClaimTarget),
}

pub fn build_network_platform_claim_manifest(
    input: NetworkPlatformClaimManifestInput,
) -> Result<NetworkPlatformClaimManifestProof, NetworkPlatformClaimManifestError> {
    reject_unsupported_claims(&input.unsupported_claims)?;
    let manifest_ref = normalize_ref(&input.manifest_ref)
        .ok_or(NetworkPlatformClaimManifestError::EmptyManifestRef)?;
    if input.proof_sources.is_empty() {
        return Err(NetworkPlatformClaimManifestError::EmptyProofSources);
    }

    let mut entries = Vec::new();
    for source in input.proof_sources {
        let entry = entry_from_source(source);
        if entry.adapter_authorized_by_proof
            && entry.claim_state != NetworkPlatformClaimState::Ready
        {
            return Err(
                NetworkPlatformClaimManifestError::ProofSourceAuthorizesNonReadyAdapter(
                    entry.target,
                ),
            );
        }
        if entry.enforcement_command_published {
            return Err(
                NetworkPlatformClaimManifestError::ProofSourcePublishedEnforcementCommand(
                    entry.target,
                ),
            );
        }
        entries.push(entry);
    }

    let manual_followups = manual_followups(&entries);
    let ready_claims = count_state(&entries, NetworkPlatformClaimState::Ready);
    let dry_run_claims = count_state(&entries, NetworkPlatformClaimState::DryRun);
    let research_only_claims = count_state(&entries, NetworkPlatformClaimState::ResearchOnly);
    let manual_required_claims = count_state(&entries, NetworkPlatformClaimState::ManualRequired);
    let unavailable_claims = count_state(&entries, NetworkPlatformClaimState::Unavailable);
    let every_claim_names_platform = entries
        .iter()
        .all(|entry| !entry.device_or_os_refs.is_empty());
    let every_claim_names_permission_or_manual_followup = entries.iter().all(|entry| {
        !entry.permission_or_entitlement_refs.is_empty()
            || !entry.missing_required_artifacts.is_empty()
            || entry.claim_state == NetworkPlatformClaimState::Unavailable
    });
    let every_claim_names_audit_ref = entries.iter().all(|entry| !entry.audit_refs.is_empty());

    Ok(NetworkPlatformClaimManifestProof {
        manifest_ref,
        entries,
        ready_claims,
        dry_run_claims,
        research_only_claims,
        manual_required_claims,
        unavailable_claims,
        manual_followups,
        every_claim_names_platform,
        every_claim_names_permission_or_manual_followup,
        every_claim_names_audit_ref,
        no_enforcement_commands_published: true,
        no_live_adapter_execution_claimed: true,
        ui_has_no_policy_authority: true,
    })
}

fn reject_unsupported_claims(
    claims: &NetworkPlatformUnsupportedClaims,
) -> Result<(), NetworkPlatformClaimManifestError> {
    [
        (
            claims.exact_url_claimed,
            NetworkPlatformClaimManifestError::UnsupportedExactUrlClaim,
        ),
        (
            claims.decrypted_payload_claimed,
            NetworkPlatformClaimManifestError::UnsupportedDecryptedPayloadClaim,
        ),
        (
            claims.page_content_claimed,
            NetworkPlatformClaimManifestError::UnsupportedPageContentClaim,
        ),
        (
            claims.generic_platform_support_claimed,
            NetworkPlatformClaimManifestError::GenericPlatformSupportClaimRejected,
        ),
        (
            claims.live_adapter_execution_claimed,
            NetworkPlatformClaimManifestError::LiveAdapterExecutionClaimRejected,
        ),
        (
            claims.enforcement_command_claimed,
            NetworkPlatformClaimManifestError::EnforcementCommandClaimRejected,
        ),
        (
            claims.ui_policy_authority_claimed,
            NetworkPlatformClaimManifestError::UiPolicyAuthorityClaimRejected,
        ),
    ]
    .into_iter()
    .find_map(|(claimed, error)| claimed.then_some(error))
    .map_or(Ok(()), Err)
}

fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

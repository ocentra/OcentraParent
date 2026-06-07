use serde::{Deserialize, Serialize};

use crate::{
    platform_claim_values::{
        android_artifact_label, android_vpn_state, apple_artifact_label, apple_state, apple_target,
        compact_refs, linux_artifact_label, linux_state, linux_target,
        windows_firewall_artifact_label, windows_firewall_state, windows_wfp_artifact_label,
        windows_wfp_state,
    },
    NetworkAndroidVpnServiceGateProof, NetworkAppleNetworkExtensionGateProof,
    NetworkLinuxAdapterGateProof, NetworkWindowsFirewallAdapterProof, NetworkWindowsWfpGateProof,
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
    WindowsFirewall(NetworkWindowsFirewallAdapterProof),
    WindowsWfp(NetworkWindowsWfpGateProof),
    AndroidVpnService(NetworkAndroidVpnServiceGateProof),
    AppleNetworkExtension(NetworkAppleNetworkExtensionGateProof),
    LinuxAdapter(NetworkLinuxAdapterGateProof),
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
        no_enforcement_commands_published: true,
        no_live_adapter_execution_claimed: true,
        ui_has_no_policy_authority: true,
    })
}

fn reject_unsupported_claims(
    claims: &NetworkPlatformUnsupportedClaims,
) -> Result<(), NetworkPlatformClaimManifestError> {
    if claims.exact_url_claimed {
        return Err(NetworkPlatformClaimManifestError::UnsupportedExactUrlClaim);
    }
    if claims.decrypted_payload_claimed {
        return Err(NetworkPlatformClaimManifestError::UnsupportedDecryptedPayloadClaim);
    }
    if claims.page_content_claimed {
        return Err(NetworkPlatformClaimManifestError::UnsupportedPageContentClaim);
    }
    if claims.generic_platform_support_claimed {
        return Err(NetworkPlatformClaimManifestError::GenericPlatformSupportClaimRejected);
    }
    if claims.live_adapter_execution_claimed {
        return Err(NetworkPlatformClaimManifestError::LiveAdapterExecutionClaimRejected);
    }
    if claims.enforcement_command_claimed {
        return Err(NetworkPlatformClaimManifestError::EnforcementCommandClaimRejected);
    }
    if claims.ui_policy_authority_claimed {
        return Err(NetworkPlatformClaimManifestError::UiPolicyAuthorityClaimRejected);
    }
    Ok(())
}

fn entry_from_source(source: NetworkPlatformClaimProofSource) -> NetworkPlatformClaimEntry {
    match source {
        NetworkPlatformClaimProofSource::WindowsFirewall(proof) => windows_firewall_entry(proof),
        NetworkPlatformClaimProofSource::WindowsWfp(proof) => windows_wfp_entry(proof),
        NetworkPlatformClaimProofSource::AndroidVpnService(proof) => android_vpn_entry(proof),
        NetworkPlatformClaimProofSource::AppleNetworkExtension(proof) => apple_entry(proof),
        NetworkPlatformClaimProofSource::LinuxAdapter(proof) => linux_entry(proof),
    }
}

fn windows_firewall_entry(proof: NetworkWindowsFirewallAdapterProof) -> NetworkPlatformClaimEntry {
    NetworkPlatformClaimEntry {
        target: NetworkPlatformClaimTarget::WindowsFirewall,
        claim_state: windows_firewall_state(proof.proof_state),
        policy_decision_ref: proof.policy_decision_ref,
        parent_rule_ref: proof.parent_rule_ref,
        evidence_refs: proof.evidence_refs,
        device_or_os_refs: compact_refs(vec![
            Some(format!("{:?}", proof.target_kind)),
            Some(proof.target_ref),
            Some(proof.firewall_rule_ref),
        ]),
        permission_or_entitlement_refs: compact_refs(vec![proof.adapter_authorization_ref]),
        adapter_capability_refs: compact_refs(vec![proof.adapter_capability_proof_ref]),
        missing_required_artifacts: proof
            .missing_required_artifacts
            .into_iter()
            .map(windows_firewall_artifact_label)
            .map(ToOwned::to_owned)
            .collect(),
        audit_refs: compact_refs(vec![proof.audit_event_ref]),
        adapter_authorized_by_proof: proof.adapter_apply_authorized,
        enforcement_command_published: proof.enforcement_command_published,
    }
}

fn windows_wfp_entry(proof: NetworkWindowsWfpGateProof) -> NetworkPlatformClaimEntry {
    NetworkPlatformClaimEntry {
        target: NetworkPlatformClaimTarget::WindowsWfp,
        claim_state: windows_wfp_state(proof.gate_state),
        policy_decision_ref: proof.policy_decision_ref,
        parent_rule_ref: proof.parent_rule_ref,
        evidence_refs: proof.evidence_refs,
        device_or_os_refs: compact_refs(vec![
            Some(proof.target_ref),
            Some(proof.wfp_provider_ref),
            Some(proof.wfp_layer_ref),
        ]),
        permission_or_entitlement_refs: compact_refs(vec![
            proof.administrator_permission_proof_ref,
            proof.driver_signing_proof_ref,
            proof.driver_package_proof_ref,
            proof.provider_registration_plan_ref,
            proof.layer_capability_matrix_ref,
        ]),
        adapter_capability_refs: compact_refs(vec![proof.lab_result_artifact_ref.clone()]),
        missing_required_artifacts: proof
            .missing_required_artifacts
            .into_iter()
            .map(windows_wfp_artifact_label)
            .map(ToOwned::to_owned)
            .collect(),
        audit_refs: compact_refs(vec![proof.audit_event_ref]),
        adapter_authorized_by_proof: proof.wfp_lab_proof_ready,
        enforcement_command_published: proof.enforcement_command_published,
    }
}

fn android_vpn_entry(proof: NetworkAndroidVpnServiceGateProof) -> NetworkPlatformClaimEntry {
    NetworkPlatformClaimEntry {
        target: NetworkPlatformClaimTarget::AndroidVpnService,
        claim_state: android_vpn_state(proof.gate_state),
        policy_decision_ref: proof.policy_decision_ref,
        parent_rule_ref: proof.parent_rule_ref,
        evidence_refs: proof.evidence_refs,
        device_or_os_refs: compact_refs(vec![
            Some(proof.package_ref),
            Some(proof.vpn_service_ref),
            proof.physical_device_proof_ref.clone(),
        ]),
        permission_or_entitlement_refs: compact_refs(vec![
            proof.vpn_service_declaration_ref,
            proof.user_consent_proof_ref,
            proof.package_identity_proof_ref,
            proof.virtual_interface_proof_ref,
            proof.device_owner_proof_ref,
        ]),
        adapter_capability_refs: compact_refs(vec![proof.traffic_observation_proof_ref]),
        missing_required_artifacts: proof
            .missing_required_artifacts
            .into_iter()
            .map(android_artifact_label)
            .map(ToOwned::to_owned)
            .collect(),
        audit_refs: compact_refs(vec![proof.audit_event_ref]),
        adapter_authorized_by_proof: proof.physical_device_proof_ready,
        enforcement_command_published: proof.enforcement_command_published,
    }
}

fn apple_entry(proof: NetworkAppleNetworkExtensionGateProof) -> NetworkPlatformClaimEntry {
    NetworkPlatformClaimEntry {
        target: apple_target(proof.platform),
        claim_state: apple_state(proof.gate_state),
        policy_decision_ref: proof.policy_decision_ref,
        parent_rule_ref: proof.parent_rule_ref,
        evidence_refs: proof.evidence_refs,
        device_or_os_refs: compact_refs(vec![
            Some(proof.bundle_ref),
            Some(proof.network_extension_ref),
            proof.device_or_testflight_proof_ref.clone(),
        ]),
        permission_or_entitlement_refs: compact_refs(vec![
            proof.developer_team_proof_ref,
            proof.entitlement_approval_proof_ref,
            proof.provisioning_profile_proof_ref,
            proof.signing_proof_ref,
            proof.network_extension_declaration_ref,
            proof.supervision_or_mdm_proof_ref,
        ]),
        adapter_capability_refs: compact_refs(vec![proof.extension_configuration_proof_ref]),
        missing_required_artifacts: proof
            .missing_required_artifacts
            .into_iter()
            .map(apple_artifact_label)
            .map(ToOwned::to_owned)
            .collect(),
        audit_refs: compact_refs(vec![proof.audit_event_ref]),
        adapter_authorized_by_proof: proof.apple_entitlement_proof_ready,
        enforcement_command_published: proof.enforcement_command_published,
    }
}

fn linux_entry(proof: NetworkLinuxAdapterGateProof) -> NetworkPlatformClaimEntry {
    NetworkPlatformClaimEntry {
        target: linux_target(proof.adapter_kind),
        claim_state: linux_state(proof.gate_state),
        policy_decision_ref: proof.policy_decision_ref,
        parent_rule_ref: proof.parent_rule_ref,
        evidence_refs: proof.evidence_refs,
        device_or_os_refs: compact_refs(vec![
            Some(proof.distro_ref),
            Some(proof.kernel_ref),
            proof.distro_kernel_proof_ref.clone(),
        ]),
        permission_or_entitlement_refs: compact_refs(vec![
            proof.permission_proof_ref,
            proof.adapter_api_capability_proof_ref,
            proof.adapter_plan_proof_ref,
            proof.service_manager_scope_proof_ref,
        ]),
        adapter_capability_refs: compact_refs(vec![proof.lab_result_artifact_ref]),
        missing_required_artifacts: proof
            .missing_required_artifacts
            .into_iter()
            .map(linux_artifact_label)
            .map(ToOwned::to_owned)
            .collect(),
        audit_refs: compact_refs(vec![proof.audit_event_ref]),
        adapter_authorized_by_proof: proof.distro_proof_ready,
        enforcement_command_published: proof.enforcement_command_published,
    }
}

fn manual_followups(
    entries: &[NetworkPlatformClaimEntry],
) -> Vec<NetworkPlatformClaimManualFollowup> {
    entries
        .iter()
        .filter(|entry| !entry.missing_required_artifacts.is_empty())
        .map(|entry| NetworkPlatformClaimManualFollowup {
            target: entry.target,
            missing_required_artifacts: entry.missing_required_artifacts.clone(),
        })
        .collect()
}

fn count_state(entries: &[NetworkPlatformClaimEntry], state: NetworkPlatformClaimState) -> usize {
    entries
        .iter()
        .filter(|entry| entry.claim_state == state)
        .count()
}

fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

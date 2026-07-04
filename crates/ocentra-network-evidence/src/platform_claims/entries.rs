use crate::platform_claim_values::{
    android_artifact_label, android_vpn_state, apple_artifact_label, apple_state, apple_target,
    compact_refs, linux_artifact_label, linux_state, linux_target, windows_firewall_artifact_label,
    windows_firewall_state, windows_wfp_artifact_label, windows_wfp_state,
};

use super::{
    NetworkPlatformClaimEntry, NetworkPlatformClaimProofSource, NetworkPlatformClaimTarget,
};

pub(super) fn entry_from_source(
    source: NetworkPlatformClaimProofSource,
) -> NetworkPlatformClaimEntry {
    match source {
        NetworkPlatformClaimProofSource::WindowsFirewall(proof) => windows_firewall_entry(proof),
        NetworkPlatformClaimProofSource::WindowsWfp(proof) => windows_wfp_entry(proof),
        NetworkPlatformClaimProofSource::AndroidVpnService(proof) => android_vpn_entry(proof),
        NetworkPlatformClaimProofSource::AppleNetworkExtension(proof) => apple_entry(proof),
        NetworkPlatformClaimProofSource::LinuxAdapter(proof) => linux_entry(proof),
    }
}

fn windows_firewall_entry(
    proof: crate::NetworkWindowsFirewallAdapterProof,
) -> NetworkPlatformClaimEntry {
    NetworkPlatformClaimEntry {
        target: NetworkPlatformClaimTarget::WindowsFirewall,
        claim_state: windows_firewall_state(proof.proof_state),
        policy_decision_ref: proof.policy_decision_ref,
        parent_rule_ref: proof.parent_rule_ref,
        evidence_refs: proof.evidence_refs,
        device_or_os_refs: compact_refs(vec![Some(proof.windows_os_scope_ref)]),
        permission_or_entitlement_refs: compact_refs(vec![proof.adapter_authorization_ref]),
        adapter_capability_refs: compact_refs(vec![
            proof.adapter_capability_proof_ref,
            Some(proof.target_ref),
            Some(proof.firewall_rule_ref),
        ]),
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

fn windows_wfp_entry(proof: crate::NetworkWindowsWfpGateProof) -> NetworkPlatformClaimEntry {
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

fn android_vpn_entry(proof: crate::NetworkAndroidVpnServiceGateProof) -> NetworkPlatformClaimEntry {
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

fn apple_entry(proof: crate::NetworkAppleNetworkExtensionGateProof) -> NetworkPlatformClaimEntry {
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

fn linux_entry(proof: crate::NetworkLinuxAdapterGateProof) -> NetworkPlatformClaimEntry {
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

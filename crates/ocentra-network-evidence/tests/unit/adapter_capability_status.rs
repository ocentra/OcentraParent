use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::adapter_capability_status::*;
use ocentra_network_evidence::platform_claims::*;

#[test]
fn adapter_capability_status_derives_specific_states_from_platform_manifest() {
    let ready_targets = [
        NetworkPlatformClaimTarget::WindowsFirewall,
        NetworkPlatformClaimTarget::WindowsWfp,
        NetworkPlatformClaimTarget::AndroidVpnService,
        NetworkPlatformClaimTarget::AppleNetworkExtensionMacOs,
        NetworkPlatformClaimTarget::AppleNetworkExtensionIos,
        NetworkPlatformClaimTarget::LinuxNftables,
        NetworkPlatformClaimTarget::LinuxEbpf,
    ];
    let mut entries = ready_targets
        .into_iter()
        .map(|target| entry(target, NetworkPlatformClaimState::Ready))
        .collect::<Vec<_>>();
    entries.push(entry(
        NetworkPlatformClaimTarget::LinuxTun,
        NetworkPlatformClaimState::DryRun,
    ));

    let proof = build_network_adapter_capability_status(input_with_manifest(
        manifest_with_entries(entries, Vec::new()),
    ))
    .expect_value("complete platform manifest should produce adapter status");

    let statuses: Vec<NetworkAdapterCapabilityStatusState> = proof
        .entries
        .iter()
        .map(|entry| entry.capability_status)
        .collect();
    assert_eq!(
        statuses,
        vec![
            NetworkAdapterCapabilityStatusState::Supported,
            NetworkAdapterCapabilityStatusState::LabReady,
            NetworkAdapterCapabilityStatusState::PhysicalDeviceReady,
            NetworkAdapterCapabilityStatusState::AppleDeviceReady,
            NetworkAdapterCapabilityStatusState::AppleDeviceReady,
            NetworkAdapterCapabilityStatusState::DistroReady,
            NetworkAdapterCapabilityStatusState::DistroReady,
            NetworkAdapterCapabilityStatusState::DryRun,
        ]
    );
    assert_eq!(proof.supported_count, 1);
    assert_eq!(proof.lab_ready_count, 1);
    assert_eq!(proof.physical_device_ready_count, 1);
    assert_eq!(proof.apple_device_ready_count, 2);
    assert_eq!(proof.distro_ready_count, 2);
    assert_eq!(proof.dry_run_count, 1);
    assert_eq!(proof.portal_status_proof_ref, "portal-status-proof-ref");
    assert!(proof.no_live_adapter_execution_claimed);
    assert!(proof.no_enforcement_commands_published);
    assert!(proof.ui_has_no_policy_authority);
}

#[test]
fn adapter_capability_status_preserves_manual_followups_and_unavailable_rows() {
    let proof =
        build_network_adapter_capability_status(input_with_manifest(manifest_with_entries(
            vec![
                manual_entry(
                    NetworkPlatformClaimTarget::WindowsWfp,
                    NetworkPlatformClaimState::ManualRequired,
                    MissingArtifactCase::WindowsWfpAdministratorPermission,
                ),
                manual_entry(
                    NetworkPlatformClaimTarget::LinuxTun,
                    NetworkPlatformClaimState::Unavailable,
                    MissingArtifactCase::LinuxAdapterPermission,
                ),
            ],
            vec![
                followup(
                    NetworkPlatformClaimTarget::WindowsWfp,
                    MissingArtifactCase::WindowsWfpAdministratorPermission,
                ),
                followup(
                    NetworkPlatformClaimTarget::LinuxTun,
                    MissingArtifactCase::LinuxAdapterPermission,
                ),
            ],
        )))
        .expect_value("manual and unavailable rows should stay reportable");

    assert_eq!(proof.manual_required_count, 1);
    assert_eq!(proof.unavailable_count, 1);
    assert_eq!(proof.manual_followup_count, 2);
    assert_eq!(
        proof.entries[0].missing_required_artifacts,
        vec!["windows-wfp.administrator-permission".to_owned()]
    );
    assert_eq!(
        proof.entries[1].missing_required_artifacts,
        vec!["linux-adapter.permission".to_owned()]
    );
    assert!(!proof.entries[0].enforcement_command_published);
    assert!(!proof.entries[1].adapter_authorized_by_proof);
}

fn input_with_manifest(
    platform_manifest: NetworkPlatformClaimManifestProof,
) -> NetworkAdapterCapabilityStatusInput {
    NetworkAdapterCapabilityStatusInput {
        status_ref: "adapter-capability-status-ref".to_owned(),
        platform_manifest,
        portal_status_proof_ref: Some(" portal-status-proof-ref ".to_owned()),
        generic_platform_support_claimed: false,
        live_adapter_execution_claimed: false,
        enforcement_command_claimed: false,
        ui_policy_authority_claimed: false,
        broader_platform_capability_ux_claimed: false,
    }
}

fn manifest_with_entries(
    entries: Vec<NetworkPlatformClaimEntry>,
    manual_followups: Vec<NetworkPlatformClaimManualFollowup>,
) -> NetworkPlatformClaimManifestProof {
    NetworkPlatformClaimManifestProof {
        manifest_ref: " platform-claim-manifest-ref ".to_owned(),
        ready_claims: entries
            .iter()
            .filter(|entry| entry.claim_state == NetworkPlatformClaimState::Ready)
            .count(),
        dry_run_claims: entries
            .iter()
            .filter(|entry| entry.claim_state == NetworkPlatformClaimState::DryRun)
            .count(),
        research_only_claims: entries
            .iter()
            .filter(|entry| entry.claim_state == NetworkPlatformClaimState::ResearchOnly)
            .count(),
        manual_required_claims: entries
            .iter()
            .filter(|entry| entry.claim_state == NetworkPlatformClaimState::ManualRequired)
            .count(),
        unavailable_claims: entries
            .iter()
            .filter(|entry| entry.claim_state == NetworkPlatformClaimState::Unavailable)
            .count(),
        entries,
        manual_followups,
        every_claim_names_platform: true,
        every_claim_names_permission_or_manual_followup: true,
        every_claim_names_audit_ref: true,
        no_enforcement_commands_published: true,
        no_live_adapter_execution_claimed: true,
        ui_has_no_policy_authority: true,
    }
}

fn entry(
    target: NetworkPlatformClaimTarget,
    claim_state: NetworkPlatformClaimState,
) -> NetworkPlatformClaimEntry {
    NetworkPlatformClaimEntry {
        target,
        claim_state,
        policy_decision_ref: "policy-decision-ref".to_owned(),
        parent_rule_ref: "parent-rule-ref".to_owned(),
        evidence_refs: vec!["network-evidence-ref".to_owned()],
        device_or_os_refs: vec![format!("{target:?}-device-ref")],
        permission_or_entitlement_refs: vec![format!("{target:?}-permission-ref")],
        adapter_capability_refs: vec![format!("{target:?}-capability-ref")],
        missing_required_artifacts: Vec::new(),
        audit_refs: vec![format!("{target:?}-audit-ref")],
        adapter_authorized_by_proof: claim_state == NetworkPlatformClaimState::Ready,
        enforcement_command_published: false,
    }
}

#[derive(Clone, Copy)]
enum MissingArtifactCase {
    WindowsWfpAdministratorPermission,
    LinuxAdapterPermission,
}

fn manual_entry(
    target: NetworkPlatformClaimTarget,
    claim_state: NetworkPlatformClaimState,
    missing_artifact: MissingArtifactCase,
) -> NetworkPlatformClaimEntry {
    let missing_artifact = match missing_artifact {
        MissingArtifactCase::WindowsWfpAdministratorPermission => {
            "windows-wfp.administrator-permission"
        }
        MissingArtifactCase::LinuxAdapterPermission => "linux-adapter.permission",
    };

    NetworkPlatformClaimEntry {
        adapter_capability_refs: Vec::new(),
        permission_or_entitlement_refs: Vec::new(),
        missing_required_artifacts: vec![missing_artifact.to_owned()],
        adapter_authorized_by_proof: false,
        ..entry(target, claim_state)
    }
}

fn followup(
    target: NetworkPlatformClaimTarget,
    missing_artifact: MissingArtifactCase,
) -> NetworkPlatformClaimManualFollowup {
    let missing_artifact = match missing_artifact {
        MissingArtifactCase::WindowsWfpAdministratorPermission => {
            "windows-wfp.administrator-permission"
        }
        MissingArtifactCase::LinuxAdapterPermission => "linux-adapter.permission",
    };

    NetworkPlatformClaimManualFollowup {
        target,
        missing_required_artifacts: vec![missing_artifact.to_owned()],
    }
}

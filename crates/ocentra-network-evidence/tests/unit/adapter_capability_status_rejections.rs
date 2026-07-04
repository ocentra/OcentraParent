use ocentra_network_evidence::adapter_capability_status::*;
use ocentra_network_evidence::platform_claims::*;

#[test]
fn adapter_capability_status_rejects_parallel_or_live_authority_claims() {
    let manifest = manifest();

    assert_eq!(
        build_network_adapter_capability_status(NetworkAdapterCapabilityStatusInput {
            live_adapter_execution_claimed: true,
            ..input_with_manifest(manifest.clone())
        }),
        Err(NetworkAdapterCapabilityStatusError::LiveAdapterExecutionClaimRejected)
    );
    assert_eq!(
        build_network_adapter_capability_status(NetworkAdapterCapabilityStatusInput {
            broader_platform_capability_ux_claimed: true,
            ..input_with_manifest(manifest.clone())
        }),
        Err(NetworkAdapterCapabilityStatusError::BroaderPlatformCapabilityUxClaimRejected)
    );
    assert_eq!(
        build_network_adapter_capability_status(NetworkAdapterCapabilityStatusInput {
            portal_status_proof_ref: None,
            ..input_with_manifest(manifest)
        }),
        Err(NetworkAdapterCapabilityStatusError::MissingPortalStatusProofRef)
    );
}

#[test]
fn adapter_capability_status_rejects_unsafe_platform_manifest_entries() {
    let mut live_claim_manifest = manifest();
    live_claim_manifest.no_live_adapter_execution_claimed = false;

    assert_eq!(
        build_network_adapter_capability_status(input_with_manifest(live_claim_manifest)),
        Err(NetworkAdapterCapabilityStatusError::PlatformManifestClaimsLiveAdapterExecution)
    );

    let mut command_manifest = manifest();
    command_manifest.entries[0].enforcement_command_published = true;

    assert_eq!(
        build_network_adapter_capability_status(input_with_manifest(command_manifest)),
        Err(
            NetworkAdapterCapabilityStatusError::PlatformEntryPublishedEnforcementCommand(
                NetworkPlatformClaimTarget::WindowsFirewall
            )
        )
    );
}

#[test]
fn adapter_capability_status_rejects_adapter_authorization_on_non_ready_rows() {
    let mut non_ready_manifest = manifest();
    non_ready_manifest.entries[0].claim_state = NetworkPlatformClaimState::DryRun;
    non_ready_manifest.entries[0].adapter_authorized_by_proof = true;
    non_ready_manifest.ready_claims = 0;
    non_ready_manifest.dry_run_claims = 1;

    assert_eq!(
        build_network_adapter_capability_status(input_with_manifest(non_ready_manifest)),
        Err(
            NetworkAdapterCapabilityStatusError::PlatformEntryAuthorizesNonReadyAdapter(
                NetworkPlatformClaimTarget::WindowsFirewall
            )
        )
    );
}

#[test]
fn adapter_capability_status_rejects_stale_platform_manifest_summary() {
    let mut count_mismatch_manifest = manifest();
    count_mismatch_manifest.ready_claims = 0;

    assert_eq!(
        build_network_adapter_capability_status(input_with_manifest(count_mismatch_manifest)),
        Err(NetworkAdapterCapabilityStatusError::PlatformManifestEntryCountsMismatch)
    );

    let mut missing_platform_manifest = manifest();
    missing_platform_manifest.every_claim_names_platform = false;

    assert_eq!(
        build_network_adapter_capability_status(input_with_manifest(missing_platform_manifest)),
        Err(NetworkAdapterCapabilityStatusError::PlatformManifestClaimsMissingPlatformRef)
    );

    let mut missing_permission_manifest = manifest();
    missing_permission_manifest.every_claim_names_permission_or_manual_followup = false;

    assert_eq!(
        build_network_adapter_capability_status(input_with_manifest(missing_permission_manifest)),
        Err(
            NetworkAdapterCapabilityStatusError::PlatformManifestClaimsMissingPermissionOrManualFollowup
        )
    );

    let mut missing_audit_manifest = manifest();
    missing_audit_manifest.every_claim_names_audit_ref = false;

    assert_eq!(
        build_network_adapter_capability_status(input_with_manifest(missing_audit_manifest)),
        Err(NetworkAdapterCapabilityStatusError::PlatformManifestClaimsMissingAuditRef)
    );
}

#[test]
fn adapter_capability_status_rejects_mismatched_manual_followups() {
    let mut missing_followup_manifest = manifest();
    missing_followup_manifest.entries[0].claim_state = NetworkPlatformClaimState::ManualRequired;
    missing_followup_manifest.entries[0]
        .adapter_capability_refs
        .clear();
    missing_followup_manifest.entries[0]
        .missing_required_artifacts
        .push("windows-missing-adapter-proof".to_owned());
    missing_followup_manifest.ready_claims = 0;
    missing_followup_manifest.manual_required_claims = 1;

    assert_eq!(
        build_network_adapter_capability_status(input_with_manifest(missing_followup_manifest)),
        Err(NetworkAdapterCapabilityStatusError::PlatformManifestManualFollowupMismatch)
    );
}

fn input_with_manifest(
    platform_manifest: NetworkPlatformClaimManifestProof,
) -> NetworkAdapterCapabilityStatusInput {
    NetworkAdapterCapabilityStatusInput {
        status_ref: "adapter-capability-status-ref".to_owned(),
        platform_manifest,
        portal_status_proof_ref: Some("portal-status-proof-ref".to_owned()),
        generic_platform_support_claimed: false,
        live_adapter_execution_claimed: false,
        enforcement_command_claimed: false,
        ui_policy_authority_claimed: false,
        broader_platform_capability_ux_claimed: false,
    }
}

fn manifest() -> NetworkPlatformClaimManifestProof {
    NetworkPlatformClaimManifestProof {
        manifest_ref: "platform-claim-manifest-ref".to_owned(),
        ready_claims: 1,
        dry_run_claims: 0,
        research_only_claims: 0,
        manual_required_claims: 0,
        unavailable_claims: 0,
        entries: vec![entry()],
        manual_followups: Vec::new(),
        every_claim_names_platform: true,
        every_claim_names_permission_or_manual_followup: true,
        every_claim_names_audit_ref: true,
        no_enforcement_commands_published: true,
        no_live_adapter_execution_claimed: true,
        ui_has_no_policy_authority: true,
    }
}

fn entry() -> NetworkPlatformClaimEntry {
    NetworkPlatformClaimEntry {
        target: NetworkPlatformClaimTarget::WindowsFirewall,
        claim_state: NetworkPlatformClaimState::Ready,
        policy_decision_ref: "policy-decision-ref".to_owned(),
        parent_rule_ref: "parent-rule-ref".to_owned(),
        evidence_refs: vec!["network-evidence-ref".to_owned()],
        device_or_os_refs: vec!["windows-device-ref".to_owned()],
        permission_or_entitlement_refs: vec!["windows-permission-ref".to_owned()],
        adapter_capability_refs: vec!["windows-capability-ref".to_owned()],
        missing_required_artifacts: Vec::new(),
        audit_refs: vec!["windows-audit-ref".to_owned()],
        adapter_authorized_by_proof: true,
        enforcement_command_published: false,
    }
}

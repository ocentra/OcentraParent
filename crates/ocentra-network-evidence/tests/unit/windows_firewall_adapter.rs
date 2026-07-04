use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::policy::*;
use ocentra_network_evidence::windows_firewall_adapter::*;

#[test]
fn windows_firewall_adapter_allows_apply_ready_with_policy_capability_artifacts_and_audit_refs() {
    let proof = plan_network_windows_firewall_adapter_proof(apply_ready_input())
        .expect_value("complete Windows Firewall adapter proof should become apply-ready");

    assert_eq!(
        proof.proof_state,
        NetworkWindowsFirewallProofState::ApplyReady
    );
    assert_eq!(
        proof.requested_action,
        NetworkWindowsFirewallAdapterAction::BlockOutbound
    );
    assert_eq!(
        proof.target_kind,
        NetworkWindowsFirewallTargetKind::RemoteAddress
    );
    assert_eq!(proof.target_ref, "remote-endpoint-ref-38");
    assert_eq!(proof.firewall_rule_ref, "windows-firewall-rule-ref-38");
    assert_eq!(proof.policy_decision_ref, "policy-decision-network-38");
    assert_eq!(proof.parent_rule_ref, "parent-rule-network-38");
    assert_eq!(proof.evidence_refs, vec!["network-evidence-38"]);
    assert_eq!(proof.boundary_reasons, Vec::new());
    assert_eq!(proof.missing_required_artifacts, Vec::new());
    assert_eq!(
        proof.adapter_authorization_ref,
        Some("adapter-authorization-ref-38".to_owned())
    );
    assert_eq!(
        proof.adapter_capability_proof_ref,
        Some("windows-firewall-capability-proof-ref-38".to_owned())
    );
    assert_eq!(proof.windows_os_scope_ref, "windows-os-scope-ref-38");
    assert_eq!(
        proof.apply_artifact_ref,
        Some("windows-firewall-apply-artifact-ref-38".to_owned())
    );
    assert_eq!(
        proof.result_artifact_ref,
        Some("windows-firewall-result-artifact-ref-38".to_owned())
    );
    assert_eq!(
        proof.rollback_artifact_ref,
        Some("windows-firewall-rollback-artifact-ref-38".to_owned())
    );
    assert_eq!(
        proof.audit_event_ref,
        Some("windows-firewall-audit-event-ref-38".to_owned())
    );
    assert!(proof.adapter_apply_authorized);
    assert!(!proof.enforcement_command_published);
    assert!(!proof.host_firewall_mutation_claimed);
    assert!(!proof.netsh_command_invoked);
    assert!(!proof.powershell_command_invoked);
    assert!(!proof.exact_url_available);
    assert!(!proof.decrypted_payload_available);
    assert!(!proof.page_content_available);
}

#[test]
fn windows_firewall_adapter_dry_run_is_non_executable_without_adapter_artifacts() {
    let proof =
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            dry_run: true,
            adapter_authorization_ref: None,
            adapter_capability_proof_ref: None,
            apply_artifact_ref: None,
            result_artifact_ref: None,
            rollback_artifact_ref: None,
            audit_event_ref: None,
            ..apply_ready_input()
        })
        .expect_value("dry-run Windows Firewall proof should be allowed without apply artifacts");

    assert_eq!(proof.proof_state, NetworkWindowsFirewallProofState::DryRun);
    assert_eq!(
        proof.boundary_reasons,
        vec![
            NetworkWindowsFirewallBoundaryReason::DryRunRequested,
            NetworkWindowsFirewallBoundaryReason::MissingRequiredArtifact
        ]
    );
    assert_eq!(
        proof.missing_required_artifacts,
        vec![
            NetworkWindowsFirewallRequiredArtifact::AdapterAuthorization,
            NetworkWindowsFirewallRequiredArtifact::CapabilityProof,
            NetworkWindowsFirewallRequiredArtifact::ApplyArtifact,
            NetworkWindowsFirewallRequiredArtifact::ResultArtifact,
            NetworkWindowsFirewallRequiredArtifact::RollbackArtifact,
            NetworkWindowsFirewallRequiredArtifact::AuditEvent
        ]
    );
    assert!(!proof.adapter_apply_authorized);
    assert!(!proof.enforcement_command_published);
}

#[test]
fn windows_firewall_adapter_routes_weak_or_parent_review_policy_to_manual_required() {
    let proof =
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            policy_mapping: policy_mapping(
                NetworkEvidenceGrade::B,
                NetworkEvidencePolicyAction::Block,
            ),
            ..apply_ready_input()
        })
        .expect_value("grade B block policy handoff should not become firewall apply-ready");

    assert_eq!(
        proof.proof_state,
        NetworkWindowsFirewallProofState::ManualRequired
    );
    assert_eq!(
        proof.boundary_reasons,
        vec![
            NetworkWindowsFirewallBoundaryReason::EvidenceGradeBelowApplyThreshold,
            NetworkWindowsFirewallBoundaryReason::PolicyNotFirewallApproved
        ]
    );
    assert!(!proof.adapter_apply_authorized);

    let limit =
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            policy_mapping: policy_mapping(
                NetworkEvidenceGrade::A,
                NetworkEvidencePolicyAction::Limit,
            ),
            ..apply_ready_input()
        })
        .expect_value(
            "non-block mapped actions should stay outside the firewall apply-ready boundary",
        );

    assert_eq!(
        limit.proof_state,
        NetworkWindowsFirewallProofState::ManualRequired
    );
    assert_eq!(
        limit.boundary_reasons,
        vec![NetworkWindowsFirewallBoundaryReason::PolicyNotFirewallApproved]
    );
}

#[test]
fn windows_firewall_adapter_marks_capability_manual_required_or_unavailable_without_commands() {
    let manual =
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            capability_state: NetworkWindowsFirewallCapabilityState::ManualRequired,
            ..apply_ready_input()
        })
        .expect_value("manual-required capability should stay reportable");
    assert_eq!(
        manual.proof_state,
        NetworkWindowsFirewallProofState::ManualRequired
    );
    assert_eq!(
        manual.boundary_reasons,
        vec![NetworkWindowsFirewallBoundaryReason::CapabilityManualRequired]
    );
    assert!(!manual.adapter_apply_authorized);

    let unavailable =
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            capability_state: NetworkWindowsFirewallCapabilityState::Unavailable,
            ..apply_ready_input()
        })
        .expect_value("unavailable capability should stay reportable");
    assert_eq!(
        unavailable.proof_state,
        NetworkWindowsFirewallProofState::Unavailable
    );
    assert_eq!(
        unavailable.boundary_reasons,
        vec![NetworkWindowsFirewallBoundaryReason::CapabilityUnavailable]
    );
    assert!(!unavailable.adapter_apply_authorized);
}

#[test]
fn windows_firewall_adapter_requires_authorization_capability_apply_result_rollback_and_audit_refs()
{
    let proof =
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            adapter_authorization_ref: None,
            adapter_capability_proof_ref: None,
            apply_artifact_ref: None,
            result_artifact_ref: None,
            rollback_artifact_ref: None,
            audit_event_ref: None,
            ..apply_ready_input()
        })
        .expect_value("missing Windows Firewall artifacts should produce a manual-required proof");

    assert_eq!(
        proof.proof_state,
        NetworkWindowsFirewallProofState::ManualRequired
    );
    assert_eq!(
        proof.boundary_reasons,
        vec![NetworkWindowsFirewallBoundaryReason::MissingRequiredArtifact]
    );
    assert_eq!(
        proof.missing_required_artifacts,
        vec![
            NetworkWindowsFirewallRequiredArtifact::AdapterAuthorization,
            NetworkWindowsFirewallRequiredArtifact::CapabilityProof,
            NetworkWindowsFirewallRequiredArtifact::ApplyArtifact,
            NetworkWindowsFirewallRequiredArtifact::ResultArtifact,
            NetworkWindowsFirewallRequiredArtifact::RollbackArtifact,
            NetworkWindowsFirewallRequiredArtifact::AuditEvent
        ]
    );
    assert!(!proof.adapter_apply_authorized);
}

#[test]
fn windows_firewall_adapter_rejects_network_only_content_and_live_mutation_claims() {
    assert_eq!(
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            exact_url_claimed: true,
            ..apply_ready_input()
        }),
        Err(NetworkWindowsFirewallAdapterProofError::ExactUrlClaimRejected)
    );
    assert_eq!(
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            decrypted_payload_claimed: true,
            ..apply_ready_input()
        }),
        Err(NetworkWindowsFirewallAdapterProofError::DecryptedPayloadClaimRejected)
    );
    assert_eq!(
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            page_content_claimed: true,
            ..apply_ready_input()
        }),
        Err(NetworkWindowsFirewallAdapterProofError::PageContentClaimRejected)
    );
    assert_eq!(
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            host_firewall_mutation_claimed: true,
            ..apply_ready_input()
        }),
        Err(NetworkWindowsFirewallAdapterProofError::HostFirewallMutationClaimRejected)
    );
}

#[test]
fn windows_firewall_adapter_rejects_command_invocation_and_upstream_authority_bypass() {
    assert_eq!(
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            netsh_command_invoked: true,
            ..apply_ready_input()
        }),
        Err(NetworkWindowsFirewallAdapterProofError::NetshCommandInvocationRejected)
    );
    assert_eq!(
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            powershell_command_invoked: true,
            ..apply_ready_input()
        }),
        Err(NetworkWindowsFirewallAdapterProofError::PowershellCommandInvocationRejected)
    );

    let mut mapping = policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block);
    mapping.enforcement_command_authorized = true;
    assert_eq!(
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            policy_mapping: mapping,
            ..apply_ready_input()
        }),
        Err(NetworkWindowsFirewallAdapterProofError::PolicyMappingAuthorityRejected)
    );
}

#[test]
fn windows_firewall_adapter_rejects_empty_target_rule_or_artifact_refs() {
    assert_eq!(
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            target_ref: " ".to_owned(),
            ..apply_ready_input()
        }),
        Err(NetworkWindowsFirewallAdapterProofError::EmptyTargetRef)
    );
    assert_eq!(
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            firewall_rule_ref: " ".to_owned(),
            ..apply_ready_input()
        }),
        Err(NetworkWindowsFirewallAdapterProofError::EmptyFirewallRuleRef)
    );
    assert_eq!(
        plan_network_windows_firewall_adapter_proof(NetworkWindowsFirewallAdapterProofInput {
            apply_artifact_ref: Some(" ".to_owned()),
            ..apply_ready_input()
        }),
        Err(
            NetworkWindowsFirewallAdapterProofError::EmptyRequiredArtifactRef(
                NetworkWindowsFirewallRequiredArtifact::ApplyArtifact
            )
        )
    );
}

fn apply_ready_input() -> NetworkWindowsFirewallAdapterProofInput {
    NetworkWindowsFirewallAdapterProofInput {
        firewall_adapter_plan_ref: " windows-firewall-adapter-plan-38 ".to_owned(),
        policy_mapping: policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block),
        requested_action: NetworkWindowsFirewallAdapterAction::BlockOutbound,
        windows_os_scope_ref: " windows-os-scope-ref-38 ".to_owned(),
        target_kind: NetworkWindowsFirewallTargetKind::RemoteAddress,
        target_ref: " remote-endpoint-ref-38 ".to_owned(),
        firewall_rule_ref: " windows-firewall-rule-ref-38 ".to_owned(),
        capability_state: NetworkWindowsFirewallCapabilityState::Supported,
        adapter_authorization_ref: Some(" adapter-authorization-ref-38 ".to_owned()),
        adapter_capability_proof_ref: Some(" windows-firewall-capability-proof-ref-38 ".to_owned()),
        apply_artifact_ref: Some(" windows-firewall-apply-artifact-ref-38 ".to_owned()),
        result_artifact_ref: Some(" windows-firewall-result-artifact-ref-38 ".to_owned()),
        rollback_artifact_ref: Some(" windows-firewall-rollback-artifact-ref-38 ".to_owned()),
        audit_event_ref: Some(" windows-firewall-audit-event-ref-38 ".to_owned()),
        dry_run: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        host_firewall_mutation_claimed: false,
        netsh_command_invoked: false,
        powershell_command_invoked: false,
    }
}

fn policy_mapping(
    evidence_grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
) -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: " policy-decision-network-38 ".to_owned(),
        parent_rule_ref: " parent-rule-network-38 ".to_owned(),
        evidence_refs: vec![
            " network-evidence-38 ".to_owned(),
            "network-evidence-38".to_owned(),
        ],
        local_ai_result_ref: Some(" local-ai-result-ref-38 ".to_owned()),
        evidence_grade,
        requested_action,
        adapter_capability_proof_ref: None,
    })
    .expect_value("policy mapping input should be valid")
}
